<script lang="ts">
  import { uiStore } from '$lib/stores/ui.store.svelte';
  import type { NavItem } from '$lib/types/common.types';
  import Home from 'lucide-svelte/icons/home';
  import Brush from 'lucide-svelte/icons/brush';
  import Trash2 from 'lucide-svelte/icons/trash-2';
  import BarChart3 from 'lucide-svelte/icons/bar-chart-3';
  import Zap from 'lucide-svelte/icons/zap';
  import Info from 'lucide-svelte/icons/info';
  import type { ComponentType } from 'svelte';

  interface NavItemWithIcon extends Omit<NavItem, 'icon'> {
    icon: ComponentType;
  }

  const navItems: NavItemWithIcon[] = [
    { id: 'dashboard', label: 'Dashboard', icon: Home, shortcut: '1' },
    { id: 'clean', label: 'Clean', icon: Brush, shortcut: '2' },
    { id: 'uninstall', label: 'Uninstall', icon: Trash2, shortcut: '3' },
    { id: 'analyze', label: 'Analyze', icon: BarChart3, shortcut: '4' },
    { id: 'optimize', label: 'Optimize', icon: Zap, shortcut: '5' },
    { id: 'status', label: 'Status', icon: Info, shortcut: '6' },
  ];
</script>

<aside class="w-52 vibrancy border-r border-border-secondary flex flex-col pt-12">
  <nav class="flex-1 px-2 py-2">
    {#each navItems as item}
      {@const IconComponent = item.icon}
      <button
        class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-md text-left transition-all duration-150
               {uiStore.activePanel === item.id
                 ? 'bg-surface-tertiary/80 text-content-primary font-medium'
                 : 'text-content-secondary hover:bg-surface-tertiary/50 hover:text-content-primary'}"
        onclick={() => uiStore.setPanel(item.id)}
      >
        <IconComponent size={16} strokeWidth={1.75} class="flex-shrink-0" />
        <span class="flex-1 text-[13px]">{item.label}</span>
        <span class="text-[11px] text-content-tertiary font-mono">{item.shortcut}</span>
      </button>
    {/each}
  </nav>
</aside>
