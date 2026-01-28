export type CleanupCategory = 'cache' | 'logs' | 'trash' | 'downloads' | 'xcode' | 'homebrew';

export interface CleanupItem {
  path: string;
  size: number;
  category: CleanupCategory;
  description: string;
}

export interface CategorySummary {
  cache: number;
  logs: number;
  trash: number;
  downloads: number;
  xcode: number;
  homebrew: number;
}

export interface ScanResult {
  items: CleanupItem[];
  totalSize: number;
  categories: CategorySummary;
}

export interface CleanupResult {
  freedSize: number;
  itemsRemoved: number;
  errors: string[];
}
