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

/// Run a full cleanup.
///
/// `mo clean` has no per-category flags — it cleans everything it finds, and
/// rejects unknown options outright (mo 1.55 exits 1 on `--cache`). The category
/// breakdown reported by `scan_cleanup` is therefore a preview of what will be
/// removed, not a filter. The UI states this explicitly.
#[tauri::command]
pub async fn run_cleanup() -> Result<CleanupResult, String> {
    let mo = super::find_mo_binary().ok_or("mo CLI not found")?;

    let output = Command::new(mo)
        .arg("clean")
        .output()
        .map_err(|e| format!("Failed to execute mo: {}", e))?;

    // Since mo 1.54 errors go to stderr; fall back to stdout when it is empty.
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = if stderr.trim().is_empty() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            stderr.trim().to_string()
        };
        return Err(format!("mo clean failed: {}", detail));
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

/// Parse the output of a real `mo clean` run.
///
/// The report uses the same section/item layout as the dry run, so the verified
/// item parser is reused. mo's wording for the summary differs between dry runs
/// ("Potential space:") and real runs, so several labels are accepted and the
/// summed item sizes act as a fallback.
fn parse_cleanup_output(output: &str) -> Result<CleanupResult, String> {
    let clean_output = strip_ansi_codes(output);

    // Only totals are reported back, so item categories are not tracked here.
    let items: Vec<CleanupItem> = clean_output
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('\u{2192}'))
        .filter_map(|line| parse_item_line(line, ""))
        .collect();

    let summed: u64 = items.iter().map(|i| i.size).sum();
    let freed_size = extract_cleanup_total(&clean_output).unwrap_or(summed);

    Ok(CleanupResult {
        freed_size,
        items_removed: items.len() as u32,
        errors: collect_cleanup_errors(&clean_output),
    })
}

/// Find the reclaimed total on a summary line, accepting the several labels mo
/// uses across dry runs and real runs.
fn extract_cleanup_total(output: &str) -> Option<u64> {
    const LABELS: &[&str] = &[
        "Freed space:",
        "Freed:",
        "Reclaimed:",
        "Cleaned:",
        "Space freed:",
        "Potential space:",
    ];

    for line in output.lines() {
        for label in LABELS {
            if let Some(idx) = line.find(label) {
                let after = &line[idx + label.len()..];
                let value = after.split('|').next()?.trim();
                if let Some(size) = parse_size(value) {
                    return Some(size);
                }
            }
        }
    }
    None
}

