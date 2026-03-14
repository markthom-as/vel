use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_core::CaptureId;

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
