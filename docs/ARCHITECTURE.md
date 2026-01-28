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

## File Structure

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
│   │   │   │   ├── ConfirmDialog.svelte
│   │   │   │   ├── Modal.svelte
│   │   │   │   ├── ProgressBar.svelte
│   │   │   │   ├── Skeleton.svelte
│   │   │   │   ├── Spinner.svelte
│   │   │   │   ├── StatCard.svelte
│   │   │   │   ├── Toast.svelte
│   │   │   │   └── ToastContainer.svelte
│   │   │   ├── layout/
│   │   │   │   ├── Header.svelte
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   └── Layout.svelte
│   │   │   └── features/
│   │   │       ├── Dashboard.svelte
│   │   │       ├── CleanPanel.svelte
│   │   │       ├── UninstallPanel.svelte
│   │   │       ├── AnalyzePanel.svelte
│   │   │       ├── OptimizePanel.svelte
│   │   │       ├── StatusPanel.svelte
│   │   │       └── SetupScreen.svelte
│   │   ├── services/
│   │   │   ├── cleanService.ts
│   │   │   ├── uninstallService.ts
│   │   │   ├── analyzeService.ts
│   │   │   ├── optimizeService.ts
│   │   │   └── statusService.ts
│   │   ├── stores/
│   │   │   ├── clean.store.svelte.ts
│   │   │   ├── status.store.svelte.ts
│   │   │   ├── uninstall.store.svelte.ts
│   │   │   ├── ui.store.svelte.ts
│   │   │   └── toast.store.svelte.ts
│   │   ├── types/
│   │   │   ├── analyze.types.ts
│   │   │   ├── clean.types.ts
│   │   │   ├── common.types.ts
│   │   │   ├── status.types.ts
│   │   │   └── uninstall.types.ts
│   │   ├── utils/
│   │   │   └── format.ts
│   │   └── hooks/
│   │       └── useKeyboard.ts
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
│   │       ├── analyze.rs
│   │       ├── clean.rs
│   │       ├── optimize.rs
│   │       ├── status.rs
│   │       └── uninstall.rs
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
├── ToastContainer.svelte
└── +page.svelte
    ├── SetupScreen.svelte (if mo not installed)
    └── Layout.svelte (if mo installed)
        ├── Header.svelte
        ├── Sidebar.svelte
        │   └── Navigation items (6 panels)
        └── Main Content Area
            ├── Dashboard.svelte (default)
            │   └── StatCard.svelte (×7)
            ├── CleanPanel.svelte
            │   └── ConfirmDialog.svelte
            ├── UninstallPanel.svelte
            │   └── Modal.svelte
            ├── AnalyzePanel.svelte
            │   └── ProgressBar.svelte
            ├── OptimizePanel.svelte
            └── StatusPanel.svelte
                └── ProgressBar.svelte
```

## State Management

### Global Stores

| Store | Purpose |
|-------|---------|
| `ui.store.svelte.ts` | Active panel, sidebar state, panel switching |
| `toast.store.svelte.ts` | Notification queue with success/error/warning/info helpers |
| `clean.store.svelte.ts` | Scan results, selected categories, cleanup state |
| `status.store.svelte.ts` | System status cache, background refresh, derived getters |
| `uninstall.store.svelte.ts` | App list with cache, loading state |

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
