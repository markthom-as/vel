use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{ArtifactId, ArtifactStorageKind, CaptureId, PrivacyClass, RunId, SyncClass};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMeta {
    pub request_id: String,
    #[serde(default)]
    pub degraded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorDetail>,
    #[serde(default)]
    pub warnings: Vec<String>,
    pub meta: ApiMeta,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, request_id: impl Into<String>) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
            warnings: Vec::new(),
            meta: ApiMeta {
                request_id: request_id.into(),
                degraded: false,
            },
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>, request_id: impl Into<String>) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(ApiErrorDetail {
                code: code.into(),
                message: message.into(),
            }),
            warnings: Vec::new(),
            meta: ApiMeta {
                request_id: request_id.into(),
                degraded: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthData {
    pub status: String,
    pub db: String,
    pub version: String,
}

/// Status of a single diagnostic check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticStatus {
    Ok,
    Warn,
    Fail,
}

/// A single diagnostic check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub name: String,
    pub status: DiagnosticStatus,
    pub message: String,
}

/// Results of diagnostic checks for `vel doctor`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorData {
    pub checks: Vec<DiagnosticCheck>,
    pub schema_version: u32,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureCreateRequest {
    pub content_text: String,
    #[serde(default = "default_capture_type")]
    pub capture_type: String,
    pub source_device: Option<String>,
}

fn default_capture_type() -> String {
    "quick_note".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureCreateResponse {
    pub capture_id: CaptureId,
    pub accepted_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchQuery {
    pub q: String,
    pub capture_type: Option<String>,
    pub source_device: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub snippet: String,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub source_device: Option<String>,
}

impl From<vel_core::SearchResult> for SearchResult {
    fn from(s: vel_core::SearchResult) -> Self {
        Self {
            capture_id: s.capture_id,
            capture_type: s.capture_type,
            snippet: s.snippet,
            occurred_at: s.occurred_at,
            created_at: s.created_at,
            source_device: s.source_device,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCapture {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub content_text: String,
    pub occurred_at: OffsetDateTime,
    pub source_device: Option<String>,
}

impl From<vel_core::ContextCapture> for ContextCapture {
    fn from(c: vel_core::ContextCapture) -> Self {
        Self {
            capture_id: c.capture_id,
            capture_type: c.capture_type,
            content_text: c.content_text,
            occurred_at: c.occurred_at,
            source_device: c.source_device,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodayData {
    pub date: String,
    pub recent_captures: Vec<ContextCapture>,
    pub focus_candidates: Vec<String>,
    pub reminders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningData {
    pub date: String,
    pub top_active_threads: Vec<String>,
    pub pending_commitments: Vec<String>,
    pub suggested_focus: Option<String>,
    pub key_reminders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfDayData {
    pub date: String,
    pub what_was_done: Vec<ContextCapture>,
    pub what_remains_open: Vec<String>,
    pub what_may_matter_tomorrow: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactCreateRequest {
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    #[serde(default)]
    pub storage_kind: ArtifactStorageKind,
    #[serde(default)]
    pub privacy_class: PrivacyClass,
    #[serde(default)]
    pub sync_class: SyncClass,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactCreateResponse {
    pub artifact_id: ArtifactId,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactData {
    pub artifact_id: ArtifactId,
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    pub storage_kind: String,
    pub privacy_class: String,
    pub sync_class: String,
    pub content_hash: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// --- Runs (spec Section 15) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummaryData {
    pub id: RunId,
    pub kind: String,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEventData {
    pub seq: u32,
    pub event_type: String,
    pub payload: JsonValue,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunDetailData {
    pub id: RunId,
    pub kind: String,
    pub status: String,
    pub input: JsonValue,
    pub output: Option<JsonValue>,
    pub error: Option<JsonValue>,
    pub created_at: OffsetDateTime,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    pub events: Vec<RunEventData>,
}
