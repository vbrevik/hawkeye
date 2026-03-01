import type { SearchResult, Facets } from './types';

export async function search(query: string, limit = 25): Promise<SearchResult[]> {
	const res = await fetch(`/search?q=${encodeURIComponent(query)}&limit=${limit}`);
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}

export async function fetchFacets(): Promise<Facets> {
	const res = await fetch('/facets');
	return res.json();
}
