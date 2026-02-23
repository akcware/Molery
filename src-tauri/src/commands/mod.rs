pub mod analyze;
pub mod clean;
pub mod optimize;
pub mod status;
pub mod uninstall;

use std::path::PathBuf;
use std::sync::OnceLock;

/// Cached path to the `mo` binary.
static MO_PATH: OnceLock<Option<String>> = OnceLock::new();

/// Common paths where `mo` may be installed.
/// GUI apps on macOS don't inherit the shell PATH, so we check these directly.
const MO_SEARCH_PATHS: &[&str] = &[
    "/opt/homebrew/bin/mo",
    "/usr/local/bin/mo",
    "/usr/bin/mo",
];

/// Find the `mo` binary by checking common installation paths and the user's
/// cargo bin directory. The result is cached for the lifetime of the process.
pub fn find_mo_binary() -> Option<&'static str> {
    MO_PATH
        .get_or_init(|| {
            // Check common system paths
            for path in MO_SEARCH_PATHS {
                if PathBuf::from(path).is_file() {
                    return Some(path.to_string());
                }
            }

            // Check cargo bin (~/.cargo/bin/mo)
            if let Ok(home) = std::env::var("HOME") {
                let cargo_path = format!("{}/.cargo/bin/mo", home);
                if PathBuf::from(&cargo_path).is_file() {
                    return Some(cargo_path);
                }
            }

            // Fallback: try `which mo` (works when launched from terminal)
            if let Ok(output) = std::process::Command::new("which").arg("mo").output() {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path.is_empty() {
                        return Some(path);
                    }
                }
            }

            None
        })
        .as_deref()
}
