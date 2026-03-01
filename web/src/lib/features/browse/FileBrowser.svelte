<script lang="ts">
	import { browse } from '$lib/api/browse';
	import { ingest } from '$lib/api/ingest';
	import { addToast } from '$lib/stores/toast';
	import { fetchFacets } from '$lib/api/search';

	let { onfacetsrefresh }: { onfacetsrefresh?: () => void } = $props();

	let browserPath = $state<string | null>(null);
	let entries = $state<string[]>([]);
	let parent = $state<string | null>(null);
	let mdCount = $state(0);
	let loading = $state(false);
	let ingesting = $state(false);

	async function browseDir(path?: string) {
		loading = true;
		try {
			const data = await browse(path);
			browserPath = data.path;
			entries = data.entries;
			parent = data.parent;
			mdCount = data.md_file_count;
		} catch {
			entries = [];
		} finally {
			loading = false;
		}
	}

	async function doIngest() {
		if (!browserPath) return;
		ingesting = true;
		try {
			const data = await ingest(browserPath);
			addToast(`Queued ${data.files_queued} files, skipped ${data.files_skipped}`, 'success');
			if (data.files_queued > 0) {
				setTimeout(() => onfacetsrefresh?.(), 5000);
			}
		} catch (e) {
			addToast(`Error: ${e instanceof Error ? e.message : 'Unknown'}`, 'error');
		} finally {
			ingesting = false;
		}
	}

	const shortPath = $derived.by(() => {
		if (!browserPath) return '';
		const parts = browserPath.split('/').filter(Boolean);
		return parts.length > 2 ? '…/' + parts.slice(-2).join('/') : browserPath;
	});

	// Browse on mount
	$effect(() => { browseDir(); });
</script>

<div class="browser-wrap">
	<div class="browser-crumb">{shortPath}</div>
	<div class="browser-list">
		{#if loading}
			<div class="browser-empty">Loading…</div>
		{:else}
			{#if parent}
				<button class="browser-row parent" onclick={() => browseDir(parent!)}>
					<span class="browser-icon">↑</span>
					<span>Parent</span>
				</button>
			{/if}
			{#if entries.length === 0}
				<div class="browser-empty">No subdirectories</div>
			{:else}
				{#each entries as name}
					<button class="browser-row" onclick={() => browseDir(browserPath + '/' + name)}>
						<span class="browser-icon">📁</span>
						<span>{name}</span>
					</button>
				{/each}
			{/if}
		{/if}
	</div>
	<div class="browser-md-count" class:none={mdCount === 0}>
		{mdCount === 0 ? 'No .md files here' : `${mdCount} .md file${mdCount !== 1 ? 's' : ''}`}
	</div>
	<button class="btn" onclick={doIngest} disabled={ingesting || !browserPath}>
		{ingesting ? 'Ingesting…' : 'Ingest this directory'}
	</button>
</div>

<style>
	.browser-wrap { display: flex; flex-direction: column; gap: 6px; }
	.browser-crumb {
		font-size: 11px; color: var(--text-2); font-family: var(--mono);
		word-break: break-all; line-height: 1.4; min-height: 14px;
	}
	.browser-list {
		max-height: 180px; overflow-y: auto;
		border: 1px solid var(--border); border-radius: var(--r-sm);
		background: var(--bg);
	}
	.browser-row {
		display: flex; align-items: center; gap: 8px;
		padding: 6px 10px; font-size: 12px; cursor: pointer;
		color: var(--text-2); user-select: none;
		transition: background 0.12s, color 0.12s;
		background: none; border: none; width: 100%; text-align: left;
		font-family: var(--font);
	}
	.browser-row:hover { background: var(--accent-dim); color: var(--text); }
	.browser-row.parent { color: var(--text-3); font-style: italic; }
	.browser-row.parent:hover { color: var(--text-2); }
	.browser-icon { opacity: 0.5; flex-shrink: 0; font-size: 13px; }
	.browser-empty {
		padding: 12px 10px; font-size: 12px; color: var(--text-3); text-align: center;
	}
	.browser-md-count { font-size: 11px; color: var(--text-2); min-height: 16px; font-weight: 500; }
	.browser-md-count.none { color: var(--text-3); font-weight: 400; }

	.btn {
		width: 100%; background: var(--gradient-accent); border: none; border-radius: var(--r-sm);
		color: #fff; font-size: 12px; font-weight: 600; padding: 8px 12px;
		cursor: pointer; transition: transform 0.15s, box-shadow 0.15s, opacity 0.15s;
		letter-spacing: 0.01em;
	}
	.btn:hover { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(99, 102, 241, 0.3); }
	.btn:active { transform: translateY(0); }
	.btn:disabled { opacity: 0.35; cursor: not-allowed; transform: none; box-shadow: none; }
</style>
