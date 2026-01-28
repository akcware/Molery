<script lang="ts">
  import { onMount } from 'svelte';
  import StatCard from '$lib/components/ui/StatCard.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { getStatus } from '$lib/services/statusService';
  import { formatBytes, formatPercentage } from '$lib/utils/format';
  import { uiStore } from '$lib/stores/ui.store.svelte';
  import type { SystemStatus } from '$lib/types/status.types';

  let status = $state<SystemStatus | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      status = await getStatus();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load status';
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold text-content-primary">Welcome to Molery</h1>
    <p class="text-content-secondary">Your system optimization dashboard</p>
  </div>

  {#if loading}
    <div class="grid grid-cols-4 gap-4">
      {#each [1, 2, 3, 4] as _}
        <div class="bg-surface-secondary rounded-xl p-4 animate-pulse">
          <div class="h-8 bg-surface-tertiary rounded mb-2"></div>
          <div class="h-4 bg-surface-tertiary rounded w-2/3"></div>
        </div>
      {/each}
    </div>
  {:else if error}
    <div class="bg-accent-red/10 border border-accent-red rounded-lg p-4 text-accent-red">
      {error}
    </div>
  {:else if status}
    <div class="grid grid-cols-4 gap-4">
      <StatCard
        icon="📦"
        value={formatBytes(status.diskUsed)}
        label="Disk Used"
        sublabel={formatPercentage(status.diskUsed, status.diskTotal)}
      />
      <StatCard
        icon="🧹"
        value={formatBytes(status.cleanableSize)}
        label="Cleanable"
        onclick={() => uiStore.setPanel('clean')}
      />
      <StatCard
        icon="💾"
        value={formatBytes(status.diskAvailable)}
        label="Available"
      />
      <StatCard
        icon="✓"
        value={status.moVersion !== 'Not installed' ? 'Ready' : 'Setup'}
        label="mo CLI"
        sublabel={status.moVersion}
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
