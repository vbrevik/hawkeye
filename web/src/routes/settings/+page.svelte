<script lang="ts">
	import type { CreateWorkspaceResponse, CreateApiKeyResponse } from '$lib/api/types';
	import { setAuthToken } from '$lib/api/client';
	import { addToast } from '$lib/stores/toast';

	import CreateWorkspace from '$lib/features/settings/CreateWorkspace.svelte';
	import GenerateApiKey from '$lib/features/settings/GenerateApiKey.svelte';
	import WorkspaceDocs from '$lib/features/settings/WorkspaceDocs.svelte';

	let workspace = $state<CreateWorkspaceResponse | null>(null);
	let apiKey = $state<CreateApiKeyResponse | null>(null);

	function handleWorkspaceCreated() {
		apiKey = null;
	}

	function handleClearAuth() {
		setAuthToken(null);
		workspace = null;
		apiKey = null;
		addToast('Auth token cleared', 'info');
	}
</script>

<div class="settings-page">
	<header class="settings-header">
		<a href="/" class="back-link">← Search</a>
		<h1>Settings</h1>
		<p class="subtitle">Manage workspaces and API keys</p>
	</header>

	<div class="settings-body">
		<CreateWorkspace bind:workspace oncreated={handleWorkspaceCreated} />

		{#key workspace?.id}
			<GenerateApiKey {workspace} bind:apiKey />
			<WorkspaceDocs {workspace} {apiKey} />
		{/key}

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
</style>
