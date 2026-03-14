import { apiFetch } from './client';
import type {
	SearchResult,
	HybridResult,
	SemanticResult,
	DisplayResult,
	Facets,
	ReindexResponse
} from './types';

export async function search(query: string, limit = 25): Promise<SearchResult[]> {
	return apiFetch<SearchResult[]>(`/search?q=${encodeURIComponent(query)}&limit=${limit}`);
}

export async function hybridSearch(query: string, limit = 25): Promise<HybridResult[]> {
	return apiFetch<HybridResult[]>(`/search/hybrid?q=${encodeURIComponent(query)}&limit=${limit}`);
}

export async function semanticSearch(query: string, limit = 25): Promise<SemanticResult[]> {
	return apiFetch<SemanticResult[]>(`/search/semantic?q=${encodeURIComponent(query)}&limit=${limit}`);
}

export function normalizeKeyword(results: SearchResult[]): DisplayResult[] {
	return results.map((r, i) => ({
		...r,
		keyword_rank: i + 1,
		keyword_score: r.score,
	}));
}

export function normalizeHybrid(results: HybridResult[]): DisplayResult[] {
	return results.map((r) => ({
		file: r.file,
		title: r.title,
		tldr: r.tldr,
		tags: r.tags,
		entities: r.entities,
		topics: r.topics,
		score: r.hybrid_score,
		keyword_rank: r.keyword_rank,
		semantic_rank: r.semantic_rank,
		keyword_score: r.keyword_score,
		semantic_distance: r.semantic_distance,
	}));
}

export function normalizeSemantic(results: SemanticResult[]): DisplayResult[] {
	return results.map((r, i) => {
		const filename = r.source_path.split('/').pop() ?? r.source_path;
		return {
			file: filename,
			title: r.title || filename,
			tldr: r.tldr,
			tags: '',
			entities: '',
			topics: '',
			score: r.distance > 0 ? 1 / (1 + r.distance) : 0,
			semantic_rank: i + 1,
			semantic_distance: r.distance,
		};
	});
}

export async function fetchFacets(): Promise<Facets> {
	return apiFetch<Facets>('/facets');
}

export async function reindex(): Promise<ReindexResponse> {
	return apiFetch<ReindexResponse>('/reindex', { method: 'POST' });
}
