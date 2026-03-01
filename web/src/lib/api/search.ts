import { apiFetch } from './client';
import type { SearchResult, Facets, ReindexResponse } from './types';

export async function search(query: string, limit = 25): Promise<SearchResult[]> {
	return apiFetch<SearchResult[]>(`/search?q=${encodeURIComponent(query)}&limit=${limit}`);
}

export async function fetchFacets(): Promise<Facets> {
	return apiFetch<Facets>('/facets');
}

export async function reindex(): Promise<ReindexResponse> {
	return apiFetch<ReindexResponse>('/reindex', { method: 'POST' });
}
