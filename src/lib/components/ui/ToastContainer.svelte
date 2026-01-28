<script lang="ts">
  import Toast from './Toast.svelte';
  import { toastStore } from '$lib/stores/toast.store.svelte';
</script>

{#if toastStore.all.length > 0}
  <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 w-80">
    {#each toastStore.all as toast (toast.id)}
      <Toast {toast} />
    {/each}
  </div>
{/if}
