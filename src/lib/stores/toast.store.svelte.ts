import type { Toast } from '$lib/types/common.types';

let toasts = $state<Toast[]>([]);

export const toastStore = {
  get all() {
    return toasts;
  },

  success(message: string, duration = 3000) {
    addToast('success', message, duration);
  },

  error(message: string, duration = 5000) {
    addToast('error', message, duration);
  },

  warning(message: string, duration = 4000) {
    addToast('warning', message, duration);
  },

  info(message: string, duration = 3000) {
    addToast('info', message, duration);
  },

  dismiss(id: string) {
    toasts = toasts.filter((t) => t.id !== id);
  }
};

function addToast(type: Toast['type'], message: string, duration: number) {
  const id = crypto.randomUUID();
  toasts = [...toasts, { id, type, message, duration }];

  if (duration > 0) {
    setTimeout(() => toastStore.dismiss(id), duration);
  }
}
