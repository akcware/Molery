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

  let elapsed = $state(0);
  let timer: ReturnType<typeof setInterval> | undefined;

  function startTimer() {
    elapsed = 0;
    timer = setInterval(() => (elapsed += 1), 1000);
  }

  function stopTimer() {
    if (timer) clearInterval(timer);
    timer = undefined;
  }

  function formatElapsed(seconds: number) {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return m > 0 ? `${m}m ${s}s` : `${s}s`;
  }

  async function handleScan() {
    cleanStore.setScanning(true);
    startTimer();
    try {
      const result = await scanCleanup();
      cleanStore.setScanResult(result);
      toastStore.success(`Found ${formatBytes(result.totalSize)} to clean`);
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Scan failed');
    } finally {
      stopTimer();
      cleanStore.setScanning(false);
    }
  }

  async function confirmClean() {
    showConfirm = false;
    cleanStore.setCleaning(true);
    try {
      const result = await runCleanup();
      toastStore.success(`Freed ${formatBytes(result.freedSize)}`);
      if (result.errors.length > 0) {
        toastStore.warning(`${result.errors.length} item(s) could not be removed`);
      }
      cleanStore.reset();
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Cleanup failed');
    } finally {
      cleanStore.setCleaning(false);
    }
  }

  let totalSize = $derived(cleanStore.scanResult?.totalSize ?? 0);
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-xl font-semibold text-content-primary">System Cleanup</h1>
    <p class="text-[13px] text-content-secondary">Scan and remove unnecessary files</p>
  </div>

  {#if cleanStore.isScanning}
    <Card>
      <div class="flex flex-col items-center justify-center py-8 gap-4">
        <Spinner size="lg" />
        <div class="text-center">
          <p class="text-content-primary font-medium">Scanning your system... {formatElapsed(elapsed)}</p>
          <p class="text-sm text-content-secondary mt-1">Looking for cache files, logs, and other cleanable items</p>
          <p class="text-[13px] text-content-tertiary mt-2">
            Mole reports no progress while it works, so this stays on screen until it
            finishes — a first scan can take several minutes.
          </p>
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
            <div class="flex items-center gap-3 px-4 py-3">
              <span class="flex-1 text-content-primary">{categoryLabels[category]}</span>
              <span class="text-content-secondary">{formatBytes(size)}</span>
            </div>
          {/if}
        {/each}
      </div>
    </Card>

    <p class="text-[13px] text-content-secondary">
      The Mole CLI cleans everything it finds — it has no per-category option. This
      breakdown is a preview of what will be removed, not a selection, and the
      per-category figures are approximate; the total above is Mole's own number.
    </p>

    <Button size="lg" onclick={() => (showConfirm = true)} disabled={cleanStore.isCleaning}>
      {#if cleanStore.isCleaning}
        <span class="inline-flex items-center gap-2">
          <Spinner size="sm" /> Cleaning...
        </span>
      {:else}
        Clean All ({formatBytes(totalSize)})
      {/if}
    </Button>
  {/if}
</div>

<ConfirmDialog
  open={showConfirm}
  title="Confirm Cleanup"
  message="This runs a full Mole cleanup and removes everything it finds, up to {formatBytes(totalSize)}. This action cannot be undone."
  confirmLabel="Clean All"
  variant="warning"
  onconfirm={confirmClean}
  oncancel={() => showConfirm = false}
/>
