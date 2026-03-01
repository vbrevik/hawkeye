import type { HealthResponse } from './types';

export async function fetchHealth(): Promise<HealthResponse> {
	const res = await fetch('/health');
	return res.json();
}
