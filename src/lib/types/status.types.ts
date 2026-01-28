export interface SystemStatus {
  diskTotal: number;
  diskUsed: number;
  diskAvailable: number;
  cleanableSize: number;
  lastClean: string | null;
  moVersion: string;
}
