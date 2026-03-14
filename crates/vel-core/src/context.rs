//! Domain types for capture/context and search results. Storage returns these; API layer maps to DTOs.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::CaptureId;

/// A single capture as used in context snapshots and lists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCapture {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub content_text: String,
    pub occurred_at: OffsetDateTime,
    pub source_device: Option<String>,
}

/// A single search result (lexical/semantic hit with snippet).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub snippet: String,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub source_device: Option<String>,
}

/// Snapshot of recent captures for today and the past week (orientation/context generation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrientationSnapshot {
    pub recent_today: Vec<ContextCapture>,
    pub recent_week: Vec<ContextCapture>,
}
