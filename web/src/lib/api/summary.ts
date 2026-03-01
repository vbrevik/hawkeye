import { apiFetch } from './client';
import type { Summary } from './types';

export async function fetchSummary(file: string): Promise<Summary> {
	return apiFetch<Summary>(`/summary/${encodeURIComponent(file)}`);
}
