import { invoke } from '@tauri-apps/api/core';

export interface OptimizeResult {
  success: boolean;
  message: string;
}

export async function clearDnsCache(): Promise<OptimizeResult> {
  return invoke<OptimizeResult>('clear_dns_cache');
}

export async function rebuildSpotlightIndex(): Promise<OptimizeResult> {
  return invoke<OptimizeResult>('rebuild_spotlight_index');
}

export async function freeMemory(): Promise<OptimizeResult> {
  return invoke<OptimizeResult>('free_memory');
}
