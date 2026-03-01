import { writable } from 'svelte/store';

export interface Toast {
	id: number;
	message: string;
	type: 'success' | 'error' | 'info';
}

export const toasts = writable<Toast[]>([]);

let nextId = 0;

export function addToast(message: string, type: Toast['type'] = 'info') {
	const id = nextId++;
	toasts.update((t) => [...t, { id, message, type }]);
	setTimeout(() => removeToast(id), 4000);
}

export function removeToast(id: number) {
	toasts.update((t) => t.filter((toast) => toast.id !== id));
}
