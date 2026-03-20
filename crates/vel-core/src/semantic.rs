use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecordId(pub String);

impl SemanticRecordId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticSourceKind {
    Capture,
    Artifact,
    Project,
    Note,
    TranscriptNote,
    Thread,
    Message,
    Person,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SemanticProvenance {
    #[serde(default)]
    pub capture_id: Option<String>,
    #[serde(default)]
    pub artifact_id: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub note_path: Option<String>,
    #[serde(default)]
    pub transcript_id: Option<String>,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub person_id: Option<String>,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticMemoryRecord {
    pub record_id: SemanticRecordId,
    pub source_kind: SemanticSourceKind,
    pub source_id: String,
    pub chunk_id: String,
    pub content_text: String,
    pub embedding_model: String,
    pub embedding_revision: String,
    pub token_count: u32,
    #[serde(default)]
    pub metadata_json: Value,
    pub provenance: SemanticProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SemanticQueryFilters {
    #[serde(default)]
    pub source_kinds: Vec<SemanticSourceKind>,
    #[serde(default)]
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalStrategy {
    LexicalOnly,
    SemanticOnly,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridRetrievalPolicy {
    pub lexical_weight: f32,
    pub semantic_weight: f32,
    pub rerank_window: u32,
    pub min_combined_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticQuery {
    pub query_text: String,
    pub top_k: u32,
    pub strategy: RetrievalStrategy,
    pub include_provenance: bool,
    pub filters: SemanticQueryFilters,
    #[serde(default)]
    pub policy: Option<HybridRetrievalPolicy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticHit {
    pub record_id: SemanticRecordId,
    pub source_kind: SemanticSourceKind,
    pub source_id: String,
    pub snippet: String,
    pub lexical_score: f32,
    pub semantic_score: f32,
    pub combined_score: f32,
    pub provenance: SemanticProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecallContextSourceCount {
    pub source_kind: SemanticSourceKind,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecallContextHit {
    pub record_id: SemanticRecordId,
    pub source_kind: SemanticSourceKind,
    pub source_id: String,
    pub snippet: String,
    pub lexical_score: f32,
    pub semantic_score: f32,
    pub combined_score: f32,
    pub provenance: SemanticProvenance,
}

impl From<SemanticHit> for RecallContextHit {
    fn from(value: SemanticHit) -> Self {
        Self {
            record_id: value.record_id,
            source_kind: value.source_kind,
            source_id: value.source_id,
            snippet: value.snippet,
            lexical_score: value.lexical_score,
            semantic_score: value.semantic_score,
            combined_score: value.combined_score,
            provenance: value.provenance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecallContextPack {
    pub query_text: String,
    pub hit_count: u32,
    #[serde(default)]
    pub source_counts: Vec<RecallContextSourceCount>,
    #[serde(default)]
    pub hits: Vec<RecallContextHit>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    fn repo_file(relative: &str) -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join(relative),
        )
        .expect("repo file should be readable")
    }

    #[test]
    fn semantic_query_example_parses() {
        let raw = repo_file("config/examples/semantic-query.example.json");
        let query: SemanticQuery = serde_json::from_str(&raw).expect("semantic query should parse");
        assert_eq!(query.top_k, 5);
        assert_eq!(query.strategy, RetrievalStrategy::Hybrid);
    }

    #[test]
    fn semantic_memory_record_example_parses() {
        let raw = repo_file("config/examples/semantic-memory-record.example.json");
        let record: SemanticMemoryRecord =
            serde_json::from_str(&raw).expect("semantic record should parse");
        assert_eq!(record.source_kind, SemanticSourceKind::Capture);
        assert_eq!(
            record.provenance.capture_id.as_deref(),
            Some("cap_example_01")
        );
    }

    #[test]
    fn recall_context_pack_serializes_named_counts_and_hits() {
        let value = serde_json::to_value(RecallContextPack {
            query_text: "accountant follow up".to_string(),
            hit_count: 1,
            source_counts: vec![RecallContextSourceCount {
                source_kind: SemanticSourceKind::Note,
                count: 1,
            }],
            hits: vec![RecallContextHit {
                record_id: SemanticRecordId::new("sem_note_1"),
                source_kind: SemanticSourceKind::Note,
                source_id: "projects/tax/accountant.md".to_string(),
                snippet: "Need accountant follow up on quarterly estimate.".to_string(),
                lexical_score: 0.4,
                semantic_score: 0.9,
                combined_score: 0.775,
                provenance: SemanticProvenance {
                    note_path: Some("projects/tax/accountant.md".to_string()),
                    ..Default::default()
                },
            }],
        })
        .expect("recall context should serialize");

        assert_eq!(value["query_text"], "accountant follow up");
        assert_eq!(value["hit_count"], 1);
        assert_eq!(value["source_counts"][0]["source_kind"], "note");
        assert_eq!(
            value["hits"][0]["provenance"]["note_path"],
            "projects/tax/accountant.md"
        );
    }
}
