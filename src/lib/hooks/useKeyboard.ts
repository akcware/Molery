import { onMount } from 'svelte';
import { uiStore } from '$lib/stores/ui.store.svelte';
import type { Panel } from '$lib/types/common.types';

const panelShortcuts: Record<string, Panel> = {
  '1': 'dashboard',
  '2': 'clean',
  '3': 'uninstall',
  '4': 'analyze',
  '5': 'optimize',
  '6': 'status',
};

export function useKeyboard() {
  onMount(() => {
    function handleKeydown(event: KeyboardEvent) {
      // Check for Cmd/Ctrl + number
      if (event.metaKey || event.ctrlKey) {
        const panel = panelShortcuts[event.key];
        if (panel) {
          event.preventDefault();
          uiStore.setPanel(panel);
        }
      }

      // Escape to close modals (handled by individual components)
    }

    window.addEventListener('keydown', handleKeydown);

    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });
}
