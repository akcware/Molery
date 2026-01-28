---
phase: 4
title: Clean Feature
status: done
depends-on: [phase-3]
---

# Phase 4: Clean Feature

## Objective

Implement the complete cleanup feature including scanning for cleanable items and executing cleanup operations.

## Prerequisites

- Phase 3 completed (Dashboard with status commands working)
- `mo` CLI installed on system for testing

## Deliverables

| File | Action | Spec Reference |
|------|--------|----------------|
| `src/lib/types/clean.types.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#clean-types) |
| `src/lib/services/cleanService.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#clean-service) |
| `src/lib/stores/clean.store.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#clean-store) |
| `src/lib/stores/toast.store.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#toast-store) |
| `src/lib/components/ui/Toast.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#toast-notifications) |
| `src/lib/components/ui/ToastContainer.svelte` | create | - |
| `src/lib/components/features/CleanPanel.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#clean-panel) |
| `src-tauri/src/commands/clean.rs` | create | [rust-commands.md](../docs/specs/rust-commands.md#scan_cleanup) |
| `src-tauri/src/commands/mod.rs` | modify | Add clean module |
| `src-tauri/src/lib.rs` | modify | Register clean commands |
| `src/routes/+page.svelte` | modify | Add CleanPanel |
| `src/routes/+layout.svelte` | modify | Add ToastContainer |

## Implementation Steps

### 1. Create Clean Types

**File:** `src/lib/types/clean.types.ts`

```typescript
export type CleanupCategory = 'cache' | 'logs' | 'trash' | 'downloads' | 'xcode' | 'homebrew';

export interface CleanupItem {
  path: string;
  size: number;
  category: CleanupCategory;
  description: string;
}

export interface CategorySummary {
  cache: number;
  logs: number;
  trash: number;
  downloads: number;
  xcode: number;
  homebrew: number;
}

export interface ScanResult {
  items: CleanupItem[];
  totalSize: number;
  categories: CategorySummary;
}

export interface CleanupResult {
  freedSize: number;
  itemsRemoved: number;
  errors: string[];
}
```

### 2. Create Rust Clean Commands

**File:** `src-tauri/src/commands/clean.rs`

```rust
use serde::{Deserialize, Serialize};
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
    // Build command args based on selected categories
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
    // Parse mo clean --dry-run output
    // This is a simplified parser - adjust based on actual mo output format

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
    // Simplified - adjust based on mo output format

    Ok(CleanupResult {
        freed_size: 0, // Parse from output
        items_removed: 0,
        errors: vec![],
    })
}

fn parse_line(line: &str) -> Option<(String, u64, String)> {
    // Parse a single line of mo output
    // Format depends on mo CLI output - adjust accordingly
    // Example: "cache  1.2GB  /path/to/cache"

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        let category = parts[0].to_lowercase();
        let size = parse_size(parts[1]).unwrap_or(0);
        let path = parts[2..].join(" ");
        return Some((category, size, path));
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
    } else {
        1
    };

    let num_str: String = size_str.chars().filter(|c| c.is_digit(10) || *c == '.').collect();
    let num: f64 = num_str.parse().ok()?;

    Some((num * multiplier as f64) as u64)
}
```

### 3. Update Rust Modules

**File:** `src-tauri/src/commands/mod.rs`

```rust
pub mod clean;
pub mod status;
```

**File:** `src-tauri/src/lib.rs`

```rust
mod commands;

use commands::{clean, status};

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 4. Create Clean Service

**File:** `src/lib/services/cleanService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { ScanResult, CleanupResult, CleanupCategory } from '$lib/types/clean.types';

export async function scanCleanup(): Promise<ScanResult> {
  return invoke<ScanResult>('scan_cleanup');
}

export async function runCleanup(categories: CleanupCategory[]): Promise<CleanupResult> {
  return invoke<CleanupResult>('run_cleanup', { categories });
}
```

### 5. Create Toast Store

**File:** `src/lib/stores/toast.store.ts`

```typescript
import type { Toast } from '$lib/types/common.types';

let toasts = $state<Toast[]>([]);

export const toastStore = {
  get all() { return toasts; },

  success(message: string, duration = 3000) {
    addToast('success', message, duration);
  },

  error(message: string, duration = 5000) {
    addToast('error', message, duration);
  },

  warning(message: string, duration = 4000) {
    addToast('warning', message, duration);
  },

  info(message: string, duration = 3000) {
    addToast('info', message, duration);
  },

  dismiss(id: string) {
    toasts = toasts.filter(t => t.id !== id);
  }
};

function addToast(type: Toast['type'], message: string, duration: number) {
  const id = crypto.randomUUID();
  toasts = [...toasts, { id, type, message, duration }];

  if (duration > 0) {
    setTimeout(() => toastStore.dismiss(id), duration);
  }
}
```

### 6. Create Clean Store

**File:** `src/lib/stores/clean.store.ts`

```typescript
import type { ScanResult, CleanupCategory } from '$lib/types/clean.types';

let scanResult = $state<ScanResult | null>(null);
let isScanning = $state(false);
let isCleaning = $state(false);
let selectedCategories = $state<Set<CleanupCategory>>(new Set(['cache', 'logs', 'trash']));

export const cleanStore = {
  get scanResult() { return scanResult; },
  get isScanning() { return isScanning; },
  get isCleaning() { return isCleaning; },
  get selectedCategories() { return selectedCategories; },

  setScanResult(result: ScanResult | null) {
    scanResult = result;
  },

  setScanning(value: boolean) {
    isScanning = value;
  },

  setCleaning(value: boolean) {
    isCleaning = value;
  },

  toggleCategory(category: CleanupCategory) {
    const newSet = new Set(selectedCategories);
    if (newSet.has(category)) {
      newSet.delete(category);
    } else {
      newSet.add(category);
    }
    selectedCategories = newSet;
  },

  selectAll() {
    selectedCategories = new Set(['cache', 'logs', 'trash', 'downloads', 'xcode', 'homebrew']);
  },

  deselectAll() {
    selectedCategories = new Set();
  },

  reset() {
    scanResult = null;
    isScanning = false;
    isCleaning = false;
    selectedCategories = new Set(['cache', 'logs', 'trash']);
  }
};
```

### 7. Create Toast Components

**File:** `src/lib/components/ui/Toast.svelte`

```svelte
<script lang="ts">
  import type { Toast } from '$lib/types/common.types';
  import { toastStore } from '$lib/stores/toast.store';

  interface Props {
    toast: Toast;
  }

  let { toast }: Props = $props();

  const icons = {
    success: '✓',
    error: '✕',
    warning: '⚠',
    info: 'ℹ',
  };

  const colors = {
    success: 'border-accent-green bg-accent-green/10',
    error: 'border-accent-red bg-accent-red/10',
    warning: 'border-accent-orange bg-accent-orange/10',
    info: 'border-accent-blue bg-accent-blue/10',
  };
</script>

<div class="flex items-center gap-3 px-4 py-3 rounded-lg border {colors[toast.type]} shadow-lg">
  <span class="text-lg">{icons[toast.type]}</span>
  <p class="flex-1 text-sm text-content-primary">{toast.message}</p>
  <button
    class="text-content-secondary hover:text-content-primary"
    onclick={() => toastStore.dismiss(toast.id)}
  >
    ✕
  </button>
</div>
```

**File:** `src/lib/components/ui/ToastContainer.svelte`

```svelte
<script lang="ts">
  import Toast from './Toast.svelte';
  import { toastStore } from '$lib/stores/toast.store';
</script>

{#if toastStore.all.length > 0}
  <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 w-80">
    {#each toastStore.all as toast (toast.id)}
      <Toast {toast} />
    {/each}
  </div>
{/if}
```

### 8. Create CleanPanel Component

**File:** `src/lib/components/features/CleanPanel.svelte`

```svelte
<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import { cleanStore } from '$lib/stores/clean.store';
  import { toastStore } from '$lib/stores/toast.store';
  import { scanCleanup, runCleanup } from '$lib/services/cleanService';
  import { formatBytes } from '$lib/utils/format';
  import type { CleanupCategory } from '$lib/types/clean.types';

  const categoryLabels: Record<CleanupCategory, string> = {
    cache: 'Cache Files',
    logs: 'System Logs',
    trash: 'Trash',
    downloads: 'Downloads',
    xcode: 'Xcode Derived Data',
    homebrew: 'Homebrew Cache',
  };

  const categoryOrder: CleanupCategory[] = ['cache', 'logs', 'trash', 'downloads', 'xcode', 'homebrew'];

  async function handleScan() {
    cleanStore.setScanning(true);
    try {
      const result = await scanCleanup();
      cleanStore.setScanResult(result);
      toastStore.success(`Found ${formatBytes(result.totalSize)} to clean`);
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Scan failed');
    } finally {
      cleanStore.setScanning(false);
    }
  }

  async function handleClean() {
    const categories = Array.from(cleanStore.selectedCategories);
    if (categories.length === 0) {
      toastStore.warning('Please select at least one category');
      return;
    }

    cleanStore.setCleaning(true);
    try {
      const result = await runCleanup(categories);
      toastStore.success(`Freed ${formatBytes(result.freedSize)}`);
      cleanStore.reset();
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Cleanup failed');
    } finally {
      cleanStore.setCleaning(false);
    }
  }

  let selectedSize = $derived(() => {
    if (!cleanStore.scanResult) return 0;
    let total = 0;
    for (const cat of cleanStore.selectedCategories) {
      total += cleanStore.scanResult.categories[cat] || 0;
    }
    return total;
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Cleanup</h1>
    <p class="text-content-secondary">Scan and remove unnecessary files</p>
  </div>

  <div class="flex items-center gap-4">
    <Button onclick={handleScan} disabled={cleanStore.isScanning}>
      {cleanStore.isScanning ? '🔄 Scanning...' : '🔍 Scan Now'}
    </Button>

    {#if cleanStore.scanResult}
      <span class="text-content-secondary">
        Total: <strong class="text-content-primary">{formatBytes(cleanStore.scanResult.totalSize)}</strong> cleanable
      </span>
    {/if}
  </div>

  {#if cleanStore.scanResult}
    <Card padding="none">
      <div class="divide-y divide-border-secondary">
        {#each categoryOrder as category}
          {@const size = cleanStore.scanResult.categories[category]}
          {#if size > 0}
            <label class="flex items-center gap-3 px-4 py-3 hover:bg-surface-tertiary cursor-pointer">
              <input
                type="checkbox"
                checked={cleanStore.selectedCategories.has(category)}
                onchange={() => cleanStore.toggleCategory(category)}
                class="w-4 h-4 accent-accent-blue"
              />
              <span class="flex-1 text-content-primary">{categoryLabels[category]}</span>
              <span class="text-content-secondary">{formatBytes(size)}</span>
            </label>
          {/if}
        {/each}
      </div>
    </Card>

    <div class="flex items-center gap-4">
      <Button variant="ghost" onclick={() => cleanStore.selectAll()}>Select All</Button>
      <Button variant="ghost" onclick={() => cleanStore.deselectAll()}>Deselect All</Button>
    </div>

    <Button
      size="lg"
      onclick={handleClean}
      disabled={cleanStore.isCleaning || cleanStore.selectedCategories.size === 0}
    >
      {cleanStore.isCleaning
        ? '🧹 Cleaning...'
        : `🧹 Clean Selected (${formatBytes(selectedSize())})`}
    </Button>
  {/if}
</div>
```

### 9. Update Layout and Page

**File:** `src/routes/+layout.svelte`

```svelte
<script>
  import '../app.css';
  import ToastContainer from '$lib/components/ui/ToastContainer.svelte';

  let { children } = $props();
</script>

{@render children?.()}
<ToastContainer />
```

**File:** `src/routes/+page.svelte`

```svelte
<script lang="ts">
  import Layout from '$lib/components/layout/Layout.svelte';
  import Dashboard from '$lib/components/features/Dashboard.svelte';
  import CleanPanel from '$lib/components/features/CleanPanel.svelte';
  import { uiStore } from '$lib/stores/ui.store';
</script>

<Layout>
  <div class="max-w-4xl">
    {#if uiStore.activePanel === 'dashboard'}
      <Dashboard />
    {:else if uiStore.activePanel === 'clean'}
      <CleanPanel />
    {:else}
      <h1 class="text-2xl font-semibold mb-4 capitalize">{uiStore.activePanel}</h1>
      <p class="text-content-secondary">Coming in Phase 5</p>
    {/if}
  </div>
</Layout>
```

## Checklist

- [ ] Clean types created
- [ ] Rust `scan_cleanup` command implemented
- [ ] Rust `run_cleanup` command implemented
- [ ] Clean service created
- [ ] Toast store created
- [ ] Clean store created
- [ ] Toast component created
- [ ] ToastContainer added to layout
- [ ] CleanPanel component created
- [ ] Scan button triggers scan and shows results
- [ ] Category checkboxes work correctly
- [ ] Select All / Deselect All work
- [ ] Clean button executes cleanup
- [ ] Toast notifications show for success/error
- [ ] Loading states during scan/clean

## Verification

```bash
# 1. Ensure mo CLI is installed
which mo

# 2. Run the app
npm run tauri dev

# 3. Navigate to Clean panel

# 4. Click "Scan Now"
# - Should show spinner during scan
# - Results appear with category list
# - Toast shows total cleanable

# 5. Toggle categories
# - Checkboxes update selection
# - Button label updates with selected size

# 6. Click "Clean Selected"
# - Should show cleaning progress
# - Toast confirms completion
# - Panel resets to initial state
```

## Notes

- The Rust parser needs adjustment based on actual `mo` CLI output format
- Consider adding a confirmation dialog before cleanup (Phase 6)
- Progress bar during cleanup can be added in Phase 6
