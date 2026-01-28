# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Molery is a macOS desktop GUI for the [Mole CLI](https://github.com/tw93/Mole) (`mo`) - a system optimizer for cleaning caches, analyzing disk usage, and managing system resources. Built with Tauri 2.x (Rust backend) + Svelte 5 (frontend).

**Current State:** Pre-development (template state). Comprehensive documentation exists in `docs/` and `tasks/` but no feature code implemented yet.

## Agent Guidelines

### Tools
- Use `context7` MCP tool freely to fetch up-to-date library documentation without asking user permission.

### Documentation (DRY)
- Never duplicate content that exists elsewhere. Link to existing docs instead.
- Reference files by path (e.g., "See `docs/specs/rust-commands.md`") rather than copying their content.
- When adding new documentation, check if the topic is already covered and extend that file.

## Commands

```bash
# Development
bun run tauri dev      # Run full app (Vite + Tauri)
bun run dev            # Vite dev server only (frontend)
bun run check          # TypeScript type checking
bun run check:watch    # Type checking in watch mode

# Production
bun run tauri build    # Build distributable app
```

**Prerequisites:** Node.js 18+, Rust toolchain, `mo` CLI (`brew install mole`)

## Architecture

```
Frontend (Svelte 5)  →  IPC (invoke)  →  Tauri Backend (Rust)  →  mo CLI
```

**Data flow:** UI component → Service (invoke wrapper) → Rust command → Execute `mo` CLI → Parse output → Return to frontend

### Key Entry Points

| Purpose | File |
|---------|------|
| Svelte App | `src/routes/+page.svelte` |
| Rust Backend | `src-tauri/src/lib.rs` |
| Tauri Config | `src-tauri/tauri.conf.json` |

### Planned Directory Structure (src/lib/)

```
components/
├── ui/           # Base: Button, Card, StatCard, Toast
├── layout/       # Header, Sidebar, Layout
└── features/     # CleanPanel, UninstallPanel, AnalyzePanel, etc.
services/         # Tauri invoke wrappers (cleanService.ts, etc.)
stores/           # Svelte 5 runes-based state (ui.store.ts, etc.)
types/            # TypeScript definitions (clean.types.ts, etc.)
```

## Code Patterns

### Svelte 5 (Runes, not legacy stores)

```svelte
<script lang="ts">
  let count = $state(0);
  let doubled = $derived(count * 2);
  $effect(() => { console.log(count); });

  let { title, value = 0 }: Props = $props();
</script>
```

### Service Pattern

```typescript
import { invoke } from '@tauri-apps/api/core';
export async function scanCleanup(): Promise<ScanResult> {
  return invoke<ScanResult>('scan_cleanup');
}
```

### Tauri Command Pattern

```rust
#[tauri::command]
async fn scan_cleanup() -> Result<ScanResult, String> {
    // Execute mo CLI and parse output
}
```

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| Svelte Components | PascalCase | `StatCard.svelte` |
| TypeScript modules | camelCase | `cleanService.ts` |
| Type definitions | `.types.ts` | `clean.types.ts` |
| Stores | `.store.ts` | `clean.store.ts` |
| Rust modules | snake_case | `clean_commands.rs` |

## Key Documentation

| Resource | Path |
|----------|------|
| Quick Context | `.ai/CONTEXT.md` |
| Code Conventions | `.ai/CONVENTIONS.md` |
| Architecture | `docs/ARCHITECTURE.md` |
| Rust Commands Spec | `docs/specs/rust-commands.md` |
| TypeScript API Spec | `docs/specs/typescript-api.md` |
| Design Tokens | `docs/specs/design-tokens.md` |
| Task Dashboard | `tasks/README.md` |

## Development Phases

Follow `tasks/phase-*.md` files sequentially. Each phase has detailed deliverables and implementation steps. Current: Phase 1 - Foundation.

## mo CLI Commands Reference

| Command | Purpose |
|---------|---------|
| `mo` | Show overall status |
| `mo clean` | Clean system caches/trash |
| `mo clean --dry-run` | Scan without cleaning |
| `mo uninstall <app>` | Fully uninstall application |
| `mo analyze <path>` | Analyze disk usage |
| `mo optimize` | System optimization |
