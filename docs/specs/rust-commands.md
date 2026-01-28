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

---

### run_cleanup

Executes cleanup for specified categories.

```rust
#[tauri::command]
async fn run_cleanup(categories: Vec<String>) -> Result<CleanupResult, String>
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `categories` | `Vec<String>` | Categories to clean: `["cache", "logs", "trash"]` |

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
