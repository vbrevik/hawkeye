import { apiFetch } from './client';
import type { QueueStatus, MlxStatus } from './types';

export async function fetchStatus(): Promise<QueueStatus> {
	return apiFetch<QueueStatus>('/status');
}

export async function fetchMlxStatus(): Promise<MlxStatus> {
	return apiFetch<MlxStatus>('/mlx-status');
}
