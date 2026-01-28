<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { checkMoInstalled } from '$lib/services/statusService';
  import { toastStore } from '$lib/stores/toast.store.svelte';

  interface Props {
    oninstalled: () => void;
  }

  let { oninstalled }: Props = $props();
  let checking = $state(false);

  async function handleCheck() {
    checking = true;
    try {
      const installed = await checkMoInstalled();
      if (installed) {
        toastStore.success('mo CLI detected! Ready to use.');
        oninstalled();
      } else {
        toastStore.error('mo CLI not found. Please install and try again.');
      }
    } catch {
      toastStore.error('Failed to check mo installation');
    } finally {
      checking = false;
    }
  }

  function copyCommand(command: string) {
    navigator.clipboard.writeText(command);
    toastStore.success('Command copied to clipboard');
  }

  function openLink(url: string) {
    window.open(url, '_blank');
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-surface-primary p-8">
  <div class="max-w-lg w-full space-y-6 animate-fade-in">
    <div class="text-center">
      <div class="text-6xl mb-4">🔧</div>
      <h1 class="text-2xl font-semibold text-content-primary">Molery Setup</h1>
      <p class="text-content-secondary mt-2">
        Molery requires the 'mo' CLI tool to work. Please install it using one of these methods:
      </p>
    </div>

    <Card>
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <span>🍺</span>
          <span class="font-medium text-content-primary">Homebrew (Recommended)</span>
        </div>
        <code class="block bg-surface-tertiary rounded-lg px-3 py-2 text-sm font-mono text-content-primary">
          brew install tw93/brew/mole
        </code>
        <div class="flex gap-2">
          <Button variant="secondary" size="sm" onclick={() => copyCommand('brew install tw93/brew/mole')}>
            Copy Command
          </Button>
        </div>
      </div>
    </Card>

    <Card>
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <span>📦</span>
          <span class="font-medium text-content-primary">Cargo (Rust)</span>
        </div>
        <code class="block bg-surface-tertiary rounded-lg px-3 py-2 text-sm font-mono text-content-primary">
          cargo install mole
        </code>
        <Button variant="secondary" size="sm" onclick={() => copyCommand('cargo install mole')}>
          Copy Command
        </Button>
      </div>
    </Card>

    <Card>
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <span>📥</span>
          <span class="font-medium text-content-primary">Manual Download</span>
        </div>
        <p class="text-sm text-content-secondary">Download from GitHub releases</p>
        <Button variant="secondary" size="sm" onclick={() => openLink('https://github.com/tw93/Mole/releases')}>
          Open GitHub
        </Button>
      </div>
    </Card>

    <div class="text-center">
      <Button onclick={handleCheck} disabled={checking}>
        {#if checking}
          <span class="flex items-center gap-2">
            <Spinner size="sm" />
            Checking...
          </span>
        {:else}
          Check Again
        {/if}
      </Button>
    </div>
  </div>
</div>
