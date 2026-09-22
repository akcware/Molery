use serde::Serialize;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::process::{Command, Stdio};

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

/// Uninstall an app through `mo uninstall`.
///
/// mo asks for confirmation on stdin and has no `--yes` flag, so the answer is
/// piped in. It asks twice: `Proceed with uninstallation? [y/N]` followed by
/// `Enter confirm, ESC cancel`, hence "y\n\n". With stdin closed (the default
/// for `Command::output()`) mo aborts with exit 1 and an empty stderr.
#[tauri::command]
pub async fn uninstall_app(app_name: String) -> Result<UninstallResult, String> {
    let mo = super::find_mo_binary().ok_or("mo CLI not found")?;

    let mut child = Command::new(mo)
        .args(["uninstall", &app_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute mo: {}", e))?;

    child
        .stdin
        .take()
        .ok_or("could not open mo stdin")?
        .write_all(b"y\n\n")
        .map_err(|e| format!("Failed to confirm uninstall: {}", e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to read mo output: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // mo sends errors to stderr since 1.54, but an aborted confirmation
        // leaves it empty — fall back to the last meaningful stdout line.
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = if stderr.trim().is_empty() {
            stdout
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .last()
                .unwrap_or("mo exited without a message")
                .to_string()
        } else {
            stderr.trim().to_string()
        };
        return Err(format!("Uninstall failed: {}", detail));
    }

    let (freed_size, removed_files) = parse_uninstall_output(&stdout);

    Ok(UninstallResult {
        removed_files,
        freed_size,
        success: true,
    })
}

/// Pull the freed size and the removed paths out of mo's uninstall report.
///
/// The summary line reads "Removed 1 app, freed 642.9MB: Obsidian"
/// (dry runs say "Would remove 1 app, would free 642.9MB: Obsidian").
fn parse_uninstall_output(output: &str) -> (u64, Vec<String>) {
    let clean = crate::commands::clean::strip_ansi_codes(output);

    let freed_size = clean
        .lines()
        .filter(|line| line.to_lowercase().contains("free"))
        .find_map(|line| {
            let lower = line.to_lowercase();
            let idx = lower.find("free")?;
            line[idx..]
                .split_whitespace()
                .find_map(|tok| {
                    crate::commands::clean::parse_size(tok.trim_matches(|c: char| {
                        c == ',' || c == ':' || c == '(' || c == ')'
                    }))
                })
        })
        .unwrap_or(0);

    // mo marks each removed path with a check mark.
    let removed_files = clean
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix('\u{2713}'))
        .map(|rest| {
            rest.split(" , ")
                .next()
                .unwrap_or(rest)
                .trim()
                .to_string()
        })
        .filter(|p| !p.is_empty())
        .collect();

    (freed_size, removed_files)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_real_run_summary() {
        let out = "\u{2713} ~/Library/Application Support/Obsidian , 137.5MB\n\
                   \u{2713} ~/Library/Preferences/md.obsidian.plist , 4KB\n\
                   Removed 1 app, freed 642.9MB: Obsidian\n";
        let (freed, files) = parse_uninstall_output(out);
        assert_eq!(freed, (642.9 * 1024.0 * 1024.0) as u64);
        assert_eq!(files.len(), 2);
        assert_eq!(files[0], "~/Library/Application Support/Obsidian");
    }

    #[test]
    fn parses_dry_run_summary() {
        let out = "Would remove 1 app, would free 642.9MB: Obsidian\n";
        let (freed, _) = parse_uninstall_output(out);
        assert_eq!(freed, (642.9 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn missing_summary_yields_zero() {
        let (freed, files) = parse_uninstall_output("nothing useful here\n");
        assert_eq!(freed, 0);
        assert!(files.is_empty());
    }

    /// Live check that the piped confirmation actually gets mo past its two
    /// prompts. Uses --dry-run, so nothing is removed.
    /// Run with: cargo test -- --ignored --nocapture
    #[test]
    #[ignore]
    fn live_dry_run_uninstall_passes_confirmation() {
        let mo = crate::commands::find_mo_binary().expect("mo CLI not found");
        let mut child = Command::new(mo)
            .args(["uninstall", "--dry-run", "Obsidian"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn failed");
        child.stdin.take().unwrap().write_all(b"y\n\n").unwrap();
        let output = child.wait_with_output().expect("wait failed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        eprintln!("exit: {:?}", output.status.code());
        let (freed, files) = parse_uninstall_output(&stdout);
        eprintln!("parsed freed={} files={}", freed, files.len());

        assert!(
            output.status.success(),
            "mo aborted \u{2014} the piped confirmation did not get through"
        );
        assert!(
            stdout.contains("dry run") || stdout.to_lowercase().contains("would"),
            "expected a dry-run report"
        );
    }
}
