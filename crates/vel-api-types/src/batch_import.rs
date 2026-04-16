use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    ProjectFamilyData, ProjectProvisionRequestData, ProjectRootRefData, ProjectStatusData,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BatchImportItem {
    Capture(BatchImportCapture),
    Signal(BatchImportSignal),
    Project(BatchImportProject),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportCapture {
    pub capture_id: String,
    pub content_text: String,
    #[serde(default = "default_batch_import_capture_type")]
    pub capture_type: String,
    #[serde(default)]
    pub source_device: Option<String>,
}

fn default_batch_import_capture_type() -> String {
    "quick_note".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportSignal {
    pub signal_type: String,
    pub source: String,
    #[serde(default)]
    pub source_ref: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportProject {
    pub slug: String,
    pub name: String,
    pub family: ProjectFamilyData,
    #[serde(default)]
    pub status: Option<ProjectStatusData>,
    pub primary_repo: ProjectRootRefData,
    pub primary_notes_root: ProjectRootRefData,
    #[serde(default)]
    pub secondary_repos: Vec<ProjectRootRefData>,
    #[serde(default)]
    pub secondary_notes_roots: Vec<ProjectRootRefData>,
    #[serde(default)]
    pub upstream_ids: BTreeMap<String, String>,
    #[serde(default)]
    pub pending_provision: ProjectProvisionRequestData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportRequest {
    pub items: Vec<BatchImportItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportResponse {
    pub results: Vec<BatchImportItemResult>,
    pub summary: BatchImportSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportItemResult {
    pub index: usize,
    pub kind: String,
    pub status: BatchImportItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchImportItemStatus {
    Created,
    Skipped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImportSummary {
    pub total: usize,
    pub created: usize,
    pub skipped: usize,
    pub errors: usize,
}
