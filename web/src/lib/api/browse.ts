import type { BrowseResponse } from './types';

export async function browse(path?: string): Promise<BrowseResponse> {
	const url = path ? `/browse?path=${encodeURIComponent(path)}` : '/browse';
	const res = await fetch(url);
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}
