export interface SystemStatus {
  diskTotal: number;
  diskUsed: number;
  diskAvailable: number;
  cleanableSize: number;
  lastClean: string | null;
  moVersion: string;
}

export interface MacOsInfo {
  version: string;
  buildVersion: string;
  computerName: string;
  modelName: string;
  modelIdentifier: string;
}

export interface MemoryInfo {
  total: number;
  used: number;
  available: number;
  wired: number;
  active: number;
  inactive: number;
  compressed: number;
  pressureLevel: 'normal' | 'warn' | 'critical';
}

export interface CpuInfo {
  model: string;
  coreCount: number;
  performanceCores: number;
  efficiencyCores: number;
  usagePercent: number;
}

export interface ExtendedSystemStatus extends SystemStatus {
  macos: MacOsInfo;
  memory: MemoryInfo;
  cpu: CpuInfo;
}
