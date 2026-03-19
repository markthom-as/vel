use time::OffsetDateTime;

use crate::CaptureId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleClientSurface {
    IosVoice,
    IosCapture,
    WatchBriefing,
    WatchQuickAction,
    MacContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleRequestedOperation {
    CaptureOnly,
    QueryOnly,
    CaptureAndQuery,
    Mutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleVoiceIntent {
    Capture,
    MorningBriefing,
    CurrentSchedule,
    NextCommitment,
    ActiveNudges,
    ExplainWhy,
    BehaviorSummary,
    CompleteCommitment,
    SnoozeNudge,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppleTurnProvenance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript_origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_captured_at: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleVoiceTurnRequest {
    pub transcript: String,
    pub surface: AppleClientSurface,
    pub operation: AppleRequestedOperation,
    #[serde(default)]
    pub intents: Vec<AppleVoiceIntent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<AppleTurnProvenance>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleResponseMode {
    SpokenSummary,
    Card,
    Confirmation,
    ClarificationRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleResponseEvidence {
    pub kind: String,
    pub label: String,
    pub detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleVoiceTurnQueuedMutationSummary {
    pub mutation_kind: String,
    pub queued: bool,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleScheduleEvent {
    pub title: String,
    pub start_ts: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_by_ts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleScheduleSnapshot {
    pub generated_at: i64,
    pub timezone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_event: Option<AppleScheduleEvent>,
    #[serde(default)]
    pub upcoming_events: Vec<AppleScheduleEvent>,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleBehaviorSummaryScope {
    Daily,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleBehaviorMetric {
    pub metric_key: String,
    pub display_label: String,
    pub value: f64,
    pub unit: String,
    pub recorded_at: i64,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleBehaviorSummary {
    pub generated_at: i64,
    pub timezone: String,
    pub scope: AppleBehaviorSummaryScope,
    pub headline: String,
    #[serde(default)]
    pub metrics: Vec<AppleBehaviorMetric>,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleVoiceTurnResponse {
    pub operation: AppleRequestedOperation,
    pub mode: AppleResponseMode,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_id: Option<CaptureId>,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<AppleResponseEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_mutation: Option<AppleVoiceTurnQueuedMutationSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<AppleScheduleSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior_summary: Option<AppleBehaviorSummary>,
}
