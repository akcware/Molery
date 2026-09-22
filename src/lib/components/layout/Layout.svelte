<script lang="ts">
  import type { Snippet } from 'svelte';
  import Header from './Header.svelte';
  import Sidebar from './Sidebar.svelte';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="h-screen flex flex-col vibrancy">
  <Header />

  <div class="flex-1 flex overflow-hidden">
    <Sidebar />

    <main class="flex-1 overflow-auto p-6 bg-surface-primary">
      {#if children}
        {@render children()}
      {/if}
    </main>
  </div>
</div>
