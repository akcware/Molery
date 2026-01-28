use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub bundle_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UninstallResult {
    pub removed_files: Vec<String>,
    pub freed_size: u64,
    pub success: bool,
}

#[tauri::command]
pub async fn list_apps() -> Result<Vec<AppInfo>, String> {
    let apps_dir = Path::new("/Applications");
    let mut apps = Vec::new();

    let entries = fs::read_dir(apps_dir)
        .map_err(|e| format!("Failed to read Applications: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "app") {
            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let size = get_dir_size(&path).unwrap_or(0);
            let bundle_id = get_bundle_id(&path);

            apps.push(AppInfo {
                name,
                path: path.to_string_lossy().to_string(),
                size,
                bundle_id,
            });
        }
    }

    apps.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(apps)
}

#[tauri::command]
pub async fn uninstall_app(app_name: String) -> Result<UninstallResult, String> {
    let output = Command::new("mo")
        .args(["uninstall", &app_name])
        .output()
        .map_err(|e| format!("Failed to execute mo: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Uninstall failed: {}", stderr));
    }

    // Parse output for removed files and size
    Ok(UninstallResult {
        removed_files: vec![],
        freed_size: 0,
        success: true,
    })
}

fn get_dir_size(path: &Path) -> Result<u64, std::io::Error> {
    let mut size = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            size += get_dir_size(&entry.path()).unwrap_or(0);
        } else {
            size += metadata.len();
        }
    }
    Ok(size)
}

fn get_bundle_id(app_path: &Path) -> Option<String> {
    let plist_path = app_path.join("Contents/Info.plist");
    if !plist_path.exists() {
        return None;
    }

    let output = Command::new("/usr/libexec/PlistBuddy")
        .args(["-c", "Print :CFBundleIdentifier", &plist_path.to_string_lossy()])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
