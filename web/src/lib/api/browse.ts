import { apiFetch } from './client';
import type { BrowseResponse } from './types';

export async function browse(path?: string): Promise<BrowseResponse> {
	const url = path ? `/browse?path=${encodeURIComponent(path)}` : '/browse';
	return apiFetch<BrowseResponse>(url);
}
