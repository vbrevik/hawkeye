<script lang="ts">
	import type { SearchResult, Summary } from '$lib/api/types';
	import { fetchSummary } from '$lib/api/summary';

	let {
		result,
		open = false,
		onclose,
		ontagclick
	}: {
		result: SearchResult | null;
		open: boolean;
		onclose: () => void;
		ontagclick: (tag: string) => void;
	} = $props();

	let summary = $state<Summary | null>(null);
	let loading = $state(false);

	$effect(() => {
		if (result && open) {
			loadSummary(result.file);
		} else {
			summary = null;
		}
	});

	async function loadSummary(file: string) {
		loading = true;
		try {
			summary = await fetchSummary(file);
		} catch {
			summary = null;
		} finally {
			loading = false;
		}
	}

	const title = $derived(
		summary?.title ?? result?.title ?? result?.file ?? '—'
	);

	const tags = $derived(
		summary?.tags ?? (result?.tags || '').split(' ').filter(Boolean)
	);
	const entities = $derived(
		summary?.entities ?? (result?.entities || '').split(' ').filter(Boolean)
	);
	const topics = $derived(
		summary?.topics ?? (result?.topics || '').split(' ').filter(Boolean)
	);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="drawer-overlay" class:open onclick={onclose}></div>
<div class="drawer" class:open>
	<div class="drawer-header">
		<div class="drawer-title">{title}</div>
		<button class="drawer-close" onclick={onclose}>✕</button>
	</div>
	<div class="drawer-body">
		{#if loading}
			<div class="skeleton" style="height: 140px;"></div>
		{:else}
			<div class="drawer-section">
				<div class="drawer-slabel">File</div>
				<div class="drawer-file">{result?.file ?? '—'}</div>
			</div>

			<div class="drawer-section">
				<div class="drawer-slabel">TL;DR</div>
				<div class="drawer-text">{summary?.tldr ?? result?.tldr ?? ''}</div>
			</div>

			{#if tags.length > 0}
				<div class="drawer-section">
					<div class="drawer-slabel">Tags</div>
					<div class="drawer-chips">
						{#each tags as tag}
							<button class="drawer-chip" onclick={() => ontagclick(tag)}>#{tag}</button>
						{/each}
					</div>
				</div>
			{/if}

			{#if entities.length > 0}
				<div class="drawer-section">
					<div class="drawer-slabel">Entities</div>
					<div class="drawer-chips">
						{#each entities as entity}
							<button class="drawer-chip entity-styled" onclick={() => ontagclick(entity)}>{entity}</button>
						{/each}
					</div>
				</div>
			{/if}

			{#if topics.length > 0}
				<div class="drawer-section">
					<div class="drawer-slabel">Topics</div>
					<div class="drawer-chips">
						{#each topics as topic}
							<button class="drawer-chip topic-styled" onclick={() => ontagclick(topic)}>{topic}</button>
						{/each}
					</div>
				</div>
			{/if}

			{#if summary}
				<div class="drawer-meta">
					<div><strong>{(summary.word_count || 0).toLocaleString()}</strong> words</div>
					<div>Indexed {summary.created_at ? new Date(summary.created_at).toLocaleDateString() : '—'}</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.drawer-overlay {
		position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px); -webkit-backdrop-filter: blur(4px);
		opacity: 0; pointer-events: none; transition: opacity 0.3s; z-index: 40;
	}
	.drawer-overlay.open { opacity: 1; pointer-events: all; }

	.drawer {
		position: fixed; top: 0; right: 0; bottom: 0; width: var(--drawer-w, 360px);
		background: var(--surface); border-left: 1px solid var(--border);
		transform: translateX(100%);
		transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		z-index: 50; display: flex; flex-direction: column; overflow: hidden;
	}
	.drawer.open { transform: translateX(0); }

	.drawer-header {
		padding: 18px 20px; border-bottom: 1px solid var(--border);
		display: flex; align-items: flex-start; justify-content: space-between; gap: 12px;
	}
	.drawer-title { font-size: 16px; font-weight: 700; color: var(--text); line-height: 1.3; }
	.drawer-close {
		background: var(--surface-2); border: 1px solid var(--border); border-radius: var(--r-sm);
		color: var(--text-2); font-size: 13px; width: 30px; height: 30px;
		display: flex; align-items: center; justify-content: center;
		cursor: pointer; flex-shrink: 0; transition: background 0.15s, transform 0.1s;
	}
	.drawer-close:hover { background: var(--border); transform: scale(1.05); }

	.drawer-body {
		flex: 1; overflow-y: auto; padding: 18px 20px;
		display: flex; flex-direction: column; gap: 20px;
	}

	.drawer-section { display: flex; flex-direction: column; gap: 6px; }
	.drawer-slabel {
		font-size: 10px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.08em; color: var(--text-3);
	}
	.drawer-file {
		font-size: 11px; font-family: var(--mono); color: var(--text-3); word-break: break-all;
	}
	.drawer-text { font-size: 13px; color: var(--text-2); line-height: 1.7; }
	.drawer-chips { display: flex; flex-wrap: wrap; gap: 4px; }
	.drawer-chip {
		background: var(--surface-2); border-radius: 4px; color: var(--text-2);
		font-size: 12px; padding: 3px 8px; cursor: pointer;
		transition: background 0.15s, color 0.15s; border: none; font-family: var(--font);
	}
	.drawer-chip:hover { background: var(--accent-dim); color: var(--accent-hover); }
	.drawer-chip.topic-styled { background: var(--topic-bg); color: var(--topic-text); }
	.drawer-chip.topic-styled:hover { background: rgba(56, 189, 248, 0.15); }
	.drawer-chip.entity-styled { background: var(--entity-bg); color: var(--entity-text); }
	.drawer-chip.entity-styled:hover { background: rgba(251, 191, 36, 0.15); }

	.drawer-meta {
		display: flex; gap: 16px; padding-top: 8px; border-top: 1px solid var(--border);
		font-size: 12px; color: var(--text-3);
	}
	.drawer-meta strong { color: var(--text-2); font-weight: 600; }

	.skeleton {
		border-radius: var(--r);
		background: linear-gradient(90deg, var(--surface) 25%, var(--surface-2) 50%, var(--surface) 75%);
		background-size: 200% 100%;
		animation: skeleton-sweep 1.6s ease infinite;
	}
</style>
