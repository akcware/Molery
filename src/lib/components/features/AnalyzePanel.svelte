<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import { analyzePath } from '$lib/services/analyzeService';
  import { getHomeDir } from '$lib/services/statusService';
  import { toastStore } from '$lib/stores/toast.store.svelte';
  import { formatBytes } from '$lib/utils/format';
  import type { AnalyzeResult, DiskItem } from '$lib/types/analyze.types';
  import Folder from 'lucide-svelte/icons/folder';
  import FileText from 'lucide-svelte/icons/file-text';

  let path = $state('');
  let result = $state<AnalyzeResult | null>(null);
  let loading = $state(false);

  onMount(async () => {
    try {
      path = await getHomeDir();
    } catch {
      // Fallback if home dir detection fails
      path = '/Users';
    }
  });

  async function handleAnalyze() {
    if (!path) {
      toastStore.warning('Please enter a path');
      return;
    }

    loading = true;
    try {
      result = await analyzePath(path);
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Analysis failed');
    } finally {
      loading = false;
    }
  }

  function drillDown(item: DiskItem) {
    if (item.isDirectory) {
      path = item.path;
      handleAnalyze();
    }
  }
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-xl font-semibold text-content-primary">Disk Analyzer</h1>
    <p class="text-[13px] text-content-secondary">Visualize disk usage by folder</p>
  </div>

  <div class="flex gap-2">
    <input
      type="text"
      placeholder="/Users/username"
      bind:value={path}
      disabled={loading}
      class="flex-1 px-4 py-2 bg-surface-secondary border border-border-primary rounded-lg text-content-primary disabled:opacity-50"
    />
    <Button onclick={handleAnalyze} disabled={loading}>
      {#if loading}
        <span class="inline-flex items-center gap-2">
          <Spinner size="sm" /> Analyzing...
        </span>
      {:else}
        Analyze
      {/if}
    </Button>
  </div>

  {#if loading}
    <Card>
      <div class="flex flex-col items-center justify-center py-8 gap-4">
        <Spinner size="lg" />
        <div class="text-center">
          <p class="text-content-primary font-medium">Analyzing disk usage...</p>
          <p class="text-sm text-content-secondary mt-1">Scanning {path}</p>
        </div>
      </div>
    </Card>
  {:else if result}
    <div>
      <p class="text-sm text-content-secondary mb-2">
        {result.path} ({formatBytes(result.totalSize)})
      </p>
      <Card padding="none">
        <div class="divide-y divide-border-secondary">
          {#each result.items.slice(0, 10) as item}
            <button
              class="w-full flex items-center gap-3 px-4 py-3 hover:bg-surface-tertiary text-left transition-colors"
              onclick={() => drillDown(item)}
            >
              {#if item.isDirectory}
                <Folder size={16} strokeWidth={1.75} class="text-content-tertiary flex-shrink-0" />
              {:else}
                <FileText size={16} strokeWidth={1.75} class="text-content-tertiary flex-shrink-0" />
              {/if}
              <div class="flex-1 min-w-0">
                <p class="text-content-primary truncate">{item.name}</p>
                <ProgressBar value={item.size} max={result.totalSize} />
              </div>
              <span class="text-sm text-content-secondary whitespace-nowrap">
                {formatBytes(item.size)}
              </span>
            </button>
          {/each}
        </div>
      </Card>
    </div>

    <p class="text-xs text-content-tertiary">Click on a folder to drill down</p>
  {/if}
</div>
