---
phase: 3
title: Dashboard
status: pending
depends-on: [phase-2]
---

# Phase 3: Dashboard

## Objective

Build the dashboard view with stat cards displaying system information and quick action buttons.

## Prerequisites

- Phase 2 completed (Layout, Sidebar, navigation working)

## Deliverables

| File | Action | Spec Reference |
|------|--------|----------------|
| `src/lib/components/ui/StatCard.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#dashboard-panel) |
| `src/lib/components/features/Dashboard.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#dashboard-panel) |
| `src/lib/types/status.types.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#status-types) |
| `src/lib/services/statusService.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#status-service) |
| `src/lib/utils/format.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#utility-functions) |
| `src-tauri/src/commands/mod.rs` | create | [rust-commands.md](../docs/specs/rust-commands.md#registration) |
| `src-tauri/src/commands/status.rs` | create | [rust-commands.md](../docs/specs/rust-commands.md#get_status) |
| `src-tauri/src/lib.rs` | modify | Register status commands |
| `src/routes/+page.svelte` | modify | Use Dashboard component |

## Implementation Steps

### 1. Create Format Utilities

**File:** `src/lib/utils/format.ts`

```typescript
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

export function formatPercentage(value: number, total: number): string {
  if (total === 0) return '0%';
  return `${((value / total) * 100).toFixed(1)}%`;
}
```

### 2. Create Status Types

**File:** `src/lib/types/status.types.ts`

```typescript
export interface SystemStatus {
  diskTotal: number;
  diskUsed: number;
  diskAvailable: number;
  cleanableSize: number;
  lastClean: string | null;
  moVersion: string;
}
```

### 3. Create StatCard Component

**File:** `src/lib/components/ui/StatCard.svelte`

```svelte
<script lang="ts">
  import Card from './Card.svelte';

  interface Props {
    icon: string;
    value: string;
    label: string;
    sublabel?: string;
    trend?: 'up' | 'down' | null;
    onclick?: () => void;
  }

  let { icon, value, label, sublabel, trend = null, onclick } = $props();
</script>

