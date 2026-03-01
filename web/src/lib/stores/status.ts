import { writable } from 'svelte/store';
import type { QueueStatus, MlxStatus } from '$lib/api/types';
import { fetchStatus, fetchMlxStatus } from '$lib/api/status';

export const queueStatus = writable<QueueStatus>({
	total: 0,
	completed: 0,
	failed: 0,
	in_progress: 0,
	errors: []
});

export const mlxStatus = writable<MlxStatus>({
	online: false,
	model: null,
	message: 'Checking…'
});

let interval: ReturnType<typeof setInterval> | null = null;

async function poll() {
	try {
		const s = await fetchStatus();
		queueStatus.set(s);
	} catch { /* ignore */ }

	try {
		const m = await fetchMlxStatus();
		mlxStatus.set(m);
	} catch { /* ignore */ }
}

export function startPolling() {
	if (interval) return;
	poll();
	interval = setInterval(poll, 3000);
}

export function stopPolling() {
	if (interval) {
		clearInterval(interval);
		interval = null;
	}
}
