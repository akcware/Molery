export interface AppInfo {
  name: string;
  path: string;
  size: number;
  bundleId: string | null;
  icon?: string;
}

export interface UninstallResult {
  removedFiles: string[];
  freedSize: number;
  success: boolean;
}
