<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    padding?: 'none' | 'sm' | 'md' | 'lg';
    hoverable?: boolean;
    children?: Snippet;
  }

  let { padding = 'md', hoverable = false, children }: Props = $props();

  const paddingClasses = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  };
</script>

<div class="bg-surface-secondary rounded-xl border border-border-primary {paddingClasses[padding]} {hoverable ? 'hover-lift' : ''}">
  {#if children}
    {@render children()}
  {/if}
</div>
