use serde::Serialize;
use std::env;
use std::process::Command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    pub disk_total: u64,
    pub disk_used: u64,
    pub disk_available: u64,
    pub cleanable_size: u64,
    pub last_clean: Option<String>,
    pub mo_version: String,
}

#[tauri::command]
pub async fn check_mo_installed() -> Result<bool, String> {
    let output = Command::new("which")
        .arg("mo")
        .output()
        .map_err(|e| format!("Failed to check mo: {}", e))?;

    Ok(output.status.success())
}

#[tauri::command]
pub async fn get_home_dir() -> Result<String, String> {
    env::var("HOME").map_err(|_| "Could not determine home directory".to_string())
}

#[tauri::command]
pub async fn get_status() -> Result<SystemStatus, String> {
    // Get disk space using df command
    let df_output = Command::new("df")
        .args(["-k", "/"])
        .output()
        .map_err(|e| format!("Failed to get disk info: {}", e))?;

    let df_str = String::from_utf8_lossy(&df_output.stdout);
    let (disk_total, disk_used, disk_available) = parse_df_output(&df_str)?;

    // Check mo version
    let mo_version = get_mo_version().unwrap_or_else(|_| "Not installed".to_string());

    // Placeholder for cleanable size (will be updated after scan)
    let cleanable_size = 0;

    Ok(SystemStatus {
        disk_total,
        disk_used,
        disk_available,
        cleanable_size,
        last_clean: None,
        mo_version,
    })
}

fn parse_df_output(output: &str) -> Result<(u64, u64, u64), String> {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() < 2 {
        return Err("Invalid df output".to_string());
    }

    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 4 {
        return Err("Invalid df output format".to_string());
    }

    // df -k returns values in KB, convert to bytes
    let total = parts[1].parse::<u64>().map_err(|e| e.to_string())? * 1024;
    let available = parts[3].parse::<u64>().map_err(|e| e.to_string())? * 1024;

    // On macOS APFS, the "Used" column only shows the current volume/snapshot usage,
    // not total disk usage. Calculate actual used space as total - available.
    let used = total.saturating_sub(available);

    Ok((total, used, available))
}

fn get_mo_version() -> Result<String, String> {
    let output = Command::new("mo")
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to get mo version: {}", e))?;

    if !output.status.success() {
        return Err("mo command failed".to_string());
    }

    let version = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    Ok(version)
}
