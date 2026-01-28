<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'default' | 'outlined' | 'ghost';
    padding?: 'none' | 'sm' | 'md' | 'lg';
    hoverable?: boolean;
    children?: Snippet;
  }

  let { variant = 'default', padding = 'md', hoverable = false, children }: Props = $props();

  const paddingClasses = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  };

  const variantClasses = {
    default: 'bg-surface-secondary border border-border-primary shadow-sm',
    outlined: 'bg-transparent border border-border-primary',
    ghost: 'bg-transparent',
  };
</script>

<div
  class="rounded-xl transition-all duration-150 {variantClasses[variant]} {paddingClasses[padding]} {hoverable ? 'hover:shadow-md hover:-translate-y-0.5' : ''}"
>
  {#if children}
    {@render children()}
  {/if}
</div>
