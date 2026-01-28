import type { Panel } from '$lib/types/common.types';

let activePanel = $state<Panel>('dashboard');
let sidebarCollapsed = $state(false);

export const uiStore = {
  get activePanel() { return activePanel; },
  get sidebarCollapsed() { return sidebarCollapsed; },

  setPanel(panel: Panel) {
    activePanel = panel;
  },

  toggleSidebar() {
    sidebarCollapsed = !sidebarCollapsed;
  },
};
