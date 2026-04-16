use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_core::CaptureId;

use crate::UnixSeconds;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleClientSurfaceData {
    IosVoice,
    IosCapture,
    WatchBriefing,
    WatchQuickAction,
    MacContext,
}

impl From<vel_core::AppleClientSurface> for AppleClientSurfaceData {
    fn from(value: vel_core::AppleClientSurface) -> Self {
        match value {
            vel_core::AppleClientSurface::IosVoice => Self::IosVoice,
            vel_core::AppleClientSurface::IosCapture => Self::IosCapture,
            vel_core::AppleClientSurface::WatchBriefing => Self::WatchBriefing,
            vel_core::AppleClientSurface::WatchQuickAction => Self::WatchQuickAction,
            vel_core::AppleClientSurface::MacContext => Self::MacContext,
        }
    }
}

impl From<AppleClientSurfaceData> for vel_core::AppleClientSurface {
    fn from(value: AppleClientSurfaceData) -> Self {
        match value {
            AppleClientSurfaceData::IosVoice => Self::IosVoice,
            AppleClientSurfaceData::IosCapture => Self::IosCapture,
            AppleClientSurfaceData::WatchBriefing => Self::WatchBriefing,
            AppleClientSurfaceData::WatchQuickAction => Self::WatchQuickAction,
            AppleClientSurfaceData::MacContext => Self::MacContext,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleRequestedOperationData {
    CaptureOnly,
    QueryOnly,
    CaptureAndQuery,
    Mutation,
}

impl From<vel_core::AppleRequestedOperation> for AppleRequestedOperationData {
    fn from(value: vel_core::AppleRequestedOperation) -> Self {
        match value {
            vel_core::AppleRequestedOperation::CaptureOnly => Self::CaptureOnly,
            vel_core::AppleRequestedOperation::QueryOnly => Self::QueryOnly,
            vel_core::AppleRequestedOperation::CaptureAndQuery => Self::CaptureAndQuery,
            vel_core::AppleRequestedOperation::Mutation => Self::Mutation,
        }
    }
}

impl From<AppleRequestedOperationData> for vel_core::AppleRequestedOperation {
    fn from(value: AppleRequestedOperationData) -> Self {
        match value {
            AppleRequestedOperationData::CaptureOnly => Self::CaptureOnly,
            AppleRequestedOperationData::QueryOnly => Self::QueryOnly,
            AppleRequestedOperationData::CaptureAndQuery => Self::CaptureAndQuery,
            AppleRequestedOperationData::Mutation => Self::Mutation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleVoiceIntentData {
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

impl From<vel_core::AppleVoiceIntent> for AppleVoiceIntentData {
    fn from(value: vel_core::AppleVoiceIntent) -> Self {
        match value {
            vel_core::AppleVoiceIntent::Capture => Self::Capture,
            vel_core::AppleVoiceIntent::MorningBriefing => Self::MorningBriefing,
            vel_core::AppleVoiceIntent::CurrentSchedule => Self::CurrentSchedule,
            vel_core::AppleVoiceIntent::NextCommitment => Self::NextCommitment,
            vel_core::AppleVoiceIntent::ActiveNudges => Self::ActiveNudges,
            vel_core::AppleVoiceIntent::ExplainWhy => Self::ExplainWhy,
            vel_core::AppleVoiceIntent::BehaviorSummary => Self::BehaviorSummary,
            vel_core::AppleVoiceIntent::CompleteCommitment => Self::CompleteCommitment,
            vel_core::AppleVoiceIntent::SnoozeNudge => Self::SnoozeNudge,
        }
    }
}

impl From<AppleVoiceIntentData> for vel_core::AppleVoiceIntent {
    fn from(value: AppleVoiceIntentData) -> Self {
        match value {
            AppleVoiceIntentData::Capture => Self::Capture,
            AppleVoiceIntentData::MorningBriefing => Self::MorningBriefing,
            AppleVoiceIntentData::CurrentSchedule => Self::CurrentSchedule,
            AppleVoiceIntentData::NextCommitment => Self::NextCommitment,
            AppleVoiceIntentData::ActiveNudges => Self::ActiveNudges,
            AppleVoiceIntentData::ExplainWhy => Self::ExplainWhy,
            AppleVoiceIntentData::BehaviorSummary => Self::BehaviorSummary,
            AppleVoiceIntentData::CompleteCommitment => Self::CompleteCommitment,
            AppleVoiceIntentData::SnoozeNudge => Self::SnoozeNudge,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppleTurnProvenanceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_origin: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_captured_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<OffsetDateTime>,
}

impl From<vel_core::AppleTurnProvenance> for AppleTurnProvenanceData {
    fn from(value: vel_core::AppleTurnProvenance) -> Self {
        Self {
            source_device: value.source_device,
            locale: value.locale,
            transcript_origin: value.transcript_origin,
            recorded_at: value.recorded_at,
            offline_captured_at: value.offline_captured_at,
            queued_at: value.queued_at,
        }
    }
}

impl From<AppleTurnProvenanceData> for vel_core::AppleTurnProvenance {
    fn from(value: AppleTurnProvenanceData) -> Self {
        Self {
            source_device: value.source_device,
            locale: value.locale,
            transcript_origin: value.transcript_origin,
            recorded_at: value.recorded_at,
            offline_captured_at: value.offline_captured_at,
            queued_at: value.queued_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleVoiceTurnRequestData {
    pub transcript: String,
    pub surface: AppleClientSurfaceData,
    pub operation: AppleRequestedOperationData,
    #[serde(default)]
    pub intents: Vec<AppleVoiceIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<AppleTurnProvenanceData>,
}

impl From<vel_core::AppleVoiceTurnRequest> for AppleVoiceTurnRequestData {
    fn from(value: vel_core::AppleVoiceTurnRequest) -> Self {
        Self {
            transcript: value.transcript,
            surface: value.surface.into(),
            operation: value.operation.into(),
            intents: value.intents.into_iter().map(Into::into).collect(),
            provenance: value.provenance.map(Into::into),
        }
    }
}

impl From<AppleVoiceTurnRequestData> for vel_core::AppleVoiceTurnRequest {
    fn from(value: AppleVoiceTurnRequestData) -> Self {
        Self {
            transcript: value.transcript,
            surface: value.surface.into(),
            operation: value.operation.into(),
            intents: value.intents.into_iter().map(Into::into).collect(),
            provenance: value.provenance.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleResponseModeData {
    SpokenSummary,
    Card,
    Confirmation,
    ClarificationRequired,
}

impl From<vel_core::AppleResponseMode> for AppleResponseModeData {
    fn from(value: vel_core::AppleResponseMode) -> Self {
        match value {
            vel_core::AppleResponseMode::SpokenSummary => Self::SpokenSummary,
            vel_core::AppleResponseMode::Card => Self::Card,
            vel_core::AppleResponseMode::Confirmation => Self::Confirmation,
            vel_core::AppleResponseMode::ClarificationRequired => Self::ClarificationRequired,
        }
    }
}

impl From<AppleResponseModeData> for vel_core::AppleResponseMode {
    fn from(value: AppleResponseModeData) -> Self {
        match value {
            AppleResponseModeData::SpokenSummary => Self::SpokenSummary,
            AppleResponseModeData::Card => Self::Card,
            AppleResponseModeData::Confirmation => Self::Confirmation,
            AppleResponseModeData::ClarificationRequired => Self::ClarificationRequired,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleResponseEvidenceData {
    pub kind: String,
    pub label: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

impl From<vel_core::AppleResponseEvidence> for AppleResponseEvidenceData {
    fn from(value: vel_core::AppleResponseEvidence) -> Self {
        Self {
            kind: value.kind,
            label: value.label,
            detail: value.detail,
            source_id: value.source_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleVoiceTurnQueuedMutationSummaryData {
    pub mutation_kind: String,
    pub queued: bool,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_reference_id: Option<String>,
}

impl From<vel_core::AppleVoiceTurnQueuedMutationSummary>
    for AppleVoiceTurnQueuedMutationSummaryData
{
    fn from(value: vel_core::AppleVoiceTurnQueuedMutationSummary) -> Self {
        Self {
            mutation_kind: value.mutation_kind,
            queued: value.queued,
            summary: value.summary,
            action_reference_id: value.action_reference_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleScheduleEventData {
    pub title: String,
    pub start_ts: UnixSeconds,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<UnixSeconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_by_ts: Option<UnixSeconds>,
}

impl From<vel_core::AppleScheduleEvent> for AppleScheduleEventData {
    fn from(value: vel_core::AppleScheduleEvent) -> Self {
        Self {
            title: value.title,
            start_ts: value.start_ts,
            end_ts: value.end_ts,
            location: value.location,
            leave_by_ts: value.leave_by_ts,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleScheduleSnapshotData {
    pub generated_at: UnixSeconds,
    pub timezone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_event: Option<AppleScheduleEventData>,
    #[serde(default)]
    pub upcoming_events: Vec<AppleScheduleEventData>,
    #[serde(default)]
    pub reasons: Vec<String>,
}

impl From<vel_core::AppleScheduleSnapshot> for AppleScheduleSnapshotData {
    fn from(value: vel_core::AppleScheduleSnapshot) -> Self {
        Self {
            generated_at: value.generated_at,
            timezone: value.timezone,
            focus_summary: value.focus_summary,
            next_event: value.next_event.map(Into::into),
            upcoming_events: value.upcoming_events.into_iter().map(Into::into).collect(),
            reasons: value.reasons,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppleBehaviorSummaryScopeData {
    Daily,
}

impl From<vel_core::AppleBehaviorSummaryScope> for AppleBehaviorSummaryScopeData {
    fn from(value: vel_core::AppleBehaviorSummaryScope) -> Self {
        match value {
            vel_core::AppleBehaviorSummaryScope::Daily => Self::Daily,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleBehaviorMetricData {
    pub metric_key: String,
    pub display_label: String,
    pub value: f64,
    pub unit: String,
    pub recorded_at: UnixSeconds,
    #[serde(default)]
    pub reasons: Vec<String>,
}

impl From<vel_core::AppleBehaviorMetric> for AppleBehaviorMetricData {
    fn from(value: vel_core::AppleBehaviorMetric) -> Self {
        Self {
            metric_key: value.metric_key,
            display_label: value.display_label,
            value: value.value,
            unit: value.unit,
            recorded_at: value.recorded_at,
            reasons: value.reasons,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleBehaviorSummaryData {
    pub generated_at: UnixSeconds,
    pub timezone: String,
    pub scope: AppleBehaviorSummaryScopeData,
    pub headline: String,
    #[serde(default)]
    pub metrics: Vec<AppleBehaviorMetricData>,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness_seconds: Option<i64>,
}

impl From<vel_core::AppleBehaviorSummary> for AppleBehaviorSummaryData {
    fn from(value: vel_core::AppleBehaviorSummary) -> Self {
        Self {
            generated_at: value.generated_at,
            timezone: value.timezone,
            scope: value.scope.into(),
            headline: value.headline,
            metrics: value.metrics.into_iter().map(Into::into).collect(),
            reasons: value.reasons,
            freshness_seconds: value.freshness_seconds,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleVoiceTurnResponseData {
    pub operation: AppleRequestedOperationData,
    pub mode: AppleResponseModeData,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_id: Option<CaptureId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<AppleResponseEvidenceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_mutation: Option<AppleVoiceTurnQueuedMutationSummaryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<AppleScheduleSnapshotData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_summary: Option<AppleBehaviorSummaryData>,
}

impl From<vel_core::AppleVoiceTurnResponse> for AppleVoiceTurnResponseData {
    fn from(value: vel_core::AppleVoiceTurnResponse) -> Self {
        Self {
            operation: value.operation.into(),
            mode: value.mode.into(),
            summary: value.summary,
            capture_id: value.capture_id,
            thread_id: value.thread_id,
            reasons: value.reasons,
            evidence: value.evidence.into_iter().map(Into::into).collect(),
            queued_mutation: value.queued_mutation.map(Into::into),
            schedule: value.schedule.map(Into::into),
            behavior_summary: value.behavior_summary.map(Into::into),
        }
    }
}
