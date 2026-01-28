<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // Left padding accounts for native traffic lights in overlay titlebar mode
  async function startDrag() {
    await getCurrentWindow().startDragging();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<header
  class="h-12 bg-surface-primary border-b border-border-secondary flex items-center pl-20 pr-4"
  onmousedown={startDrag}
>
  <div class="flex items-center gap-2 pointer-events-none">
    <span class="text-xl">🦡</span>
    <span class="font-semibold text-content-primary">Molery</span>
  </div>

  <div class="flex-1"></div>

  <div class="flex items-center gap-2">
    <button class="p-2 text-content-secondary hover:text-content-primary rounded-lg hover:bg-surface-secondary pointer-events-auto">
      ⚙️
    </button>
  </div>
</header>
