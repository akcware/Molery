<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import { cleanStore } from '$lib/stores/clean.store.svelte';
  import { toastStore } from '$lib/stores/toast.store.svelte';
  import { scanCleanup, runCleanup } from '$lib/services/cleanService';
  import { formatBytes } from '$lib/utils/format';
  import type { CleanupCategory } from '$lib/types/clean.types';

  const categoryLabels: Record<CleanupCategory, string> = {
    cache: 'Cache Files',
    logs: 'System Logs',
    trash: 'Trash',
    downloads: 'Downloads',
    xcode: 'Xcode Derived Data',
    homebrew: 'Homebrew Cache'
  };

  const categoryOrder: CleanupCategory[] = [
    'cache',
    'logs',
    'trash',
    'downloads',
    'xcode',
    'homebrew'
  ];

  let showConfirm = $state(false);

  async function handleScan() {
    cleanStore.setScanning(true);
    try {
      const result = await scanCleanup();
      cleanStore.setScanResult(result);
      toastStore.success(`Found ${formatBytes(result.totalSize)} to clean`);
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Scan failed');
    } finally {
      cleanStore.setScanning(false);
    }
  }

  function requestClean() {
    if (cleanStore.selectedCategories.size === 0) {
      toastStore.warning('Please select at least one category');
      return;
    }
    showConfirm = true;
  }

  async function confirmClean() {
    showConfirm = false;
    await handleClean();
  }

  async function handleClean() {
    const categories = Array.from(cleanStore.selectedCategories);
    if (categories.length === 0) {
      toastStore.warning('Please select at least one category');
      return;
    }

    cleanStore.setCleaning(true);
    try {
      const result = await runCleanup(categories);
      toastStore.success(`Freed ${formatBytes(result.freedSize)}`);
      cleanStore.reset();
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Cleanup failed');
    } finally {
      cleanStore.setCleaning(false);
    }
  }

  let selectedSize = $derived(() => {
    if (!cleanStore.scanResult) return 0;
    let total = 0;
    for (const cat of cleanStore.selectedCategories) {
      total += cleanStore.scanResult.categories[cat] || 0;
    }
    return total;
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Cleanup</h1>
    <p class="text-content-secondary">Scan and remove unnecessary files</p>
  </div>

  {#if cleanStore.isScanning}
    <Card>
      <div class="flex flex-col items-center justify-center py-8 gap-4">
        <Spinner size="lg" />
        <div class="text-center">
          <p class="text-content-primary font-medium">Scanning your system...</p>
          <p class="text-sm text-content-secondary mt-1">Looking for cache files, logs, and other cleanable items</p>
        </div>
      </div>
    </Card>
  {:else}
    <div class="flex items-center gap-4">
      <Button onclick={handleScan} disabled={cleanStore.isScanning}>
        Scan Now
      </Button>

      {#if cleanStore.scanResult}
        <span class="text-content-secondary">
          Total: <strong class="text-content-primary"
            >{formatBytes(cleanStore.scanResult.totalSize)}</strong
          > cleanable
        </span>
      {/if}
    </div>
  {/if}

  {#if cleanStore.scanResult}
    <Card padding="none">
      <div class="divide-y divide-border-secondary">
        {#each categoryOrder as category}
          {@const size = cleanStore.scanResult.categories[category]}
          {#if size > 0}
            <label
              class="flex items-center gap-3 px-4 py-3 hover:bg-surface-tertiary cursor-pointer transition-colors"
            >
              <input
                type="checkbox"
                checked={cleanStore.selectedCategories.has(category)}
                onchange={() => cleanStore.toggleCategory(category)}
                class="w-4 h-4 accent-accent-blue"
              />
              <span class="flex-1 text-content-primary">{categoryLabels[category]}</span>
              <span class="text-content-secondary">{formatBytes(size)}</span>
            </label>
          {/if}
        {/each}
      </div>
    </Card>

    <div class="flex items-center gap-4">
      <Button variant="ghost" onclick={() => cleanStore.selectAll()}>Select All</Button>
      <Button variant="ghost" onclick={() => cleanStore.deselectAll()}>Deselect All</Button>
    </div>

    <Button
      size="lg"
      onclick={requestClean}
      disabled={cleanStore.isCleaning || cleanStore.selectedCategories.size === 0}
    >
      {#if cleanStore.isCleaning}
        <span class="inline-flex items-center gap-2">
          <Spinner size="sm" /> Cleaning...
        </span>
      {:else}
        Clean Selected ({formatBytes(selectedSize())})
      {/if}
    </Button>
  {/if}
</div>

<ConfirmDialog
  open={showConfirm}
  title="Confirm Cleanup"
  message="Are you sure you want to delete {formatBytes(selectedSize())} of files? This action cannot be undone."
  confirmLabel="Clean"
  variant="warning"
  onconfirm={confirmClean}
  oncancel={() => showConfirm = false}
/>