/// Collect lines mo marks as failures so they surface in the UI instead of
/// being silently reported as a success.
///
/// A successful dry run contains no failure lines, so these markers are matched
/// defensively: the cross marker plus the usual permission wording.
fn collect_cleanup_errors(output: &str) -> Vec<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| {
            let lower = line.to_lowercase();
            lower.starts_with('\u{2717}')
                || lower.contains("permission denied")
                || lower.contains("operation not permitted")
                || lower.starts_with("error:")
        })
        .map(|line| line.trim_start_matches('\u{2717}').trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

/// Strip ANSI color/escape codes from a string
pub(crate) fn strip_ansi_codes(s: &str) -> String {
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

    let mut size = 0u64;

    // Split the human-readable description from the trailing size/status text.
    // mo >= 1.55 uses a middle dot separator ("Chrome cache \u{b7} 3 items, 964.9MB dry");
    // older builds and the "would clean" form use a comma
    // ("Chrome Service Worker, would clean 379.9MB, 0 protected").
    let (desc_part, rest) = match trimmed.find(" \u{b7} ") {
        Some(idx) => (&trimmed[..idx], &trimmed[idx + " \u{b7} ".len()..]),
        None => match trimmed.find(',') {
            Some(idx) => (&trimmed[..idx], &trimmed[idx + 1..]),
            None => (trimmed, ""),
        },
    };

    let description = desc_part.trim().to_string();

    // Scan the trailing text right-to-left for the first token that reads as a size.
    // This covers "3 items, 964.9MB dry", "964.9MB dry" and
    // "would clean 379.9MB, 0 protected" with one code path.
    for token in rest.split_whitespace().rev() {
        let token = token.trim_matches(|c: char| c == ',' || c == '(' || c == ')');
        if let Some(parsed) = parse_size(token) {
            size = parsed;
            break;
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

pub(crate) fn parse_size(size_str: &str) -> Option<u64> {
    let size_str = size_str.to_uppercase();
    let multiplier = if size_str.ends_with("TB") {
        1024_u64 * 1024 * 1024 * 1024
    } else if size_str.ends_with("GB") {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: parse a line and return (description, size).
    fn parse(line: &str) -> Option<(String, u64)> {
        parse_item_line(line, "app caches").map(|i| (i.description, i.size))
    }

    /// Live check against the `mo` binary actually installed on this machine.
    /// Ignored by default: it shells out and takes a few minutes.
    /// Run with: cargo test -- --ignored --nocapture
    #[test]
    #[ignore]
    fn live_scan_against_installed_mo() {
        let mo = crate::commands::find_mo_binary().expect("mo CLI not found");
        let out = std::process::Command::new(mo)
            .args(["clean", "--dry-run"])
            .output()
            .expect("failed to run mo");
        assert!(out.status.success(), "mo clean --dry-run failed");

        let stdout = String::from_utf8_lossy(&out.stdout);
        let result = parse_scan_output(&stdout).expect("parse failed");

        let item_lines = strip_ansi_codes(&stdout)
            .lines()
            .filter(|l| l.trim().starts_with('\u{2192}'))
            .count();

        eprintln!(
            "live: {} item lines, {} parsed, total {} bytes",
            item_lines,
            result.items.len(),
            result.total_size
        );
        for item in result.items.iter().take(5) {
            eprintln!("  [{}] {} = {}", item.category, item.description, item.size);
        }

        assert!(item_lines > 0, "mo produced no item lines");
        assert!(result.total_size > 0, "total size parsed as zero");
        // The 1.55 parser should recover the large majority of item lines.
        assert!(
            result.items.len() * 100 / item_lines >= 80,
            "parsed only {}/{} item lines",
            result.items.len(),
            item_lines
        );
        // Descriptions must not still carry the item count (the 1.55 dot format).
        assert!(
            !result.items.iter().any(|i| i.description.ends_with("items")),
            "description still contains item count"
        );
    }

    #[test]
    fn parses_mo_155_dot_separated_with_item_count() {
        // mo >= 1.55 format
        let (desc, size) = parse("  → User app cache · 68 items, 13.83GB dry").unwrap();
        assert_eq!(desc, "User app cache");
        assert_eq!(size, (13.83 * 1024.0 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn parses_mo_155_dot_separated_without_item_count() {
        // Regression: this shape was dropped entirely before the 1.55 fix
        let (desc, size) = parse("  → Chrome on-device model cache · 4.27GB dry").unwrap();
        assert_eq!(desc, "Chrome on-device model cache");
        assert_eq!(size, (4.27 * 1024.0 * 1024.0 * 1024.0) as u64);

        let (desc, size) = parse("  → Chrome crash reports · 905KB dry").unwrap();
        assert_eq!(desc, "Chrome crash reports");
        assert_eq!(size, 905 * 1024);
    }

    #[test]
    fn parses_would_clean_form() {
        // Regression: size sits mid-line, followed by ", 0 protected"
        let (desc, size) = parse("  → Chrome Service Worker, would clean 379.9MB, 0 protected").unwrap();
        assert_eq!(desc, "Chrome Service Worker");
        assert_eq!(size, (379.9 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn parses_legacy_mo_136_comma_format() {
        // Older builds had no middle dot; must keep working
        let (desc, size) = parse("  → User app cache 201 items, 14.56GB dry").unwrap();
        assert_eq!(desc, "User app cache 201 items");
        assert_eq!(size, (14.56 * 1024.0 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn skips_lines_with_no_reclaimable_size() {
        assert!(parse("  → npm cache · would clean").is_none());
        assert!(parse("  → Wallpaper aerials temp files · 0B dry").is_none());
    }

    #[test]
    fn parse_size_handles_all_units() {
        assert_eq!(parse_size("512B"), Some(512));
        assert_eq!(parse_size("905KB"), Some(905 * 1024));
        assert_eq!(parse_size("1.5MB"), Some((1.5 * 1024.0 * 1024.0) as u64));
        assert_eq!(parse_size("2TB"), Some(2 * 1024 * 1024 * 1024 * 1024));
        assert_eq!(parse_size("dry"), None);
        assert_eq!(parse_size("protected"), None);
    }

    #[test]
    fn cleanup_output_sums_item_lines_and_ignores_headers() {
        let out = "\u{27A4} User essentials\n  \u{2192} User app logs \u{b7} 28 items, 31.3MB dry\n  \u{2192} Chrome cache \u{b7} 964.9MB dry\n  \u{2713} Nothing to clean\n";
        let r = parse_cleanup_output(out).unwrap();
        assert_eq!(r.items_removed, 2);
        assert_eq!(
            r.freed_size,
            (31.3 * 1024.0 * 1024.0) as u64 + (964.9 * 1024.0 * 1024.0) as u64
        );
        assert!(r.errors.is_empty());
    }

    #[test]
    fn cleanup_output_prefers_summary_line_over_item_sum() {
        let out = "\u{27A4} User essentials\n  \u{2192} User app cache \u{b7} 1GB dry\nFreed space: 42.5GB | Items: 10\n";
        let r = parse_cleanup_output(out).unwrap();
        assert_eq!(r.freed_size, (42.5 * 1024.0 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn cleanup_output_surfaces_failures() {
        let out = "  \u{2192} Xcode cache \u{b7} 1MB dry\n  \u{2717} /Library/Caches: permission denied\n";
        let r = parse_cleanup_output(out).unwrap();
        assert_eq!(r.errors.len(), 1);
        assert!(r.errors[0].contains("permission denied"));
    }

    #[test]
    fn extracts_summary_total_from_mo_155() {
        let out = "Potential space: 62.41GB | Items: 3803 | Categories: 7";
        assert_eq!(
            extract_summary_total(out),
            Some((62.41 * 1024.0 * 1024.0 * 1024.0) as u64)
        );
    }

    #[test]
    fn strips_ansi_codes_from_legacy_output() {
        // mo < 1.54 emitted color codes even when redirected
        let raw = "\x1b[0;33m→\x1b[0m Chrome cache 3 items\x1b[0m, \x1b[0;33m964.9MB dry\x1b[0m";
        let clean = strip_ansi_codes(raw);
        assert_eq!(clean, "→ Chrome cache 3 items, 964.9MB dry");
    }
}
