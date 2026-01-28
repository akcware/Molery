---
phase: 2
title: UI Shell
status: pending
depends-on: [phase-1]
---

# Phase 2: UI Shell

## Objective

Create the main application layout with Header, Sidebar navigation, and content area structure.

## Prerequisites

- Phase 1 completed (TailwindCSS, folder structure, Tauri permissions)

## Deliverables

| File | Action | Spec Reference |
|------|--------|----------------|
| `src/lib/components/layout/Header.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#header) |
| `src/lib/components/layout/Sidebar.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#sidebar) |
| `src/lib/components/layout/Layout.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#layout-structure) |
| `src/lib/components/ui/Button.svelte` | create | - |
| `src/lib/components/ui/Card.svelte` | create | - |
| `src/lib/stores/ui.store.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#ui-store) |
| `src/lib/types/common.types.ts` | create | [typescript-api.md](../docs/specs/typescript-api.md#common-types) |
| `src/routes/+layout.svelte` | create | - |
| `src/routes/+page.svelte` | modify | Use Layout component |

## Implementation Steps

### 1. Create Common Types

**File:** `src/lib/types/common.types.ts`

```typescript
export type Panel = 'dashboard' | 'clean' | 'uninstall' | 'analyze' | 'optimize' | 'status';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
}

export interface NavItem {
  id: Panel;
  label: string;
  icon: string;
  shortcut: string;
}
```

### 2. Create UI Store

**File:** `src/lib/stores/ui.store.ts`

```typescript
import type { Panel } from '$lib/types/common.types';

let activePanel = $state<Panel>('dashboard');
let sidebarCollapsed = $state(false);

export const uiStore = {
  get activePanel() { return activePanel; },
  get sidebarCollapsed() { return sidebarCollapsed; },

  setPanel(panel: Panel) {
    activePanel = panel;
  },

  toggleSidebar() {
    sidebarCollapsed = !sidebarCollapsed;
  },
};
```

### 3. Create Button Component

**File:** `src/lib/components/ui/Button.svelte`

```svelte
<script lang="ts">
  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    onclick?: () => void;
  }

  let { variant = 'primary', size = 'md', disabled = false, onclick, children } = $props();

  const baseClasses = 'inline-flex items-center justify-center font-medium rounded-lg transition-colors';

  const variantClasses = {
    primary: 'bg-accent-blue text-content-inverse hover:opacity-90',
    secondary: 'bg-surface-secondary text-content-primary hover:bg-surface-tertiary border border-border-primary',
    ghost: 'text-content-secondary hover:text-content-primary hover:bg-surface-secondary',
  };

  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg',
  };
</script>

<button
  class="{baseClasses} {variantClasses[variant]} {sizeClasses[size]}"
  {disabled}
  onclick={onclick}
>
  {@render children?.()}
</button>
```

### 4. Create Card Component

**File:** `src/lib/components/ui/Card.svelte`

```svelte
<script lang="ts">
  interface Props {
    padding?: 'none' | 'sm' | 'md' | 'lg';
  }

  let { padding = 'md', children } = $props();

  const paddingClasses = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  };
</script>

<div class="bg-surface-secondary rounded-xl border border-border-primary {paddingClasses[padding]}">
  {@render children?.()}
</div>
```

### 5. Create Header Component

**File:** `src/lib/components/layout/Header.svelte`

```svelte
<script lang="ts">
  // Header is a draggable region on macOS
</script>

<header class="h-12 bg-surface-primary border-b border-border-secondary flex items-center px-4" data-tauri-drag-region>
  <div class="flex items-center gap-2">
    <span class="text-xl">🦡</span>
    <span class="font-semibold text-content-primary">Molery</span>
  </div>

  <div class="flex-1" data-tauri-drag-region></div>

  <div class="flex items-center gap-2">
    <button class="p-2 text-content-secondary hover:text-content-primary rounded-lg hover:bg-surface-secondary">
      ⚙️
    </button>
  </div>
</header>
```

### 6. Create Sidebar Component

**File:** `src/lib/components/layout/Sidebar.svelte`

```svelte
<script lang="ts">
  import { uiStore } from '$lib/stores/ui.store';
  import type { NavItem } from '$lib/types/common.types';

  const navItems: NavItem[] = [
    { id: 'dashboard', label: 'Dashboard', icon: '🏠', shortcut: '⌘1' },
    { id: 'clean', label: 'Clean', icon: '🧹', shortcut: '⌘2' },
    { id: 'uninstall', label: 'Uninstall', icon: '🗑️', shortcut: '⌘3' },
    { id: 'analyze', label: 'Analyze', icon: '📊', shortcut: '⌘4' },
    { id: 'optimize', label: 'Optimize', icon: '⚡', shortcut: '⌘5' },
    { id: 'status', label: 'Status', icon: 'ℹ️', shortcut: '⌘6' },
  ];
</script>

<aside class="w-52 bg-surface-secondary border-r border-border-secondary flex flex-col">
  <nav class="flex-1 p-2">
    {#each navItems as item}
      <button
        class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors
               {uiStore.activePanel === item.id
                 ? 'bg-surface-tertiary text-accent-blue border-l-3 border-accent-blue'
                 : 'text-content-secondary hover:bg-surface-tertiary hover:text-content-primary'}"
        onclick={() => uiStore.setPanel(item.id)}
      >
        <span class="text-lg">{item.icon}</span>
        <span class="flex-1">{item.label}</span>
        <span class="text-xs text-content-tertiary">{item.shortcut}</span>
      </button>
    {/each}
  </nav>
</aside>
```

### 7. Create Layout Component

**File:** `src/lib/components/layout/Layout.svelte`

```svelte
<script lang="ts">
  import Header from './Header.svelte';
  import Sidebar from './Sidebar.svelte';

  let { children } = $props();
</script>

<div class="h-screen flex flex-col bg-surface-primary">
  <Header />

  <div class="flex-1 flex overflow-hidden">
    <Sidebar />

    <main class="flex-1 overflow-auto p-6">
      {@render children?.()}
    </main>
  </div>
</div>
```

### 8. Update Routes

**File:** `src/routes/+layout.svelte`

```svelte
<script>
  import '../app.css';
  let { children } = $props();
</script>

{@render children?.()}
```

**File:** `src/routes/+page.svelte`

```svelte
<script lang="ts">
  import Layout from '$lib/components/layout/Layout.svelte';
  import { uiStore } from '$lib/stores/ui.store';
</script>

<Layout>
  <div class="max-w-4xl">
    {#if uiStore.activePanel === 'dashboard'}
      <h1 class="text-2xl font-semibold mb-4">Dashboard</h1>
      <p class="text-content-secondary">Welcome to Molery</p>
    {:else if uiStore.activePanel === 'clean'}
      <h1 class="text-2xl font-semibold mb-4">Clean</h1>
      <p class="text-content-secondary">Coming in Phase 4</p>
    {:else}
      <h1 class="text-2xl font-semibold mb-4">{uiStore.activePanel}</h1>
      <p class="text-content-secondary">Coming soon</p>
    {/if}
  </div>
</Layout>
```

## Checklist

- [ ] Common types created
- [ ] UI store created with panel switching
- [ ] Button component created with variants
- [ ] Card component created
- [ ] Header component with drag region
- [ ] Sidebar component with navigation items
- [ ] Layout component combining all parts
- [ ] Navigation works (clicking items changes panel)
- [ ] Dark/light colors respond to system preference

## Verification

```bash
# 1. Run the app
npm run tauri dev

# 2. Verify layout appears correctly:
# - Header at top with logo
# - Sidebar on left with nav items
# - Main content area

# 3. Test navigation:
# - Click each sidebar item
# - Content area should update

# 4. Test dark mode:
# - Change system appearance
# - Colors should adapt
```

## Notes

- Keyboard shortcuts (⌘1-6) will be implemented in Phase 6
- Settings modal will be implemented in Phase 6
- Window dragging requires `data-tauri-drag-region` attribute
