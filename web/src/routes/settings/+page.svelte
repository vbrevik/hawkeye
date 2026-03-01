<script lang="ts">
	import type {
		CreateWorkspaceResponse,
		CreateApiKeyResponse,
		WorkspaceDoc
	} from '$lib/api/types';
	import { createWorkspace, createApiKey, fetchWorkspaceDocs } from '$lib/api/workspaces';
	import { setAuthToken } from '$lib/api/client';
	import { addToast } from '$lib/stores/toast';

	let wsName = $state('');
	let wsCreating = $state(false);
	let workspace = $state<CreateWorkspaceResponse | null>(null);

	let keyLabel = $state('');
	let keyCreating = $state(false);
	let apiKey = $state<CreateApiKeyResponse | null>(null);
	let keyCopied = $state(false);

	let docs = $state<WorkspaceDoc[]>([]);
	let docsLoading = $state(false);
	let docsLoaded = $state(false);

	async function handleCreateWorkspace() {
		const name = wsName.trim();
		if (!name) return;
		wsCreating = true;
		try {
			workspace = await createWorkspace(name);
			wsName = '';
			apiKey = null;
			docs = [];
			docsLoaded = false;
			addToast(`Workspace "${workspace.name}" created`, 'success');
		} catch (e) {
			addToast(`Failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
		} finally {
			wsCreating = false;
		}
	}

	async function handleCreateKey() {
		if (!workspace) return;
		keyCreating = true;
		try {
			apiKey = await createApiKey(workspace.id, keyLabel.trim() || undefined);
			keyLabel = '';
			keyCopied = false;
			setAuthToken(apiKey.key);
			addToast('API key created — stored in browser for requests', 'success');
		} catch (e) {
			addToast(`Failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
		} finally {
			keyCreating = false;
		}
	}

	async function handleCopyKey() {
		if (!apiKey) return;
		await navigator.clipboard.writeText(apiKey.key);
		keyCopied = true;
		addToast('API key copied to clipboard', 'info');
		setTimeout(() => { keyCopied = false; }, 2000);
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

	function handleClearAuth() {
		setAuthToken(null);
		workspace = null;
		apiKey = null;
		docs = [];
		docsLoaded = false;
		addToast('Auth token cleared', 'info');
	}

	function handleWsKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') handleCreateWorkspace();
	}

	function handleKeyKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') handleCreateKey();
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('en-US', {
			month: 'short', day: 'numeric', year: 'numeric',
			hour: '2-digit', minute: '2-digit'
		});
	}
</script>

