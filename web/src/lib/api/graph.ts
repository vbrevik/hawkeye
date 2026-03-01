import { apiFetch } from './client';
import type { GraphResponse } from './types';

export async function fetchEntityGraph(name: string): Promise<GraphResponse> {
	return apiFetch<GraphResponse>(`/graph/entity/${encodeURIComponent(name)}`);
}

export async function fetchDocumentGraph(id: string): Promise<GraphResponse> {
	return apiFetch<GraphResponse>(`/graph/document/${encodeURIComponent(id)}`);
}
