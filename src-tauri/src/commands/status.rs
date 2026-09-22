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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacOsInfo {
    pub version: String,
    pub build_version: String,
    pub computer_name: String,
    pub model_name: String,
    pub model_identifier: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub wired: u64,
    pub active: u64,
    pub inactive: u64,
    pub compressed: u64,
    pub pressure_level: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub model: String,
    pub core_count: u32,
    pub performance_cores: u32,
    pub efficiency_cores: u32,
    pub usage_percent: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedSystemStatus {
    pub disk_total: u64,
    pub disk_used: u64,
    pub disk_available: u64,
    pub cleanable_size: u64,
    pub last_clean: Option<String>,
    pub mo_version: String,
    pub macos: MacOsInfo,
    pub memory: MemoryInfo,
    pub cpu: CpuInfo,
}

#[tauri::command]
pub async fn check_mo_installed() -> Result<bool, String> {
    Ok(super::find_mo_binary().is_some())
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
    let mo = super::find_mo_binary().ok_or("mo not found")?;

    let output = Command::new(mo)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to get mo version: {}", e))?;

    if !output.status.success() {
        return Err("mo command failed".to_string());
    }

    // `mo --version` prints a multi-line report (version, macOS, kernel, SIP, ...).
    // The UI renders this inline, so keep just the version number.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = stdout
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("Mole version ")
                .map(|v| v.trim().to_string())
        })
        .unwrap_or_else(|| stdout.trim().lines().next().unwrap_or("").trim().to_string());

    if version.is_empty() {
        return Err("could not parse mo version".to_string());
    }

    Ok(version)
}

fn get_macos_info() -> MacOsInfo {
    let version = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let build_version = Command::new("sw_vers")
        .arg("-buildVersion")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let computer_name = Command::new("scutil")
        .arg("--get")
        .arg("ComputerName")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let model_identifier = Command::new("sysctl")
        .args(["-n", "hw.model"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    // Get model name from system_profiler (slower but gives human-readable name)
    let model_name = Command::new("system_profiler")
        .args(["SPHardwareDataType", "-json"])
        .output()
        .ok()
        .and_then(|o| {
            let json_str = String::from_utf8_lossy(&o.stdout);
            // Parse JSON to extract model name
            if let Some(start) = json_str.find("\"machine_name\"") {
                let after_key = &json_str[start + 16..];
                if let Some(quote_start) = after_key.find('"') {
                    let value_start = &after_key[quote_start + 1..];
                    if let Some(quote_end) = value_start.find('"') {
                        return Some(value_start[..quote_end].to_string());
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| model_identifier.clone());

    MacOsInfo {
        version,
        build_version,
        computer_name,
        model_name,
        model_identifier,
    }
}

fn get_memory_info() -> MemoryInfo {
    // Get total memory
    let total = Command::new("sysctl")
        .args(["-n", "hw.memsize"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u64>()
                .ok()
        })
        .unwrap_or(0);

    // Get page size
    let page_size = Command::new("sysctl")
        .args(["-n", "hw.pagesize"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u64>()
                .ok()
        })
        .unwrap_or(4096);

    // Parse vm_stat output
    let vm_stat = Command::new("vm_stat")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let parse_pages = |key: &str| -> u64 {
        vm_stat
            .lines()
            .find(|line| line.contains(key))
            .and_then(|line| {
                line.split(':')
                    .nth(1)
                    .map(|v| v.trim().trim_end_matches('.').parse::<u64>().unwrap_or(0))
            })
            .unwrap_or(0)
    };

    let pages_free = parse_pages("Pages free");
    let pages_active = parse_pages("Pages active");
    let pages_inactive = parse_pages("Pages inactive");
    let pages_wired = parse_pages("Pages wired down");
    let pages_compressed = parse_pages("Pages occupied by compressor");

    let free = pages_free * page_size;
    let active = pages_active * page_size;
    let inactive = pages_inactive * page_size;
    let wired = pages_wired * page_size;
    let compressed = pages_compressed * page_size;

    // Available = free + inactive (inactive can be reclaimed)
    let available = free + inactive;
    let used = total.saturating_sub(available);

    // Calculate pressure level based on available percentage
    let available_percent = if total > 0 {
        (available as f64 / total as f64) * 100.0
    } else {
        100.0
    };

    let pressure_level = if available_percent > 50.0 {
        "normal".to_string()
    } else if available_percent > 25.0 {
        "warn".to_string()
    } else {
        "critical".to_string()
    };

    MemoryInfo {
        total,
        used,
        available,
        wired,
        active,
        inactive,
        compressed,
        pressure_level,
    }
}

fn get_cpu_info() -> CpuInfo {
    let model = Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let core_count = Command::new("sysctl")
        .args(["-n", "hw.ncpu"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u32>()
                .ok()
        })
        .unwrap_or(0);

    // Performance cores (Apple Silicon only, returns error on Intel)
    let performance_cores = Command::new("sysctl")
        .args(["-n", "hw.perflevel0.physicalcpu"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u32>()
                .ok()
        })
        .unwrap_or(0);

    // Efficiency cores (Apple Silicon only)
    let efficiency_cores = Command::new("sysctl")
        .args(["-n", "hw.perflevel1.physicalcpu"])
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u32>()
                .ok()
        })
        .unwrap_or(0);

    // Get CPU usage from top
    let usage_percent = Command::new("top")
        .args(["-l", "1", "-n", "0", "-s", "0"])
        .output()
        .ok()
        .and_then(|o| {
            let output = String::from_utf8_lossy(&o.stdout);
            // Look for line like "CPU usage: 5.26% user, 10.52% sys, 84.21% idle"
            output.lines().find(|line| line.contains("CPU usage")).and_then(|line| {
                // Extract user and sys percentages
                let parts: Vec<&str> = line.split(',').collect();
                let user = parts.first().and_then(|p| {
                    p.split_whitespace()
                        .find(|s| s.ends_with('%'))
                        .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                });
                let sys = parts.get(1).and_then(|p| {
                    p.split_whitespace()
                        .find(|s| s.ends_with('%'))
                        .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                });
                match (user, sys) {
                    (Some(u), Some(s)) => Some(u + s),
                    _ => None,
                }
            })
        })
        .unwrap_or(0.0);

    CpuInfo {
        model,
        core_count,
        performance_cores,
        efficiency_cores,
        usage_percent,
    }
}

#[tauri::command]
pub async fn get_extended_status() -> Result<ExtendedSystemStatus, String> {
    // Get disk space using df command
    let df_output = Command::new("df")
        .args(["-k", "/"])
        .output()
        .map_err(|e| format!("Failed to get disk info: {}", e))?;

    let df_str = String::from_utf8_lossy(&df_output.stdout);
    let (disk_total, disk_used, disk_available) = parse_df_output(&df_str)?;

    // Check mo version
    let mo_version = get_mo_version().unwrap_or_else(|_| "Not installed".to_string());

    // Get extended info
    let macos = get_macos_info();
    let memory = get_memory_info();
    let cpu = get_cpu_info();

    Ok(ExtendedSystemStatus {
        disk_total,
        disk_used,
        disk_available,
        cleanable_size: 0,
        last_clean: None,
        mo_version,
        macos,
        memory,
        cpu,
    })
}
