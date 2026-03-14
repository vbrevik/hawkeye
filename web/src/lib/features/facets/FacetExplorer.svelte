<script lang="ts">
	import type { Facets, SearchMode } from '$lib/api/types';

	let {
		facets,
		activeFilters = [],
		searchMode = 'hybrid' as SearchMode,
		onaddfilter,
		onremovefilter
	}: {
		facets: Facets;
		activeFilters: string[];
		searchMode?: SearchMode;
		onaddfilter: (name: string) => void;
		onremovefilter: (name: string) => void;
	} = $props();

	const filtersPaused = $derived(searchMode === 'semantic' && activeFilters.length > 0);

	let expanded = $state(false);
	const hasAnyFacets = $derived(
		facets.tags.length + facets.topics.length + facets.entities.length > 0
	);

	const displayTags = $derived(expanded ? facets.tags : facets.tags.slice(0, 8));
	const displayTopics = $derived(expanded ? facets.topics : facets.topics.slice(0, 6));
	const displayEntities = $derived(expanded ? facets.entities : facets.entities.slice(0, 6));
	const hasMore = $derived(
		facets.tags.length > 8 || facets.topics.length > 6 || facets.entities.length > 6
	);
</script>

{#if hasAnyFacets || activeFilters.length > 0}
	<div class="facet-explorer">
		{#if activeFilters.length > 0}
			<div class="active-filters-bar" class:paused={filtersPaused}>
				<span class="filter-label">Filters</span>
				{#each activeFilters as filter}
					<button class="filter-pill" onclick={() => onremovefilter(filter)}>
						{filter}
						<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor"
							stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="pill-x">
							<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
						</svg>
					</button>
				{/each}
				{#if filtersPaused}
					<span class="paused-notice">not applied in semantic mode</span>
				{/if}
				<button class="clear-all" onclick={() => { for (const f of [...activeFilters]) onremovefilter(f); }}>
					Clear all
				</button>
			</div>
		{/if}

		{#if hasAnyFacets}
			<div class="facet-strips">
				{#if displayTags.length > 0}
					<div class="facet-strip">
						<span class="strip-label tag-accent">Tags</span>
						<div class="strip-chips">
							{#each displayTags as entry}
								<button
									class="strip-chip tag-chip"
									class:active={activeFilters.includes(entry.name)}
									onclick={() => onaddfilter(entry.name)}
								>
									#{entry.name} <span class="count">{entry.count}</span>
								</button>
							{/each}
						</div>
					</div>
				{/if}
				{#if displayTopics.length > 0}
					<div class="facet-strip">
						<span class="strip-label topic-accent">Topics</span>
						<div class="strip-chips">
							{#each displayTopics as entry}
								<button
									class="strip-chip topic-chip"
									class:active={activeFilters.includes(entry.name)}
									onclick={() => onaddfilter(entry.name)}
								>
									{entry.name} <span class="count">{entry.count}</span>
								</button>
							{/each}
						</div>
					</div>
				{/if}
				{#if displayEntities.length > 0}
					<div class="facet-strip">
						<span class="strip-label entity-accent">Entities</span>
						<div class="strip-chips">
							{#each displayEntities as entry}
								<button
									class="strip-chip entity-chip"
									class:active={activeFilters.includes(entry.name)}
									onclick={() => onaddfilter(entry.name)}
								>
									{entry.name} <span class="count">{entry.count}</span>
								</button>
							{/each}
						</div>
					</div>
				{/if}
				{#if hasMore}
					<button class="expand-toggle" onclick={() => expanded = !expanded}>
						{expanded ? 'Show less' : 'Show more facets'}
					</button>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	.facet-explorer {
		padding: 0 32px 12px; display: flex; flex-direction: column; gap: 8px;
	}

	/* Active Filters Bar */
	.active-filters-bar {
		display: flex; flex-wrap: wrap; gap: 6px; align-items: center;
		background: var(--accent-dim); border: 1px solid rgba(16, 185, 129, 0.15);
		border-radius: var(--r-sm); padding: 6px 12px;
		transition: opacity 0.25s, border-color 0.25s;
	}
	.active-filters-bar.paused {
		opacity: 0.5;
		border-style: dashed;
	}
	.paused-notice {
		font-size: 10px; color: var(--mode-semantic);
		margin-left: auto; white-space: nowrap;
	}
	.filter-label {
		font-size: 10px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.08em; color: var(--accent-hover); margin-right: 2px;
	}
	.filter-pill {
		display: inline-flex; align-items: center; gap: 4px;
		background: rgba(16, 185, 129, 0.12); border: 1px solid rgba(16, 185, 129, 0.25);
		border-radius: var(--r-sm); color: var(--accent-hover); font-size: 11px;
		font-weight: 500; padding: 2px 8px; cursor: pointer;
		transition: background var(--duration-fast), transform 0.1s;
		font-family: var(--font);
	}
	.filter-pill:hover { background: rgba(16, 185, 129, 0.22); }
	.pill-x { opacity: 0.5; }
	.filter-pill:hover .pill-x { opacity: 1; }
	.clear-all {
		font-size: 10px; color: var(--text-3); background: none; border: none;
		cursor: pointer; font-family: var(--font); padding: 2px 4px;
		transition: color var(--duration-fast);
	}
	.clear-all:hover { color: var(--text-2); }

	/* Facet Strips */
	.facet-strips {
		display: flex; flex-direction: column; gap: 8px;
	}
	.facet-strip {
		display: flex; align-items: flex-start; gap: 8px;
	}
	.strip-label {
		font-size: 9px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.06em; padding: 3px 7px; border-radius: var(--r-sm);
		flex-shrink: 0; margin-top: 1px;
	}
	.tag-accent { background: var(--tag-bg); color: var(--tag-text); }
	.topic-accent { background: var(--topic-bg); color: var(--topic-text); }
	.entity-accent { background: var(--entity-bg); color: var(--entity-text); }

	.strip-chips { display: flex; flex-wrap: wrap; gap: 4px; }
	.strip-chip {
		display: inline-flex; align-items: center; gap: 3px;
		border-radius: var(--r-sm); font-size: 11px; padding: 2px 8px;
		cursor: pointer; font-weight: 500; border: 1px solid;
		font-family: var(--font);
		transition: transform 0.1s, background var(--duration-fast);
	}
	.strip-chip:hover { background-blend-mode: multiply; }
	.strip-chip:active { transform: scale(0.98); }
	.count { opacity: 0.45; font-size: 9px; font-weight: 400; }

	.tag-chip { background: var(--tag-bg); border-color: var(--tag-border); color: var(--tag-text); }
	.tag-chip:hover { background: rgba(165, 180, 252, 0.15); }
	.tag-chip.active { background: rgba(165, 180, 252, 0.18); border-color: rgba(165, 180, 252, 0.35); }

	.topic-chip { background: var(--topic-bg); border-color: var(--topic-border); color: var(--topic-text); }
	.topic-chip:hover { background: rgba(56, 189, 248, 0.15); }
	.topic-chip.active { background: rgba(56, 189, 248, 0.2); border-color: rgba(56, 189, 248, 0.35); }

	.entity-chip { background: var(--entity-bg); border-color: var(--entity-border); color: var(--entity-text); }
	.entity-chip:hover { background: rgba(251, 191, 36, 0.15); }
	.entity-chip.active { background: rgba(251, 191, 36, 0.2); border-color: rgba(251, 191, 36, 0.35); }

	.expand-toggle {
		font-size: 11px; color: var(--text-3); background: none; border: none;
		cursor: pointer; font-family: var(--font); padding: 2px 0;
		transition: color var(--duration-fast);
	}
	.expand-toggle:hover { color: var(--accent-hover); }
</style>
