<script lang="ts">
	import type { CreateWorkspaceResponse, CreateApiKeyResponse, WorkspaceDoc } from '$lib/api/types';
	import { fetchWorkspaceDocs } from '$lib/api/workspaces';
	import { addToast } from '$lib/stores/toast';

	let {
		workspace,
		apiKey,
	}: {
		workspace: CreateWorkspaceResponse | null;
		apiKey: CreateApiKeyResponse | null;
	} = $props();

	let docs = $state<WorkspaceDoc[]>([]);
	let docsLoading = $state(false);
	let docsLoaded = $state(false);

	export function reset() {
		docs = [];
		docsLoaded = false;
	}

	async function handleLoadDocs() {
		if (!workspace || !apiKey) return;
		docsLoading = true;
		try {
			const resp = await fetchWorkspaceDocs(workspace.id);
			docs = resp.documents;
			docsLoaded = true;
		} catch (e) {
			addToast(`Failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
		} finally {
			docsLoading = false;
		}
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('en-US', {
			month: 'short', day: 'numeric', year: 'numeric',
			hour: '2-digit', minute: '2-digit'
		});
	}
</script>

<section class="card" class:disabled={!apiKey}>
	<div class="card-header">
		<span class="step-badge">3</span>
		<h2>Workspace Documents</h2>
	</div>
	<p class="card-desc">View documents ingested into this workspace.</p>

	{#if apiKey}
		{#if !docsLoaded}
			<button onclick={handleLoadDocs} disabled={docsLoading} class="btn-secondary">
				{docsLoading ? 'Loading…' : 'Load Documents'}
			</button>
		{:else if docs.length === 0}
			<div class="empty-docs">
				<span class="empty-icon">📄</span>
				<p>No documents ingested yet. Use <code>POST /ingest</code> to add markdown files.</p>
			</div>
		{:else}
			<div class="doc-count">{docs.length} document{docs.length !== 1 ? 's' : ''}</div>
			<div class="doc-list">
				{#each docs as doc (doc.id)}
					<div class="doc-row">
						<div class="doc-info">
							<div class="doc-title">{doc.title ?? doc.source_path.split('/').pop()}</div>
							<div class="doc-path">{doc.source_path}</div>
						</div>
						<div class="doc-meta">
							<span class="doc-hash" title={doc.source_hash}>{doc.source_hash.slice(0, 15)}…</span>
							<span class="doc-date">{formatDate(doc.created_at)}</span>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{:else}
		<p class="card-hint">Generate an API key first.</p>
	{/if}
</section>

<style>
	.card {
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--r);
		padding: 20px;
		transition: opacity 0.2s;
	}
	.card.disabled { opacity: 0.4; pointer-events: none; }

	.card-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 6px;
	}

	.step-badge {
		width: 22px; height: 22px;
		border-radius: 50%;
		background: var(--gradient-accent);
		color: #fff;
		font-size: 11px;
		font-weight: 700;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	h2 {
		font-size: 14px;
		font-weight: 700;
		color: var(--text);
	}

	.card-desc {
		font-size: 12px;
		color: var(--text-3);
		margin-bottom: 14px;
		line-height: 1.5;
	}

	.card-hint {
		font-size: 12px;
		color: var(--text-3);
		font-style: italic;
	}

	.btn-secondary {
		padding: 8px 16px;
		background: var(--surface-2);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text-2);
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}
	.btn-secondary:hover {
		background: var(--accent-dim);
		border-color: rgba(99, 102, 241, 0.3);
		color: var(--accent-hover);
	}
	.btn-secondary:disabled { opacity: 0.5; cursor: default; }

	.empty-docs {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 6px;
		padding: 20px;
		color: var(--text-3);
		text-align: center;
	}
	.empty-docs .empty-icon { font-size: 28px; }
	.empty-docs p { font-size: 12px; }
	.empty-docs code {
		font-family: var(--mono);
		background: var(--surface-2);
		padding: 1px 5px;
		border-radius: 3px;
		font-size: 11px;
	}

	.doc-count {
		font-size: 11px;
		color: var(--text-3);
		margin-bottom: 8px;
		font-weight: 500;
	}

	.doc-list {
		display: flex;
		flex-direction: column;
		gap: 4px;
		max-height: 300px;
		overflow-y: auto;
	}

	.doc-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 10px;
		background: var(--bg);
		border: 1px solid var(--border-subtle);
		border-radius: var(--r-sm);
		gap: 12px;
	}

	.doc-info { min-width: 0; flex: 1; }
	.doc-title {
		font-size: 12px;
		font-weight: 600;
		color: var(--text);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	.doc-path {
		font-size: 10px;
		color: var(--text-3);
		font-family: var(--mono);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.doc-meta {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 2px;
		flex-shrink: 0;
	}
	.doc-hash {
		font-size: 10px;
		color: var(--text-3);
		font-family: var(--mono);
	}
	.doc-date {
		font-size: 10px;
		color: var(--text-3);
	}
</style>
