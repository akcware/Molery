# Setup Flow

## Overview

Installation and configuration flow for the `mo` CLI tool, which Molery depends on.

## Flow Diagram

```
┌─────────────────┐
│   App Launch    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     No      ┌─────────────────┐
│  mo installed?  │───────────▶│  Setup Screen   │
└────────┬────────┘             └────────┬────────┘
         │ Yes                           │
         ▼                               ▼
┌─────────────────┐             ┌─────────────────┐
│   Dashboard     │             │ Install Options │
└─────────────────┘             └────────┬────────┘
                                         │
                     ┌───────────────────┼───────────────────┐
                     │                   │                   │
                     ▼                   ▼                   ▼
              ┌───────────┐      ┌───────────┐      ┌───────────┐
              │ Homebrew  │      │  Cargo    │      │  Manual   │
              │ (Recom.)  │      │           │      │           │
              └─────┬─────┘      └─────┬─────┘      └─────┬─────┘
                    │                  │                  │
                    ▼                  ▼                  ▼
              ┌─────────────────────────────────────────────┐
              │            Verify Installation              │
              └──────────────────┬──────────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │ Success                 │ Failed
                    ▼                         ▼
             ┌───────────┐             ┌───────────┐
             │ Dashboard │             │  Retry    │
             └───────────┘             │  Screen   │
                                       └───────────┘
```

## Detection

### Check Command

```rust
#[tauri::command]
async fn check_mo_installed() -> Result<bool, String> {
    use std::process::Command;

    let output = Command::new("which")
        .arg("mo")
        .output()
        .map_err(|e| e.to_string())?;

    Ok(output.status.success())
}
```

### Frontend Check

```typescript
// src/lib/services/statusService.ts
import { invoke } from '@tauri-apps/api/core';

export async function checkMoInstalled(): Promise<boolean> {
  return invoke<boolean>('check_mo_installed');
}
```

## Setup Screen

**File:** `src/lib/components/features/SetupScreen.svelte`

### UI Layout

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│            🔧 Molery Setup                             │
│                                                         │
│     Molery requires the 'mo' CLI tool to work.         │
│     Please install it using one of these methods:      │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  🍺 Homebrew (Recommended)                       │   │
│  │                                                  │   │
│  │  brew install tw93/brew/mole                   │   │
│  │                                                  │   │
│  │  [Copy Command]        [Open Terminal]          │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  📦 Cargo (Rust)                                │   │
│  │                                                  │   │
│  │  cargo install mole                            │   │
│  │                                                  │   │
│  │  [Copy Command]                                 │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  📥 Manual Download                             │   │
│  │                                                  │   │
│  │  Download from GitHub releases                  │   │
│  │                                                  │   │
│  │  [Open GitHub]                                  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│                 [🔄 Check Again]                       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Installation Commands

### Homebrew (Recommended)

```bash
brew install tw93/brew/mole
```

**Why Recommended:**
- Automatic updates via `brew upgrade`
- Easy uninstall via `brew uninstall mole`
- Handles PATH configuration automatically

### Cargo

```bash
cargo install mole
```

**Prerequisites:**
- Rust toolchain installed
- Cargo in PATH

### Manual

1. Visit: https://github.com/tw93/Mole/releases
2. Download latest binary for macOS
3. Move to `/usr/local/bin/mo`
4. Make executable: `chmod +x /usr/local/bin/mo`

## Verification

After installation attempt:

```typescript
async function verifyAndProceed() {
  const isInstalled = await checkMoInstalled();

  if (isInstalled) {
    // Navigate to dashboard
    uiStore.setPanel('dashboard');
    toastStore.success('mo CLI detected! Ready to use.');
  } else {
    toastStore.error('mo CLI not found. Please try again.');
  }
}
```

## App Startup Logic

**File:** `src/routes/+page.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { checkMoInstalled } from '$lib/services/statusService';
  import Layout from '$lib/components/layout/Layout.svelte';
  import SetupScreen from '$lib/components/features/SetupScreen.svelte';

  let moInstalled = $state<boolean | null>(null);
  let checking = $state(true);

  onMount(async () => {
    try {
      moInstalled = await checkMoInstalled();
    } catch {
      moInstalled = false;
    } finally {
      checking = false;
    }
  });
</script>

{#if checking}
  <LoadingScreen />
{:else if moInstalled}
  <Layout />
{:else}
  <SetupScreen onInstalled={() => moInstalled = true} />
{/if}
```

## Tauri Permissions

Required in `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "shell:allow-execute",
    "opener:default"
  ]
}
```

The `shell:allow-execute` permission is needed to:
- Run `which mo` for detection
- Execute `mo` commands for all features

The `opener:default` permission is needed to:
- Open terminal for installation
- Open GitHub releases page
