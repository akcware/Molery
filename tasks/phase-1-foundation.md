---
phase: 1
title: Foundation
status: pending
depends-on: null
---

# Phase 1: Foundation

## Objective

Set up TailwindCSS, establish folder structure, and configure Tauri permissions for CLI execution.

## Prerequisites

- Node.js and npm installed
- Rust toolchain installed
- Project cloned and dependencies installed (`npm install`)

## Deliverables

| File | Action | Spec Reference |
|------|--------|----------------|
| `tailwind.config.js` | create | [design-tokens.md](../docs/specs/design-tokens.md#tailwind-configuration) |
| `postcss.config.js` | create | - |
| `src/app.css` | create | [design-tokens.md](../docs/specs/design-tokens.md#css-custom-properties) |
| `src/app.html` | modify | Add TailwindCSS stylesheet |
| `src/lib/components/ui/.gitkeep` | create | [ARCHITECTURE.md](../docs/ARCHITECTURE.md#target-file-structure) |
| `src/lib/components/layout/.gitkeep` | create | - |
| `src/lib/components/features/.gitkeep` | create | - |
| `src/lib/services/.gitkeep` | create | - |
| `src/lib/stores/.gitkeep` | create | - |
| `src/lib/types/.gitkeep` | create | - |
| `src/lib/utils/.gitkeep` | create | - |
| `src-tauri/capabilities/default.json` | modify | [setup-flow.md](../docs/flows/setup-flow.md#tauri-permissions) |
| `src-tauri/Cargo.toml` | modify | Add `tauri-plugin-shell` |
| `package.json` | modify | Add TailwindCSS dependencies |

## Implementation Steps

### 1. Install TailwindCSS

```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

### 2. Create tailwind.config.js

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        surface: {
          primary: 'var(--color-surface-primary)',
          secondary: 'var(--color-surface-secondary)',
          tertiary: 'var(--color-surface-tertiary)',
          elevated: 'var(--color-surface-elevated)',
        },
        content: {
          primary: 'var(--color-content-primary)',
          secondary: 'var(--color-content-secondary)',
          tertiary: 'var(--color-content-tertiary)',
          inverse: 'var(--color-content-inverse)',
        },
        accent: {
          blue: 'var(--color-accent-blue)',
          green: 'var(--color-accent-green)',
          orange: 'var(--color-accent-orange)',
          red: 'var(--color-accent-red)',
        },
        border: {
          primary: 'var(--color-border-primary)',
          secondary: 'var(--color-border-secondary)',
        },
      },
    },
  },
  plugins: [],
};
```

### 3. Create src/app.css

See [design-tokens.md](../docs/specs/design-tokens.md#css-custom-properties) for full content.

### 4. Update src/app.html

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <link rel="icon" href="%sveltekit.assets%/favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover" class="bg-surface-primary text-content-primary">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
```

### 5. Create Folder Structure

```bash
mkdir -p src/lib/components/ui
mkdir -p src/lib/components/layout
mkdir -p src/lib/components/features
mkdir -p src/lib/services
mkdir -p src/lib/stores
mkdir -p src/lib/types
mkdir -p src/lib/utils
```

### 6. Update Tauri Capabilities

```json
{
  "identifier": "default",
  "description": "Default capabilities for Molery",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "opener:default",
    "shell:allow-execute"
  ]
}
```

### 7. Add Shell Plugin to Cargo.toml

```toml
[dependencies]
tauri-plugin-shell = "2"
```

### 8. Initialize Shell Plugin in lib.rs

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Checklist

- [ ] TailwindCSS installed and configured
- [ ] Design tokens defined in `app.css`
- [ ] Folder structure created matching ARCHITECTURE.md
- [ ] Tauri shell plugin added to Cargo.toml
- [ ] Tauri capabilities updated for shell execution
- [ ] Shell plugin initialized in lib.rs
- [ ] App compiles without errors (`npm run tauri dev`)

## Verification

```bash
# 1. Check folder structure
tree src/lib

# 2. Verify TailwindCSS works
# Add a test class to +page.svelte: <div class="bg-accent-blue">Test</div>

# 3. Verify Tauri builds
npm run tauri dev

# 4. Verify shell capability (in dev tools console)
# Should not throw permission error
```

## Notes

- Remove the default template content from `+page.svelte` styles
- Keep the `greet` command for now; will be replaced in Phase 4
