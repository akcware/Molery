<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import { formatBytes, formatPercentage } from '$lib/utils/format';
  import { statusStore } from '$lib/stores/status.store.svelte';

  onMount(() => {
    statusStore.loadStatus();
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Status</h1>
    <p class="text-content-secondary">Overview of your system health</p>
  </div>

  {#if statusStore.loading}
    <div class="text-content-secondary">Loading...</div>
  {:else if statusStore.status}
    <Card>
      <h3 class="font-medium text-content-primary mb-3">Disk Space</h3>
      <div class="space-y-2">
        <ProgressBar value={statusStore.status.diskUsed} max={statusStore.status.diskTotal} />
        <div class="flex justify-between text-sm">
          <span class="text-content-secondary">
            {formatBytes(statusStore.status.diskUsed)} / {formatBytes(statusStore.status.diskTotal)}
          </span>
          <span class="text-content-secondary">
            {formatPercentage(statusStore.status.diskUsed, statusStore.status.diskTotal)} used
          </span>
        </div>
      </div>
    </Card>

    <Card>
      <h3 class="font-medium text-content-primary mb-3">mo CLI Status</h3>
      <div class="flex items-center gap-2">
        <span class="{statusStore.moInstalled ? 'text-accent-green' : 'text-accent-red'}">
          {statusStore.moInstalled ? '✓' : '✕'}
        </span>
        <span class="text-content-primary">
          {statusStore.moInstalled ? 'Installed' : 'Not Installed'}
        </span>
        {#if statusStore.moInstalled}
          <span class="text-content-secondary text-sm">
            Version {statusStore.status.moVersion}
          </span>
        {/if}
      </div>
    </Card>

    {#if statusStore.status.cleanableSize > 0}
      <Card>
        <h3 class="font-medium text-content-primary mb-3">Recommendations</h3>
        <ul class="text-sm text-content-secondary space-y-1 list-disc list-inside">
          <li>Clean {formatBytes(statusStore.status.cleanableSize)} of temporary files</li>
        </ul>
      </Card>
    {/if}
  {/if}
</div>