<Card>
  <button
    class="w-full text-left {onclick ? 'cursor-pointer hover:opacity-80' : 'cursor-default'}"
    onclick={onclick}
    disabled={!onclick}
  >
    <div class="flex items-start gap-3">
      <span class="text-2xl">{icon}</span>
      <div class="flex-1">
        <div class="flex items-baseline gap-2">
          <span class="text-2xl font-semibold text-content-primary">{value}</span>
          {#if trend}
            <span class="text-sm {trend === 'up' ? 'text-accent-red' : 'text-accent-green'}">
              {trend === 'up' ? '↑' : '↓'}
            </span>
          {/if}
        </div>
        <p class="text-sm text-content-secondary">{label}</p>
        {#if sublabel}
          <p class="text-xs text-content-tertiary mt-1">{sublabel}</p>
        {/if}
      </div>
    </div>
  </button>
</Card>
```

### 4. Create Rust Status Commands

**File:** `src-tauri/src/commands/mod.rs`

```rust
pub mod status;
```

**File:** `src-tauri/src/commands/status.rs`

```rust
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    pub disk_total: u64,
    pub disk_used: u64,
    pub disk_available: u64,
    pub cleanable_size: u64,
    pub last_clean: Option<String>,
    pub mo_version: String,
}

#[tauri::command]
pub async fn check_mo_installed() -> Result<bool, String> {
    let output = Command::new("which")
        .arg("mo")
        .output()
        .map_err(|e| format!("Failed to check mo: {}", e))?;

    Ok(output.status.success())
}

#[tauri::command]
pub async fn get_status() -> Result<SystemStatus, String> {
    // Get disk space using df command
    let df_output = Command::new("df")
        .args(["-k", "/"])
        .output()
        .map_err(|e| format!("Failed to get disk info: {}", e))?;

    let df_str = String::from_utf8_lossy(&df_output.stdout);
    let (disk_total, disk_used, disk_available) = parse_df_output(&df_str)?;

    // Check mo version
    let mo_version = get_mo_version().unwrap_or_else(|_| "Not installed".to_string());

    // Placeholder for cleanable size (will be updated after scan)
    let cleanable_size = 0;

    Ok(SystemStatus {
        disk_total,
        disk_used,
        disk_available,
        cleanable_size,
        last_clean: None,
        mo_version,
    })
}

fn parse_df_output(output: &str) -> Result<(u64, u64, u64), String> {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() < 2 {
        return Err("Invalid df output".to_string());
    }

    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 4 {
        return Err("Invalid df output format".to_string());
    }

    // df -k returns values in KB, convert to bytes
    let total = parts[1].parse::<u64>().map_err(|e| e.to_string())? * 1024;
    let used = parts[2].parse::<u64>().map_err(|e| e.to_string())? * 1024;
    let available = parts[3].parse::<u64>().map_err(|e| e.to_string())? * 1024;

    Ok((total, used, available))
}

fn get_mo_version() -> Result<String, String> {
    let output = Command::new("mo")
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to get mo version: {}", e))?;

    if !output.status.success() {
        return Err("mo command failed".to_string());
    }

    let version = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    Ok(version)
}
```

### 5. Update lib.rs

**File:** `src-tauri/src/lib.rs`

```rust
mod commands;

use commands::status;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            status::check_mo_installed,
            status::get_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 6. Create Status Service

**File:** `src/lib/services/statusService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { SystemStatus } from '$lib/types/status.types';

export async function getStatus(): Promise<SystemStatus> {
  return invoke<SystemStatus>('get_status');
}

export async function checkMoInstalled(): Promise<boolean> {
  return invoke<boolean>('check_mo_installed');
}
```

### 7. Create Dashboard Component

**File:** `src/lib/components/features/Dashboard.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import StatCard from '$lib/components/ui/StatCard.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { getStatus } from '$lib/services/statusService';
  import { formatBytes, formatPercentage } from '$lib/utils/format';
  import { uiStore } from '$lib/stores/ui.store';
  import type { SystemStatus } from '$lib/types/status.types';

  let status = $state<SystemStatus | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      status = await getStatus();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load status';
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">Welcome to Molery</h1>
    <p class="text-content-secondary">Your system optimization dashboard</p>
  </div>

  {#if loading}
    <div class="grid grid-cols-4 gap-4">
      {#each [1, 2, 3, 4] as _}
        <div class="bg-surface-secondary rounded-xl p-4 animate-pulse">
          <div class="h-8 bg-surface-tertiary rounded mb-2"></div>
          <div class="h-4 bg-surface-tertiary rounded w-2/3"></div>
        </div>
      {/each}
    </div>
  {:else if error}
    <div class="bg-accent-red/10 border border-accent-red rounded-lg p-4 text-accent-red">
      {error}
    </div>
  {:else if status}
    <div class="grid grid-cols-4 gap-4">
      <StatCard
        icon="📦"
        value={formatBytes(status.diskUsed)}
        label="Disk Used"
        sublabel={formatPercentage(status.diskUsed, status.diskTotal)}
      />
      <StatCard
        icon="🧹"
        value={formatBytes(status.cleanableSize)}
        label="Cleanable"
        onclick={() => uiStore.setPanel('clean')}
      />
      <StatCard
        icon="💾"
        value={formatBytes(status.diskAvailable)}
        label="Available"
      />
      <StatCard
        icon="✓"
        value={status.moVersion !== 'Not installed' ? 'Ready' : 'Setup'}
        label="mo CLI"
        sublabel={status.moVersion}
      />
    </div>
  {/if}

  <div>
    <h2 class="text-lg font-medium text-content-primary mb-3">Quick Actions</h2>
    <div class="flex gap-3">
      <Button onclick={() => uiStore.setPanel('clean')}>
        🧹 Quick Clean
      </Button>
      <Button variant="secondary" onclick={() => uiStore.setPanel('analyze')}>
        📊 Analyze Disk
      </Button>
    </div>
  </div>
</div>
```

### 8. Update +page.svelte

**File:** `src/routes/+page.svelte`

```svelte
<script lang="ts">
  import Layout from '$lib/components/layout/Layout.svelte';
  import Dashboard from '$lib/components/features/Dashboard.svelte';
  import { uiStore } from '$lib/stores/ui.store';
</script>

<Layout>
  <div class="max-w-4xl">
    {#if uiStore.activePanel === 'dashboard'}
      <Dashboard />
    {:else if uiStore.activePanel === 'clean'}
      <h1 class="text-2xl font-semibold mb-4">Clean</h1>
      <p class="text-content-secondary">Coming in Phase 4</p>
    {:else}
      <h1 class="text-2xl font-semibold mb-4 capitalize">{uiStore.activePanel}</h1>
      <p class="text-content-secondary">Coming soon</p>
    {/if}
  </div>
</Layout>
```

## Checklist

- [ ] Format utilities created
- [ ] Status types created
- [ ] StatCard component created
- [ ] Rust commands module structure created
- [ ] `check_mo_installed` command implemented
- [ ] `get_status` command implemented (disk space)
- [ ] Status service created
- [ ] Dashboard component with stat cards
- [ ] Loading skeleton displayed while fetching
- [ ] Error state handled gracefully
- [ ] Quick action buttons navigate to panels

## Verification

```bash
# 1. Run the app
npm run tauri dev

# 2. Dashboard should show:
# - 4 stat cards with system info
# - Disk used, cleanable, available, mo status
# - Quick action buttons

# 3. Test loading state:
# - Skeleton loaders visible briefly on load

# 4. Test navigation:
# - Click "Quick Clean" → navigates to Clean panel
# - Click "Analyze Disk" → navigates to Analyze panel

# 5. Test mo detection:
# - If mo is installed: shows version
# - If not installed: shows "Not installed"
```

## Notes

- Cleanable size will be 0 until Phase 4 implements scan
- App count stat will be added in Phase 5 (Uninstall panel)
- Error handling shows inline error message
