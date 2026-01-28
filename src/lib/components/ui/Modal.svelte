<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    title: string;
    onclose: () => void;
    children?: Snippet;
  }

  let { open, title, onclose, children }: Props = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center animate-fade-in">
    <button
      class="absolute inset-0 bg-black/50"
      onclick={onclose}
      aria-label="Close modal"
    ></button>
    <div class="relative bg-surface-elevated rounded-xl shadow-xl max-w-md w-full mx-4 animate-slide-in-up">
      <div class="flex items-center justify-between p-4 border-b border-border-secondary">
        <h2 class="text-lg font-semibold text-content-primary">{title}</h2>
        <button
          class="text-content-secondary hover:text-content-primary"
          onclick={onclose}
          aria-label="Close"
        >
          ✕
        </button>
      </div>
      <div class="p-4">
        {#if children}
          {@render children()}
        {/if}
      </div>
    </div>
  </div>
{/if}
