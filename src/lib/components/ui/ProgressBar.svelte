<script lang="ts">
  interface Props {
    value: number;
    max?: number;
    showLabel?: boolean;
  }

  let { value, max = 100, showLabel = false }: Props = $props();

  let percentage = $derived(Math.min(100, (value / max) * 100));
</script>

<div class="space-y-1">
  <div class="h-2 bg-surface-tertiary rounded-full overflow-hidden">
    <div
      class="h-full bg-accent-blue transition-all duration-300"
      style="width: {percentage}%"
    ></div>
  </div>
  {#if showLabel}
    <p class="text-xs text-content-secondary text-right">{percentage.toFixed(1)}%</p>
  {/if}
</div>
