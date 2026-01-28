<script lang="ts">
  interface Props {
    width?: string;
    height?: string;
    rounded?: 'none' | 'sm' | 'md' | 'lg' | 'full';
  }

  let { width = '100%', height = '1rem', rounded = 'md' }: Props = $props();

  const roundedClasses = {
    none: '',
    sm: 'rounded',
    md: 'rounded-md',
    lg: 'rounded-lg',
    full: 'rounded-full',
  };
</script>

<div
  class="bg-surface-tertiary animate-pulse {roundedClasses[rounded]}"
  style="width: {width}; height: {height}"
></div>
