use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{ArtifactId, RunId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummaryData {
    pub id: RunId,
    pub kind: String,
    pub status: String,
    pub trace_id: String,
    pub parent_run_id: Option<RunId>,
    pub automatic_retry_supported: bool,
    pub automatic_retry_reason: Option<String>,
    pub unsupported_retry_override: bool,
    pub unsupported_retry_override_reason: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub started_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub finished_at: Option<OffsetDateTime>,
    /// Duration in milliseconds; present when run has started_at and finished_at.
    pub duration_ms: Option<i64>,
    /// Optional retry schedule metadata for operator workflows.
    #[serde(with = "time::serde::rfc3339::option")]
    pub retry_scheduled_at: Option<OffsetDateTime>,
    /// Optional operator reason attached when scheduling a retry.
    pub retry_reason: Option<String>,
    /// Optional operator reason attached when marking a run blocked.
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEventData {
    pub seq: u32,
    pub event_type: String,
    pub trace_id: Option<String>,
    pub payload: JsonValue,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

/// Summary of an artifact linked to a run (e.g. via refs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSummaryData {
    pub artifact_id: ArtifactId,
    pub artifact_type: String,
    pub title: Option<String>,
    pub storage_uri: String,
    pub storage_kind: String,
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunUpdateRequest {
    pub status: String,
    #[serde(default, alias = "retry_scheduled_at")]
    pub retry_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub retry_after_seconds: Option<u32>,
    #[serde(default, alias = "retry_reason")]
    pub reason: Option<String>,
    #[serde(default)]
    pub allow_unsupported_retry: bool,
    #[serde(default)]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunDetailData {
    pub id: RunId,
    pub kind: String,
    pub status: String,
    pub trace_id: String,
    pub parent_run_id: Option<RunId>,
    pub automatic_retry_supported: bool,
    pub automatic_retry_reason: Option<String>,
    pub unsupported_retry_override: bool,
    pub unsupported_retry_override_reason: Option<String>,
    pub input: JsonValue,
    pub output: Option<JsonValue>,
    pub error: Option<JsonValue>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub started_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub finished_at: Option<OffsetDateTime>,
    /// Duration in milliseconds; present when run has started_at and finished_at.
    pub duration_ms: Option<i64>,
    /// Optional retry schedule metadata for operator workflows.
    #[serde(with = "time::serde::rfc3339::option")]
    pub retry_scheduled_at: Option<OffsetDateTime>,
    /// Optional operator reason attached when scheduling a retry.
    pub retry_reason: Option<String>,
    /// Optional operator reason attached when marking a run blocked.
    pub blocked_reason: Option<String>,
    pub events: Vec<RunEventData>,
    pub artifacts: Vec<ArtifactSummaryData>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_summary_datetimes_serialize_as_rfc3339_strings() {
        let created_at = OffsetDateTime::from_unix_timestamp(1_710_590_400).unwrap();
        let finished_at = OffsetDateTime::from_unix_timestamp(1_710_590_640).unwrap();
        let value = serde_json::to_value(RunSummaryData {
            id: "run_1".to_string().into(),
            kind: "search".to_string(),
            status: "completed".to_string(),
            trace_id: "trace_1".to_string(),
            parent_run_id: None,
            automatic_retry_supported: false,
            automatic_retry_reason: None,
            unsupported_retry_override: false,
            unsupported_retry_override_reason: None,
            created_at,
            started_at: Some(created_at),
            finished_at: Some(finished_at),
            duration_ms: Some(240_000),
            retry_scheduled_at: None,
            retry_reason: None,
            blocked_reason: None,
        })
        .unwrap();

        assert!(value["created_at"].is_string());
        assert!(value["started_at"].is_string());
        assert!(value["finished_at"].is_string());
    }
}
