use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskItem {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_directory: bool,
    pub children: Option<Vec<DiskItem>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeResult {
    pub path: String,
    pub total_size: u64,
    pub items: Vec<DiskItem>,
}

#[tauri::command]
pub async fn analyze_path(path: String) -> Result<AnalyzeResult, String> {
    let path_obj = Path::new(&path);

    if !path_obj.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    if !path_obj.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let items = analyze_directory(path_obj, 1)?;
    let total_size = items.iter().map(|i| i.size).sum();

    Ok(AnalyzeResult {
        path,
        total_size,
        items,
    })
}

fn analyze_directory(path: &Path, depth: u32) -> Result<Vec<DiskItem>, String> {
    let entries = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut items: Vec<DiskItem> = Vec::new();

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let metadata = entry.metadata().ok();

        let name = entry_path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        // Skip hidden files at top level
        if name.starts_with('.') && depth == 1 {
            continue;
        }

        let is_directory = metadata.as_ref().map_or(false, |m| m.is_dir());
        let size = if is_directory {
            get_dir_size_fast(&entry_path)
        } else {
            metadata.map_or(0, |m| m.len())
        };

        let children = if is_directory && depth < 2 {
            analyze_directory(&entry_path, depth + 1).ok()
        } else {
            None
        };

        items.push(DiskItem {
            name,
            path: entry_path.to_string_lossy().to_string(),
            size,
            is_directory,
            children,
        });
    }

    items.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(items)
}

fn get_dir_size_fast(path: &Path) -> u64 {
    // Use du command for faster calculation
    let output = std::process::Command::new("du")
        .args(["-sk", &path.to_string_lossy()])
        .output()
        .ok();

    output.and_then(|o| {
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout.split_whitespace().next()?.parse::<u64>().ok()
    }).map(|kb| kb * 1024).unwrap_or(0)
}
