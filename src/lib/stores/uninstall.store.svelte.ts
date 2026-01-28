import { listApps } from '$lib/services/uninstallService';
import type { AppInfo } from '$lib/types/uninstall.types';

const STORAGE_KEY = 'molery:uninstall:apps';

function loadFromStorage(): AppInfo[] {
  if (typeof localStorage === 'undefined') return [];
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch {
    // Corrupted data, ignore
  }
  return [];
}

function saveToStorage(data: AppInfo[]) {
  if (typeof localStorage === 'undefined') return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
  } catch {
    // Storage full or unavailable
  }
}

// Initialize from localStorage
let apps = $state<AppInfo[]>(loadFromStorage());
let loading = $state(false);

export const uninstallStore = {
  get apps() {
    return apps;
  },
  get loading() {
    return loading;
  },
  get hasCache() {
    return apps.length > 0;
  },

  async loadApps() {
    if (apps.length > 0) {
      // Has cache: show immediately, refresh silently in background
      this.refreshInBackground();
      return;
    }
    // No cache: full load with loading state
    loading = true;
    try {
      apps = await listApps();
      saveToStorage(apps);
    } finally {
      loading = false;
    }
  },

  async refreshInBackground() {
    try {
      const freshApps = await listApps();
      apps = freshApps;
      saveToStorage(apps);
    } catch {
      // Silent failure for background refresh
    }
  },

  removeApp(appName: string) {
    apps = apps.filter((a) => a.name !== appName);
    saveToStorage(apps);
  },

  reset() {
    apps = [];
    loading = false;
    if (typeof localStorage !== 'undefined') {
      localStorage.removeItem(STORAGE_KEY);
    }
  }
};
