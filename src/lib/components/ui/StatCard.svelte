<script lang="ts">
  import Card from './Card.svelte';
  import type { ComponentType } from 'svelte';

  interface Props {
    icon: string | ComponentType;
    value: string;
    label: string;
    sublabel?: string;
    trend?: 'up' | 'down' | null;
    onclick?: () => void;
  }

  let { icon, value, label, sublabel, trend = null, onclick }: Props = $props();

  // Check if icon is a component (Lucide) or string (emoji)
  let isComponent = $derived(typeof icon !== 'string');
</script>

<Card>
  <button
    class="w-full text-left {onclick ? 'cursor-pointer hover:opacity-80' : 'cursor-default'}"
    onclick={onclick}
    disabled={!onclick}
  >
    <div class="flex items-start gap-3">
      <!-- Icon container with rounded background -->
      <div class="w-9 h-9 rounded-lg bg-surface-tertiary/50 flex items-center justify-center flex-shrink-0">
        {#if isComponent && typeof icon !== 'string'}
          {@const IconComponent = icon}
          <IconComponent size={18} strokeWidth={1.75} class="text-content-secondary" />
        {:else if typeof icon === 'string'}
          <span class="text-lg">{icon}</span>
        {/if}
      </div>
      <div class="flex-1 min-w-0">
        <div class="flex items-baseline gap-2">
          <span class="text-xl font-semibold text-content-primary">{value}</span>
          {#if trend}
            <span class="text-xs {trend === 'up' ? 'text-accent-red' : 'text-accent-green'}">
              {trend === 'up' ? '↑' : '↓'}
            </span>
          {/if}
        </div>
        <p class="text-[13px] text-content-secondary">{label}</p>
        {#if sublabel}
          <p class="text-[11px] text-content-tertiary mt-0.5 truncate">{sublabel}</p>
        {/if}
      </div>
    </div>
  </button>
</Card>
