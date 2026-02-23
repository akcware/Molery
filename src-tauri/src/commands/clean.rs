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
    let mo = super::find_mo_binary().ok_or("mo CLI not found")?;

    let output = Command::new(mo)
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

    let mo = super::find_mo_binary().ok_or("mo CLI not found")?;

    let output = Command::new(mo)
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

    // Strip ANSI color codes for easier parsing
    let clean_output = strip_ansi_codes(output);
    let mut current_section = String::new();

    for line in clean_output.lines() {
        let trimmed = line.trim();

        // Detect section headers (e.g., "➤ User essentials" or "➤ Developer tools")
        if trimmed.starts_with('➤') || trimmed.starts_with("➤") {
            current_section = trimmed
                .trim_start_matches('➤')
                .trim_start_matches("➤")
                .trim()
                .to_lowercase();
            continue;
        }

        // Parse items with sizes (e.g., "→ User app cache 110 items (249.6MB dry)")
        if trimmed.starts_with('→') || trimmed.starts_with("→") {
            if let Some(item) = parse_item_line(trimmed, &current_section) {
                // Categorize based on section and description
                let category = categorize_item(&current_section, &item.description);
                match category.as_str() {
                    "cache" => categories.cache += item.size,
                    "logs" => categories.logs += item.size,
                    "trash" => categories.trash += item.size,
                    "downloads" => categories.downloads += item.size,
                    "xcode" => categories.xcode += item.size,
                    "homebrew" => categories.homebrew += item.size,
                    _ => categories.cache += item.size, // Default to cache
                }
                items.push(item);
            }
        }
    }

    // Also try to extract total from summary line
    // Format: "Potential space: 0.39GB | Items: 40 | Categories: 14"
    let total_size = if let Some(summary_total) = extract_summary_total(&clean_output) {
        summary_total
    } else {
        items.iter().map(|i| i.size).sum()
    };

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

/// Strip ANSI color/escape codes from a string
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we hit a letter (end of escape sequence)
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse a line like "→ User app cache 150 items, 4.50GB dry" or "→ Bun cache 3344 items, 1.69GB dry"
fn parse_item_line(line: &str, section: &str) -> Option<CleanupItem> {
    let trimmed = line
        .trim()
        .trim_start_matches('→')
        .trim_start_matches("→")
        .trim();

    // Try to extract size from comma-separated format: "description, SIZEdry" or "description, SIZE dry"
    // Also try parenthesized format as fallback: "description (SIZE dry)"
    let mut size = 0u64;
    let mut description = trimmed.to_string();

    // First try: comma-separated format (e.g., "User app cache 150 items, 4.50GB dry")
    if let Some(last_comma) = trimmed.rfind(',') {
        let after_comma = trimmed[last_comma + 1..].trim();
        let size_candidate = after_comma
            .replace(" dry", "")
            .replace("dry", "")
            .trim()
            .to_string();
        if let Some(parsed) = parse_size(&size_candidate) {
            size = parsed;
            description = trimmed[..last_comma].trim().to_string();
        }
    }

    // Fallback: parenthesized format (e.g., "description (249.6MB dry)")
    if size == 0 {
        if let Some(start) = trimmed.rfind('(') {
            if let Some(end) = trimmed.rfind(')') {
                let size_part = &trimmed[start + 1..end];
                let size_str = size_part.replace(" dry", "").replace("dry", "");
                if let Some(parsed) = parse_size(size_str.trim()) {
                    size = parsed;
                    description = trimmed[..start].trim().to_string();
                }
            }
        }
    }

    // Skip items with no size (they typically say "would clean" without a size)
    if size == 0 {
        return None;
    }

    Some(CleanupItem {
        path: String::new(), // mo doesn't provide paths in this format
        size,
        category: categorize_item(section, &description),
        description,
    })
}

/// Categorize an item based on section name and description
fn categorize_item(section: &str, description: &str) -> String {
    let section_lower = section.to_lowercase();
    let desc_lower = description.to_lowercase();

    if desc_lower.contains("log") {
        "logs".to_string()
    } else if desc_lower.contains("homebrew") || section_lower.contains("homebrew") {
        "homebrew".to_string()
    } else if desc_lower.contains("xcode") || section_lower.contains("xcode") {
        "xcode".to_string()
    } else if desc_lower.contains("download") {
        "downloads".to_string()
    } else if desc_lower.contains("trash") {
        "trash".to_string()
    } else {
        "cache".to_string()
    }
}

/// Extract total size from summary line like "Potential space: 0.39GB | Items: 40 | Categories: 14"
fn extract_summary_total(output: &str) -> Option<u64> {
    for line in output.lines() {
        if line.contains("Potential space:") {
            // Find the size after "Potential space:"
            if let Some(start) = line.find("Potential space:") {
                let after = &line[start + "Potential space:".len()..];
                // Get the first word which should be the size
                let size_str = after.trim().split('|').next()?.trim();
                return parse_size(size_str);
            }
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
