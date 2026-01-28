<script lang="ts">
  import type { Toast } from '$lib/types/common.types';
  import { toastStore } from '$lib/stores/toast.store.svelte';

  interface Props {
    toast: Toast;
  }

  let { toast }: Props = $props();

  const icons: Record<Toast['type'], string> = {
    success: '✓',
    error: '✕',
    warning: '⚠',
    info: 'ℹ'
  };

  const colors: Record<Toast['type'], string> = {
    success: 'border-accent-green bg-accent-green/10',
    error: 'border-accent-red bg-accent-red/10',
    warning: 'border-accent-orange bg-accent-orange/10',
    info: 'border-accent-blue bg-accent-blue/10'
  };
</script>

<div class="toast-enter flex items-center gap-3 px-4 py-3 rounded-lg border {colors[toast.type]} shadow-lg bg-surface-elevated">
  <span class="text-lg">{icons[toast.type]}</span>
  <p class="flex-1 text-sm text-content-primary">{toast.message}</p>
  <button
    class="text-content-secondary hover:text-content-primary"
    onclick={() => toastStore.dismiss(toast.id)}
  >
    ✕
  </button>
</div>
