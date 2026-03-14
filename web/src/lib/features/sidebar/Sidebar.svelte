<script lang="ts">
	import { cancelJobs } from "$lib/api/ingest";
	import { reindex } from "$lib/api/search";
	import { startPolling, stopPolling, queueStatus } from "$lib/stores/status";
	import {
		startHealthPolling,
		stopHealthPolling,
		healthSummary,
	} from "$lib/stores/health";
	import { addToast } from "$lib/stores/toast";

	import FileBrowser from "$lib/features/browse/FileBrowser.svelte";
	import QueueStats from "$lib/features/status/QueueStats.svelte";
	import SystemHealthCard from "$lib/features/status/SystemHealthCard.svelte";

	let {
		pageMode,
		onnavsearch,
		onnavgraph,
		onrefreshfacets,
	}: {
		pageMode: "search" | "graph";
		onnavsearch: () => void;
		onnavgraph: () => void;
		onrefreshfacets: () => void;
	} = $props();

	let cancelConfirm = $state(false);
	let cancelTimer: ReturnType<typeof setTimeout> | null = null;

	let reindexing = $state(false);
	let reindexConfirm = $state(false);
	let reindexTimer: ReturnType<typeof setTimeout> | null = null;

	let showIngest = $state(false);
	let showQueue = $state(false);

	async function handleCancel() {
		if (!cancelConfirm) {
			cancelConfirm = true;
			cancelTimer = setTimeout(() => {
				cancelConfirm = false;
			}, 3000);
			return;
		}
		if (cancelTimer) clearTimeout(cancelTimer);
		cancelConfirm = false;
		try {
			const result = await cancelJobs();
			addToast(
				`Cancelled ${result.cancelled} job${result.cancelled !== 1 ? "s" : ""}`,
				"success",
			);
		} catch (e) {
			addToast(
				`Cancel failed: ${e instanceof Error ? e.message : "Unknown"}`,
				"error",
			);
		}
	}

	async function handleReindex() {
		if (!reindexConfirm) {
			reindexConfirm = true;
			reindexTimer = setTimeout(() => {
				reindexConfirm = false;
			}, 3000);
			return;
		}
		if (reindexTimer) clearTimeout(reindexTimer);
		reindexConfirm = false;
		reindexing = true;
		try {
			const result = await reindex();
			addToast(
				`Rebuilt search index — ${result.indexed} document${result.indexed !== 1 ? "s" : ""} indexed`,
				"success",
			);
			onrefreshfacets();
		} catch (e) {
			addToast(
				`Reindex failed: ${e instanceof Error ? e.message : "Unknown"}`,
				"error",
			);
		} finally {
			reindexing = false;
		}
	}

	$effect(() => {
		if ($queueStatus.in_progress > 0) showQueue = true;
	});

	$effect(() => {
		startPolling();
		startHealthPolling();
		return () => {
			stopPolling();
			stopHealthPolling();
			if (cancelTimer) clearTimeout(cancelTimer);
			if (reindexTimer) clearTimeout(reindexTimer);
		};
	});
	function navSearch(e: MouseEvent | KeyboardEvent) {
		e.preventDefault();
		if (onnavsearch) onnavsearch();
	}

	function navGraph(e: MouseEvent | KeyboardEvent) {
		e.preventDefault();
		if (onnavgraph) onnavgraph();
	}
</script>

