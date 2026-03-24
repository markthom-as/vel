//! Domain types for capture/context and search results. Storage returns these; API layer maps to DTOs.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use time::OffsetDateTime;

use crate::{CaptureId, ReflowSeverity, ReflowTriggerKind, VelCoreError};

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
    pub recent_signal_summaries: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CurrentContextReflowStatusKind {
    Applied,
    Editing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentContextReflowStatus {
    pub source_context_computed_at: i64,
    pub recorded_at: i64,
    pub kind: CurrentContextReflowStatusKind,
    pub trigger: ReflowTriggerKind,
    pub severity: ReflowSeverity,
    pub headline: String,
    pub detail: String,
    #[serde(default)]
    pub preview_lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CurrentContextTaskLanes {
    #[serde(default)]
    pub active_commitment_ids: Vec<String>,
    #[serde(default)]
    pub next_up_commitment_ids: Vec<String>,
    #[serde(default)]
    pub if_time_allows_commitment_ids: Vec<String>,
    #[serde(default)]
    pub completed_commitment_ids: Vec<String>,
}

/// Versioned typed representation for current context state.
/// Unknown fields are preserved in `extra` for forward compatibility.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrentContextV1 {
    #[serde(default)]
    pub computed_at: i64,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub morning_state: String,
    #[serde(default)]
    pub inferred_activity: String,
    #[serde(default)]
    pub next_commitment_id: Option<String>,
    #[serde(default)]
    pub next_commitment_due_at: Option<i64>,
    #[serde(default)]
    pub task_lanes: CurrentContextTaskLanes,
    #[serde(default)]
    pub prep_window_active: bool,
    #[serde(default)]
    pub commute_window_active: bool,
    #[serde(default)]
    pub meds_status: String,
    #[serde(default)]
    pub active_nudge_ids: Vec<String>,
    #[serde(default)]
    pub top_risk_commitment_ids: Vec<String>,
    #[serde(default)]
    pub global_risk_level: String,
    #[serde(default)]
    pub global_risk_score: Option<f64>,
    #[serde(default)]
    pub global_risk_missing: bool,
    #[serde(default)]
    pub signals_used: Vec<String>,
    #[serde(default)]
    pub commitments_used: Vec<String>,
    #[serde(default)]
    pub risk_used: Vec<String>,
    #[serde(default)]
    pub attention_state: String,
    #[serde(default)]
    pub drift_type: Option<String>,
    #[serde(default)]
    pub drift_severity: Option<String>,
    #[serde(default)]
    pub attention_confidence: Option<f64>,
    #[serde(default)]
    pub attention_reasons: Vec<String>,
    #[serde(default)]
    pub health_summary: Option<Value>,
    #[serde(default)]
    pub git_activity_summary: Option<Value>,
    #[serde(default)]
    pub mood_summary: Option<Value>,
    #[serde(default)]
    pub pain_summary: Option<Value>,
    #[serde(default)]
    pub note_document_summary: Option<Value>,
    #[serde(default)]
    pub assistant_message_summary: Option<Value>,
    #[serde(default)]
    pub message_waiting_on_me_count: Option<u64>,
    #[serde(default)]
    pub message_waiting_on_others_count: Option<u64>,
    #[serde(default)]
    pub message_scheduling_thread_count: Option<u64>,
    #[serde(default)]
    pub message_urgent_thread_count: Option<u64>,
    #[serde(default)]
    pub message_summary: Option<Value>,
    #[serde(default)]
    pub leave_by_ts: Option<i64>,
    #[serde(default)]
    pub next_event_start_ts: Option<i64>,
    #[serde(default)]
    pub reflow_status: Option<CurrentContextReflowStatus>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl CurrentContextV1 {
    pub fn into_json(self) -> Value {
        serde_json::to_value(self).unwrap_or_else(|_| Value::Object(Map::new()))
    }
}

pub struct ContextMigrator;

impl ContextMigrator {
    pub fn from_json_value(value: Value) -> Result<CurrentContextV1, VelCoreError> {
        serde_json::from_value::<CurrentContextV1>(value)
            .map_err(|error| VelCoreError::Validation(format!("current context parse: {error}")))
    }

    pub fn from_json_str(raw: &str) -> Result<CurrentContextV1, VelCoreError> {
        let value = serde_json::from_str::<Value>(raw)
            .map_err(|error| VelCoreError::Validation(format!("current context json: {error}")))?;
        Self::from_json_value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ContextMigrator, CurrentContextReflowStatus, CurrentContextReflowStatusKind,
        CurrentContextV1,
    };
    use crate::{ReflowSeverity, ReflowTriggerKind};

    #[test]
    fn context_migrator_parses_known_context_shape() {
        let raw = r#"{
          "computed_at": 1710000000,
          "mode": "morning_mode",
          "morning_state": "awake_unstarted",
          "meds_status": "pending",
          "attention_state": "drifting",
          "drift_type": "morning_drift",
          "active_nudge_ids": ["nud_1"],
          "signals_used": ["sig_1"],
          "custom_future_field": { "ok": true }
        }"#;

        let context = ContextMigrator::from_json_str(raw).expect("context should parse");
        assert_eq!(context.mode, "morning_mode");
        assert_eq!(context.morning_state, "awake_unstarted");
        assert_eq!(context.meds_status, "pending");
        assert_eq!(context.attention_state, "drifting");
        assert_eq!(context.drift_type.as_deref(), Some("morning_drift"));
        assert_eq!(context.active_nudge_ids.len(), 1);
        assert!(context.task_lanes.active_commitment_ids.is_empty());
        assert!(context.extra.contains_key("custom_future_field"));
    }

    #[test]
    fn context_migrator_uses_defaults_for_missing_fields() {
        let context = ContextMigrator::from_json_str("{}").expect("empty context should parse");
        assert_eq!(context.mode, "");
        assert_eq!(context.meds_status, "");
        assert!(context.active_nudge_ids.is_empty());
        assert!(context.task_lanes.next_up_commitment_ids.is_empty());
        assert!(context.extra.is_empty());
    }

    #[test]
    fn current_context_v1_serializes_back_to_json() {
        let context = CurrentContextV1 {
            mode: "today_mode".to_string(),
            meds_status: "done".to_string(),
            reflow_status: Some(CurrentContextReflowStatus {
                source_context_computed_at: 1_710_000_000,
                recorded_at: 1_710_000_600,
                kind: CurrentContextReflowStatusKind::Applied,
                trigger: ReflowTriggerKind::StaleSchedule,
                severity: ReflowSeverity::High,
                headline: "Reflow accepted".to_string(),
                detail: "Vel marked the current schedule for reflow review.".to_string(),
                preview_lines: vec!["Current context is 30 minutes old.".to_string()],
                thread_id: None,
            }),
            ..CurrentContextV1::default()
        };
        let value = context.into_json();
        assert_eq!(value["mode"], "today_mode");
        assert_eq!(value["meds_status"], "done");
        assert_eq!(value["reflow_status"]["kind"], "applied");
    }
}
