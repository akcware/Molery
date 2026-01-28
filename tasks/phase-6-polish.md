---
phase: 6
title: Polish
status: done
depends-on: [phase-5]
---

# Phase 6: Polish

## Objective

Add loading states, error handling improvements, keyboard shortcuts, animations, and final UI polish.

## Prerequisites

- Phase 5 completed (All panels implemented and functional)

## Deliverables

| File | Action | Spec Reference |
|------|--------|----------------|
| `src/lib/components/ui/Spinner.svelte` | create | - |
| `src/lib/components/ui/Skeleton.svelte` | create | [ui-screens.md](../docs/specs/ui-screens.md#loading-states) |
| `src/lib/components/ui/ConfirmDialog.svelte` | create | - |
| `src/lib/components/features/SetupScreen.svelte` | create | [setup-flow.md](../docs/flows/setup-flow.md#setup-screen) |
| `src/lib/hooks/useKeyboard.ts` | create | - |
| `src/app.css` | modify | Add animations |
| `src/routes/+page.svelte` | modify | Add setup check, keyboard shortcuts |
| Multiple panel files | modify | Add loading states, error handling |

## Implementation Steps

### 1. Create Loading Components

**File:** `src/lib/components/ui/Spinner.svelte`

```svelte
<script lang="ts">
  interface Props {
    size?: 'sm' | 'md' | 'lg';
  }

  let { size = 'md' }: Props = $props();

  const sizeClasses = {
    sm: 'h-4 w-4 border-2',
    md: 'h-6 w-6 border-2',
    lg: 'h-8 w-8 border-3',
  };
</script>

<div
  class="animate-spin rounded-full border-accent-blue border-t-transparent {sizeClasses[size]}"
></div>
```

**File:** `src/lib/components/ui/Skeleton.svelte`

```svelte
<script lang="ts">
  interface Props {
    width?: string;
    height?: string;
    rounded?: 'none' | 'sm' | 'md' | 'lg' | 'full';
  }

  let { width = '100%', height = '1rem', rounded = 'md' }: Props = $props();

  const roundedClasses = {
    none: '',
    sm: 'rounded',
    md: 'rounded-md',
    lg: 'rounded-lg',
    full: 'rounded-full',
  };
</script>

<div
  class="bg-surface-tertiary animate-pulse {roundedClasses[rounded]}"
  style="width: {width}; height: {height}"
></div>
```

### 2. Create Confirm Dialog

**File:** `src/lib/components/ui/ConfirmDialog.svelte`

```svelte
<script lang="ts">
  import Button from './Button.svelte';
  import Modal from './Modal.svelte';

  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    variant?: 'danger' | 'warning' | 'default';
    onconfirm: () => void;
    oncancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = 'Confirm',
    cancelLabel = 'Cancel',
    variant = 'default',
    onconfirm,
    oncancel
  }: Props = $props();
</script>

<Modal {open} {title} onclose={oncancel}>
  <div class="space-y-4">
    <p class="text-content-secondary">{message}</p>
    <div class="flex justify-end gap-2">
      <Button variant="secondary" onclick={oncancel}>{cancelLabel}</Button>
      <Button onclick={onconfirm}>{confirmLabel}</Button>
    </div>
  </div>
</Modal>
```

### 3. Create Setup Screen

**File:** `src/lib/components/features/SetupScreen.svelte`

```svelte
<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import { checkMoInstalled } from '$lib/services/statusService';
  import { toastStore } from '$lib/stores/toast.store';

  interface Props {
    oninstalled: () => void;
  }

  let { oninstalled }: Props = $props();
  let checking = $state(false);

  async function handleCheck() {
    checking = true;
    try {
      const installed = await checkMoInstalled();
      if (installed) {
        toastStore.success('mo CLI detected! Ready to use.');
        oninstalled();
      } else {
        toastStore.error('mo CLI not found. Please install and try again.');
      }
    } catch {
      toastStore.error('Failed to check mo installation');
    } finally {
      checking = false;
    }
  }

  function copyCommand(command: string) {
    navigator.clipboard.writeText(command);
    toastStore.success('Command copied to clipboard');
  }

  function openLink(url: string) {
    window.open(url, '_blank');
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-surface-primary p-8">
  <div class="max-w-lg w-full space-y-6">
    <div class="text-center">
      <span class="text-6xl">🔧</span>
      <h1 class="text-2xl font-semibold text-content-primary mt-4">Molery Setup</h1>
      <p class="text-content-secondary mt-2">
        Molery requires the 'mo' CLI tool to work. Please install it using one of these methods:
      </p>
    </div>

    <Card>
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <span>🍺</span>
          <span class="font-medium text-content-primary">Homebrew (Recommended)</span>
        </div>
        <code class="block bg-surface-tertiary rounded-lg px-3 py-2 text-sm font-mono text-content-primary">
          brew install tw93/brew/mole
        </code>
        <div class="flex gap-2">
          <Button variant="secondary" size="sm" onclick={() => copyCommand('brew install tw93/brew/mole')}>
            Copy Command
          </Button>
        </div>
      </div>
    </Card>

    <Card>
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <span>📦</span>
          <span class="font-medium text-content-primary">Cargo (Rust)</span>
        </div>
        <code class="block bg-surface-tertiary rounded-lg px-3 py-2 text-sm font-mono text-content-primary">
          cargo install mole
        </code>
        <Button variant="secondary" size="sm" onclick={() => copyCommand('cargo install mole')}>
          Copy Command
        </Button>
      </div>
    </Card>

    <Card>
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <span>📥</span>
          <span class="font-medium text-content-primary">Manual Download</span>
        </div>
        <p class="text-sm text-content-secondary">Download from GitHub releases</p>
        <Button variant="secondary" size="sm" onclick={() => openLink('https://github.com/tw93/Mole/releases')}>
          Open GitHub
        </Button>
      </div>
    </Card>

    <div class="text-center">
      <Button onclick={handleCheck} disabled={checking}>
        {checking ? '🔄 Checking...' : '🔄 Check Again'}
      </Button>
    </div>
  </div>
</div>
```

### 4. Create Keyboard Hook

**File:** `src/lib/hooks/useKeyboard.ts`

```typescript
import { onMount } from 'svelte';
import { uiStore } from '$lib/stores/ui.store';
import type { Panel } from '$lib/types/common.types';

const panelShortcuts: Record<string, Panel> = {
  '1': 'dashboard',
  '2': 'clean',
  '3': 'uninstall',
  '4': 'analyze',
  '5': 'optimize',
  '6': 'status',
};

export function useKeyboard() {
  onMount(() => {
    function handleKeydown(event: KeyboardEvent) {
      // Check for Cmd/Ctrl + number
      if (event.metaKey || event.ctrlKey) {
        const panel = panelShortcuts[event.key];
        if (panel) {
          event.preventDefault();
          uiStore.setPanel(panel);
        }
      }

      // Escape to close modals (handled by individual components)
    }

    window.addEventListener('keydown', handleKeydown);

    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });
}
```

### 5. Add CSS Animations

**File:** `src/app.css` (add to existing)

```css
/* Animations */
@keyframes fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slide-in-right {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes slide-in-up {
  from {
    transform: translateY(10px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.animate-fade-in {
  animation: fade-in 0.2s ease-out;
}

.animate-slide-in-right {
  animation: slide-in-right 0.3s ease-out;
}

.animate-slide-in-up {
  animation: slide-in-up 0.2s ease-out;
}

/* Toast animation */
.toast-enter {
  animation: slide-in-right 0.3s ease-out;
}

/* Panel transitions */
.panel-content {
  animation: slide-in-up 0.2s ease-out;
}

/* Hover transitions */
.hover-lift {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.hover-lift:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* Focus styles */
*:focus-visible {
  outline: 2px solid var(--color-accent-blue);
  outline-offset: 2px;
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--color-surface-secondary);
}

::-webkit-scrollbar-thumb {
  background: var(--color-border-primary);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--color-content-tertiary);
}
```

### 6. Update Main Page with Setup Check

**File:** `src/routes/+page.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import Layout from '$lib/components/layout/Layout.svelte';
  import Dashboard from '$lib/components/features/Dashboard.svelte';
  import CleanPanel from '$lib/components/features/CleanPanel.svelte';
  import UninstallPanel from '$lib/components/features/UninstallPanel.svelte';
  import AnalyzePanel from '$lib/components/features/AnalyzePanel.svelte';
  import OptimizePanel from '$lib/components/features/OptimizePanel.svelte';
  import StatusPanel from '$lib/components/features/StatusPanel.svelte';
  import SetupScreen from '$lib/components/features/SetupScreen.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { uiStore } from '$lib/stores/ui.store';
  import { checkMoInstalled } from '$lib/services/statusService';
  import { useKeyboard } from '$lib/hooks/useKeyboard';

  let moInstalled = $state<boolean | null>(null);
  let checking = $state(true);

  useKeyboard();

  onMount(async () => {
    try {
      moInstalled = await checkMoInstalled();
    } catch {
      moInstalled = false;
    } finally {
      checking = false;
    }
  });

  function handleInstalled() {
    moInstalled = true;
  }
</script>

{#if checking}
  <div class="min-h-screen flex items-center justify-center bg-surface-primary">
    <div class="text-center space-y-4">
      <Spinner size="lg" />
      <p class="text-content-secondary">Loading...</p>
    </div>
  </div>
{:else if !moInstalled}
  <SetupScreen oninstalled={handleInstalled} />
{:else}
  <Layout>
    <div class="max-w-4xl panel-content">
      {#if uiStore.activePanel === 'dashboard'}
        <Dashboard />
      {:else if uiStore.activePanel === 'clean'}
        <CleanPanel />
      {:else if uiStore.activePanel === 'uninstall'}
        <UninstallPanel />
      {:else if uiStore.activePanel === 'analyze'}
        <AnalyzePanel />
      {:else if uiStore.activePanel === 'optimize'}
        <OptimizePanel />
      {:else if uiStore.activePanel === 'status'}
        <StatusPanel />
      {/if}
    </div>
  </Layout>
{/if}
```

### 7. Update Toast with Animation

**File:** `src/lib/components/ui/Toast.svelte` (update)

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

<div class="toast-enter flex items-center gap-3 px-4 py-3 rounded-lg border {colors[toast.type]} shadow-lg bg-surface-elevated">
  <span class="text-lg">{icons[toast.type]}</span>
  <p class="flex-1 text-sm text-content-primary">{toast.message}</p>
  <button
    class="text-content-secondary hover:text-content-primary transition-colors"
    onclick={() => toastStore.dismiss(toast.id)}
  >
    ✕
  </button>
</div>
```

### 8. Add Confirmation Before Cleanup

**Update:** `src/lib/components/features/CleanPanel.svelte`

Add confirmation dialog before running cleanup:

```svelte
<!-- Add to CleanPanel.svelte -->
<script>
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';

  let showConfirm = $state(false);

  function requestClean() {
    if (cleanStore.selectedCategories.size === 0) {
      toastStore.warning('Please select at least one category');
      return;
    }
    showConfirm = true;
  }

  async function confirmClean() {
    showConfirm = false;
    await handleClean();
  }
</script>

<!-- Replace the Clean button with: -->
<Button
  size="lg"
  onclick={requestClean}
  disabled={cleanStore.isCleaning || cleanStore.selectedCategories.size === 0}
>
  🧹 Clean Selected ({formatBytes(selectedSize())})
</Button>

<ConfirmDialog
  open={showConfirm}
  title="Confirm Cleanup"
  message="Are you sure you want to delete {formatBytes(selectedSize())} of files? This action cannot be undone."
  confirmLabel="Clean"
  onconfirm={confirmClean}
  oncancel={() => showConfirm = false}
/>
```

## Checklist

- [ ] Spinner component created
- [ ] Skeleton component created
- [ ] ConfirmDialog component created
- [ ] SetupScreen component created
- [ ] Keyboard shortcuts implemented (⌘1-6)
- [ ] CSS animations added
- [ ] Toast animations working
- [ ] Panel transitions smooth
- [ ] Setup check on app launch
- [ ] Confirmation dialog before cleanup
- [ ] Focus styles visible for accessibility
- [ ] Scrollbar styled
- [ ] Loading states in all async operations
- [ ] Error messages user-friendly

## Verification

```bash
# 1. Run the app
npm run tauri dev

# 2. Test setup flow:
# - Uninstall mo temporarily
# - Launch app → shows setup screen
# - Install mo
# - Click "Check Again" → proceeds to main app

# 3. Test keyboard shortcuts:
# - Press ⌘1 → Dashboard
# - Press ⌘2 → Clean
# - etc.

# 4. Test animations:
# - Switch panels → smooth slide-in
# - Toast appears → slide from right
# - Hover cards → subtle lift

# 5. Test confirmation:
# - Go to Clean panel
# - Scan and select categories
# - Click Clean → confirmation dialog appears
# - Cancel → stays on page
# - Confirm → cleanup runs

# 6. Test loading states:
# - All async operations show spinners
# - Skeleton loaders where appropriate
```

## Notes

- Consider adding settings modal for theme toggle
- Consider adding "About" dialog
- Consider adding update checker
- Performance: lazy-load panel components if needed