<aside class="sidebar">
	<div class="logo">
		<div class="logo-icon">
			<svg
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="12" cy="12" r="3" />
				<path d="M2 12s4-8 10-8 10 8 10 8-4 8-10 8-10-8-10-8z" />
			</svg>
		</div>
		<span class="logo-text">HAWK<em>EYE</em></span>
	</div>

	<nav class="sidebar-nav">
		<button
			class="nav-tab"
			class:active={pageMode === "search"}
			onclick={navSearch}
		>
			<svg
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" />
			</svg>
			Search
		</button>
		<button
			class="nav-tab"
			class:active={pageMode === "graph"}
			onclick={navGraph}
		>
			<svg
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="18" cy="5" r="3" /><circle
					cx="6"
					cy="12"
					r="3"
				/><circle cx="18" cy="19" r="3" />
				<path d="M8.59 13.51 15.42 17.49" /><path
					d="M15.41 6.51 8.59 10.49"
				/>
			</svg>
			Graph
		</button>
	</nav>

	<div class="sidebar-divider"></div>

	<button class="collapse-toggle" onclick={() => (showIngest = !showIngest)}>
		<span class="collapse-label">
			<span class="section-bar"></span>
			<span class="section-title">Ingest</span>
		</span>
		<span class="toggle-chevron" class:open={showIngest}>
			<svg
				width="10"
				height="10"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<polyline points="6 9 12 15 18 9" />
			</svg>
		</span>
	</button>
	{#if showIngest}
		<div class="collapsible-body">
			<FileBrowser onfacetsrefresh={onrefreshfacets} />
		</div>
	{/if}

	<div class="sidebar-divider"></div>

	<button class="collapse-toggle" onclick={() => (showQueue = !showQueue)}>
		<span class="collapse-label">
			<span class="section-bar" class:active={$queueStatus.in_progress > 0}></span>
			<span class="section-title">Queue</span>
		</span>
		<span class="collapse-right">
			{#if $queueStatus.in_progress > 0}
				<span class="queue-badge">{$queueStatus.in_progress}</span>
			{/if}
			<span class="toggle-chevron" class:open={showQueue}>
				<svg
					width="10"
					height="10"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<polyline points="6 9 12 15 18 9" />
				</svg>
			</span>
		</span>
	</button>
	{#if showQueue}
		<div class="collapsible-body">
			<QueueStats />
			<button
				class="btn btn--ghost"
				class:confirm={cancelConfirm}
				onclick={handleCancel}
			>
				{cancelConfirm ? "Confirm cancel?" : "Cancel queued jobs"}
			</button>
		</div>
	{/if}

	<div class="sidebar-divider"></div>

	<button
		class="btn--tool"
		class:confirm={reindexConfirm}
		class:running={reindexing}
		onclick={handleReindex}
		disabled={reindexing}
	>
		{#if reindexing}
			<svg
				class="spin"
				width="12"
				height="12"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M21 12a9 9 0 1 1-6.219-8.56" />
			</svg>
			Rebuilding…
		{:else if reindexConfirm}
			<svg
				width="12"
				height="12"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
				<path d="M3 3v5h5" /><path
					d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"
				/>
				<path d="M16 16h5v5" />
			</svg>
			Confirm rebuild?
		{:else}
			<svg
				width="12"
				height="12"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
				<path d="M3 3v5h5" /><path
					d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"
				/>
				<path d="M16 16h5v5" />
			</svg>
			Rebuild search index
		{/if}
	</button>

	<div class="sidebar-footer" style="margin-top: auto;">
		<SystemHealthCard />
	</div>
</aside>

<style>
	.sidebar {
		width: var(--sidebar-w);
		flex-shrink: 0;
		background: var(--surface);
		border-right: 1px solid var(--border-subtle);
		padding: 14px 12px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		height: 100vh;
		overflow-y: auto;
		overflow-x: hidden;
		position: relative;
		z-index: 50;
	}
	.sidebar::-webkit-scrollbar { width: 3px; }
	.sidebar::-webkit-scrollbar-track { background: transparent; }
	.sidebar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 2px; }

	@media (max-width: 900px) {
		.sidebar { display: none; }
	}

	.logo {
		display: flex;
		align-items: center;
		gap: 8px;
		padding-bottom: 2px;
	}
	.logo-icon {
		width: 26px;
		height: 26px;
		border-radius: var(--r);
		background: var(--gradient-accent);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		flex-shrink: 0;
	}
	.logo-text {
		font-size: 14px;
		font-weight: 800;
		color: var(--text);
		letter-spacing: 0.1em;
		line-height: 1;
	}
	.logo-text em {
		font-style: normal;
		color: var(--accent);
	}

	.sidebar-nav {
		display: flex;
		gap: 4px;
	}
	.nav-tab {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 5px;
		padding: 7px 6px;
		border-radius: var(--r-sm);
		background: transparent;
		border: 1px solid var(--border-subtle);
		color: var(--text-3);
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		transition: all var(--duration-fast);
		cursor: pointer;
	}
	.nav-tab:hover {
		background: var(--surface-2);
		color: var(--text-2);
		border-color: var(--border);
	}
	.nav-tab.active {
		background: var(--accent-dim);
		border-color: rgba(16, 185, 129, 0.3);
		color: var(--accent-hover);
		box-shadow: inset 0 -2px 0 var(--accent);
	}

	.collapse-toggle {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 2px 0;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--text-3);
		transition: color var(--duration-fast);
	}
	.collapse-toggle:hover { color: var(--text-2); }
	.collapse-label {
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.section-bar {
		width: 3px;
		height: 12px;
		background: var(--border);
		border-radius: 1px;
		transition: background var(--duration-fast);
		flex-shrink: 0;
	}
	.section-bar.active {
		background: var(--accent);
		animation: pulse 2.5s ease-in-out infinite;
	}
	.collapse-toggle:hover .section-bar { background: var(--accent); }
	.section-title {
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
	}
	.collapse-right {
		display: flex;
		align-items: center;
		gap: 6px;
	}
	.toggle-chevron {
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--duration-fast);
		opacity: 0.4;
	}
	.toggle-chevron.open { transform: rotate(180deg); opacity: 0.6; }
	.queue-badge {
		font-size: 10px;
		font-weight: 700;
		color: var(--accent-hover);
		background: var(--accent-dim);
		border-radius: var(--r-sm);
		padding: 1px 6px;
		font-family: var(--mono);
		animation: pulse 2.5s ease-in-out infinite;
	}

	.collapsible-body { animation: fadeInUp 0.15s ease both; }

	.sidebar-divider {
		height: 1px;
		background: var(--border-subtle);
		margin: 0;
	}

	.btn--ghost {
		width: 100%;
		background: transparent;
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text-2);
		font-size: 11px;
		padding: 5px 10px;
		margin-top: 6px;
		cursor: pointer;
		transition: border-color 0.1s, color 0.1s, background 0.1s;
		font-family: var(--font);
	}
	.btn--ghost:hover {
		border-color: rgba(248, 113, 113, 0.4);
		color: var(--red);
		background: rgba(248, 113, 113, 0.08);
	}
	.btn--ghost.confirm {
		border-color: rgba(248, 113, 113, 0.5);
		color: var(--red);
		background: rgba(248, 113, 113, 0.08);
	}

	.btn--tool {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 6px;
		background: transparent;
		border: 1px solid var(--border-subtle);
		border-radius: var(--r-sm);
		color: var(--text-3);
		font-size: 11px;
		padding: 7px 10px;
		cursor: pointer;
		font-weight: 500;
		transition: all var(--duration-fast);
		font-family: var(--font);
	}
	.btn--tool:hover {
		border-color: rgba(16, 185, 129, 0.3);
		color: var(--accent-hover);
		background: var(--accent-dim);
	}
	.btn--tool.confirm {
		border-color: rgba(16, 185, 129, 0.4);
		color: var(--accent-hover);
		background: var(--accent-dim);
	}
	.btn--tool.running {
		border-color: rgba(16, 185, 129, 0.25);
		color: var(--accent);
		background: var(--accent-dim);
		cursor: default;
	}
	.btn--tool:disabled { opacity: 0.7; }
	.btn--tool .spin { animation: spin 1s linear infinite; }
	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>
