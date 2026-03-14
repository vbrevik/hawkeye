use crate::features::summary::Summary;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::{AllQuery, QueryParser};
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy, TantivyDocument};

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub file: String,
    pub tldr: String,
    pub title: String,
    pub tags: String,
    pub entities: String,
    pub topics: String,
    pub score: f32,
}

pub struct SearchIndexer {
    index: Index,
    writer: IndexWriter,
    #[allow(dead_code)] // stored for potential future schema access
    schema: Schema,
    source_path: Field,
    tldr: Field,
    title: Field,
    tags: Field,
    entities: Field,
    topics: Field,
}

impl SearchIndexer {
    pub fn new_in_dir(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let (schema, fields) = Self::build_schema();
        std::fs::create_dir_all(path)?;
        let index = Index::create_in_dir(path, schema.clone())
            .or_else(|_| Index::open_in_dir(path))?;
        let writer = index.writer(50_000_000)?;
        Ok(Self {
            index,
            writer,
            schema,
            source_path: fields.0,
            tldr: fields.1,
            title: fields.2,
            tags: fields.3,
            entities: fields.4,
            topics: fields.5,
        })
    }

    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let (schema, fields) = Self::build_schema();
        let index = Index::create_in_ram(schema.clone());
        let writer = index.writer(50_000_000)?;
        Ok(Self {
            index,
            writer,
            schema,
            source_path: fields.0,
            tldr: fields.1,
            title: fields.2,
            tags: fields.3,
            entities: fields.4,
            topics: fields.5,
        })
    }

    fn build_schema() -> (Schema, (Field, Field, Field, Field, Field, Field)) {
        let mut builder = Schema::builder();
        let source_path = builder.add_text_field("source_path", STRING | STORED);
        let tldr = builder.add_text_field("tldr", TEXT | STORED);
        let title = builder.add_text_field("title", TEXT | STORED);
        let tags = builder.add_text_field("tags", TEXT | STORED);
        let entities = builder.add_text_field("entities", TEXT | STORED);
        let topics = builder.add_text_field("topics", TEXT | STORED);
        (
            builder.build(),
            (source_path, tldr, title, tags, entities, topics),
        )
    }

    pub fn clear_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.delete_all_documents()?;
        self.writer.commit()?;
        Ok(())
    }

    pub fn index_summaries(&mut self, summaries: &[Summary]) -> Result<usize, Box<dyn std::error::Error>> {
        for summary in summaries {
            self.writer.add_document(doc!(
                self.source_path => summary.source.clone(),
                self.tldr => summary.tldr.clone(),
                self.title => summary.title.clone(),
                self.tags => summary.tags.join(" "),
                self.entities => summary.entities.join(" "),
                self.topics => summary.topics.join(" ")
            ))?;
        }
        self.writer.commit()?;
        Ok(summaries.len())
    }

    pub fn index_summary(&mut self, summary: &Summary) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.add_document(doc!(
            self.source_path => summary.source.clone(),
            self.tldr => summary.tldr.clone(),
            self.title => summary.title.clone(),
            self.tags => summary.tags.join(" "),
            self.entities => summary.entities.join(" "),
            self.topics => summary.topics.join(" ")
        ))?;
        self.writer.commit()?;
        Ok(())
    }

    /// Returns the top `limit` values for a given stored text field, by frequency.
    fn top_values_for_field(
        &self,
        field: Field,
        limit: usize,
    ) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();
        let all_docs = searcher.search(&AllQuery, &TopDocs::with_limit(100_000))?;

        let mut counts: HashMap<String, usize> = HashMap::new();
        for (_, doc_address) in all_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            if let Some(s) = doc.get_first(field).and_then(|v| v.as_str()) {
                for token in s.split_whitespace() {
                    if !token.is_empty() {
                        *counts.entry(token.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        sorted.truncate(limit);
        Ok(sorted)
    }

    pub fn num_docs(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();
        Ok(searcher.num_docs() as usize)
    }

    pub fn top_tags(&self, limit: usize) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
        self.top_values_for_field(self.tags, limit)
    }

    pub fn top_topics(&self, limit: usize) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
        self.top_values_for_field(self.topics, limit)
    }

    pub fn top_entities(&self, limit: usize) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
        self.top_values_for_field(self.entities, limit)
    }

    pub fn search(
        &self,
        query_str: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.tldr, self.title, self.tags, self.entities, self.topics],
        );
        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            let get = |field: Field| -> String {
                doc.get_first(field)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };

            results.push(SearchResult {
                file: get(self.source_path),
                tldr: get(self.tldr),
                title: get(self.title),
                tags: get(self.tags),
                entities: get(self.entities),
                topics: get(self.topics),
                score,
            });
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::summary::Summary;
    use chrono::Utc;

    fn make_summary(source: &str, tldr: &str, title: &str, tags: Vec<&str>) -> Summary {
        Summary {
            source: source.to_string(),
            source_hash: "sha256:test".to_string(),
            created_at: Utc::now(),
            tldr: tldr.to_string(),
            title: title.to_string(),
            tags: tags.into_iter().map(String::from).collect(),
            entities: vec![],
            topics: vec![],
            relationships: vec![],
            word_count: 100,
        }
    }

    #[test]
    fn test_index_and_search() {
        let mut indexer = SearchIndexer::new_in_memory().unwrap();

        indexer
            .index_summary(&make_summary(
                "auth.md",
                "Guide to setting up OAuth2 authentication",
                "OAuth2 Setup Guide",
                vec!["auth", "oauth2"],
            ))
            .unwrap();
        indexer
            .index_summary(&make_summary(
                "deploy.md",
                "Steps for deploying to Kubernetes cluster",
                "K8s Deployment",
                vec!["kubernetes", "devops"],
            ))
            .unwrap();
        indexer
            .index_summary(&make_summary(
                "rust.md",
                "Introduction to Rust ownership and borrowing",
                "Rust Ownership",
                vec!["rust", "programming"],
            ))
            .unwrap();

        let results = indexer.search("OAuth2", 10).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].file, "auth.md");

        let results = indexer.search("kubernetes", 10).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].file, "deploy.md");
    }

    #[test]
    fn test_search_empty_index() {
        let indexer = SearchIndexer::new_in_memory().unwrap();
        let results = indexer.search("anything", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_field_specific_search() {
        let mut indexer = SearchIndexer::new_in_memory().unwrap();

        indexer
            .index_summary(&make_summary(
                "auth.md",
                "Authentication guide",
                "Auth Guide",
                vec!["auth", "security"],
            ))
            .unwrap();
        indexer
            .index_summary(&make_summary(
                "other.md",
                "Something about auth mentioned in passing",
                "Other Doc",
                vec!["unrelated"],
            ))
            .unwrap();

        let results = indexer.search("tags:auth", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file, "auth.md");
    }
}
