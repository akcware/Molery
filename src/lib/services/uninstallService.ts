import { invoke } from '@tauri-apps/api/core';
import type { AppInfo, UninstallResult } from '$lib/types/uninstall.types';

export async function listApps(): Promise<AppInfo[]> {
  return invoke<AppInfo[]>('list_apps');
}

export async function uninstallApp(appName: string): Promise<UninstallResult> {
  return invoke<UninstallResult>('uninstall_app', { appName });
}
