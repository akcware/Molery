import { invoke } from '@tauri-apps/api/core';
import type { ScanResult, CleanupResult, CleanupCategory } from '$lib/types/clean.types';

export async function scanCleanup(): Promise<ScanResult> {
  return invoke<ScanResult>('scan_cleanup');
}

export async function runCleanup(categories: CleanupCategory[]): Promise<CleanupResult> {
  return invoke<CleanupResult>('run_cleanup', { categories });
}
