export interface DiskItem {
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  children?: DiskItem[];
}

export interface AnalyzeResult {
  path: string;
  totalSize: number;
  items: DiskItem[];
}
