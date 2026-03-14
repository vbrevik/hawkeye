export interface GraphNode {
	id: string;
	label: string;
	type: 'entity' | 'document' | 'tag' | 'topic';
	source_path?: string;
}

export interface GraphEdge {
	source: string;
	target: string;
	label: string;
	context?: string;
}

export interface GraphResponse {
	nodes: GraphNode[];
	edges: GraphEdge[];
}

export interface SearchResult {
	file: string;
	title: string;
	tldr: string;
	tags: string;
	entities: string;
	topics: string;
	score: number;
}

export type SearchMode = 'hybrid' | 'keyword' | 'semantic';

export interface HybridResult {
	file: string;
	title: string;
	tldr: string;
	tags: string;
	entities: string;
	topics: string;
	hybrid_score: number;
	keyword_rank: number | null;
	semantic_rank: number | null;
	keyword_score: number | null;
	semantic_distance: number | null;
}

export interface DisplayResult {
	file: string;
	title: string;
	tldr: string;
	tags: string;
	entities: string;
	topics: string;
	score: number;
	keyword_rank?: number | null;
	semantic_rank?: number | null;
	keyword_score?: number | null;
	semantic_distance?: number | null;
}

export interface SemanticResult {
	doc_id: string;
	source_path: string;
	title: string;
	tldr: string;
	distance: number;
}

export interface FacetEntry {
	name: string;
	count: number;
}

export interface Facets {
	document_count: number;
	tags: FacetEntry[];
	topics: FacetEntry[];
	entities: FacetEntry[];
}

export interface BrowseResponse {
	path: string;
	parent: string | null;
	entries: string[];
	md_file_count: number;
}

export interface IngestResponse {
	message: string;
	files_queued: number;
	files_skipped: number;
}

export interface CancelResult {
	cancelled: number;
	already_completed: number;
	already_failed: number;
}

export interface Relationship {
	from: string;
	rel: string;
	to: string;
	context: string;
}

export interface Summary {
	source: string;
	source_hash: string;
	created_at: string;
	tldr: string;
	title: string;
	tags: string[];
	entities: string[];
	topics: string[];
	relationships: Relationship[];
	word_count: number;
}

export interface FileError {
	file: string;
	error: string;
	attempts: number;
}

export interface QueueStatus {
	total: number;
	completed: number;
	failed: number;
	in_progress: number;
	errors: FileError[];
}

export interface MlxStatus {
	online: boolean;
	model: string | null;
	message: string;
}

export interface ServiceHealth {
	name: string;
	status: 'up' | 'degraded' | 'down';
	latency_ms: number | null;
}

export interface HealthResponse {
	services: ServiceHealth[];
	checked_at: string;
}

export interface ShutdownResponse {
	message: string;
}

export interface ReindexResponse {
	indexed: number;
}

export interface CreateWorkspaceResponse {
	id: string;
	name: string;
	created_at: string;
}

export interface CreateApiKeyResponse {
	id: string;
	key: string;
	key_prefix: string;
	workspace_id: string;
	label: string;
	created_at: string;
}

export interface WorkspaceDoc {
	id: string;
	source_path: string;
	source_hash: string;
	created_at: string;
	title: string | null;
}

export interface WorkspaceDocsResponse {
	workspace_id: string;
	documents: WorkspaceDoc[];
}

export interface DocumentDoneEvent {
	type: 'document_done';
	file: string;
}

export interface DocumentFailedEvent {
	type: 'document_failed';
	file: string;
	error: string;
}
