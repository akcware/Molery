# Rust Commands Specification

## Overview

All Tauri commands that wrap the `mo` CLI. Commands are defined in `src-tauri/src/commands/`.

## Command Index

| Command | Module | CLI Equivalent | Returns |
|---------|--------|----------------|---------|
| [`scan_cleanup`](#scan_cleanup) | `clean.rs` | `mo clean --dry-run` | `ScanResult` |
| [`run_cleanup`](#run_cleanup) | `clean.rs` | `mo clean` | `CleanupResult` |
| [`get_status`](#get_status) | `status.rs` | `mo` | `SystemStatus` |
| [`list_apps`](#list_apps) | `uninstall.rs` | Internal | `Vec<AppInfo>` |
| [`uninstall_app`](#uninstall_app) | `uninstall.rs` | `mo uninstall <app>` | `UninstallResult` |
| [`analyze_path`](#analyze_path) | `analyze.rs` | `mo analyze <path>` | `AnalyzeResult` |
| [`check_mo_installed`](#check_mo_installed) | `status.rs` | `which mo` | `bool` |

---

## Command Details

### scan_cleanup

Scans system for cleanable items without deleting.

```rust
#[tauri::command]
async fn scan_cleanup() -> Result<ScanResult, String>
```

**Returns:**

```rust
struct ScanResult {
    items: Vec<CleanupItem>,
    total_size: u64,          // bytes
    categories: CategorySummary,
}

struct CleanupItem {
    path: String,
    size: u64,
    category: CleanupCategory,
    description: String,
}

enum CleanupCategory {
    Cache,
    Logs,
    Trash,
    Downloads,
    Xcode,
    Homebrew,
}

struct CategorySummary {
    cache: u64,
    logs: u64,
    trash: u64,
    downloads: u64,
    xcode: u64,
    homebrew: u64,
}
```

**Implementation:**

```rust
use std::process::Command;

#[tauri::command]
async fn scan_cleanup() -> Result<ScanResult, String> {
    let output = Command::new("mo")
        .args(["clean", "--dry-run"])
        .output()
        .map_err(|e| format!("Failed to execute mo: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_scan_output(&stdout)
}
```

**Parsed output format (mo 1.55):**

`mo` has no `--json` for `clean`, so `parse_scan_output` reads the human-readable
report. Sections start with `➤`, items with `→`. Item lines take three shapes:

```
→ User app cache · 68 items, 13.83GB dry        # description · count, size
→ Chrome on-device model cache · 4.27GB dry     # description · size (no count)
→ Chrome Service Worker, would clean 379.9MB, 0 protected
```

`parse_item_line` splits the description on ` · ` (falling back to the first comma
for pre-1.55 output), then scans the remainder right-to-left for the first
size-like token. Lines with no reclaimable size (`→ npm cache · would clean`,
`→ ... · 0B dry`) are skipped.

The grand total comes from the summary line, not the sum of items:

```
Potential space: 62.41GB | Items: 3803 | Categories: 7
```

> **Format changes to watch.** 1.55 introduced the ` · ` separator; parsing 1.55
> output with the pre-1.55 parser silently dropped ~37% of items. Since 1.54 `mo`
> also omits ANSI colour codes when stdout is not a TTY and sends errors to stderr.
> `strip_ansi_codes` is retained for compatibility with older builds.
> Unit tests in `clean.rs` cover all shapes above for both 1.36 and 1.55.

---

### run_cleanup

Runs a full cleanup via `mo clean`.

```rust
#[tauri::command]
async fn run_cleanup() -> Result<CleanupResult, String>
```

**Parameters:** none.

> **No per-category cleanup.** `mo clean` cleans everything it finds and has no
> category flags. It previously took a `categories: Vec<String>` argument and
> passed `--cache`, `--logs`, etc.; mo 1.55 rejects those with
> `Unknown option for mo clean: --cache` (exit 1), while mo 1.36 accepted the flag
> and ran unfiltered. The category breakdown from `scan_cleanup`
> is a preview of what will be removed, not a filter, and the UI says so.

`mo clean` is non-interactive when stdout is not a TTY, so it does not block on a
confirmation prompt. Molery gates it behind its own confirm dialog instead.

**Returns:**

```rust
struct CleanupResult {
    freed_size: u64,
    items_removed: u32,
    errors: Vec<String>,
}
```

---

### get_status

Gets overall system status and statistics.

```rust
#[tauri::command]
async fn get_status() -> Result<SystemStatus, String>
```

**Returns:**

```rust
struct SystemStatus {
    disk_total: u64,
    disk_used: u64,
    disk_available: u64,
    cleanable_size: u64,
    last_clean: Option<String>,  // ISO date
    mo_version: String,
}
```

---

### list_apps

Lists installed applications that can be uninstalled.

```rust
#[tauri::command]
async fn list_apps() -> Result<Vec<AppInfo>, String>
```

**Returns:**

```rust
struct AppInfo {
    name: String,
    path: String,
    size: u64,
    bundle_id: Option<String>,
}
```

---

### uninstall_app

Fully uninstalls an application and its associated files.

> **mo asks for confirmation on stdin.** `mo uninstall` prompts twice
> (`Proceed with uninstallation? [y/N]`, then `Enter confirm, ESC cancel`) and has
> no `--yes` flag. With stdin closed — the default for `Command::output()` — mo
> aborts with exit 1 and an **empty stderr**, so the command must spawn with
> `Stdio::piped()` and write `"y\n\n"`. Verified live with `--dry-run`
> (`cargo test -- --ignored`).

```rust
#[tauri::command]
async fn uninstall_app(app_name: String) -> Result<UninstallResult, String>
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `app_name` | `String` | Name of application to uninstall |

**Returns:**

```rust
struct UninstallResult {
    removed_files: Vec<String>,
    freed_size: u64,
    success: bool,
}
```

---

### analyze_path

Analyzes disk usage for a given path.

```rust
#[tauri::command]
async fn analyze_path(path: String) -> Result<AnalyzeResult, String>
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` | Absolute path to analyze |

**Returns:**

```rust
struct AnalyzeResult {
    path: String,
    total_size: u64,
    items: Vec<DiskItem>,
}

struct DiskItem {
    name: String,
    path: String,
    size: u64,
    is_directory: bool,
    children: Option<Vec<DiskItem>>,
}
```

---

### check_mo_installed

Checks if the `mo` CLI is installed and accessible.

```rust
#[tauri::command]
async fn check_mo_installed() -> Result<bool, String>
```

**Returns:** `true` if `mo` is found in PATH.

---

## Error Handling

All commands return `Result<T, String>` where errors are human-readable messages.

**Common Error Cases:**

| Error | Cause | Frontend Handling |
|-------|-------|-------------------|
| `"mo CLI not found"` | mo not installed | Show setup flow |
| `"Permission denied"` | Insufficient permissions | Show permission dialog |
| `"Operation cancelled"` | User cancelled | Silent dismiss |

---

## Registration

Commands must be registered in `src-tauri/src/lib.rs`:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::clean::scan_cleanup,
            commands::clean::run_cleanup,
            commands::status::get_status,
            commands::status::check_mo_installed,
            commands::uninstall::list_apps,
            commands::uninstall::uninstall_app,
            commands::analyze::analyze_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
