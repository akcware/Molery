import { invoke } from '@tauri-apps/api/core';
import type { AnalyzeResult } from '$lib/types/analyze.types';

export async function analyzePath(path: string): Promise<AnalyzeResult> {
  return invoke<AnalyzeResult>('analyze_path', { path });
}