<div class="settings-page">
	<header class="settings-header">
		<a href="/" class="back-link">← Search</a>
		<h1>Settings</h1>
		<p class="subtitle">Manage workspaces and API keys</p>
	</header>

	<div class="settings-body">
		<!-- Step 1: Create Workspace -->
		<section class="card">
			<div class="card-header">
				<span class="step-badge">1</span>
				<h2>Create Workspace</h2>
			</div>
			<p class="card-desc">Workspaces isolate documents and API keys. Create one to get started.</p>

			{#if workspace}
				<div class="success-block">
					<span class="success-icon">✓</span>
					<div>
						<div class="success-title">{workspace.name}</div>
						<div class="success-meta">{workspace.id}</div>
					</div>
				</div>
			{:else}
				<div class="input-row">
					<input
						type="text"
						bind:value={wsName}
						onkeydown={handleWsKeydown}
						placeholder="Workspace name"
						aria-label="Workspace name"
						class="input"
					/>
					<button
						onclick={handleCreateWorkspace}
						disabled={wsCreating || !wsName.trim()}
						class="btn-primary"
					>
						{wsCreating ? 'Creating…' : 'Create'}
					</button>
				</div>
			{/if}
		</section>

		<!-- Step 2: Generate API Key -->
		<section class="card" class:disabled={!workspace}>
			<div class="card-header">
				<span class="step-badge">2</span>
				<h2>Generate API Key</h2>
			</div>
			<p class="card-desc">API keys authenticate requests to protected workspace endpoints.</p>

			{#if apiKey}
				<div class="key-block">
					<div class="key-label-row">
						<span class="key-badge">🔑</span>
						<span class="key-label">{apiKey.label || 'Unnamed key'}</span>
						<span class="key-prefix">{apiKey.key_prefix}…</span>
					</div>
					<div class="key-value-row">
						<code class="key-value">{apiKey.key}</code>
						<button onclick={handleCopyKey} class="btn-copy">
							{keyCopied ? '✓ Copied' : 'Copy'}
						</button>
					</div>
					<p class="key-warning">⚠ This key is shown only once. Copy it now.</p>
				</div>
			{:else if workspace}
				<div class="input-row">
					<input
						type="text"
						bind:value={keyLabel}
						onkeydown={handleKeyKeydown}
						placeholder="Key label (optional)"
						aria-label="API key label"
						class="input"
					/>
					<button
						onclick={handleCreateKey}
						disabled={keyCreating}
						class="btn-primary"
					>
						{keyCreating ? 'Generating…' : 'Generate Key'}
					</button>
				</div>
			{:else}
				<p class="card-hint">Create a workspace first.</p>
			{/if}
		</section>

		<!-- Step 3: Workspace Documents -->
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

		<!-- Clear auth -->
		{#if workspace || apiKey}
			<button onclick={handleClearAuth} class="btn-danger">
				Clear session &amp; start over
			</button>
		{/if}
	</div>
</div>

<style>
	.settings-page {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: var(--bg);
		overflow: hidden;
	}

	.settings-header {
		padding: 16px 24px 12px;
		border-bottom: 1px solid var(--border);
		background: var(--surface);
	}

	.back-link {
		font-size: 13px;
		color: var(--text-2);
		text-decoration: none;
		margin-bottom: 4px;
		display: inline-block;
	}
	.back-link:hover { color: var(--accent-hover); text-decoration: none; }

	h1 {
		font-size: 20px;
		font-weight: 700;
		margin: 4px 0 4px;
		background: var(--gradient-accent);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.subtitle {
		font-size: 13px;
		color: var(--text-3);
		margin-bottom: 4px;
	}

	.settings-body {
		flex: 1;
		overflow-y: auto;
		padding: 24px;
		display: flex;
		flex-direction: column;
		gap: 16px;
		max-width: 640px;
		width: 100%;
		margin: 0 auto;
	}

	/* ── Cards ── */
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

	/* ── Input Row ── */
	.input-row {
		display: flex;
		gap: 8px;
	}

	.input {
		flex: 1;
		padding: 9px 12px;
		background: var(--bg);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text);
		font-size: 13px;
		outline: none;
		transition: border-color 0.15s;
	}
	.input:focus { border-color: var(--accent); }
	.input::placeholder { color: var(--text-3); }

	/* ── Buttons ── */
	.btn-primary {
		padding: 9px 18px;
		background: var(--gradient-accent);
		color: #fff;
		border: none;
		border-radius: var(--r-sm);
		font-weight: 600;
		font-size: 12px;
		cursor: pointer;
		transition: opacity 0.15s;
		white-space: nowrap;
	}
	.btn-primary:hover { opacity: 0.9; }
	.btn-primary:disabled { opacity: 0.5; cursor: default; }

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

	.btn-copy {
		padding: 5px 12px;
		background: var(--accent-dim);
		border: 1px solid rgba(99, 102, 241, 0.25);
		border-radius: var(--r-sm);
		color: var(--accent-hover);
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
	}
	.btn-copy:hover { background: var(--accent-glow); }

	.btn-danger {
		padding: 8px 16px;
		background: transparent;
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text-3);
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
		align-self: flex-start;
	}
	.btn-danger:hover {
		border-color: rgba(248, 113, 113, 0.4);
		color: var(--red);
		background: rgba(248, 113, 113, 0.08);
	}

	/* ── Success Block ── */
	.success-block {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 14px;
		background: rgba(52, 211, 153, 0.08);
		border: 1px solid rgba(52, 211, 153, 0.2);
		border-radius: var(--r-sm);
	}

	.success-icon {
		width: 22px; height: 22px;
		border-radius: 50%;
		background: var(--green);
		color: #000;
		font-size: 12px;
		font-weight: 700;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.success-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--green);
	}

	.success-meta {
		font-size: 11px;
		color: var(--text-3);
		font-family: var(--mono);
	}

	/* ── Key Block ── */
	.key-block {
		padding: 14px;
		background: var(--bg);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.key-label-row {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 12px;
	}

	.key-badge { font-size: 14px; }
	.key-label { font-weight: 600; color: var(--text); }
	.key-prefix { color: var(--text-3); font-family: var(--mono); font-size: 11px; margin-left: auto; }

	.key-value-row {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.key-value {
		flex: 1;
		padding: 8px 10px;
		background: var(--surface);
		border: 1px solid var(--border-subtle);
		border-radius: 4px;
		font-family: var(--mono);
		font-size: 11px;
		color: var(--accent-hover);
		word-break: break-all;
		line-height: 1.4;
	}

	.key-warning {
		font-size: 11px;
		color: var(--yellow);
		font-weight: 500;
	}

	/* ── Documents ── */
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
