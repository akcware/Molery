<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    onclick?: () => void;
    children?: Snippet;
  }

  let { variant = 'primary', size = 'md', disabled = false, onclick, children }: Props = $props();

  const baseClasses = 'inline-flex items-center justify-center font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed';

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
  {#if children}
    {@render children()}
  {/if}
</button>
