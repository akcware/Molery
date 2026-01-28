<script lang="ts">
  import Card from './Card.svelte';

  interface Props {
    icon: string;
    value: string;
    label: string;
    sublabel?: string;
    trend?: 'up' | 'down' | null;
    onclick?: () => void;
  }

  let { icon, value, label, sublabel, trend = null, onclick }: Props = $props();
</script>

<Card>
  <button
    class="w-full text-left {onclick ? 'cursor-pointer hover:opacity-80' : 'cursor-default'}"
    onclick={onclick}
    disabled={!onclick}
  >
    <div class="flex items-start gap-3">
      <span class="text-2xl">{icon}</span>
      <div class="flex-1">
        <div class="flex items-baseline gap-2">
          <span class="text-2xl font-semibold text-content-primary">{value}</span>
          {#if trend}
            <span class="text-sm {trend === 'up' ? 'text-accent-red' : 'text-accent-green'}">
              {trend === 'up' ? '↑' : '↓'}
            </span>
          {/if}
        </div>
        <p class="text-sm text-content-secondary">{label}</p>
        {#if sublabel}
          <p class="text-xs text-content-tertiary mt-1">{sublabel}</p>
        {/if}
      </div>
    </div>
  </button>
</Card>
