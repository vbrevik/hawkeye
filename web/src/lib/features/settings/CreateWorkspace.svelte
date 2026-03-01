<script lang="ts">
	import type { CreateWorkspaceResponse } from '$lib/api/types';
	import { createWorkspace } from '$lib/api/workspaces';
	import { addToast } from '$lib/stores/toast';

	let {
		workspace = $bindable(),
		oncreated,
	}: {
		workspace: CreateWorkspaceResponse | null;
		oncreated: () => void;
	} = $props();

	let wsName = $state('');
	let wsCreating = $state(false);

	async function handleCreate() {
		const name = wsName.trim();
		if (!name) return;
		wsCreating = true;
		try {
			workspace = await createWorkspace(name);
			wsName = '';
			addToast(`Workspace "${workspace!.name}" created`, 'success');
			oncreated();
		} catch (e) {
			addToast(`Failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
		} finally {
			wsCreating = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') handleCreate();
	}
</script>

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
				onkeydown={handleKeydown}
				placeholder="Workspace name"
				aria-label="Workspace name"
				class="input"
			/>
			<button
				onclick={handleCreate}
				disabled={wsCreating || !wsName.trim()}
				class="btn-primary"
			>
				{wsCreating ? 'Creating…' : 'Create'}
			</button>
		</div>
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
</style>
