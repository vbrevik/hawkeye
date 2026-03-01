use crate::features::summary::types::RelationType;
use neo4rs::{query, Graph};
use serde::Serialize;
use std::collections::HashMap;

pub struct Neo4jClient {
    graph: Graph,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphResponse {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

fn node_id(node_type: &str, name: &str) -> String {
    format!("{}:{}", node_type.to_lowercase(), name)
}

fn rel_type_str(rel: &RelationType) -> &'static str {
    match rel {
        RelationType::Owns => "owns",
        RelationType::DependsOn => "depends_on",
        RelationType::Manages => "manages",
        RelationType::Uses => "uses",
        RelationType::CreatedBy => "created_by",
        RelationType::PartOf => "part_of",
        RelationType::RelatedTo => "related_to",
        RelationType::LocatedIn => "located_in",
        RelationType::MemberOf => "member_of",
        RelationType::Produces => "produces",
        RelationType::Other => "other",
    }
}

impl Neo4jClient {
    pub async fn new(
        bolt_url: &str,
        user: &str,
        password: &str,
    ) -> Result<Self, neo4rs::Error> {
        let graph = Graph::new(bolt_url, user, password).await?;
        Ok(Self { graph })
    }

    pub async fn ensure_indexes(&self) -> Result<(), neo4rs::Error> {
        self.graph
            .run(query(
                "CREATE CONSTRAINT IF NOT EXISTS FOR (e:Entity) REQUIRE e.name IS UNIQUE",
            ))
            .await?;
        self.graph
            .run(query(
                "CREATE CONSTRAINT IF NOT EXISTS FOR (d:Document) REQUIRE d.id IS UNIQUE",
            ))
            .await?;
        self.graph
            .run(query(
                "CREATE CONSTRAINT IF NOT EXISTS FOR (t:Tag) REQUIRE t.name IS UNIQUE",
            ))
            .await?;
        self.graph
            .run(query(
                "CREATE CONSTRAINT IF NOT EXISTS FOR (tp:Topic) REQUIRE tp.name IS UNIQUE",
            ))
            .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)] // each arg is a distinct graph concept
    pub async fn write_document_graph(
        &self,
        doc_id: &str,
        title: &str,
        source_path: &str,
        entities: &[String],
        tags: &[String],
        topics: &[String],
        relationships: &[(String, String, String, String)],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.graph
            .run(
                query("MERGE (d:Document {id: $id}) SET d.title = $title, d.source_path = $path")
                    .param("id", doc_id.to_string())
                    .param("title", title.to_string())
                    .param("path", source_path.to_string()),
            )
            .await?;

        for entity in entities {
            self.graph
                .run(
                    query(
                        "MERGE (e:Entity {name: $name}) \
                         WITH e \
                         MATCH (d:Document {id: $doc_id}) \
                         MERGE (d)-[:MENTIONS]->(e)",
                    )
                    .param("name", entity.clone())
                    .param("doc_id", doc_id.to_string()),
                )
                .await?;
        }

        for tag in tags {
            self.graph
                .run(
                    query(
                        "MERGE (t:Tag {name: $name}) \
                         WITH t \
                         MATCH (d:Document {id: $doc_id}) \
                         MERGE (d)-[:HAS_TAG]->(t)",
                    )
                    .param("name", tag.clone())
                    .param("doc_id", doc_id.to_string()),
                )
                .await?;
        }

        for topic in topics {
            self.graph
                .run(
                    query(
                        "MERGE (tp:Topic {name: $name}) \
                         WITH tp \
                         MATCH (d:Document {id: $doc_id}) \
                         MERGE (d)-[:ABOUT]->(tp)",
                    )
                    .param("name", topic.clone())
                    .param("doc_id", doc_id.to_string()),
                )
                .await?;
        }

        for (from, rel_type, to, context) in relationships {
            self.graph
                .run(
                    query(
                        "MERGE (a:Entity {name: $from}) \
                         MERGE (b:Entity {name: $to}) \
                         MERGE (a)-[r:RELATED_TO]->(b) \
                         SET r.rel_type = $rel_type, r.context = $ctx",
                    )
                    .param("from", from.clone())
                    .param("to", to.clone())
                    .param("rel_type", rel_type.clone())
                    .param("ctx", context.clone()),
                )
                .await?;
        }

        Ok(())
    }

