import { invoke } from '@tauri-apps/api/core';
import type { SystemStatus, ExtendedSystemStatus } from '$lib/types/status.types';

export async function getStatus(): Promise<SystemStatus> {
  return invoke<SystemStatus>('get_status');
}

export async function getExtendedStatus(): Promise<ExtendedSystemStatus> {
  return invoke<ExtendedSystemStatus>('get_extended_status');
}

export async function checkMoInstalled(): Promise<boolean> {
  return invoke<boolean>('check_mo_installed');
}

export async function getHomeDir(): Promise<string> {
  return invoke<string>('get_home_dir');
}
