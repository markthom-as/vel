use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use vel_core::ProjectId;

use crate::NowHeaderBucketKindData;

/// Thread summary/list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadData {
    pub id: String,
    pub thread_type: String,
    pub title: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<ThreadContinuationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ThreadLinkData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<ProjectId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadContinuationData {
    pub escalation_reason: String,
    pub continuation_context: JsonValue,
    #[serde(default)]
    pub review_requirements: Vec<String>,
    pub bounded_capability_state: String,
    pub continuation_category: NowHeaderBucketKindData,
    pub open_target: String,
}

/// Thread link (entity linked to a thread).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLinkData {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCreateRequest {
    pub thread_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLinkRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadUpdateRequest {
    pub status: Option<String>,
}
