use crate::shared::config::DEFAULT_WORKSPACE_ID;
use crate::shared::db::documents;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

const RRF_K: f32 = 60.0;

#[derive(Deserialize)]
pub struct HybridQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HybridResult {
    pub file: String,
    pub title: String,
    pub tldr: String,
    pub tags: String,
    pub entities: String,
    pub topics: String,
    pub hybrid_score: f32,
    pub keyword_rank: Option<usize>,
    pub semantic_rank: Option<usize>,
    pub keyword_score: Option<f32>,
    pub semantic_distance: Option<f32>,
}

pub async fn handle_hybrid_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HybridQuery>,
) -> Result<Json<Vec<HybridResult>>, AppError> {
    let limit = params.limit.unwrap_or(20);
    let fetch_limit = limit * 3;

    let keyword_fut = {
        let indexer = state.indexer.clone();
        let query = params.q.clone();
        async move {
            let idx = indexer.lock().await;
            idx.search(&query, fetch_limit)
                .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })
        }
    };

    let semantic_fut = semantic_search(&state, &params.q, fetch_limit);

    let (keyword_res, semantic_res) = tokio::join!(keyword_fut, semantic_fut);

    let keyword_results = keyword_res.map_err(AppError::internal)?;

    let semantic_results = match semantic_res {
        Ok(results) => results,
        Err(e) => {
            tracing::warn!(error = %e, "semantic search unavailable, using keyword-only");
            Vec::new()
        }
    };

    let merged = merge_rrf(&keyword_results, &semantic_results, limit);

    tracing::info!(
        query = %params.q,
        keyword_hits = keyword_results.len(),
        semantic_hits = semantic_results.len(),
        merged_hits = merged.len(),
        limit,
        "hybrid search"
    );

    Ok(Json(merged))
}

struct SemanticHit {
    file: String,
    title: String,
    tldr: String,
    distance: f32,
}

async fn semantic_search(
    state: &AppState,
    query: &str,
    limit: usize,
) -> Result<Vec<SemanticHit>, Box<dyn std::error::Error + Send + Sync>> {
    let vectors = state.embed.embed_chunks(query).await?;
    let query_vector = vectors
        .into_iter()
        .next()
        .ok_or("embedding returned no vectors")?;

    let workspace_id = DEFAULT_WORKSPACE_ID.to_string();
    let hits = state
        .milvus
        .search_similar(&query_vector, &workspace_id, limit)
        .await?;

    if hits.is_empty() {
        return Ok(vec![]);
    }

    let doc_ids: Vec<Uuid> = hits
        .iter()
        .filter_map(|h| Uuid::parse_str(&h.doc_id).ok())
        .collect();

    let rows = documents::get_doc_summaries_by_ids(&state.pg_pool, &doc_ids).await?;

    let row_map: HashMap<Uuid, documents::DocSummaryBrief> =
        rows.into_iter().map(|r| (r.id, r)).collect();

    let results: Vec<SemanticHit> = hits
        .iter()
        .filter_map(|hit| {
            let doc_uuid = Uuid::parse_str(&hit.doc_id).ok()?;
            let row = row_map.get(&doc_uuid)?;
            let file = std::path::Path::new(&row.source_path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| row.source_path.clone());
            Some(SemanticHit {
                file,
                title: row.title.clone(),
                tldr: row.tldr.clone(),
                distance: hit.distance,
            })
        })
        .collect();

    Ok(results)
}

