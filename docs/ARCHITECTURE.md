# Molery Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Molery Desktop App                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   Svelte 5 Frontend                  │    │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌────────┐  │    │
│  │  │Dashboard│  │ Clean   │  │Uninstall│  │Analyze │  │    │
│  │  │  Panel  │  │  Panel  │  │  Panel  │  │ Panel  │  │    │
│  │  └────┬────┘  └────┬────┘  └────┬────┘  └───┬────┘  │    │
│  │       │            │            │           │        │    │
│  │  ┌────┴────────────┴────────────┴───────────┴────┐  │    │
│  │  │              Services Layer                    │  │    │
│  │  │   (invoke() calls to Rust backend)            │  │    │
│  │  └────────────────────┬──────────────────────────┘  │    │
│  └───────────────────────┼─────────────────────────────┘    │
│                          │ IPC                               │
│  ┌───────────────────────┼─────────────────────────────┐    │
│  │                 Tauri Rust Backend                   │    │
│  │  ┌────────────────────┴──────────────────────────┐  │    │
│  │  │              Command Handlers                  │  │    │
│  │  │  scan_cleanup | run_cleanup | get_status | ... │  │    │
│  │  └────────────────────┬──────────────────────────┘  │    │
│  │                       │                              │    │
│  │  ┌────────────────────┴──────────────────────────┐  │    │
│  │  │           mo CLI Executor                      │  │    │
│  │  │      (subprocess calls to mo binary)          │  │    │
│  │  └───────────────────────────────────────────────┘  │    │
│  └──────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │   mo CLI Tool   │
                    │  (System Level) │
                    └─────────────────┘
```

## Data Flow

```
User Action → Svelte Component → Service → invoke() → Rust Command → mo CLI → Result
     ↑                                                                           │
     └───────────────────────── State Update ←───────────────────────────────────┘
```

### Example: Clean Operation

1. User clicks "Scan" button in CleanPanel
2. `CleanPanel.svelte` calls `cleanService.scanCleanup()`
3. Service invokes Rust command: `invoke('scan_cleanup')`
4. Rust handler executes `mo clean --dry-run` and parses output
5. Returns structured `ScanResult` to frontend
6. Store updates, UI re-renders with cleanup items

## Target File Structure

```
Molery/
├── .ai/                              # LLM Agent Context
│   ├── CONTEXT.md
│   └── CONVENTIONS.md
│
├── docs/
│   ├── README.md
│   ├── ARCHITECTURE.md               # (this file)
│   ├── specs/
│   │   ├── rust-commands.md
│   │   ├── typescript-api.md
│   │   ├── design-tokens.md
│   │   └── ui-screens.md
│   └── flows/
│       └── setup-flow.md
│
├── tasks/
│   ├── README.md
│   └── phase-*.md
│
├── src/
│   ├── app.html
│   ├── app.css                       # TailwindCSS imports + design tokens
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/                   # Base components
│   │   │   │   ├── Button.svelte
│   │   │   │   ├── Card.svelte
│   │   │   │   ├── StatCard.svelte
│   │   │   │   └── Toast.svelte
│   │   │   ├── layout/
│   │   │   │   ├── Header.svelte
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   └── Layout.svelte
│   │   │   └── features/
│   │   │       ├── CleanPanel.svelte
│   │   │       ├── UninstallPanel.svelte
│   │   │       ├── AnalyzePanel.svelte
│   │   │       ├── OptimizePanel.svelte
│   │   │       └── StatusPanel.svelte
│   │   ├── services/
│   │   │   ├── cleanService.ts
│   │   │   ├── uninstallService.ts
│   │   │   ├── analyzeService.ts
│   │   │   └── statusService.ts
│   │   ├── stores/
│   │   │   ├── clean.store.ts
│   │   │   ├── ui.store.ts
│   │   │   └── toast.store.ts
│   │   └── types/
│   │       ├── clean.types.ts
│   │       ├── uninstall.types.ts
│   │       └── common.types.ts
│   └── routes/
│       ├── +layout.svelte
│       ├── +layout.ts
│       └── +page.svelte
│
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── clean.rs
│   │       ├── uninstall.rs
│   │       ├── analyze.rs
│   │       └── status.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/
│       └── default.json
│
├── static/
│   └── favicon.png
│
├── package.json
├── svelte.config.js
├── vite.config.js
├── tsconfig.json
├── tailwind.config.js                # TailwindCSS configuration
├── postcss.config.js
└── README.md
```

## Component Hierarchy

```
+layout.svelte
└── Layout.svelte
    ├── Header.svelte
    ├── Sidebar.svelte
    │   └── Navigation items
    └── Main Content Area
        └── +page.svelte
            ├── Dashboard (default view)
            │   └── StatCard.svelte (×4)
            └── Feature Panels (tab-based)
                ├── CleanPanel.svelte
                ├── UninstallPanel.svelte
                ├── AnalyzePanel.svelte
                ├── OptimizePanel.svelte
                └── StatusPanel.svelte
```

## State Management

### Global Stores

| Store | Purpose |
|-------|---------|
| `ui.store.ts` | Active panel, sidebar state, theme |
| `toast.store.ts` | Notification queue |
| `clean.store.ts` | Scan results, cleanup state |

### Local State

Component-specific state managed with Svelte 5 runes (`$state`, `$derived`).

## Security Considerations

### Tauri Capabilities

Required permissions in `capabilities/default.json`:

- `shell:allow-execute` - Execute `mo` CLI commands
- `fs:allow-read` - Read file sizes for display
- No write permissions needed (mo handles all writes)

### Input Validation

- All user inputs sanitized before passing to CLI
- Path traversal prevention for analyze feature
- App names validated against installed apps list
