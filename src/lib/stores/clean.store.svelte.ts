import type { ScanResult } from '$lib/types/clean.types';

let scanResult = $state<ScanResult | null>(null);
let isScanning = $state(false);
let isCleaning = $state(false);

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

  setScanResult(result: ScanResult | null) {
    scanResult = result;
  },

  setScanning(value: boolean) {
    isScanning = value;
  },

  setCleaning(value: boolean) {
    isCleaning = value;
  },

  reset() {
    scanResult = null;
    isScanning = false;
    isCleaning = false;
  }
};
