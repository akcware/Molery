<script lang="ts">
  import Button from './Button.svelte';
  import Modal from './Modal.svelte';

  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    variant?: 'danger' | 'warning' | 'default';
    onconfirm: () => void;
    oncancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = 'Confirm',
    cancelLabel = 'Cancel',
    variant = 'default',
    onconfirm,
    oncancel
  }: Props = $props();
</script>

<Modal {open} {title} onclose={oncancel}>
  <div class="space-y-4">
    <p class="text-content-secondary">{message}</p>
    <div class="flex justify-end gap-2">
      <Button variant="secondary" onclick={oncancel}>{cancelLabel}</Button>
      <Button onclick={onconfirm}>{confirmLabel}</Button>
    </div>
  </div>
</Modal>
