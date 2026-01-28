<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import { getStatus, checkMoInstalled } from '$lib/services/statusService';
  import { formatBytes, formatPercentage } from '$lib/utils/format';
  import type { SystemStatus } from '$lib/types/status.types';

  let status = $state<SystemStatus | null>(null);
  let moInstalled = $state(false);
  let loading = $state(true);

  onMount(async () => {
    try {
      [status, moInstalled] = await Promise.all([
        getStatus(),
        checkMoInstalled()
      ]);
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">System Status</h1>
    <p class="text-content-secondary">Overview of your system health</p>
  </div>

  {#if loading}
    <div class="text-content-secondary">Loading...</div>
  {:else if status}
    <Card>
      <h3 class="font-medium text-content-primary mb-3">Disk Space</h3>
      <div class="space-y-2">
        <ProgressBar value={status.diskUsed} max={status.diskTotal} />
        <div class="flex justify-between text-sm">
          <span class="text-content-secondary">
            {formatBytes(status.diskUsed)} / {formatBytes(status.diskTotal)}
          </span>
          <span class="text-content-secondary">
            {formatPercentage(status.diskUsed, status.diskTotal)} used
          </span>
        </div>
      </div>
    </Card>

    <Card>
      <h3 class="font-medium text-content-primary mb-3">mo CLI Status</h3>
      <div class="flex items-center gap-2">
        <span class="{moInstalled ? 'text-accent-green' : 'text-accent-red'}">
          {moInstalled ? '✓' : '✕'}
        </span>
        <span class="text-content-primary">
          {moInstalled ? 'Installed' : 'Not Installed'}
        </span>
        {#if moInstalled}
          <span class="text-content-secondary text-sm">
            Version {status.moVersion}
          </span>
        {/if}
      </div>
    </Card>

    {#if status.cleanableSize > 0}
      <Card>
        <h3 class="font-medium text-content-primary mb-3">Recommendations</h3>
        <ul class="text-sm text-content-secondary space-y-1 list-disc list-inside">
          <li>Clean {formatBytes(status.cleanableSize)} of temporary files</li>
        </ul>
      </Card>
    {/if}
  {/if}
</div>
