<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import ProgressBar from '$lib/components/ui/ProgressBar.svelte';
  import { formatBytes, formatPercentage, formatCpuUsage, formatCores, getMemoryPressureColor } from '$lib/utils/format';
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

    {#if statusStore.macos}
      <Card>
        <h3 class="font-medium text-content-primary mb-3">macOS</h3>
        <div class="grid grid-cols-2 gap-y-2 text-sm">
          <span class="text-content-secondary">Version</span>
          <span class="text-content-primary">{statusStore.macos.version} ({statusStore.macos.buildVersion})</span>
          <span class="text-content-secondary">Computer Name</span>
          <span class="text-content-primary">{statusStore.macos.computerName}</span>
          <span class="text-content-secondary">Model</span>
          <span class="text-content-primary">{statusStore.macos.modelName}</span>
          <span class="text-content-secondary">Identifier</span>
          <span class="text-content-primary font-mono text-xs">{statusStore.macos.modelIdentifier}</span>
        </div>
      </Card>
    {/if}

    {#if statusStore.cpu}
      <Card>
        <h3 class="font-medium text-content-primary mb-3">CPU</h3>
        <div class="space-y-3">
          <div class="grid grid-cols-2 gap-y-2 text-sm">
            <span class="text-content-secondary">Processor</span>
            <span class="text-content-primary">{statusStore.cpu.model}</span>
            <span class="text-content-secondary">Cores</span>
            <span class="text-content-primary">{formatCores(statusStore.cpu.coreCount, statusStore.cpu.performanceCores, statusStore.cpu.efficiencyCores)}</span>
          </div>
          <div>
            <div class="flex justify-between text-sm mb-1">
              <span class="text-content-secondary">Usage</span>
              <span class="text-content-primary">{formatCpuUsage(statusStore.cpu.usagePercent)}</span>
            </div>
            <ProgressBar value={statusStore.cpu.usagePercent} max={100} />
          </div>
        </div>
      </Card>
    {/if}

    {#if statusStore.memory}
      <Card>
        <h3 class="font-medium text-content-primary mb-3">Memory</h3>
        <div class="space-y-3">
          <div>
            <div class="flex justify-between text-sm mb-1">
              <span class="text-content-secondary">Used</span>
              <span class="text-content-primary">
                {formatBytes(statusStore.memory.used)} / {formatBytes(statusStore.memory.total)}
              </span>
            </div>
            <ProgressBar value={statusStore.memory.used} max={statusStore.memory.total} />
          </div>
          <div class="grid grid-cols-2 gap-y-2 text-sm">
            <span class="text-content-secondary">Available</span>
            <span class="text-content-primary">{formatBytes(statusStore.memory.available)}</span>
            <span class="text-content-secondary">Pressure</span>
            <span class={getMemoryPressureColor(statusStore.memory.pressureLevel)}>
              {statusStore.memory.pressureLevel.charAt(0).toUpperCase() + statusStore.memory.pressureLevel.slice(1)}
            </span>
          </div>
          <div class="pt-2 border-t border-surface-tertiary">
            <p class="text-xs text-content-tertiary mb-2">Breakdown</p>
            <div class="grid grid-cols-2 gap-y-1 text-xs">
              <span class="text-content-secondary">Active</span>
              <span class="text-content-primary">{formatBytes(statusStore.memory.active)}</span>
              <span class="text-content-secondary">Inactive</span>
              <span class="text-content-primary">{formatBytes(statusStore.memory.inactive)}</span>
              <span class="text-content-secondary">Wired</span>
              <span class="text-content-primary">{formatBytes(statusStore.memory.wired)}</span>
              <span class="text-content-secondary">Compressed</span>
              <span class="text-content-primary">{formatBytes(statusStore.memory.compressed)}</span>
            </div>
          </div>
        </div>
      </Card>
    {/if}

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
