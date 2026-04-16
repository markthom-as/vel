use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopPhaseData {
    MorningOverview,
    Standup,
}

impl From<vel_core::DailyLoopPhase> for DailyLoopPhaseData {
    fn from(value: vel_core::DailyLoopPhase) -> Self {
        match value {
            vel_core::DailyLoopPhase::MorningOverview => Self::MorningOverview,
            vel_core::DailyLoopPhase::Standup => Self::Standup,
        }
    }
}

impl From<DailyLoopPhaseData> for vel_core::DailyLoopPhase {
    fn from(value: DailyLoopPhaseData) -> Self {
        match value {
            DailyLoopPhaseData::MorningOverview => Self::MorningOverview,
            DailyLoopPhaseData::Standup => Self::Standup,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopStatusData {
    Active,
    WaitingForInput,
    Completed,
    Cancelled,
}

impl From<vel_core::DailyLoopStatus> for DailyLoopStatusData {
    fn from(value: vel_core::DailyLoopStatus) -> Self {
        match value {
            vel_core::DailyLoopStatus::Active => Self::Active,
            vel_core::DailyLoopStatus::WaitingForInput => Self::WaitingForInput,
            vel_core::DailyLoopStatus::Completed => Self::Completed,
            vel_core::DailyLoopStatus::Cancelled => Self::Cancelled,
        }
    }
}

impl From<DailyLoopStatusData> for vel_core::DailyLoopStatus {
    fn from(value: DailyLoopStatusData) -> Self {
        match value {
            DailyLoopStatusData::Active => Self::Active,
            DailyLoopStatusData::WaitingForInput => Self::WaitingForInput,
            DailyLoopStatusData::Completed => Self::Completed,
            DailyLoopStatusData::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopStartSourceData {
    Manual,
    Automatic,
}

impl From<vel_core::DailyLoopStartSource> for DailyLoopStartSourceData {
    fn from(value: vel_core::DailyLoopStartSource) -> Self {
        match value {
            vel_core::DailyLoopStartSource::Manual => Self::Manual,
            vel_core::DailyLoopStartSource::Automatic => Self::Automatic,
        }
    }
}

impl From<DailyLoopStartSourceData> for vel_core::DailyLoopStartSource {
    fn from(value: DailyLoopStartSourceData) -> Self {
        match value {
            DailyLoopStartSourceData::Manual => Self::Manual,
            DailyLoopStartSourceData::Automatic => Self::Automatic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopSurfaceData {
    Cli,
    Web,
    AppleVoice,
    AppleText,
}

impl From<vel_core::DailyLoopSurface> for DailyLoopSurfaceData {
    fn from(value: vel_core::DailyLoopSurface) -> Self {
        match value {
            vel_core::DailyLoopSurface::Cli => Self::Cli,
            vel_core::DailyLoopSurface::Web => Self::Web,
            vel_core::DailyLoopSurface::AppleVoice => Self::AppleVoice,
            vel_core::DailyLoopSurface::AppleText => Self::AppleText,
        }
    }
}

impl From<DailyLoopSurfaceData> for vel_core::DailyLoopSurface {
    fn from(value: DailyLoopSurfaceData) -> Self {
        match value {
            DailyLoopSurfaceData::Cli => Self::Cli,
            DailyLoopSurfaceData::Web => Self::Web,
            DailyLoopSurfaceData::AppleVoice => Self::AppleVoice,
            DailyLoopSurfaceData::AppleText => Self::AppleText,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopTurnActionData {
    Submit,
    Skip,
    Resume,
}

impl From<vel_core::DailyLoopTurnAction> for DailyLoopTurnActionData {
    fn from(value: vel_core::DailyLoopTurnAction) -> Self {
        match value {
            vel_core::DailyLoopTurnAction::Submit => Self::Submit,
            vel_core::DailyLoopTurnAction::Skip => Self::Skip,
            vel_core::DailyLoopTurnAction::Resume => Self::Resume,
        }
    }
}

impl From<DailyLoopTurnActionData> for vel_core::DailyLoopTurnAction {
    fn from(value: DailyLoopTurnActionData) -> Self {
        match value {
            DailyLoopTurnActionData::Submit => Self::Submit,
            DailyLoopTurnActionData::Skip => Self::Skip,
            DailyLoopTurnActionData::Resume => Self::Resume,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopTurnStateData {
    InProgress,
    WaitingForInput,
    Completed,
}

impl From<vel_core::DailyLoopTurnState> for DailyLoopTurnStateData {
    fn from(value: vel_core::DailyLoopTurnState) -> Self {
        match value {
            vel_core::DailyLoopTurnState::InProgress => Self::InProgress,
            vel_core::DailyLoopTurnState::WaitingForInput => Self::WaitingForInput,
            vel_core::DailyLoopTurnState::Completed => Self::Completed,
        }
    }
}

impl From<DailyLoopTurnStateData> for vel_core::DailyLoopTurnState {
    fn from(value: DailyLoopTurnStateData) -> Self {
        match value {
            DailyLoopTurnStateData::InProgress => Self::InProgress,
            DailyLoopTurnStateData::WaitingForInput => Self::WaitingForInput,
            DailyLoopTurnStateData::Completed => Self::Completed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopCommitmentActionData {
    Accept,
    Defer,
    Choose,
    Close,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopStartMetadataData {
    pub source: DailyLoopStartSourceData,
    pub surface: DailyLoopSurfaceData,
}

impl From<vel_core::DailyLoopStartMetadata> for DailyLoopStartMetadataData {
    fn from(value: vel_core::DailyLoopStartMetadata) -> Self {
        Self {
            source: value.source.into(),
            surface: value.surface.into(),
        }
    }
}

impl From<DailyLoopStartMetadataData> for vel_core::DailyLoopStartMetadata {
    fn from(value: DailyLoopStartMetadataData) -> Self {
        Self {
            source: value.source.into(),
            surface: value.surface.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopStartRequestData {
    pub phase: DailyLoopPhaseData,
    pub session_date: String,
    pub start: DailyLoopStartMetadataData,
}

impl From<vel_core::DailyLoopStartRequest> for DailyLoopStartRequestData {
    fn from(value: vel_core::DailyLoopStartRequest) -> Self {
        Self {
            phase: value.phase.into(),
            session_date: value.session_date,
            start: value.start.into(),
        }
    }
}

impl From<DailyLoopStartRequestData> for vel_core::DailyLoopStartRequest {
    fn from(value: DailyLoopStartRequestData) -> Self {
        Self {
            phase: value.phase.into(),
            session_date: value.session_date,
            start: value.start.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopTurnRequestData {
    pub session_id: String,
    pub action: DailyLoopTurnActionData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_text: Option<String>,
}

impl From<vel_core::DailyLoopTurnRequest> for DailyLoopTurnRequestData {
    fn from(value: vel_core::DailyLoopTurnRequest) -> Self {
        Self {
            session_id: value.session_id.to_string(),
            action: value.action.into(),
            response_text: value.response_text,
        }
    }
}

impl From<DailyLoopTurnRequestData> for vel_core::DailyLoopTurnRequest {
    fn from(value: DailyLoopTurnRequestData) -> Self {
        Self {
            session_id: value.session_id.into(),
            action: value.action.into(),
            response_text: value.response_text,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopOverdueActionData {
    Close,
    Reschedule,
    BackToInbox,
    Tombstone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopOverdueGuessConfidenceData {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueVelGuessData {
    pub suggested_due_at: String,
    pub confidence: DailyLoopOverdueGuessConfidenceData,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueMenuRequestData {
    pub today: String,
    #[serde(default)]
    pub include_vel_guess: bool,
    #[serde(default = "default_overdue_menu_limit")]
    pub limit: u32,
}

fn default_overdue_menu_limit() -> u32 {
    50
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueMenuItemData {
    pub commitment_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    pub actions: Vec<DailyLoopOverdueActionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vel_due_guess: Option<DailyLoopOverdueVelGuessData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueMenuResponseData {
    pub session_id: String,
    pub items: Vec<DailyLoopOverdueMenuItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueReschedulePayloadData {
    pub due_at: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueConfirmRequestData {
    pub commitment_id: String,
    pub action: DailyLoopOverdueActionData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<DailyLoopOverdueReschedulePayloadData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueConfirmResponseData {
    pub proposal_id: String,
    pub confirmation_token: String,
    pub requires_confirmation: bool,
    #[serde(default)]
    pub write_scope: Vec<String>,
    pub idempotency_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueApplyRequestData {
    pub proposal_id: String,
    pub idempotency_key: String,
    pub confirmation_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueStateSnapshotData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueApplyResponseData {
    pub applied: bool,
    pub action_event_id: String,
    pub run_id: String,
    pub before: DailyLoopOverdueStateSnapshotData,
    pub after: DailyLoopOverdueStateSnapshotData,
    pub undo_supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueUndoRequestData {
    pub action_event_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopOverdueUndoResponseData {
    pub undone: bool,
    pub run_id: String,
    pub before: DailyLoopOverdueStateSnapshotData,
    pub after: DailyLoopOverdueStateSnapshotData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInEventsQueryData {
    #[serde(default)]
    pub check_in_type: Option<String>,
    #[serde(default)]
    pub session_phase: Option<String>,
    #[serde(default)]
    pub include_skipped: bool,
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInSubmitRequestData {
    pub check_in_type: String,
    pub session_phase: String,
    pub source: String,
    pub prompt_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answered_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default)]
    pub skipped: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason_text: Option<String>,
    #[serde(default)]
    pub replace_if_conflict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInSubmitResponseData {
    pub check_in_event_id: String,
    pub session_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes_event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInSkipRequestData {
    #[serde(default)]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answered_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInSkipResponseData {
    pub check_in_event_id: String,
    pub session_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes_event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInEventData {
    pub event_id: String,
    pub session_id: String,
    pub prompt_id: String,
    pub check_in_type: String,
    pub session_phase: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answered_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    pub scale_min: i64,
    pub scale_max: i64,
    pub keywords_json: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    pub schema_version: i64,
    pub skipped: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_event_id: Option<String>,
    pub meta_json: JsonValue,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopPromptKindData {
    IntentQuestion,
    CommitmentReduction,
    ConstraintCheck,
}

impl From<vel_core::DailyLoopPromptKind> for DailyLoopPromptKindData {
    fn from(value: vel_core::DailyLoopPromptKind) -> Self {
        match value {
            vel_core::DailyLoopPromptKind::IntentQuestion => Self::IntentQuestion,
            vel_core::DailyLoopPromptKind::CommitmentReduction => Self::CommitmentReduction,
            vel_core::DailyLoopPromptKind::ConstraintCheck => Self::ConstraintCheck,
        }
    }
}

impl From<DailyLoopPromptKindData> for vel_core::DailyLoopPromptKind {
    fn from(value: DailyLoopPromptKindData) -> Self {
        match value {
            DailyLoopPromptKindData::IntentQuestion => Self::IntentQuestion,
            DailyLoopPromptKindData::CommitmentReduction => Self::CommitmentReduction,
            DailyLoopPromptKindData::ConstraintCheck => Self::ConstraintCheck,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopPromptData {
    pub prompt_id: String,
    pub kind: DailyLoopPromptKindData,
    pub text: String,
    pub ordinal: u8,
    pub allow_skip: bool,
}

impl From<vel_core::DailyLoopPrompt> for DailyLoopPromptData {
    fn from(value: vel_core::DailyLoopPrompt) -> Self {
        Self {
            prompt_id: value.prompt_id,
            kind: value.kind.into(),
            text: value.text,
            ordinal: value.ordinal,
            allow_skip: value.allow_skip,
        }
    }
}

impl From<DailyLoopPromptData> for vel_core::DailyLoopPrompt {
    fn from(value: DailyLoopPromptData) -> Self {
        Self {
            prompt_id: value.prompt_id,
            kind: value.kind.into(),
            text: value.text,
            ordinal: value.ordinal,
            allow_skip: value.allow_skip,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningFrictionCalloutData {
    pub label: String,
    pub detail: String,
}

impl From<vel_core::MorningFrictionCallout> for MorningFrictionCalloutData {
    fn from(value: vel_core::MorningFrictionCallout) -> Self {
        Self {
            label: value.label,
            detail: value.detail,
        }
    }
}

impl From<MorningFrictionCalloutData> for vel_core::MorningFrictionCallout {
    fn from(value: MorningFrictionCalloutData) -> Self {
        Self {
            label: value.label,
            detail: value.detail,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MorningIntentSignalKindData {
    MustDoHint,
    FocusIntent,
    MeetingDoubt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MorningIntentSignalData {
    MustDoHint { text: String },
    FocusIntent { text: String },
    MeetingDoubt { text: String },
}

impl From<vel_core::MorningIntentSignal> for MorningIntentSignalData {
    fn from(value: vel_core::MorningIntentSignal) -> Self {
        match value {
            vel_core::MorningIntentSignal::MustDoHint { text } => Self::MustDoHint { text },
            vel_core::MorningIntentSignal::FocusIntent { text } => Self::FocusIntent { text },
            vel_core::MorningIntentSignal::MeetingDoubt { text } => Self::MeetingDoubt { text },
        }
    }
}

impl From<MorningIntentSignalData> for vel_core::MorningIntentSignal {
    fn from(value: MorningIntentSignalData) -> Self {
        match value {
            MorningIntentSignalData::MustDoHint { text } => Self::MustDoHint { text },
            MorningIntentSignalData::FocusIntent { text } => Self::FocusIntent { text },
            MorningIntentSignalData::MeetingDoubt { text } => Self::MeetingDoubt { text },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningOverviewStateData {
    pub snapshot: String,
    #[serde(default)]
    pub friction_callouts: Vec<MorningFrictionCalloutData>,
    #[serde(default)]
    pub signals: Vec<MorningIntentSignalData>,
    #[serde(default)]
    pub check_in_history: Vec<DailyLoopCheckInResolutionData>,
}

impl From<vel_core::MorningOverviewState> for MorningOverviewStateData {
    fn from(value: vel_core::MorningOverviewState) -> Self {
        Self {
            snapshot: value.snapshot,
            friction_callouts: value
                .friction_callouts
                .into_iter()
                .map(Into::into)
                .collect(),
            signals: value.signals.into_iter().map(Into::into).collect(),
            check_in_history: value.check_in_history.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<MorningOverviewStateData> for vel_core::MorningOverviewState {
    fn from(value: MorningOverviewStateData) -> Self {
        Self {
            snapshot: value.snapshot,
            friction_callouts: value
                .friction_callouts
                .into_iter()
                .map(Into::into)
                .collect(),
            signals: value.signals.into_iter().map(Into::into).collect(),
            check_in_history: value.check_in_history.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopCheckInResolutionKindData {
    Submitted,
    Bypassed,
}

impl From<vel_core::DailyLoopCheckInResolutionKind> for DailyLoopCheckInResolutionKindData {
    fn from(value: vel_core::DailyLoopCheckInResolutionKind) -> Self {
        match value {
            vel_core::DailyLoopCheckInResolutionKind::Submitted => Self::Submitted,
            vel_core::DailyLoopCheckInResolutionKind::Bypassed => Self::Bypassed,
        }
    }
}

impl From<DailyLoopCheckInResolutionKindData> for vel_core::DailyLoopCheckInResolutionKind {
    fn from(value: DailyLoopCheckInResolutionKindData) -> Self {
        match value {
            DailyLoopCheckInResolutionKindData::Submitted => Self::Submitted,
            DailyLoopCheckInResolutionKindData::Bypassed => Self::Bypassed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopCheckInResolutionData {
    pub prompt_id: String,
    pub ordinal: u8,
    pub kind: DailyLoopCheckInResolutionKindData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<String>,
}

impl From<vel_core::DailyLoopCheckInResolution> for DailyLoopCheckInResolutionData {
    fn from(value: vel_core::DailyLoopCheckInResolution) -> Self {
        Self {
            prompt_id: value.prompt_id,
            ordinal: value.ordinal,
            kind: value.kind.into(),
            response_text: value.response_text,
            note_text: value.note_text,
        }
    }
}

impl From<DailyLoopCheckInResolutionData> for vel_core::DailyLoopCheckInResolution {
    fn from(value: DailyLoopCheckInResolutionData) -> Self {
        Self {
            prompt_id: value.prompt_id,
            ordinal: value.ordinal,
            kind: value.kind.into(),
            response_text: value.response_text,
            note_text: value.note_text,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyStandupBucketData {
    Must,
    Should,
    Stretch,
}

impl From<vel_core::DailyStandupBucket> for DailyStandupBucketData {
    fn from(value: vel_core::DailyStandupBucket) -> Self {
        match value {
            vel_core::DailyStandupBucket::Must => Self::Must,
            vel_core::DailyStandupBucket::Should => Self::Should,
            vel_core::DailyStandupBucket::Stretch => Self::Stretch,
        }
    }
}

impl From<DailyStandupBucketData> for vel_core::DailyStandupBucket {
    fn from(value: DailyStandupBucketData) -> Self {
        match value {
            DailyStandupBucketData::Must => Self::Must,
            DailyStandupBucketData::Should => Self::Should,
            DailyStandupBucketData::Stretch => Self::Stretch,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCommitmentDraftData {
    pub title: String,
    pub bucket: DailyStandupBucketData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
}

impl From<vel_core::DailyCommitmentDraft> for DailyCommitmentDraftData {
    fn from(value: vel_core::DailyCommitmentDraft) -> Self {
        Self {
            title: value.title,
            bucket: value.bucket.into(),
            source_ref: value.source_ref,
        }
    }
}

impl From<DailyCommitmentDraftData> for vel_core::DailyCommitmentDraft {
    fn from(value: DailyCommitmentDraftData) -> Self {
        Self {
            title: value.title,
            bucket: value.bucket.into(),
            source_ref: value.source_ref,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyDeferredTaskData {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    pub reason: String,
}

impl From<vel_core::DailyDeferredTask> for DailyDeferredTaskData {
    fn from(value: vel_core::DailyDeferredTask) -> Self {
        Self {
            title: value.title,
            source_ref: value.source_ref,
            reason: value.reason,
        }
    }
}

impl From<DailyDeferredTaskData> for vel_core::DailyDeferredTask {
    fn from(value: DailyDeferredTaskData) -> Self {
        Self {
            title: value.title,
            source_ref: value.source_ref,
            reason: value.reason,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyFocusBlockProposalData {
    pub label: String,
    #[serde(with = "time::serde::rfc3339")]
    pub start_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub end_at: OffsetDateTime,
    pub reason: String,
}

impl From<vel_core::DailyFocusBlockProposal> for DailyFocusBlockProposalData {
    fn from(value: vel_core::DailyFocusBlockProposal) -> Self {
        Self {
            label: value.label,
            start_at: value.start_at,
            end_at: value.end_at,
            reason: value.reason,
        }
    }
}

impl From<DailyFocusBlockProposalData> for vel_core::DailyFocusBlockProposal {
    fn from(value: DailyFocusBlockProposalData) -> Self {
        Self {
            label: value.label,
            start_at: value.start_at,
            end_at: value.end_at,
            reason: value.reason,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStandupOutcomeData {
    #[serde(default)]
    pub commitments: Vec<DailyCommitmentDraftData>,
    #[serde(default)]
    pub deferred_tasks: Vec<DailyDeferredTaskData>,
    #[serde(default)]
    pub confirmed_calendar: Vec<String>,
    #[serde(default)]
    pub focus_blocks: Vec<DailyFocusBlockProposalData>,
    #[serde(default)]
    pub check_in_history: Vec<DailyLoopCheckInResolutionData>,
}

impl From<vel_core::DailyStandupOutcome> for DailyStandupOutcomeData {
    fn from(value: vel_core::DailyStandupOutcome) -> Self {
        Self {
            commitments: value.commitments.into_iter().map(Into::into).collect(),
            deferred_tasks: value.deferred_tasks.into_iter().map(Into::into).collect(),
            confirmed_calendar: value.confirmed_calendar,
            focus_blocks: value.focus_blocks.into_iter().map(Into::into).collect(),
            check_in_history: value.check_in_history.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DailyStandupOutcomeData> for vel_core::DailyStandupOutcome {
    fn from(value: DailyStandupOutcomeData) -> Self {
        Self {
            commitments: value.commitments.into_iter().map(Into::into).collect(),
            deferred_tasks: value.deferred_tasks.into_iter().map(Into::into).collect(),
            confirmed_calendar: value.confirmed_calendar,
            focus_blocks: value.focus_blocks.into_iter().map(Into::into).collect(),
            check_in_history: value.check_in_history.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum DailyLoopSessionOutcomeData {
    MorningOverview {
        signals: Vec<MorningIntentSignalData>,
        #[serde(default)]
        check_in_history: Vec<DailyLoopCheckInResolutionData>,
    },
    Standup(DailyStandupOutcomeData),
}

impl From<vel_core::DailyLoopSessionOutcome> for DailyLoopSessionOutcomeData {
    fn from(value: vel_core::DailyLoopSessionOutcome) -> Self {
        match value {
            vel_core::DailyLoopSessionOutcome::MorningOverview {
                signals,
                check_in_history,
            } => Self::MorningOverview {
                signals: signals.into_iter().map(Into::into).collect(),
                check_in_history: check_in_history.into_iter().map(Into::into).collect(),
            },
            vel_core::DailyLoopSessionOutcome::Standup(outcome) => Self::Standup(outcome.into()),
        }
    }
}

impl From<DailyLoopSessionOutcomeData> for vel_core::DailyLoopSessionOutcome {
    fn from(value: DailyLoopSessionOutcomeData) -> Self {
        match value {
            DailyLoopSessionOutcomeData::MorningOverview {
                signals,
                check_in_history,
            } => Self::MorningOverview {
                signals: signals.into_iter().map(Into::into).collect(),
                check_in_history: check_in_history.into_iter().map(Into::into).collect(),
            },
            DailyLoopSessionOutcomeData::Standup(outcome) => Self::Standup(outcome.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum DailyLoopSessionStateData {
    MorningOverview(MorningOverviewStateData),
    Standup(DailyStandupOutcomeData),
}

impl From<vel_core::DailyLoopSessionState> for DailyLoopSessionStateData {
    fn from(value: vel_core::DailyLoopSessionState) -> Self {
        match value {
            vel_core::DailyLoopSessionState::MorningOverview(state) => {
                Self::MorningOverview(state.into())
            }
            vel_core::DailyLoopSessionState::Standup(state) => Self::Standup(state.into()),
        }
    }
}

impl From<DailyLoopSessionStateData> for vel_core::DailyLoopSessionState {
    fn from(value: DailyLoopSessionStateData) -> Self {
        match value {
            DailyLoopSessionStateData::MorningOverview(state) => {
                Self::MorningOverview(state.into())
            }
            DailyLoopSessionStateData::Standup(state) => Self::Standup(state.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLoopSessionData {
    pub id: String,
    pub session_date: String,
    pub phase: DailyLoopPhaseData,
    pub status: DailyLoopStatusData,
    pub start: DailyLoopStartMetadataData,
    pub turn_state: DailyLoopTurnStateData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_prompt: Option<DailyLoopPromptData>,
    pub continuity_summary: String,
    #[serde(default)]
    pub allowed_actions: Vec<DailyLoopCommitmentActionData>,
    pub state: DailyLoopSessionStateData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<DailyLoopSessionOutcomeData>,
}

impl From<vel_core::DailyLoopSession> for DailyLoopSessionData {
    fn from(value: vel_core::DailyLoopSession) -> Self {
        let continuity_summary = daily_loop_session_continuity_summary(&value);
        let allowed_actions = daily_loop_session_allowed_actions(&value);
        Self {
            id: value.id.to_string(),
            session_date: value.session_date,
            phase: value.phase.into(),
            status: value.status.into(),
            start: value.start.into(),
            turn_state: value.turn_state.into(),
            current_prompt: value.current_prompt.map(Into::into),
            continuity_summary,
            allowed_actions,
            state: value.state.into(),
            outcome: value.outcome.map(Into::into),
        }
    }
}

impl From<DailyLoopSessionData> for vel_core::DailyLoopSession {
    fn from(value: DailyLoopSessionData) -> Self {
        Self {
            id: value.id.into(),
            session_date: value.session_date,
            phase: value.phase.into(),
            status: value.status.into(),
            start: value.start.into(),
            turn_state: value.turn_state.into(),
            current_prompt: value.current_prompt.map(Into::into),
            state: value.state.into(),
            outcome: value.outcome.map(Into::into),
        }
    }
}

fn daily_loop_session_allowed_actions(
    value: &vel_core::DailyLoopSession,
) -> Vec<DailyLoopCommitmentActionData> {
    let mut actions = vec![DailyLoopCommitmentActionData::Accept];
    if value
        .current_prompt
        .as_ref()
        .map(|prompt| prompt.allow_skip)
        .unwrap_or(false)
    {
        actions.push(DailyLoopCommitmentActionData::Defer);
    }
    actions.push(DailyLoopCommitmentActionData::Choose);
    actions.push(DailyLoopCommitmentActionData::Close);
    actions
}

fn daily_loop_session_continuity_summary(value: &vel_core::DailyLoopSession) -> String {
    match (&value.phase, &value.current_prompt, &value.state) {
        (
            vel_core::DailyLoopPhase::MorningOverview,
            Some(prompt),
            vel_core::DailyLoopSessionState::MorningOverview(state),
        ) => format!(
            "Morning overview is waiting on question {} of {} with {} captured signal(s).",
            prompt.ordinal,
            vel_core::DAILY_LOOP_MAX_QUESTIONS,
            state.signals.len()
        ),
        (
            vel_core::DailyLoopPhase::Standup,
            Some(prompt),
            vel_core::DailyLoopSessionState::Standup(state),
        ) => format!(
            "Standup is waiting on question {} with {} commitment draft(s) and {} deferred item(s).",
            prompt.ordinal,
            state.commitments.len(),
            state.deferred_tasks.len()
        ),
        (vel_core::DailyLoopPhase::MorningOverview, _, _) => {
            "Morning overview continuity is available.".to_string()
        }
        (vel_core::DailyLoopPhase::Standup, _, _) => {
            "Standup continuity is available.".to_string()
        }
    }
}
