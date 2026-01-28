---
phase: 5
title: Other Panels
status: completed
depends-on: [phase-4]
---

# Phase 5: Other Panels

## Objective

Implement the remaining feature panels: Uninstall, Analyze, Optimize, and Status.

## Prerequisites

- Phase 4 completed (Clean feature working end-to-end)

## Deliverables

| File | Action | Spec Reference |
|------|--------|----------------|
| `src/lib/types/uninstall.types.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#uninstall-types) |
| `src/lib/types/analyze.types.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#analyze-types) |
| `src/lib/services/uninstallService.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#uninstall-service) |
| `src/lib/services/analyzeService.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#analyze-service) |
| `src/lib/components/features/UninstallPanel.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#uninstall-panel) |
| `src/lib/components/features/AnalyzePanel.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#analyze-panel) |
| `src/lib/components/features/OptimizePanel.svelte` | create | - |
| `src/lib/components/features/StatusPanel.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#status-panel) |
| `src/lib/components/ui/Modal.svelte` | create | - |
| `src/lib/components/ui/ProgressBar.svelte` | create | - |
| `src-tauri/src/commands/uninstall.rs` | create | [rust-commands.md](../docs/specs/rust-commands.md#list_apps) |
| `src-tauri/src/commands/analyze.rs` | create | [rust-commands.md](../docs/specs/rust-commands.md#analyze_path) |
| `src-tauri/src/commands/mod.rs` | modify | Add modules |
| `src-tauri/src/lib.rs` | modify | Register commands |
| `src/routes/+page.svelte` | modify | Add all panels |

## Implementation Steps

### 1. Create Types

**File:** `src/lib/types/uninstall.types.ts`

```typescript
export interface AppInfo {
  name: string;
  path: string;
  size: number;
  bundleId: string | null;
  icon?: string;
}

export interface UninstallResult {
  removedFiles: string[];
  freedSize: number;
  success: boolean;
}
```

**File:** `src/lib/types/analyze.types.ts`

```typescript
export interface DiskItem {
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  children?: DiskItem[];
}

export interface AnalyzeResult {
  path: string;
  totalSize: number;
  items: DiskItem[];
}
```

### 2. Create UI Components

**File:** `src/lib/components/ui/Modal.svelte`

```svelte
<script lang="ts">
  interface Props {
    open: boolean;
    title: string;
    onclose: () => void;
  }

  let { open, title, onclose, children } = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="absolute inset-0 bg-black/50" onclick={onclose}></div>
    <div class="relative bg-surface-elevated rounded-xl shadow-xl max-w-md w-full mx-4">
      <div class="flex items-center justify-between p-4 border-b border-border-secondary">
        <h2 class="text-lg font-semibold text-content-primary">{title}</h2>
        <button class="text-content-secondary hover:text-content-primary" onclick={onclose}>
          ✕
        </button>
      </div>
      <div class="p-4">
        {@render children?.()}
      </div>
    </div>
  </div>
{/if}
```

**File:** `src/lib/components/ui/ProgressBar.svelte`

```svelte
<script lang="ts">
  interface Props {
    value: number;
    max?: number;
    showLabel?: boolean;
  }

  let { value, max = 100, showLabel = false }: Props = $props();

  let percentage = $derived(Math.min(100, (value / max) * 100));
</script>

