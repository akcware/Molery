<script lang="ts">
  import { onMount } from 'svelte';
  import StatCard from '$lib/components/ui/StatCard.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { formatBytes, formatPercentage, formatCpuUsage, formatCores } from '$lib/utils/format';
  import { uiStore } from '$lib/stores/ui.store.svelte';
  import { statusStore } from '$lib/stores/status.store.svelte';

  onMount(() => {
    statusStore.loadStatus();
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">Welcome to Molery</h1>
    <p class="text-content-secondary">Your system optimization dashboard</p>
  </div>

  {#if statusStore.loading}
    <div class="grid grid-cols-4 gap-4">
      {#each [1, 2, 3, 4] as _}
        <div class="bg-surface-secondary rounded-xl p-4 animate-pulse">
          <div class="h-8 bg-surface-tertiary rounded mb-2"></div>
          <div class="h-4 bg-surface-tertiary rounded w-2/3"></div>
        </div>
      {/each}
    </div>
    <div class="grid grid-cols-3 gap-4">
      {#each [1, 2, 3] as _}
        <div class="bg-surface-secondary rounded-xl p-4 animate-pulse">
          <div class="h-8 bg-surface-tertiary rounded mb-2"></div>
          <div class="h-4 bg-surface-tertiary rounded w-2/3"></div>
        </div>
      {/each}
    </div>
  {:else if statusStore.error}
    <div class="bg-accent-red/10 border border-accent-red rounded-lg p-4 text-accent-red">
      {statusStore.error}
    </div>
  {:else if statusStore.status}
    <!-- Row 1: Disk metrics -->
    <div class="grid grid-cols-4 gap-4">
      <StatCard
        icon="📦"
        value={formatBytes(statusStore.status.diskUsed)}
        label="Disk Used"
        sublabel={formatPercentage(statusStore.status.diskUsed, statusStore.status.diskTotal)}
      />
      <StatCard
        icon="🧹"
        value={formatBytes(statusStore.status.cleanableSize)}
        label="Cleanable"
        onclick={() => uiStore.setPanel('clean')}
      />
      <StatCard
        icon="💾"
        value={formatBytes(statusStore.status.diskAvailable)}
        label="Available"
      />
      <StatCard
        icon="✓"
        value={statusStore.status.moVersion !== 'Not installed' ? 'Ready' : 'Setup'}
        label="mo CLI"
        sublabel={statusStore.status.moVersion}
      />
    </div>

    <!-- Row 2: System info -->
    <div class="grid grid-cols-3 gap-4">
      <StatCard
        icon="🖥️"
        value={`macOS ${statusStore.macos?.version ?? '—'}`}
        label="Operating System"
        sublabel={statusStore.macos?.modelName}
      />
      <StatCard
        icon="🧠"
        value={formatBytes(statusStore.memory?.used ?? 0)}
        label="RAM Used"
        sublabel={`${formatBytes(statusStore.memory?.available ?? 0)} available`}
      />
      <StatCard
        icon="⚡"
        value={formatCpuUsage(statusStore.cpu?.usagePercent ?? 0)}
        label="CPU Usage"
        sublabel={statusStore.cpu?.model?.split(' ').slice(0, 3).join(' ')}
      />
    </div>
  {/if}

  <div>
    <h2 class="text-lg font-medium text-content-primary mb-3">Quick Actions</h2>
    <div class="flex gap-3">
      <Button onclick={() => uiStore.setPanel('clean')}>
        🧹 Quick Clean
      </Button>
      <Button variant="secondary" onclick={() => uiStore.setPanel('analyze')}>
        📊 Analyze Disk
      </Button>
    </div>
  </div>
</div>
