<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Settings from 'lucide-svelte/icons/settings';

  async function startDrag() {
    await getCurrentWindow().startDragging();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<header
  class="h-12 vibrancy border-b border-border-secondary flex items-center"
  onmousedown={startDrag}
>
  <!-- Left spacer for traffic lights (72px gives proper clearance) -->
  <div class="w-[72px] flex-shrink-0"></div>

  <!-- Centered title area -->
  <div class="flex-1 flex justify-center pointer-events-none">
    <span class="text-[13px] font-medium text-content-secondary">Molery</span>
  </div>

  <!-- Right actions area -->
  <div class="w-[72px] flex-shrink-0 flex justify-end pr-3">
    <button
      class="p-1.5 text-content-tertiary hover:text-content-primary rounded-md hover:bg-surface-tertiary/50 pointer-events-auto transition-colors"
      aria-label="Settings"
    >
      <Settings size={16} strokeWidth={1.75} />
    </button>
  </div>
</header>