<div class="space-y-1">
  <div class="h-2 bg-surface-tertiary rounded-full overflow-hidden">
    <div
      class="h-full bg-accent-blue transition-all duration-300"
      style="width: {percentage}%"
    ></div>
  </div>
  {#if showLabel}
    <p class="text-xs text-content-secondary text-right">{percentage.toFixed(1)}%</p>
  {/if}
</div>
```

### 3. Create Rust Commands

**File:** `src-tauri/src/commands/uninstall.rs`

```rust
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
```

**File:** `src-tauri/src/commands/analyze.rs`

```rust
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
```

### 4. Update mod.rs

**File:** `src-tauri/src/commands/mod.rs`

```rust
pub mod analyze;
pub mod clean;
pub mod status;
pub mod uninstall;
```

### 5. Update lib.rs

**File:** `src-tauri/src/lib.rs`

```rust
mod commands;

use commands::{analyze, clean, status, uninstall};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            status::check_mo_installed,
            status::get_status,
            clean::scan_cleanup,
            clean::run_cleanup,
            uninstall::list_apps,
            uninstall::uninstall_app,
            analyze::analyze_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 6. Create Services

**File:** `src/lib/services/uninstallService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { AppInfo, UninstallResult } from '$lib/types/uninstall.types';

export async function listApps(): Promise<AppInfo[]> {
  return invoke<AppInfo[]>('list_apps');
}

export async function uninstallApp(appName: string): Promise<UninstallResult> {
  return invoke<UninstallResult>('uninstall_app', { appName });
}
```

**File:** `src/lib/services/analyzeService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { AnalyzeResult } from '$lib/types/analyze.types';

export async function analyzePath(path: string): Promise<AnalyzeResult> {
  return invoke<AnalyzeResult>('analyze_path', { path });
}
```

### 7. Create Panel Components

**File:** `src/lib/components/features/UninstallPanel.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import { listApps, uninstallApp } from '$lib/services/uninstallService';
  import { toastStore } from '$lib/stores/toast.store';
  import { formatBytes } from '$lib/utils/format';
  import type { AppInfo } from '$lib/types/uninstall.types';

  let apps = $state<AppInfo[]>([]);
  let loading = $state(true);
  let searchQuery = $state('');
  let selectedApp = $state<AppInfo | null>(null);
  let uninstalling = $state(false);

  let filteredApps = $derived(
    apps.filter(app =>
      app.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  onMount(async () => {
    try {
      apps = await listApps();
    } catch (error) {
      toastStore.error('Failed to load apps');
    } finally {
      loading = false;
    }
  });

  async function handleUninstall() {
    if (!selectedApp) return;

    uninstalling = true;
    try {
      const result = await uninstallApp(selectedApp.name);
      if (result.success) {
        toastStore.success(`Uninstalled ${selectedApp.name}`);
        apps = apps.filter(a => a.name !== selectedApp.name);
      }
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Uninstall failed');
    } finally {
      uninstalling = false;
      selectedApp = null;
    }
  }
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">App Uninstaller</h1>
    <p class="text-content-secondary">Completely remove applications and their files</p>
  </div>

  <input
    type="text"
    placeholder="Search apps..."
    bind:value={searchQuery}
    class="w-full px-4 py-2 bg-surface-secondary border border-border-primary rounded-lg text-content-primary placeholder:text-content-tertiary"
  />

  {#if loading}
    <div class="text-content-secondary">Loading apps...</div>
  {:else}
    <p class="text-sm text-content-secondary">Installed Applications ({filteredApps.length})</p>

    <Card padding="none">
      <div class="max-h-96 overflow-auto divide-y divide-border-secondary">
        {#each filteredApps as app}
          <div class="flex items-center gap-3 px-4 py-3 hover:bg-surface-tertiary">
            <span class="text-2xl">📦</span>
            <div class="flex-1 min-w-0">
              <p class="font-medium text-content-primary truncate">{app.name}</p>
              <p class="text-xs text-content-tertiary truncate">{app.path}</p>
            </div>
            <span class="text-sm text-content-secondary">{formatBytes(app.size)}</span>
            <Button variant="ghost" size="sm" onclick={() => selectedApp = app}>
              🗑️
            </Button>
          </div>
        {/each}
      </div>
    </Card>
  {/if}
</div>

<Modal open={!!selectedApp} title="Uninstall {selectedApp?.name}?" onclose={() => selectedApp = null}>
  <div class="space-y-4">
    <p class="text-content-secondary">This will remove:</p>
    <ul class="text-sm text-content-secondary list-disc list-inside">
      <li>Application ({formatBytes(selectedApp?.size ?? 0)})</li>
      <li>Preferences</li>
      <li>Cache files</li>
      <li>Support files</li>
    </ul>
    <div class="flex justify-end gap-2">
      <Button variant="secondary" onclick={() => selectedApp = null}>Cancel</Button>
      <Button onclick={handleUninstall} disabled={uninstalling}>
        {uninstalling ? 'Uninstalling...' : 'Uninstall'}
      </Button>
    </div>
  </div>
</Modal>
```

**File:** `src/lib/components/features/AnalyzePanel.svelte`

```svelte
<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import { analyzePath } from '$lib/services/analyzeService';
  import { toastStore } from '$lib/stores/toast.store';
  import { formatBytes } from '$lib/utils/format';
  import type { AnalyzeResult, DiskItem } from '$lib/types/analyze.types';

  let path = $state('');
  let result = $state<AnalyzeResult | null>(null);
  let loading = $state(false);

  async function handleAnalyze() {
    if (!path) {
      toastStore.warning('Please enter a path');
      return;
    }

    loading = true;
    try {
      result = await analyzePath(path);
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Analysis failed');
    } finally {
      loading = false;
    }
  }

  function drillDown(item: DiskItem) {
    if (item.isDirectory) {
      path = item.path;
      handleAnalyze();
    }
  }
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">Disk Analyzer</h1>
    <p class="text-content-secondary">Visualize disk usage by folder</p>
  </div>

  <div class="flex gap-2">
    <input
      type="text"
      placeholder="/Users/username"
      bind:value={path}
      class="flex-1 px-4 py-2 bg-surface-secondary border border-border-primary rounded-lg text-content-primary"
    />
    <Button onclick={handleAnalyze} disabled={loading}>
      {loading ? 'Analyzing...' : 'Analyze'}
    </Button>
  </div>

  {#if result}
    <div>
      <p class="text-sm text-content-secondary mb-2">
        {result.path} ({formatBytes(result.totalSize)})
      </p>
      <Card padding="none">
        <div class="divide-y divide-border-secondary">
          {#each result.items.slice(0, 10) as item}
            <button
              class="w-full flex items-center gap-3 px-4 py-3 hover:bg-surface-tertiary text-left"
              onclick={() => drillDown(item)}
            >
              <span>{item.isDirectory ? '📁' : '📄'}</span>
              <div class="flex-1 min-w-0">
                <p class="text-content-primary truncate">{item.name}</p>
                <ProgressBar value={item.size} max={result.totalSize} />
              </div>
              <span class="text-sm text-content-secondary whitespace-nowrap">
                {formatBytes(item.size)}
              </span>
            </button>
          {/each}
        </div>
      </Card>
    </div>

    <p class="text-xs text-content-tertiary">Click on a folder to drill down</p>
  {/if}
</div>
```

**File:** `src/lib/components/features/OptimizePanel.svelte`

```svelte
<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Optimize</h1>
    <p class="text-content-secondary">Optimize system performance</p>
  </div>

  <Card>
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <p class="font-medium text-content-primary">Clear DNS Cache</p>
          <p class="text-sm text-content-secondary">Flush the DNS resolver cache</p>
        </div>
        <Button variant="secondary" size="sm">Run</Button>
      </div>

      <div class="flex items-center justify-between">
        <div>
          <p class="font-medium text-content-primary">Rebuild Spotlight Index</p>
          <p class="text-sm text-content-secondary">Reindex Spotlight search</p>
        </div>
        <Button variant="secondary" size="sm">Run</Button>
      </div>

      <div class="flex items-center justify-between">
        <div>
          <p class="font-medium text-content-primary">Free Memory</p>
          <p class="text-sm text-content-secondary">Purge inactive memory</p>
        </div>
        <Button variant="secondary" size="sm">Run</Button>
      </div>
    </div>
  </Card>

  <p class="text-xs text-content-tertiary">
    Note: Some operations may require administrator privileges.
  </p>
</div>
```

**File:** `src/lib/components/features/StatusPanel.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import { getStatus, checkMoInstalled } from '$lib/services/statusService';
  import { formatBytes, formatPercentage } from '$lib/utils/format';
  import type { SystemStatus } from '$lib/types/status.types';

  let status = $state<SystemStatus | null>(null);
  let moInstalled = $state(false);
  let loading = $state(true);

  onMount(async () => {
    try {
      [status, moInstalled] = await Promise.all([
        getStatus(),
        checkMoInstalled()
      ]);
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Status</h1>
    <p class="text-content-secondary">Overview of your system health</p>
  </div>

  {#if loading}
    <div class="text-content-secondary">Loading...</div>
  {:else if status}
    <Card>
      <h3 class="font-medium text-content-primary mb-3">Disk Space</h3>
      <div class="space-y-2">
        <ProgressBar value={status.diskUsed} max={status.diskTotal} />
        <div class="flex justify-between text-sm">
          <span class="text-content-secondary">
            {formatBytes(status.diskUsed)} / {formatBytes(status.diskTotal)}
          </span>
          <span class="text-content-secondary">
            {formatPercentage(status.diskUsed, status.diskTotal)} used
          </span>
        </div>
      </div>
    </Card>

    <Card>
      <h3 class="font-medium text-content-primary mb-3">mo CLI Status</h3>
      <div class="flex items-center gap-2">
        <span class="{moInstalled ? 'text-accent-green' : 'text-accent-red'}">
          {moInstalled ? '✓' : '✕'}
        </span>
        <span class="text-content-primary">
          {moInstalled ? 'Installed' : 'Not Installed'}
        </span>
        {#if moInstalled}
          <span class="text-content-secondary text-sm">
            Version {status.moVersion}
          </span>
        {/if}
      </div>
    </Card>

    {#if status.cleanableSize > 0}
      <Card>
        <h3 class="font-medium text-content-primary mb-3">Recommendations</h3>
        <ul class="text-sm text-content-secondary space-y-1 list-disc list-inside">
          <li>Clean {formatBytes(status.cleanableSize)} of temporary files</li>
        </ul>
      </Card>
    {/if}
  {/if}
</div>
```

### 8. Update +page.svelte

```svelte
<script lang="ts">
  import Layout from '$lib/components/layout/Layout.svelte';
  import Dashboard from '$lib/components/features/Dashboard.svelte';
  import CleanPanel from '$lib/components/features/CleanPanel.svelte';
  import UninstallPanel from '$lib/components/features/UninstallPanel.svelte';
  import AnalyzePanel from '$lib/components/features/AnalyzePanel.svelte';
  import OptimizePanel from '$lib/components/features/OptimizePanel.svelte';
  import StatusPanel from '$lib/components/features/StatusPanel.svelte';
  import { uiStore } from '$lib/stores/ui.store';
</script>

<Layout>
  <div class="max-w-4xl">
    {#if uiStore.activePanel === 'dashboard'}
      <Dashboard />
    {:else if uiStore.activePanel === 'clean'}
      <CleanPanel />
    {:else if uiStore.activePanel === 'uninstall'}
      <UninstallPanel />
    {:else if uiStore.activePanel === 'analyze'}
      <AnalyzePanel />
    {:else if uiStore.activePanel === 'optimize'}
      <OptimizePanel />
    {:else if uiStore.activePanel === 'status'}
      <StatusPanel />
    {/if}
  </div>
</Layout>
```

## Checklist

- [x] Uninstall types created
- [x] Analyze types created
- [x] Modal component created
- [x] ProgressBar component created
- [x] Rust `list_apps` command implemented
- [x] Rust `uninstall_app` command implemented
- [x] Rust `analyze_path` command implemented
- [x] Uninstall service created
- [x] Analyze service created
- [x] UninstallPanel with app list and search
- [x] Uninstall confirmation modal
- [x] AnalyzePanel with path input and results
- [x] OptimizePanel with action buttons
- [x] StatusPanel with disk and mo status
- [x] All panels accessible from sidebar

## Verification

```bash
# 1. Run the app
npm run tauri dev

# 2. Test each panel:

# Uninstall:
# - Shows list of installed apps
# - Search filters the list
# - Clicking trash shows confirmation modal

# Analyze:
# - Enter path like /Users/username
# - Shows folder sizes with progress bars
# - Clicking folders drills down

# Optimize:
# - Shows optimization options
# - Buttons are clickable (functionality in Phase 6)

# Status:
# - Shows disk space with progress bar
# - Shows mo CLI status
# - Shows recommendations if cleanable > 0
```

## Notes

- Optimize panel actions are placeholder - implement in Phase 6
- File browser for Analyze can be added in Phase 6
- App icons could be extracted from .app bundles (enhancement)
