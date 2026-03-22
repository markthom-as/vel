use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;

use crate::TaskId;

/// Storage-agnostic, serde-backed, version-aware durable envelope types for the `0.5` core.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectClass {
    Content,
    Registry,
    ReadModel,
    Runtime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStatus {
    Active,
    Archived,
    Deleted,
    Disabled,
    Deprecated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ObjectProvenance {
    pub origin: Option<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub basis: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SourceSummary {
    #[serde(default)]
    pub linked_providers: Vec<String>,
    pub active_link_count: u32,
    pub primary_upstream_source: Option<String>,
    pub last_sync_at: Option<OffsetDateTime>,
    pub sync_health: Option<String>,
    pub tombstone: bool,
    pub conflict: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalObjectEnvelope<Id> {
    pub id: Id,
    pub object_type: String,
    pub object_class: ObjectClass,
    pub schema_version: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub status: DurableStatus,
    pub provenance: ObjectProvenance,
    pub facets: JsonValue,
    pub deleted_at: Option<OffsetDateTime>,
    pub archived_at: Option<OffsetDateTime>,
    pub source_summary: Option<SourceSummary>,
}

impl<Id> CanonicalObjectEnvelope<Id> {
    pub fn version(&self) -> &str {
        &self.schema_version
    }
}

pub type TaskEnvelope = CanonicalObjectEnvelope<TaskId>;

#[cfg(test)]
mod tests {
    use super::{
        CanonicalObjectEnvelope, DurableStatus, ObjectClass, ObjectProvenance, SourceSummary,
        TaskEnvelope,
    };
    use crate::TaskId;
    use serde_json::json;
    use time::OffsetDateTime;

    #[test]
    fn task_envelope_serializes_source_summary_and_object_class() {
        let envelope: TaskEnvelope = CanonicalObjectEnvelope {
            id: TaskId::new(),
            object_type: "task".to_string(),
            object_class: ObjectClass::Content,
            schema_version: "0.5".to_string(),
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
            status: DurableStatus::Active,
            provenance: ObjectProvenance {
                origin: Some("seeded".to_string()),
                source_refs: vec!["module.core.orientation".to_string()],
                basis: Some("exact".to_string()),
            },
            facets: json!({}),
            deleted_at: None,
            archived_at: None,
            source_summary: Some(SourceSummary {
                linked_providers: vec!["todoist".to_string()],
                active_link_count: 1,
                primary_upstream_source: Some("todoist".to_string()),
                last_sync_at: None,
                sync_health: Some("healthy".to_string()),
                tombstone: false,
                conflict: false,
            }),
        };

        let serialized = serde_json::to_value(&envelope).expect("serialize envelope");

        assert_eq!(serialized["object_class"], "content");
        assert_eq!(serialized["schema_version"], "0.5");
        assert_eq!(serialized["source_summary"]["active_link_count"], 1);
    }
}
