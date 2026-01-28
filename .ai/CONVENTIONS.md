# Code Conventions

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| Svelte Components | PascalCase | `StatCard.svelte` |
| TypeScript modules | camelCase | `cleanService.ts` |
| Type definitions | camelCase with `.types.ts` | `clean.types.ts` |
| Stores | camelCase with `.store.ts` | `clean.store.ts` |
| Rust modules | snake_case | `clean_commands.rs` |

## Directory Structure

```
src/
├── lib/
│   ├── components/      # Reusable UI components
│   │   ├── ui/          # Base components (Button, Card, etc.)
│   │   └── features/    # Feature-specific components
│   ├── services/        # Tauri invoke wrappers
│   ├── stores/          # Svelte 5 runes-based stores
│   └── types/           # TypeScript type definitions
├── routes/              # SvelteKit pages
└── app.html             # HTML template
```

## Svelte 5 Patterns

### State Management (Runes)

```svelte
<script lang="ts">
  // Reactive state
  let count = $state(0);

  // Derived values
  let doubled = $derived(count * 2);

  // Effects
  $effect(() => {
    console.log('Count changed:', count);
  });
</script>
```

### Props

```svelte
<script lang="ts">
  interface Props {
    title: string;
    value?: number;
  }

  let { title, value = 0 }: Props = $props();
</script>
```

## TypeScript Patterns

### Service Functions

```typescript
// src/lib/services/cleanService.ts
import { invoke } from '@tauri-apps/api/core';
import type { ScanResult } from '$lib/types/clean.types';

export async function scanCleanup(): Promise<ScanResult> {
  return invoke<ScanResult>('scan_cleanup');
}
```

### Type Definitions

```typescript
// src/lib/types/clean.types.ts
export interface CleanupItem {
  path: string;
  size: number;
  category: 'cache' | 'logs' | 'trash';
}

export interface ScanResult {
  items: CleanupItem[];
  totalSize: number;
}
```

## Rust Patterns

### Tauri Commands

```rust
// Async command with error handling
#[tauri::command]
async fn scan_cleanup() -> Result<ScanResult, String> {
    // Implementation
}

// Register in lib.rs
.invoke_handler(tauri::generate_handler![scan_cleanup, run_cleanup])
```

## CSS/Tailwind Patterns

### Design Token Usage

```svelte
<!-- Use design token variables -->
<div class="bg-surface-primary text-content-primary">
  <span class="text-accent-blue">Highlighted</span>
</div>
```

### Component Styling

- Use Tailwind utilities for layout and spacing
- Reference design tokens for colors
- Keep component-specific styles in `<style>` blocks minimal

## Import Aliases

| Alias | Path |
|-------|------|
| `$lib` | `src/lib` |
| `$app` | SvelteKit runtime |

## Error Handling

### Frontend

```typescript
try {
  const result = await scanCleanup();
} catch (error) {
  // Show toast notification
  toastStore.error(error instanceof Error ? error.message : 'Unknown error');
}
```

### Backend (Rust)

```rust
#[tauri::command]
async fn dangerous_operation() -> Result<(), String> {
    some_operation().map_err(|e| e.to_string())?;
    Ok(())
}
```

## Git Commit Messages

- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`
- Keep subject line under 72 characters
- Reference issue numbers when applicable
