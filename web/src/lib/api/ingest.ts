import type { IngestResponse, CancelResult } from './types';

export async function ingest(path: string, limit?: number): Promise<IngestResponse> {
	const res = await fetch('/ingest', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path, limit })
	});
	if (!res.ok) {
		const text = await res.text();
		throw new Error(text || res.statusText);
	}
	return res.json();
}

export async function cancelJobs(): Promise<CancelResult> {
	const res = await fetch('/cancel', { method: 'POST' });
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}