    pub async fn query_entity(
        &self,
        name: &str,
    ) -> Result<GraphResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut nodes_map: HashMap<String, GraphNode> = HashMap::new();
        let mut edges: Vec<GraphEdge> = Vec::new();

        let center_id = node_id("entity", name);
        nodes_map.insert(
            center_id.clone(),
            GraphNode {
                id: center_id,
                label: name.to_string(),
                node_type: "entity".to_string(),
                source_path: None,
            },
        );

        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH path = (center:Entity {name: $name})-[*1..2]-(n) \
                     WHERE n <> center \
                     WITH relationships(path) AS rels, nodes(path) AS ns \
                     UNWIND range(0, size(rels)-1) AS idx \
                     WITH ns[idx] AS a, rels[idx] AS r, ns[idx+1] AS b \
                     RETURN DISTINCT \
                       labels(a)[0] AS from_type, \
                       coalesce(a.name, a.title) AS from_label, \
                       coalesce(a.id, a.name) AS from_id, \
                       a.source_path AS from_path, \
                       type(r) AS rel_type, \
                       r.context AS ctx, \
                       labels(b)[0] AS to_type, \
                       coalesce(b.name, b.title) AS to_label, \
                       coalesce(b.id, b.name) AS to_id, \
                       b.source_path AS to_path \
                     LIMIT 200",
                )
                .param("name", name.to_string()),
            )
            .await?;

        while let Ok(Some(row)) = result.next().await {
            let from_type: String = row.get("from_type").unwrap_or_default();
            let from_label: String = row.get("from_label").unwrap_or_default();
            let from_id_raw: String = row.get("from_id").unwrap_or_default();
            let from_path: Option<String> = row.get::<String>("from_path").ok();
            let rel_type: String = row.get("rel_type").unwrap_or_default();
            let ctx: Option<String> = row.get::<String>("ctx").ok();
            let to_type: String = row.get("to_type").unwrap_or_default();
            let to_label: String = row.get("to_label").unwrap_or_default();
            let to_id_raw: String = row.get("to_id").unwrap_or_default();
            let to_path: Option<String> = row.get::<String>("to_path").ok();

            let from_nid = node_id(&from_type, &from_id_raw);
            let to_nid = node_id(&to_type, &to_id_raw);

            nodes_map.entry(from_nid.clone()).or_insert_with(|| GraphNode {
                id: from_nid.clone(),
                label: from_label,
                node_type: from_type.to_lowercase(),
                source_path: from_path,
            });

            nodes_map.entry(to_nid.clone()).or_insert_with(|| GraphNode {
                id: to_nid.clone(),
                label: to_label,
                node_type: to_type.to_lowercase(),
                source_path: to_path,
            });

            let context = ctx.filter(|c| !c.is_empty());
            edges.push(GraphEdge {
                source: from_nid,
                target: to_nid,
                label: rel_type,
                context,
            });
        }

        Ok(GraphResponse {
            nodes: nodes_map.into_values().collect(),
            edges,
        })
    }

    pub async fn query_document(
        &self,
        doc_id: &str,
    ) -> Result<GraphResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut nodes_map: HashMap<String, GraphNode> = HashMap::new();
        let mut edges: Vec<GraphEdge> = Vec::new();

        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (d:Document {id: $id})-[r]-(n) \
                     RETURN \
                       d.title AS doc_title, \
                       d.source_path AS doc_path, \
                       type(r) AS rel_type, \
                       r.context AS ctx, \
                       labels(n)[0] AS node_type, \
                       coalesce(n.name, n.title) AS node_label, \
                       coalesce(n.id, n.name) AS node_id, \
                       n.source_path AS node_path",
                )
                .param("id", doc_id.to_string()),
            )
            .await?;

        let doc_nid = node_id("document", doc_id);
        let mut doc_title = doc_id.to_string();
        let mut doc_path: Option<String> = None;

        while let Ok(Some(row)) = result.next().await {
            if let Ok(t) = row.get::<String>("doc_title") {
                doc_title = t;
            }
            if let Ok(p) = row.get::<String>("doc_path") {
                doc_path = Some(p);
            }

            let rel_type: String = row.get("rel_type").unwrap_or_default();
            let ctx: Option<String> = row.get::<String>("ctx").ok();
            let n_type: String = row.get("node_type").unwrap_or_default();
            let n_label: String = row.get("node_label").unwrap_or_default();
            let n_id_raw: String = row.get("node_id").unwrap_or_default();
            let n_path: Option<String> = row.get::<String>("node_path").ok();

            let n_nid = node_id(&n_type, &n_id_raw);
            nodes_map.entry(n_nid.clone()).or_insert_with(|| GraphNode {
                id: n_nid.clone(),
                label: n_label,
                node_type: n_type.to_lowercase(),
                source_path: n_path,
            });

            let context = ctx.filter(|c| !c.is_empty());
            edges.push(GraphEdge {
                source: doc_nid.clone(),
                target: n_nid,
                label: rel_type,
                context,
            });
        }

        nodes_map.insert(
            doc_nid.clone(),
            GraphNode {
                id: doc_nid,
                label: doc_title,
                node_type: "document".to_string(),
                source_path: doc_path,
            },
        );

        Ok(GraphResponse {
            nodes: nodes_map.into_values().collect(),
            edges,
        })
    }
}

