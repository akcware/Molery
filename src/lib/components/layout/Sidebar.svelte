<script lang="ts">
  import { uiStore } from '$lib/stores/ui.store.svelte';
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
