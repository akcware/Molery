# Molery - Project Context

## What

macOS desktop GUI for Mole CLI (`mo`) - a system optimizer tool for cleaning caches, analyzing disk usage, and managing system resources.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Svelte 5 + TypeScript + TailwindCSS |
| Backend | Tauri 2.x (Rust) |
| CLI | `mo` ([github.com/tw93/Mole](https://github.com/tw93/Mole)) |
| Build | Vite 6.x |

## Current State

- **Phase**: Pre-development (template state)
- **Status**: Fresh Tauri + Svelte template, no features implemented yet
- **Next**: Phase 1 - Foundation setup

## Key Entry Points

| Purpose | File |
|---------|------|
| Svelte App | `src/routes/+page.svelte` |
| Rust Backend | `src-tauri/src/lib.rs` |
| Tauri Config | `src-tauri/tauri.conf.json` |
| Package Config | `package.json` |

## Quick Links

| Resource | Path |
|----------|------|
| Architecture | [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md) |
| Rust Commands | [docs/specs/rust-commands.md](../docs/specs/rust-commands.md) |
| TypeScript API | [docs/specs/typescript-api.md](../docs/specs/typescript-api.md) |
| Task Dashboard | [tasks/README.md](../tasks/README.md) |
| Design Tokens | [docs/specs/design-tokens.md](../docs/specs/design-tokens.md) |

## CLI Commands Reference

The app wraps these `mo` CLI commands:

| Command | Purpose |
|---------|---------|
| `mo` | Show overall status |
| `mo clean` | Clean system caches/trash |
| `mo uninstall <app>` | Fully uninstall an application |
| `mo analyze <path>` | Analyze disk usage |
| `mo optimize` | System optimization |
