<script lang="ts">
  import { onMount } from 'svelte';
  import Layout from '$lib/components/layout/Layout.svelte';
  import Dashboard from '$lib/components/features/Dashboard.svelte';
  import CleanPanel from '$lib/components/features/CleanPanel.svelte';
  import UninstallPanel from '$lib/components/features/UninstallPanel.svelte';
  import AnalyzePanel from '$lib/components/features/AnalyzePanel.svelte';
  import OptimizePanel from '$lib/components/features/OptimizePanel.svelte';
  import StatusPanel from '$lib/components/features/StatusPanel.svelte';
  import SetupScreen from '$lib/components/features/SetupScreen.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { uiStore } from '$lib/stores/ui.store.svelte';
  import { checkMoInstalled } from '$lib/services/statusService';
  import { useKeyboard } from '$lib/hooks/useKeyboard';

  let moInstalled = $state<boolean | null>(null);
  let checking = $state(true);

  useKeyboard();

  onMount(async () => {
    try {
      moInstalled = await checkMoInstalled();
    } catch {
      moInstalled = false;
    } finally {
      checking = false;
    }
  });

  function handleInstalled() {
    moInstalled = true;
  }
</script>

{#if checking}
  <div class="min-h-screen flex items-center justify-center bg-surface-primary">
    <div class="text-center space-y-4">
      <Spinner size="lg" />
      <p class="text-content-secondary">Loading...</p>
    </div>
  </div>
{:else if !moInstalled}
  <SetupScreen oninstalled={handleInstalled} />
{:else}
  <Layout>
    <div class="max-w-4xl panel-content">
      {#if uiStore.activePanel === 'dashboard'}
        <Dashboard />
      {:else if uiStore.activePanel === 'clean'}
        <CleanPanel />
      {:else if uiStore.activePanel === 'uninstall'}
        <UninstallPanel />
      {:else if uiStore.activePanel === 'analyze'}
        <AnalyzePanel />
      {:else if uiStore.activePanel === 'optimize'}
        <OptimizePanel />
      {:else if uiStore.activePanel === 'status'}
        <StatusPanel />
      {/if}
    </div>
  </Layout>
{/if}
