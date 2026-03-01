import type { GraphResponse } from './types';

export async function fetchEntityGraph(name: string): Promise<GraphResponse> {
	const res = await fetch(`/graph/entity/${encodeURIComponent(name)}`);
	if (!res.ok) throw new Error(`Failed to fetch entity graph: ${res.statusText}`);
	return res.json();
}

export async function fetchDocumentGraph(id: string): Promise<GraphResponse> {
	const res = await fetch(`/graph/document/${encodeURIComponent(id)}`);
	if (!res.ok) throw new Error(`Failed to fetch document graph: ${res.statusText}`);
	return res.json();
}
