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

	function sharedFacetCount(a: SearchResult, b: SearchResult): { tags: number; entities: number } {
		const aTags = new Set((a.tags || '').split(' ').filter(Boolean));
		const aEntities = new Set((a.entities || '').split(' ').filter(Boolean));
		const bTags = (b.tags || '').split(' ').filter(Boolean);
		const bEntities = (b.entities || '').split(' ').filter(Boolean);
		return {
			tags: bTags.filter(t => aTags.has(t)).length,
			entities: bEntities.filter(e => aEntities.has(e)).length,
		};
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
			<div class="detail-header-row">
				<div class="detail-title">{displayData.title}</div>
				<a href="/graph" class="view-graph-btn" title="View in Knowledge Graph">
					<svg width="14" height="14" viewBox="0 0 24 24" fill="none"
						stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/>
						<circle cx="18" cy="19" r="3"/><path d="M8.59 13.51 15.42 17.49"/>
						<path d="M15.41 6.51 8.59 10.49"/>
					</svg>
					Graph
				</a>
			</div>
			<div class="detail-tldr">{displayData.tldr}</div>

			<div class="detail-section">
				<div class="detail-slabel">Tags</div>
				<div class="detail-chips">
					{#each displayData.tags as tag}
						<button class="detail-chip clickable tag-styled" onclick={() => ontagclick(tag)}>#{tag}</button>
					{:else}
						<span class="empty">--</span>
					{/each}
				</div>
			</div>

			<div class="detail-section">
				<div class="detail-slabel">Topics</div>
				<div class="detail-chips">
					{#each displayData.topics as topic}
						<button class="detail-chip clickable topic-styled" onclick={() => ontagclick(topic)}>{topic}</button>
					{:else}
						<span class="empty">--</span>
					{/each}
				</div>
			</div>

			<div class="detail-section">
				<div class="detail-slabel">Entities</div>
				<div class="detail-chips">
					{#each displayData.entities as entity}
						<button class="detail-chip clickable entity-styled" onclick={() => ontagclick(entity)}>{entity}</button>
					{:else}
						<span class="empty">--</span>
					{/each}
				</div>
			</div>

			<div class="detail-meta">
				<div><strong>{(displayData.word_count || 0).toLocaleString()}</strong> words</div>
				<div>Indexed {displayData.created_at ? new Date(displayData.created_at).toLocaleDateString() : '--'}</div>
			</div>

			{#if related.length > 0}
				<div class="related-divider">Related Documents</div>
				<div class="related-list">
					{#each related as rel}
						{@const shared = sharedFacetCount(selected!, rel)}
						<button class="related-card" onclick={() => onselectresult(rel)}>
							<div class="related-title">{rel.title || rel.file}</div>
							<div class="related-tldr">{rel.tldr}</div>
							<div class="shared-info">
								{#if shared.tags > 0}
									<span class="shared-badge tag-accent">{shared.tags} shared tag{shared.tags !== 1 ? 's' : ''}</span>
								{/if}
								{#if shared.entities > 0}
									<span class="shared-badge entity-accent">{shared.entities} shared entit{shared.entities !== 1 ? 'ies' : 'y'}</span>
								{/if}
							</div>
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
	.detail-header-row {
		display: flex; align-items: flex-start; justify-content: space-between; gap: 12px;
	}
	.detail-title {
		font-size: 19px; font-weight: 800; color: var(--text);
		line-height: 1.3; letter-spacing: -0.025em;
	}
	.view-graph-btn {
		display: flex; align-items: center; gap: 4px;
		font-size: 11px; font-weight: 600; color: var(--text-3);
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); padding: 4px 10px;
		text-decoration: none; flex-shrink: 0;
		transition: all var(--duration-fast);
	}
	.view-graph-btn:hover {
		color: var(--accent-hover); border-color: rgba(99, 102, 241, 0.3);
		background: var(--accent-dim); text-decoration: none;
	}

	.detail-tldr { font-size: 14px; color: var(--text-2); line-height: 1.7; }

	.detail-section { display: flex; flex-direction: column; gap: 6px; }
	.detail-slabel {
		font-size: 10px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.08em; color: var(--text-3);
	}
	.detail-chips { display: flex; flex-wrap: wrap; gap: 4px; }
	.detail-chip {
		border-radius: 5px; font-size: 12px; padding: 3px 9px;
		font-weight: 500; border: none; font-family: var(--font);
	}
	.detail-chip.clickable {
		cursor: pointer; transition: background 0.15s, color 0.15s, transform 0.1s;
	}
	.detail-chip.clickable:hover { transform: scale(1.03); }

	/* Facet-colored chip variants */
	.tag-styled { background: var(--tag-bg); color: var(--tag-text); }
	.tag-styled:hover { background: rgba(99, 102, 241, 0.18); }
	.topic-styled { background: var(--topic-bg); color: var(--topic-text); }
	.topic-styled:hover { background: rgba(56, 189, 248, 0.15); }
	.entity-styled { background: var(--entity-bg); color: var(--entity-text); }
	.entity-styled:hover { background: rgba(251, 191, 36, 0.15); }

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
	.shared-info { display: flex; gap: 6px; margin-top: 6px; }
	.shared-badge {
		font-size: 10px; padding: 1px 6px; border-radius: 3px; font-weight: 500;
	}
	.shared-badge.tag-accent { background: var(--tag-bg); color: var(--tag-text); }
	.shared-badge.entity-accent { background: var(--entity-bg); color: var(--entity-text); }
</style>
