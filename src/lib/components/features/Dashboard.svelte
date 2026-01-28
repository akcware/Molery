<script lang="ts">
  import { onMount } from 'svelte';
  import StatCard from '$lib/components/ui/StatCard.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { formatBytes, formatPercentage, formatCpuUsage } from '$lib/utils/format';
  import { uiStore } from '$lib/stores/ui.store.svelte';
  import { statusStore } from '$lib/stores/status.store.svelte';
  import Package from 'lucide-svelte/icons/package';
  import Brush from 'lucide-svelte/icons/brush';
  import HardDrive from 'lucide-svelte/icons/hard-drive';
  import CheckCircle from 'lucide-svelte/icons/check-circle';
  import Monitor from 'lucide-svelte/icons/monitor';
  import Cpu from 'lucide-svelte/icons/cpu';
  import MemoryStick from 'lucide-svelte/icons/memory-stick';
  import BarChart3 from 'lucide-svelte/icons/bar-chart-3';

  onMount(() => {
    statusStore.loadStatus();
  });
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-xl font-semibold text-content-primary">Welcome to Molery</h1>
    <p class="text-[13px] text-content-secondary">Your system optimization dashboard</p>
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
    <div class="bg-accent-red/10 border border-accent-red rounded-lg p-4 text-accent-red text-[13px]">
      {statusStore.error}
    </div>
  {:else if statusStore.status}
    <!-- Row 1: Disk metrics -->
    <div class="grid grid-cols-4 gap-4">
      <StatCard
        icon={Package}
        value={formatBytes(statusStore.status.diskUsed)}
        label="Disk Used"
        sublabel={formatPercentage(statusStore.status.diskUsed, statusStore.status.diskTotal)}
      />
      <StatCard
        icon={Brush}
        value={formatBytes(statusStore.status.cleanableSize)}
        label="Cleanable"
        onclick={() => uiStore.setPanel('clean')}
      />
      <StatCard
        icon={HardDrive}
        value={formatBytes(statusStore.status.diskAvailable)}
        label="Available"
      />
      <StatCard
        icon={CheckCircle}
        value={statusStore.status.moVersion !== 'Not installed' ? 'Ready' : 'Setup'}
        label="mo CLI"
        sublabel={statusStore.status.moVersion}
      />
    </div>

    <!-- Row 2: System info -->
    <div class="grid grid-cols-3 gap-4">
      <StatCard
        icon={Monitor}
        value={`macOS ${statusStore.macos?.version ?? '—'}`}
        label="Operating System"
        sublabel={statusStore.macos?.modelName}
      />
      <StatCard
        icon={MemoryStick}
        value={formatBytes(statusStore.memory?.used ?? 0)}
        label="RAM Used"
        sublabel={`${formatBytes(statusStore.memory?.available ?? 0)} available`}
      />
      <StatCard
        icon={Cpu}
        value={formatCpuUsage(statusStore.cpu?.usagePercent ?? 0)}
        label="CPU Usage"
        sublabel={statusStore.cpu?.model?.split(' ').slice(0, 3).join(' ')}
      />
    </div>
  {/if}

  <div>
    <h2 class="text-[15px] font-medium text-content-primary mb-3">Quick Actions</h2>
    <div class="flex gap-2">
      <Button onclick={() => uiStore.setPanel('clean')}>
        <Brush size={14} strokeWidth={1.75} />
        Quick Clean
      </Button>
      <Button variant="secondary" onclick={() => uiStore.setPanel('analyze')}>
        <BarChart3 size={14} strokeWidth={1.75} />
        Analyze Disk
      </Button>
    </div>
  </div>
</div>
