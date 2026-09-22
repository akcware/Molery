# TypeScript API Specification

## Overview

Frontend TypeScript interfaces, services, and stores for communicating with the Tauri backend.

## Type Definitions

### Clean Types

**File:** `src/lib/types/clean.types.ts`

```typescript
export type CleanupCategory = 'cache' | 'logs' | 'trash' | 'downloads' | 'xcode' | 'homebrew';

export interface CleanupItem {
  path: string;
  size: number;
  category: CleanupCategory;
  description: string;
}

export interface CategorySummary {
  cache: number;
  logs: number;
  trash: number;
  downloads: number;
  xcode: number;
  homebrew: number;
}

export interface ScanResult {
  items: CleanupItem[];
  totalSize: number;
  categories: CategorySummary;
}

export interface CleanupResult {
  freedSize: number;
  itemsRemoved: number;
  errors: string[];
}
```

### Status Types

**File:** `src/lib/types/status.types.ts`

```typescript
export interface SystemStatus {
  diskTotal: number;
  diskUsed: number;
  diskAvailable: number;
  cleanableSize: number;
  lastClean: string | null;
  moVersion: string;
}
```

### Uninstall Types

**File:** `src/lib/types/uninstall.types.ts`

```typescript
export interface AppInfo {
  name: string;
  path: string;
  size: number;
  bundleId: string | null;
}

export interface UninstallResult {
  removedFiles: string[];
  freedSize: number;
  success: boolean;
}
```

### Analyze Types

**File:** `src/lib/types/analyze.types.ts`

```typescript
export interface DiskItem {
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  children?: DiskItem[];
}

export interface AnalyzeResult {
  path: string;
  totalSize: number;
  items: DiskItem[];
}
```

### Common Types

**File:** `src/lib/types/common.types.ts`

```typescript
export type Panel = 'dashboard' | 'clean' | 'uninstall' | 'analyze' | 'optimize' | 'status';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
}
```

---

## Services

### Clean Service

**File:** `src/lib/services/cleanService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { ScanResult, CleanupResult, CleanupCategory } from '$lib/types/clean.types';

export async function scanCleanup(): Promise<ScanResult> {
  return invoke<ScanResult>('scan_cleanup');
}

// mo clean has no per-category flags: this always cleans everything mo finds.
export async function runCleanup(): Promise<CleanupResult> {
  return invoke<CleanupResult>('run_cleanup');
}
```

### Status Service

**File:** `src/lib/services/statusService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { SystemStatus } from '$lib/types/status.types';

export async function getStatus(): Promise<SystemStatus> {
  return invoke<SystemStatus>('get_status');
}

export async function checkMoInstalled(): Promise<boolean> {
  return invoke<boolean>('check_mo_installed');
}
```

### Uninstall Service

**File:** `src/lib/services/uninstallService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { AppInfo, UninstallResult } from '$lib/types/uninstall.types';

export async function listApps(): Promise<AppInfo[]> {
  return invoke<AppInfo[]>('list_apps');
}

export async function uninstallApp(appName: string): Promise<UninstallResult> {
  return invoke<UninstallResult>('uninstall_app', { appName });
}
```

### Analyze Service

**File:** `src/lib/services/analyzeService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { AnalyzeResult } from '$lib/types/analyze.types';

export async function analyzePath(path: string): Promise<AnalyzeResult> {
  return invoke<AnalyzeResult>('analyze_path', { path });
}
```

---

## Stores

### UI Store

**File:** `src/lib/stores/ui.store.ts`

```typescript
import type { Panel } from '$lib/types/common.types';

// Reactive state
let activePanel = $state<Panel>('dashboard');
let sidebarCollapsed = $state(false);
let theme = $state<'light' | 'dark' | 'system'>('system');

export const uiStore = {
  get activePanel() { return activePanel; },
  get sidebarCollapsed() { return sidebarCollapsed; },
  get theme() { return theme; },

  setPanel(panel: Panel) {
    activePanel = panel;
  },

  toggleSidebar() {
    sidebarCollapsed = !sidebarCollapsed;
  },

  setTheme(newTheme: 'light' | 'dark' | 'system') {
    theme = newTheme;
  }
};
```

### Toast Store

**File:** `src/lib/stores/toast.store.ts`

```typescript
import type { Toast } from '$lib/types/common.types';

let toasts = $state<Toast[]>([]);

export const toastStore = {
  get all() { return toasts; },

  success(message: string, duration = 3000) {
    addToast('success', message, duration);
  },

  error(message: string, duration = 5000) {
    addToast('error', message, duration);
  },

  warning(message: string, duration = 4000) {
    addToast('warning', message, duration);
  },

  info(message: string, duration = 3000) {
    addToast('info', message, duration);
  },

  dismiss(id: string) {
    toasts = toasts.filter(t => t.id !== id);
  }
};

function addToast(type: Toast['type'], message: string, duration: number) {
  const id = crypto.randomUUID();
  toasts = [...toasts, { id, type, message, duration }];

  if (duration > 0) {
    setTimeout(() => toastStore.dismiss(id), duration);
  }
}
```

### Clean Store

**File:** `src/lib/stores/clean.store.ts`

```typescript
import type { ScanResult, CleanupCategory } from '$lib/types/clean.types';

let scanResult = $state<ScanResult | null>(null);
let isScanning = $state(false);
let isCleaning = $state(false);
let selectedCategories = $state<Set<CleanupCategory>>(new Set());

export const cleanStore = {
  get scanResult() { return scanResult; },
  get isScanning() { return isScanning; },
  get isCleaning() { return isCleaning; },
  get selectedCategories() { return selectedCategories; },

  setScanResult(result: ScanResult | null) {
    scanResult = result;
  },

  setScanning(value: boolean) {
    isScanning = value;
  },

  setCleaning(value: boolean) {
    isCleaning = value;
  },

  toggleCategory(category: CleanupCategory) {
    if (selectedCategories.has(category)) {
      selectedCategories.delete(category);
    } else {
      selectedCategories.add(category);
    }
    selectedCategories = new Set(selectedCategories);
  },

  selectAllCategories() {
    selectedCategories = new Set(['cache', 'logs', 'trash', 'downloads', 'xcode', 'homebrew']);
  },

  clearSelection() {
    selectedCategories = new Set();
  },

  reset() {
    scanResult = null;
    isScanning = false;
    isCleaning = false;
    selectedCategories = new Set();
  }
};
```

---

## Utility Functions

### Format Bytes

**File:** `src/lib/utils/format.ts`

```typescript
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

export function formatPercentage(value: number, total: number): string {
  if (total === 0) return '0%';
  return `${((value / total) * 100).toFixed(1)}%`;
}
```
