export type Panel = 'dashboard' | 'clean' | 'uninstall' | 'analyze' | 'optimize' | 'status';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
}

export interface NavItem {
  id: Panel;
  label: string;
  icon: string;
  shortcut: string;
}
