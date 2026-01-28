import type { ScanResult, CleanupCategory } from '$lib/types/clean.types';

let scanResult = $state<ScanResult | null>(null);
let isScanning = $state(false);
let isCleaning = $state(false);
let selectedCategories = $state<Set<CleanupCategory>>(new Set(['cache', 'logs', 'trash']));

export const cleanStore = {
  get scanResult() {
    return scanResult;
  },
  get isScanning() {
    return isScanning;
  },
  get isCleaning() {
    return isCleaning;
  },
  get selectedCategories() {
    return selectedCategories;
  },

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
    const newSet = new Set(selectedCategories);
    if (newSet.has(category)) {
      newSet.delete(category);
    } else {
      newSet.add(category);
    }
    selectedCategories = newSet;
  },

  selectAll() {
    selectedCategories = new Set(['cache', 'logs', 'trash', 'downloads', 'xcode', 'homebrew']);
  },

  deselectAll() {
    selectedCategories = new Set();
  },

  reset() {
    scanResult = null;
    isScanning = false;
    isCleaning = false;
    selectedCategories = new Set(['cache', 'logs', 'trash']);
  }
};