fn merge_rrf(
    keyword_results: &[crate::features::search::indexer::SearchResult],
    semantic_results: &[SemanticHit],
    limit: usize,
) -> Vec<HybridResult> {
    let mut candidates: HashMap<String, HybridResult> = HashMap::new();

    for (rank, kr) in keyword_results.iter().enumerate() {
        let rrf_score = 1.0 / (RRF_K + (rank + 1) as f32);
        let entry = candidates.entry(kr.file.clone()).or_insert_with(|| HybridResult {
            file: kr.file.clone(),
            title: kr.title.clone(),
            tldr: kr.tldr.clone(),
            tags: kr.tags.clone(),
            entities: kr.entities.clone(),
            topics: kr.topics.clone(),
            hybrid_score: 0.0,
            keyword_rank: None,
            semantic_rank: None,
            keyword_score: None,
            semantic_distance: None,
        });
        entry.hybrid_score += rrf_score;
        entry.keyword_rank = Some(rank + 1);
        entry.keyword_score = Some(kr.score);
    }

    for (rank, sr) in semantic_results.iter().enumerate() {
        let rrf_score = 1.0 / (RRF_K + (rank + 1) as f32);
        let entry = candidates.entry(sr.file.clone()).or_insert_with(|| HybridResult {
            file: sr.file.clone(),
            title: sr.title.clone(),
            tldr: sr.tldr.clone(),
            tags: String::new(),
            entities: String::new(),
            topics: String::new(),
            hybrid_score: 0.0,
            keyword_rank: None,
            semantic_rank: None,
            keyword_score: None,
            semantic_distance: None,
        });
        entry.hybrid_score += rrf_score;
        entry.semantic_rank = Some(rank + 1);
        entry.semantic_distance = Some(sr.distance);
        if entry.title.is_empty() {
            entry.title.clone_from(&sr.title);
        }
        if entry.tldr.is_empty() {
            entry.tldr.clone_from(&sr.tldr);
        }
    }

    let mut results: Vec<HybridResult> = candidates.into_values().collect();
    results.sort_by(|a, b| {
        b.hybrid_score
            .partial_cmp(&a.hybrid_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results.truncate(limit);
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::search::indexer::SearchResult;

    fn make_keyword(file: &str, title: &str, score: f32) -> SearchResult {
        SearchResult {
            file: file.to_string(),
            tldr: format!("{} tldr", title),
            title: title.to_string(),
            tags: "tag1 tag2".to_string(),
            entities: "entity1".to_string(),
            topics: "topic1".to_string(),
            score,
        }
    }

    fn make_semantic(file: &str, title: &str, distance: f32) -> SemanticHit {
        SemanticHit {
            file: file.to_string(),
            title: title.to_string(),
            tldr: format!("{} tldr", title),
            distance,
        }
    }

    #[test]
    fn test_merge_rrf_keyword_only() {
        let keyword = vec![
            make_keyword("a.md", "Doc A", 10.0),
            make_keyword("b.md", "Doc B", 5.0),
        ];
        let semantic: Vec<SemanticHit> = vec![];

        let results = merge_rrf(&keyword, &semantic, 10);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].file, "a.md");
        assert!(results[0].keyword_rank.is_some());
        assert!(results[0].semantic_rank.is_none());
    }

    #[test]
    fn test_merge_rrf_semantic_only() {
        let keyword: Vec<SearchResult> = vec![];
        let semantic = vec![
            make_semantic("a.md", "Doc A", 0.5),
            make_semantic("b.md", "Doc B", 0.8),
        ];

        let results = merge_rrf(&keyword, &semantic, 10);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].file, "a.md");
        assert!(results[0].keyword_rank.is_none());
        assert!(results[0].semantic_rank.is_some());
    }

    #[test]
    fn test_merge_rrf_overlap_boosts_score() {
        let keyword = vec![
            make_keyword("a.md", "Doc A", 10.0),
            make_keyword("b.md", "Doc B", 5.0),
        ];
        let semantic = vec![
            make_semantic("b.md", "Doc B", 0.3),
            make_semantic("c.md", "Doc C", 0.8),
        ];

        let results = merge_rrf(&keyword, &semantic, 10);
        assert_eq!(results.len(), 3);

        let b = results.iter().find(|r| r.file == "b.md").unwrap();
        assert!(b.keyword_rank.is_some());
        assert!(b.semantic_rank.is_some());
        assert!(
            b.hybrid_score > results.iter().find(|r| r.file == "c.md").unwrap().hybrid_score,
            "b.md appears in both lists and should outscore c.md which is semantic-only"
        );
    }

    #[test]
    fn test_merge_rrf_respects_limit() {
        let keyword = vec![
            make_keyword("a.md", "A", 10.0),
            make_keyword("b.md", "B", 9.0),
            make_keyword("c.md", "C", 8.0),
        ];
        let semantic: Vec<SemanticHit> = vec![];

        let results = merge_rrf(&keyword, &semantic, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_merge_rrf_empty_inputs() {
        let results = merge_rrf(&[], &[], 10);
        assert!(results.is_empty());
    }
}
