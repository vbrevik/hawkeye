import { apiFetch } from './client';
import type { IngestResponse, CancelResult } from './types';

export async function ingest(path: string, limit?: number): Promise<IngestResponse> {
	return apiFetch<IngestResponse>('/ingest', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, limit })
	});
}

export async function cancelJobs(): Promise<CancelResult> {
	return apiFetch<CancelResult>('/cancel', { method: 'POST' });
}
