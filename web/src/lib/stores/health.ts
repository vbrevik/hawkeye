import { writable, derived } from 'svelte/store';
import type { HealthResponse } from '$lib/api/types';
import { fetchHealth } from '$lib/api/health';

export const healthData = writable<HealthResponse | null>(null);

export const healthSummary = derived(healthData, ($data) => {
	if (!$data) return { up: 0, total: 0, worst: 'unknown' as const };
	const services = $data.services;
	const up = services.filter((s) => s.status === 'up').length;
	const worst: 'up' | 'degraded' | 'down' = services.some((s) => s.status === 'down')
		? 'down'
		: services.some((s) => s.status === 'degraded')
			? 'degraded'
			: 'up';
	return { up, total: services.length, worst };
});

let interval: ReturnType<typeof setInterval> | null = null;

async function poll() {
	try {
		const data = await fetchHealth();
		healthData.set(data);
	} catch {
		/* ignore */
	}
}

export function startHealthPolling() {
	if (interval) return;
	poll();
	interval = setInterval(poll, 5000);
}

export function stopHealthPolling() {
	if (interval) {
		clearInterval(interval);
		interval = null;
	}
}
