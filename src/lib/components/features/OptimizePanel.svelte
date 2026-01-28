<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { clearDnsCache, rebuildSpotlightIndex, freeMemory } from '$lib/services/optimizeService';
  import { toastStore } from '$lib/stores/toast.store.svelte';

  let loadingDns = $state(false);
  let loadingSpotlight = $state(false);
  let loadingMemory = $state(false);

  async function handleClearDns() {
    loadingDns = true;
    try {
      const result = await clearDnsCache();
      if (result.success) {
        toastStore.success(result.message);
      } else {
        toastStore.warning(result.message);
      }
    } catch (error) {
      toastStore.error(`Failed to clear DNS cache: ${error}`);
    } finally {
      loadingDns = false;
    }
  }

  async function handleRebuildSpotlight() {
    loadingSpotlight = true;
    try {
      const result = await rebuildSpotlightIndex();
      if (result.success) {
        toastStore.success(result.message);
      } else {
        toastStore.warning(result.message);
      }
    } catch (error) {
      toastStore.error(`Failed to rebuild Spotlight index: ${error}`);
    } finally {
      loadingSpotlight = false;
    }
  }

  async function handleFreeMemory() {
    loadingMemory = true;
    try {
      const result = await freeMemory();
      if (result.success) {
        toastStore.success(result.message);
      } else {
        toastStore.warning(result.message);
      }
    } catch (error) {
      toastStore.error(`Failed to free memory: ${error}`);
    } finally {
      loadingMemory = false;
    }
  }
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Optimize</h1>
    <p class="text-content-secondary">Optimize system performance</p>
  </div>

  <Card>
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <p class="font-medium text-content-primary">Clear DNS Cache</p>
          <p class="text-sm text-content-secondary">Flush the DNS resolver cache</p>
        </div>
        <Button variant="secondary" size="sm" onclick={handleClearDns} disabled={loadingDns}>
          {#if loadingDns}
            <span class="inline-flex items-center gap-2">
              <Spinner size="sm" /> Running...
            </span>
          {:else}
            Run
          {/if}
        </Button>
      </div>

      <div class="flex items-center justify-between">
        <div>
          <p class="font-medium text-content-primary">Rebuild Spotlight Index</p>
          <p class="text-sm text-content-secondary">Reindex Spotlight search</p>
        </div>
        <Button variant="secondary" size="sm" onclick={handleRebuildSpotlight} disabled={loadingSpotlight}>
          {#if loadingSpotlight}
            <span class="inline-flex items-center gap-2">
              <Spinner size="sm" /> Running...
            </span>
          {:else}
            Run
          {/if}
        </Button>
      </div>

      <div class="flex items-center justify-between">
        <div>
          <p class="font-medium text-content-primary">Free Memory</p>
          <p class="text-sm text-content-secondary">Purge inactive memory</p>
        </div>
        <Button variant="secondary" size="sm" onclick={handleFreeMemory} disabled={loadingMemory}>
          {#if loadingMemory}
            <span class="inline-flex items-center gap-2">
              <Spinner size="sm" /> Running...
            </span>
          {:else}
            Run
          {/if}
        </Button>
      </div>
    </div>
  </Card>

  <p class="text-xs text-content-tertiary">
    Note: Some operations may require administrator privileges.
  </p>
</div>
