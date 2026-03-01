import type { QueueStatus, MlxStatus } from './types';

export async function fetchStatus(): Promise<QueueStatus> {
	const res = await fetch('/status');
	return res.json();
}

export async function fetchMlxStatus(): Promise<MlxStatus> {
	const res = await fetch('/mlx-status');
	return res.json();
}
