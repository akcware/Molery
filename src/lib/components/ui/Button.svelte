<script lang="ts">
  import type { Snippet } from 'svelte';
  import Loader2 from 'lucide-svelte/icons/loader-2';

  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    loading?: boolean;
    onclick?: () => void;
    children?: Snippet;
  }

  let { variant = 'primary', size = 'md', disabled = false, loading = false, onclick, children }: Props = $props();

  const baseClasses = 'inline-flex items-center justify-center gap-2 font-medium rounded-lg transition-all duration-150 disabled:opacity-50 disabled:cursor-not-allowed';

  const variantClasses = {
    primary: 'bg-accent-blue text-content-inverse hover:opacity-90 active:opacity-80',
    secondary: 'bg-surface-secondary text-content-primary hover:bg-surface-tertiary border border-border-primary',
    ghost: 'text-content-secondary hover:text-content-primary hover:bg-surface-tertiary',
    danger: 'bg-accent-red text-content-inverse hover:opacity-90 active:opacity-80',
  };

  const sizeClasses = {
    sm: 'h-7 px-2.5 text-[12px]',
    md: 'h-8 px-3 text-[13px]',
    lg: 'h-9 px-4 text-[14px]',
  };

  let isDisabled = $derived(disabled || loading);
</script>

<button
  class="{baseClasses} {variantClasses[variant]} {sizeClasses[size]}"
  disabled={isDisabled}
  onclick={onclick}
>
  {#if loading}
    <Loader2 size={size === 'sm' ? 12 : size === 'md' ? 14 : 16} class="animate-spin" />
  {/if}
  {#if children}
    {@render children()}
  {/if}
</button>
