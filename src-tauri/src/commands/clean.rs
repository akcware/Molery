use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupItem {
    pub path: String,
    pub size: u64,
    pub category: String,
    pub description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySummary {
    pub cache: u64,
    pub logs: u64,
    pub trash: u64,
    pub downloads: u64,
    pub xcode: u64,
    pub homebrew: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub items: Vec<CleanupItem>,
    pub total_size: u64,
    pub categories: CategorySummary,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupResult {
    pub freed_size: u64,
    pub items_removed: u32,
    pub errors: Vec<String>,
}

#[tauri::command]
pub async fn scan_cleanup() -> Result<ScanResult, String> {
    let output = Command::new("mo")
        .args(["clean", "--dry-run"])
        .output()
        .map_err(|e| format!("Failed to execute mo: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mo clean failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_scan_output(&stdout)
}

#[tauri::command]
pub async fn run_cleanup(categories: Vec<String>) -> Result<CleanupResult, String> {
    let mut args = vec!["clean".to_string()];

    for cat in &categories {
        args.push(format!("--{}", cat));
    }

    let output = Command::new("mo")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute mo: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mo clean failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_cleanup_output(&stdout)
}

fn parse_scan_output(output: &str) -> Result<ScanResult, String> {
    let mut items = Vec::new();
    let mut categories = CategorySummary {
        cache: 0,
        logs: 0,
        trash: 0,
        downloads: 0,
        xcode: 0,
        homebrew: 0,
    };

    for line in output.lines() {
        if let Some((category, size, path)) = parse_line(line) {
            let item = CleanupItem {
                path: path.clone(),
                size,
                category: category.clone(),
                description: format!("{} file", category),
            };

            match category.as_str() {
                "cache" => categories.cache += size,
                "logs" => categories.logs += size,
                "trash" => categories.trash += size,
                "downloads" => categories.downloads += size,
                "xcode" => categories.xcode += size,
                "homebrew" => categories.homebrew += size,
                _ => {}
            }

            items.push(item);
        }
    }

    let total_size = items.iter().map(|i| i.size).sum();

    Ok(ScanResult {
        items,
        total_size,
        categories,
    })
}

fn parse_cleanup_output(output: &str) -> Result<CleanupResult, String> {
    // Parse actual cleanup output
    // Extract freed size from output if available
    let mut freed_size = 0u64;
    let mut items_removed = 0u32;

    for line in output.lines() {
        // Look for patterns like "Freed 1.2 GB" or "Removed 5 items"
        if let Some(size) = extract_freed_size(line) {
            freed_size = size;
        }
        if let Some(count) = extract_items_count(line) {
            items_removed = count;
        }
    }

    Ok(CleanupResult {
        freed_size,
        items_removed,
        errors: vec![],
    })
}

fn extract_freed_size(line: &str) -> Option<u64> {
    let line_lower = line.to_lowercase();
    if line_lower.contains("freed") || line_lower.contains("cleaned") {
        // Try to find a size pattern in the line
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if let Some(size) = parse_size(word) {
                return Some(size);
            }
            // Check if next word is a unit
            if i + 1 < words.len() {
                let combined = format!("{}{}", word, words[i + 1]);
                if let Some(size) = parse_size(&combined) {
                    return Some(size);
                }
            }
        }
    }
    None
}

fn extract_items_count(line: &str) -> Option<u32> {
    let line_lower = line.to_lowercase();
    if line_lower.contains("removed") || line_lower.contains("deleted") {
        for word in line.split_whitespace() {
            if let Ok(count) = word.parse::<u32>() {
                return Some(count);
            }
        }
    }
    None
}

fn parse_line(line: &str) -> Option<(String, u64, String)> {
    // Parse a single line of mo output
    // Format depends on mo CLI output - adjust accordingly
    // Example: "cache  1.2GB  /path/to/cache"

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        let category = parts[0].to_lowercase();
        // Only process valid categories
        if matches!(
            category.as_str(),
            "cache" | "logs" | "trash" | "downloads" | "xcode" | "homebrew"
        ) {
            let size = parse_size(parts[1]).unwrap_or(0);
            let path = parts[2..].join(" ");
            return Some((category, size, path));
        }
    }
    None
}

fn parse_size(size_str: &str) -> Option<u64> {
    let size_str = size_str.to_uppercase();
    let multiplier = if size_str.ends_with("GB") {
        1024 * 1024 * 1024
    } else if size_str.ends_with("MB") {
        1024 * 1024
    } else if size_str.ends_with("KB") {
        1024
    } else if size_str.ends_with('B') {
        1
    } else {
        return None;
    };

    let num_str: String = size_str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let num: f64 = num_str.parse().ok()?;

    Some((num * multiplier as f64) as u64)
}