pub fn build_relationship_tuples(
    relationships: &[crate::features::summary::Relationship],
) -> Vec<(String, String, String, String)> {
    relationships
        .iter()
        .map(|r| {
            (
                r.from.clone(),
                rel_type_str(&r.rel).to_string(),
                r.to.clone(),
                r.context.clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::summary::types::RelationType;
    use crate::features::summary::Relationship;

    // --- node_id ---

    #[test]
    fn test_node_id_basic() {
        assert_eq!(node_id("entity", "Rust"), "entity:Rust");
    }

    #[test]
    fn test_node_id_lowercases_type() {
        assert_eq!(node_id("Entity", "OAuth2"), "entity:OAuth2");
        assert_eq!(node_id("DOCUMENT", "doc-123"), "document:doc-123");
    }

    #[test]
    fn test_node_id_preserves_name_case() {
        assert_eq!(node_id("tag", "MachineLearning"), "tag:MachineLearning");
    }

    // --- rel_type_str ---

    #[test]
    fn test_rel_type_str_all_variants() {
        assert_eq!(rel_type_str(&RelationType::Owns), "owns");
        assert_eq!(rel_type_str(&RelationType::DependsOn), "depends_on");
        assert_eq!(rel_type_str(&RelationType::Manages), "manages");
        assert_eq!(rel_type_str(&RelationType::Uses), "uses");
        assert_eq!(rel_type_str(&RelationType::CreatedBy), "created_by");
        assert_eq!(rel_type_str(&RelationType::PartOf), "part_of");
        assert_eq!(rel_type_str(&RelationType::RelatedTo), "related_to");
        assert_eq!(rel_type_str(&RelationType::LocatedIn), "located_in");
        assert_eq!(rel_type_str(&RelationType::MemberOf), "member_of");
        assert_eq!(rel_type_str(&RelationType::Produces), "produces");
        assert_eq!(rel_type_str(&RelationType::Other), "other");
    }

    // --- build_relationship_tuples ---

    #[test]
    fn test_build_relationship_tuples_empty() {
        let tuples = build_relationship_tuples(&[]);
        assert!(tuples.is_empty());
    }

    #[test]
    fn test_build_relationship_tuples_single() {
        let rels = vec![Relationship {
            from: "Rust".to_string(),
            rel: RelationType::Uses,
            to: "Ownership".to_string(),
            context: "core concept".to_string(),
        }];
        let tuples = build_relationship_tuples(&rels);
        assert_eq!(tuples.len(), 1);
        assert_eq!(tuples[0].0, "Rust");
        assert_eq!(tuples[0].1, "uses");
        assert_eq!(tuples[0].2, "Ownership");
        assert_eq!(tuples[0].3, "core concept");
    }

    #[test]
    fn test_build_relationship_tuples_multiple() {
        let rels = vec![
            Relationship {
                from: "Axum".to_string(),
                rel: RelationType::DependsOn,
                to: "Tokio".to_string(),
                context: "async runtime".to_string(),
            },
            Relationship {
                from: "Hawkeye".to_string(),
                rel: RelationType::CreatedBy,
                to: "Developer".to_string(),
                context: String::new(),
            },
        ];
        let tuples = build_relationship_tuples(&rels);
        assert_eq!(tuples.len(), 2);
        assert_eq!(tuples[0].1, "depends_on");
        assert_eq!(tuples[1].1, "created_by");
        assert_eq!(tuples[1].3, "");
    }

    // --- GraphNode serialization ---

    #[test]
    fn test_graph_node_json_with_source_path() {
        let node = GraphNode {
            id: "entity:Rust".to_string(),
            label: "Rust".to_string(),
            node_type: "entity".to_string(),
            source_path: Some("/docs/rust.md".to_string()),
        };
        let json = serde_json::to_value(&node).unwrap();
        assert_eq!(json["id"], "entity:Rust");
        assert_eq!(json["label"], "Rust");
        assert_eq!(json["type"], "entity");
        assert_eq!(json["source_path"], "/docs/rust.md");
        assert!(json.get("node_type").is_none());
    }

    #[test]
    fn test_graph_node_json_without_source_path() {
        let node = GraphNode {
            id: "tag:auth".to_string(),
            label: "auth".to_string(),
            node_type: "tag".to_string(),
            source_path: None,
        };
        let json = serde_json::to_value(&node).unwrap();
        assert!(json.get("source_path").is_none());
    }

    // --- GraphEdge serialization ---

    #[test]
    fn test_graph_edge_json_with_context() {
        let edge = GraphEdge {
            source: "entity:Rust".to_string(),
            target: "entity:Ownership".to_string(),
            label: "RELATED_TO".to_string(),
            context: Some("core concept".to_string()),
        };
        let json = serde_json::to_value(&edge).unwrap();
        assert_eq!(json["source"], "entity:Rust");
        assert_eq!(json["target"], "entity:Ownership");
        assert_eq!(json["label"], "RELATED_TO");
        assert_eq!(json["context"], "core concept");
    }

    #[test]
    fn test_graph_edge_json_without_context() {
        let edge = GraphEdge {
            source: "document:doc1".to_string(),
            target: "tag:auth".to_string(),
            label: "HAS_TAG".to_string(),
            context: None,
        };
        let json = serde_json::to_value(&edge).unwrap();
        assert!(json.get("context").is_none());
    }

    // --- GraphResponse serialization ---

    #[test]
    fn test_graph_response_empty() {
        let resp = GraphResponse {
            nodes: vec![],
            edges: vec![],
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert!(json["nodes"].as_array().unwrap().is_empty());
        assert!(json["edges"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_graph_response_roundtrip() {
        let resp = GraphResponse {
            nodes: vec![
                GraphNode {
                    id: "document:doc1".to_string(),
                    label: "Auth Guide".to_string(),
                    node_type: "document".to_string(),
                    source_path: Some("/docs/auth.md".to_string()),
                },
                GraphNode {
                    id: "entity:OAuth2".to_string(),
                    label: "OAuth2".to_string(),
                    node_type: "entity".to_string(),
                    source_path: None,
                },
            ],
            edges: vec![GraphEdge {
                source: "document:doc1".to_string(),
                target: "entity:OAuth2".to_string(),
                label: "MENTIONS".to_string(),
                context: None,
            }],
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(json["edges"].as_array().unwrap().len(), 1);
        assert_eq!(json["edges"][0]["label"], "MENTIONS");
    }
}
