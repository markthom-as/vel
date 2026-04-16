use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{ContextCapture, UnixSeconds};

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

/// Persistent current context singleton (computed by inference engine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContextData {
    pub computed_at: UnixSeconds,
    pub context: JsonValue,
}

/// One entry in the context timeline (material context transitions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTimelineEntry {
    pub id: String,
    pub timestamp: i64,
    pub context: JsonValue,
}
