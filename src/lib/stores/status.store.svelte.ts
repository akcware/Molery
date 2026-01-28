import { getStatus, checkMoInstalled } from '$lib/services/statusService';
import type { SystemStatus } from '$lib/types/status.types';

const STORAGE_KEY = 'molery:status';

interface StatusCache {
  status: SystemStatus | null;
  moInstalled: boolean;
}

function loadFromStorage(): StatusCache {
  if (typeof localStorage === 'undefined') return { status: null, moInstalled: false };
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch {
    // Corrupted data, ignore
  }
  return { status: null, moInstalled: false };
}

function saveToStorage(data: StatusCache) {
  if (typeof localStorage === 'undefined') return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
  } catch {
    // Storage full or unavailable
  }
}

// Initialize from localStorage
const cached = loadFromStorage();
let status = $state<SystemStatus | null>(cached.status);
let moInstalled = $state(cached.moInstalled);
let loading = $state(false);
let error = $state<string | null>(null);

export const statusStore = {
  get status() {
    return status;
  },
  get moInstalled() {
    return moInstalled;
  },
  get loading() {
    return loading;
  },
  get error() {
    return error;
  },
  get hasCache() {
    return status !== null;
  },

  async loadStatus() {
    if (status !== null) {
      // Has cache: show immediately, refresh silently in background
      this.refreshInBackground();
      return;
    }
    // No cache: full load with loading state
    loading = true;
    error = null;
    try {
      const [newStatus, newMoInstalled] = await Promise.all([
        getStatus(),
        checkMoInstalled()
      ]);
      status = newStatus;
      moInstalled = newMoInstalled;
      saveToStorage({ status, moInstalled });
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load status';
    } finally {
      loading = false;
    }
  },

  async refreshInBackground() {
    try {
      const [newStatus, newMoInstalled] = await Promise.all([
        getStatus(),
        checkMoInstalled()
      ]);
      status = newStatus;
      moInstalled = newMoInstalled;
      saveToStorage({ status, moInstalled });
    } catch {
      // Silent failure for background refresh
    }
  },

  reset() {
    status = null;
    moInstalled = false;
    loading = false;
    error = null;
    if (typeof localStorage !== 'undefined') {
      localStorage.removeItem(STORAGE_KEY);
    }
  }
};
