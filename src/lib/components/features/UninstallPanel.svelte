<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import { uninstallApp } from '$lib/services/uninstallService';
  import { uninstallStore } from '$lib/stores/uninstall.store.svelte';
  import { toastStore } from '$lib/stores/toast.store.svelte';
  import { formatBytes } from '$lib/utils/format';
  import type { AppInfo } from '$lib/types/uninstall.types';
  import Package from 'lucide-svelte/icons/package';
  import Trash2 from 'lucide-svelte/icons/trash-2';

  let searchQuery = $state('');
  let selectedApp = $state<AppInfo | null>(null);
  let uninstalling = $state(false);

  let filteredApps = $derived(
    uninstallStore.apps.filter(app =>
      app.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  onMount(async () => {
    try {
      await uninstallStore.loadApps();
    } catch (error) {
      toastStore.error('Failed to load apps');
    }
  });

  async function handleUninstall() {
    if (!selectedApp) return;

    uninstalling = true;
    try {
      const result = await uninstallApp(selectedApp.name);
      if (result.success) {
        toastStore.success(`Uninstalled ${selectedApp.name}`);
        uninstallStore.removeApp(selectedApp.name);
      }
    } catch (error) {
      toastStore.error(error instanceof Error ? error.message : 'Uninstall failed');
    } finally {
      uninstalling = false;
      selectedApp = null;
    }
  }
</script>

<div class="space-y-6">
  <div>
    <h1 class="text-xl font-semibold text-content-primary">App Uninstaller</h1>
    <p class="text-[13px] text-content-secondary">Completely remove applications and their files</p>
  </div>

  <input
    type="text"
    placeholder="Search apps..."
    bind:value={searchQuery}
    class="w-full px-4 py-2 bg-surface-secondary border border-border-primary rounded-lg text-content-primary placeholder:text-content-tertiary"
  />

  {#if uninstallStore.loading}
    <div class="text-content-secondary">Loading apps...</div>
  {:else}
    <p class="text-sm text-content-secondary">Installed Applications ({filteredApps.length})</p>

    <Card padding="none">
      <div class="max-h-96 overflow-auto divide-y divide-border-secondary">
        {#each filteredApps as app}
          <div class="flex items-center gap-3 px-4 py-3 hover:bg-surface-tertiary transition-colors">
            <div class="w-8 h-8 rounded-lg bg-surface-tertiary/50 flex items-center justify-center flex-shrink-0">
              <Package size={16} strokeWidth={1.75} class="text-content-secondary" />
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-[13px] font-medium text-content-primary truncate">{app.name}</p>
              <p class="text-[11px] text-content-tertiary truncate">{app.path}</p>
            </div>
            <span class="text-[12px] text-content-secondary">{formatBytes(app.size)}</span>
            <Button variant="ghost" size="sm" onclick={() => selectedApp = app}>
              <Trash2 size={14} strokeWidth={1.75} />
            </Button>
          </div>
        {/each}
      </div>
    </Card>
  {/if}
</div>

<Modal open={!!selectedApp} title="Uninstall {selectedApp?.name}?" onclose={() => selectedApp = null}>
  <div class="space-y-4">
    <p class="text-content-secondary">This will remove:</p>
    <ul class="text-sm text-content-secondary list-disc list-inside">
      <li>Application ({formatBytes(selectedApp?.size ?? 0)})</li>
      <li>Preferences</li>
      <li>Cache files</li>
      <li>Support files</li>
    </ul>
    <div class="flex justify-end gap-2">
      <Button variant="secondary" onclick={() => selectedApp = null}>Cancel</Button>
      <Button onclick={handleUninstall} disabled={uninstalling}>
        {uninstalling ? 'Uninstalling...' : 'Uninstall'}
      </Button>
    </div>
  </div>
</Modal>
