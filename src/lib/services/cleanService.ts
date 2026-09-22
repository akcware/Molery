import { invoke } from '@tauri-apps/api/core';
import type { ScanResult, CleanupResult } from '$lib/types/clean.types';

export async function scanCleanup(): Promise<ScanResult> {
  return invoke<ScanResult>('scan_cleanup');
}

/**
 * Run a full cleanup.
 *
 * `mo clean` has no per-category flags, so this always cleans everything mo
 * finds. The category breakdown from `scanCleanup` is a preview, not a filter.
 */
export async function runCleanup(): Promise<CleanupResult> {
  return invoke<CleanupResult>('run_cleanup');
}
