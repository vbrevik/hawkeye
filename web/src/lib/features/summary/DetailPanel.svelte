<script lang="ts">
	import type { SearchResult, Summary } from '$lib/api/types';
	import { fetchSummary } from '$lib/api/summary';

	let {
		selected,
		results = [],
		ontagclick,
		onselectresult
	}: {
		selected: SearchResult | null;
		results?: SearchResult[];
		ontagclick: (tag: string) => void;
		onselectresult: (r: SearchResult) => void;
	} = $props();

	let summary = $state<Summary | null>(null);
	let loading = $state(false);

	$effect(() => {
		if (selected) {
			loadSummary(selected.file);
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

	function computeRelated(source: SearchResult): SearchResult[] {
		const myTags = new Set((source.tags || '').split(' ').filter(Boolean));
		const myEntities = new Set((source.entities || '').split(' ').filter(Boolean));
		return results.filter((r) => {
			if (r.file === source.file) return false;
			const rTags = (r.tags || '').split(' ').filter(Boolean);
			const rEntities = (r.entities || '').split(' ').filter(Boolean);
			return rTags.some((t) => myTags.has(t)) || rEntities.some((e) => myEntities.has(e));
		}).slice(0, 5);
	}

	const related = $derived(selected ? computeRelated(selected) : []);
	const displayData = $derived(summary ?? (selected ? {
		title: selected.title || selected.file,
		tldr: selected.tldr,
		tags: (selected.tags || '').split(' ').filter(Boolean),
		topics: (selected.topics || '').split(' ').filter(Boolean),
		entities: (selected.entities || '').split(' ').filter(Boolean),
		word_count: 0,
		created_at: '',
		source: selected.file,
		source_hash: '',
		relationships: []
	} : null));
</script>

<div class="detail-panel">
	{#if !selected}
		<div class="detail-placeholder">
			<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M9 18l6-6-6-6"/>
			</svg>
			<p>Select a result to explore</p>
			<div class="hint">Click any card · ESC to dismiss</div>
		</div>
	{:else if loading}
		<div class="skeleton-wrap">
			<div class="skeleton" style="height: 200px;"></div>
		</div>
	{:else if displayData}
		<div class="detail-body">
			<div class="detail-title">{displayData.title}</div>
			<div class="detail-tldr">{displayData.tldr}</div>

			<div class="detail-section">
				<div class="detail-slabel">Tags</div>
				<div class="detail-chips">
					{#each displayData.tags as tag}
						<button class="detail-chip clickable" onclick={() => ontagclick(tag)}>#{tag}</button>
					{:else}
						<span class="empty">—</span>
					{/each}
				</div>
			</div>

			<div class="detail-section">
				<div class="detail-slabel">Topics</div>
				<div class="detail-chips">
					{#each displayData.topics as topic}
						<span class="detail-chip">{topic}</span>
					{:else}
						<span class="empty">—</span>
					{/each}
				</div>
			</div>

			<div class="detail-section">
				<div class="detail-slabel">Entities</div>
				<div class="detail-chips">
					{#each displayData.entities as entity}
						<span class="detail-chip">{entity}</span>
					{:else}
						<span class="empty">—</span>
					{/each}
				</div>
			</div>

			<div class="detail-meta">
				<div><strong>{(displayData.word_count || 0).toLocaleString()}</strong> words</div>
				<div>Indexed {displayData.created_at ? new Date(displayData.created_at).toLocaleDateString() : '—'}</div>
			</div>

			{#if related.length > 0}
				<div class="related-divider">Related</div>
				<div class="related-list">
					{#each related as rel}
						<button class="related-card" onclick={() => onselectresult(rel)}>
							<div class="related-title">{rel.title || rel.file}</div>
							<div class="related-tldr">{rel.tldr}</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.detail-panel {
		width: 42%; flex-shrink: 0;
		border-left: 1px solid var(--border-subtle);
		background: var(--surface);
		overflow-y: auto; display: flex; flex-direction: column;
	}
	@media (max-width: 900px) { .detail-panel { display: none; } }

	.detail-placeholder {
		flex: 1; display: flex; flex-direction: column;
		align-items: center; justify-content: center;
		gap: 12px; color: var(--text-3); padding: 40px;
	}
	.detail-placeholder p { font-size: 13px; }
	.hint { font-size: 11px; font-family: var(--mono); opacity: 0.5; }

	.skeleton-wrap { padding: 24px; }
	.skeleton {
		border-radius: var(--r);
		background: linear-gradient(90deg, var(--surface) 25%, var(--surface-2) 50%, var(--surface) 75%);
		background-size: 200% 100%;
		animation: skeleton-sweep 1.6s ease infinite;
	}

	.detail-body {
		padding: 24px; display: flex; flex-direction: column; gap: 20px;
		animation: fadeInUp 0.3s ease both;
	}
	.detail-title {
		font-size: 19px; font-weight: 800; color: var(--text);
		line-height: 1.3; letter-spacing: -0.025em;
	}
	.detail-tldr { font-size: 14px; color: var(--text-2); line-height: 1.7; }

	.detail-section { display: flex; flex-direction: column; gap: 6px; }
	.detail-slabel {
		font-size: 10px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.08em; color: var(--text-3);
	}
	.detail-chips { display: flex; flex-wrap: wrap; gap: 4px; }
	.detail-chip {
		background: var(--surface-2); border-radius: 5px; color: var(--text-2);
		font-size: 12px; padding: 3px 9px; font-weight: 500;
		border: none; font-family: var(--font);
	}
	.detail-chip.clickable {
		cursor: pointer; transition: background 0.15s, color 0.15s, transform 0.1s;
	}
	.detail-chip.clickable:hover {
		background: var(--accent-dim); color: var(--accent-hover); transform: scale(1.03);
	}
	.empty { font-size: 12px; color: var(--text-3); }

	.detail-meta {
		font-size: 11px; color: var(--text-3); padding-top: 8px;
		border-top: 1px solid var(--border); display: flex; gap: 16px;
	}
	.detail-meta strong { color: var(--text-2); font-weight: 600; }

	.related-divider {
		font-size: 10px; color: var(--text-3); border-top: 1px solid var(--border);
		padding-top: 14px; margin-top: 4px; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.08em;
	}
	.related-list { display: flex; flex-direction: column; gap: 6px; }
	.related-card {
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: 8px; padding: 12px 14px; cursor: pointer;
		transition: border-color 0.2s, transform 0.15s, box-shadow 0.2s;
		text-align: left; width: 100%; font-family: var(--font); color: var(--text);
	}
	.related-card:hover {
		border-color: rgba(99, 102, 241, 0.4); transform: translateY(-1px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}
	.related-title { font-size: 13px; font-weight: 600; line-height: 1.3; margin-bottom: 3px; }
	.related-tldr {
		font-size: 12px; color: var(--text-3);
		display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical;
		overflow: hidden; line-height: 1.5;
	}
</style>
