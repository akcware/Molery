export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

export function formatPercentage(value: number, total: number): string {
  if (total === 0) return '0%';
  return `${((value / total) * 100).toFixed(1)}%`;
}

export function formatCpuUsage(percent: number): string {
  return `${percent.toFixed(1)}%`;
}

export function formatCores(total: number, perf: number, eff: number): string {
  if (perf > 0 && eff > 0) {
    return `${total} (${perf}P + ${eff}E)`;
  }
  return `${total}`;
}

export function getMemoryPressureColor(level: 'normal' | 'warn' | 'critical'): string {
  switch (level) {
    case 'normal':
      return 'text-accent-green';
    case 'warn':
      return 'text-accent-yellow';
    case 'critical':
      return 'text-accent-red';
  }
}
