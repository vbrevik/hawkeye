<script lang="ts">
	import type { CreateWorkspaceResponse, CreateApiKeyResponse } from '$lib/api/types';
	import { createApiKey } from '$lib/api/workspaces';
	import { setAuthToken } from '$lib/api/client';
	import { addToast } from '$lib/stores/toast';

	let {
		workspace,
		apiKey = $bindable(),
	}: {
		workspace: CreateWorkspaceResponse | null;
		apiKey: CreateApiKeyResponse | null;
	} = $props();

	let keyLabel = $state('');
	let keyCreating = $state(false);
	let keyCopied = $state(false);

	async function handleCreate() {
		if (!workspace) return;
		keyCreating = true;
		try {
			apiKey = await createApiKey(workspace.id, keyLabel.trim() || undefined);
			keyLabel = '';
			keyCopied = false;
			setAuthToken(apiKey!.key);
			addToast('API key created — stored in browser for requests', 'success');
		} catch (e) {
			addToast(`Failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
		} finally {
			keyCreating = false;
		}
	}

	async function handleCopy() {
		if (!apiKey) return;
		await navigator.clipboard.writeText(apiKey.key);
		keyCopied = true;
		addToast('API key copied to clipboard', 'info');
		setTimeout(() => { keyCopied = false; }, 2000);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') handleCreate();
	}
</script>

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
				<button onclick={handleCopy} class="btn-copy">
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
				onkeydown={handleKeydown}
				placeholder="Key label (optional)"
				aria-label="API key label"
				class="input"
			/>
			<button
				onclick={handleCreate}
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
</style>
