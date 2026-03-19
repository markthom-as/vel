use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{
    ActionItemId, ArtifactId, ArtifactStorageKind, CaptureId, CommitmentId, ConflictCaseId,
    IntegrationConnectionId, IntegrationFamily, PersonId, PrivacyClass, ProjectId, ResolvedCommand,
    RiskFactors, RiskSnapshot, RunId, SyncClass, WritebackOperationId,
};

/// Wire-level timestamp for resource DTO fields that use Unix seconds.
pub type UnixSeconds = i64;

/// Wire-level timestamp for envelope-style events that use RFC3339 strings.
pub type Rfc3339Timestamp = String;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPlanRequest {
    pub command: ResolvedCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecuteRequest {
    pub command: ResolvedCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandPlanModeData {
    Ready,
    DryRunOnly,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandValidationIssueCodeData {
    UnsupportedOperation,
    MissingTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandValidationIssueData {
    pub code: CommandValidationIssueCodeData,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandValidationData {
    pub is_valid: bool,
    #[serde(default)]
    pub issues: Vec<CommandValidationIssueData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPlanStepData {
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandIntentHintsData {
    pub target_kind: String,
    pub mode: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDelegationHintsData {
    pub worker_roles: Vec<String>,
    pub coordination: String,
    pub approval_required: bool,
    pub linked_record_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPlannedLinkData {
    pub entity_type: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPlannedRecordData {
    pub record_type: String,
    pub title: String,
    #[serde(default)]
    pub links: Vec<CommandPlannedLinkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionPlanData {
    pub operation: String,
    pub target_kinds: Vec<String>,
    pub mode: CommandPlanModeData,
    pub summary: String,
    pub steps: Vec<CommandPlanStepData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_hints: Option<CommandIntentHintsData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_hints: Option<CommandDelegationHintsData>,
    #[serde(default)]
    pub planned_records: Vec<CommandPlannedRecordData>,
    pub validation: CommandValidationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningArtifactCreatedData {
    pub artifact: ArtifactData,
    pub thread: ThreadData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionResultData {
    pub result: CommandExecutionPayloadData,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandReviewSummaryData {
    pub captures: Vec<ContextCapture>,
    pub capture_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_context_artifact: Option<ArtifactData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "result_kind", content = "data", rename_all = "snake_case")]
pub enum CommandExecutionPayloadData {
    CaptureCreated(CaptureCreateResponse),
    CommitmentCreated(CommitmentData),
    ArtifactCreated(ArtifactData),
    SpecDraftCreated(PlanningArtifactCreatedData),
    ExecutionPlanCreated(PlanningArtifactCreatedData),
    DelegationPlanCreated(PlanningArtifactCreatedData),
    SynthesisCreated(SynthesisWeekData),
    ContextExplained(ContextExplainData),
    CommitmentExplained(CommitmentExplainData),
    DriftExplained(DriftExplainData),
    ReviewToday(CommandReviewSummaryData),
    ReviewWeek(CommandReviewSummaryData),
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

    pub fn error(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupCoverageData {
    #[serde(default)]
    pub included: Vec<String>,
    #[serde(default)]
    pub omitted: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupSecretOmissionFlagsData {
    pub settings_secrets_omitted: bool,
    pub integration_tokens_omitted: bool,
    pub local_key_material_omitted: bool,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupVerificationData {
    pub verified: bool,
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(default)]
    pub checked_paths: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifestData {
    pub backup_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub output_root: String,
    pub database_snapshot_path: String,
    pub artifact_coverage: BackupCoverageData,
    pub config_coverage: BackupCoverageData,
    #[serde(default)]
    pub explicit_omissions: Vec<String>,
    pub secret_omission_flags: BackupSecretOmissionFlagsData,
    pub verification_summary: BackupVerificationData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupStatusStateData {
    Ready,
    Stale,
    Missing,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStatusData {
    pub state: BackupStatusStateData,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_id: Option<String>,
    #[serde(default)]
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_at: Option<OffsetDateTime>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_root: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_coverage: Option<BackupCoverageData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_coverage: Option<BackupCoverageData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_summary: Option<BackupVerificationData>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupTrustLevelData {
    Ok,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupFreshnessStateData {
    Current,
    Stale,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFreshnessData {
    pub state: BackupFreshnessStateData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_seconds: Option<i64>,
    pub stale_after_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupTrustData {
    pub level: BackupTrustLevelData,
    pub status: BackupStatusData,
    pub freshness: BackupFreshnessData,
    #[serde(default)]
    pub guidance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSettingsData {
    pub default_output_root: String,
    pub trust: BackupTrustData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterBootstrapData {
    pub node_id: String,
    pub node_display_name: String,
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_sync: Option<BranchSyncCapabilityData>,
    #[serde(default)]
    pub validation_profiles: Vec<ValidationProfileData>,
    #[serde(default)]
    pub linked_nodes: Vec<LinkedNodeData>,
    #[serde(default)]
    pub projects: Vec<ProjectRecordData>,
    #[serde(default)]
    pub action_items: Vec<ActionItemData>,
    #[serde(default)]
    pub pending_writebacks: Vec<WritebackOperationData>,
    #[serde(default)]
    pub conflicts: Vec<ConflictCaseData>,
    #[serde(default)]
    pub people: Vec<PersonRecordData>,
}

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
    pub state: DailyLoopSessionStateData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<DailyLoopSessionOutcomeData>,
}

impl From<vel_core::DailyLoopSession> for DailyLoopSessionData {
    fn from(value: vel_core::DailyLoopSession) -> Self {
        Self {
            id: value.id.to_string(),
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
            reasons: value.reasons,
            evidence: value.evidence.into_iter().map(Into::into).collect(),
            queued_mutation: value.queued_mutation.map(Into::into),
            schedule: value.schedule.map(Into::into),
            behavior_summary: value.behavior_summary.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationFamilyData {
    Calendar,
    Tasks,
    Activity,
    Git,
    Messaging,
    Notes,
    Transcripts,
    Documents,
    Health,
    Gaming,
}

impl From<IntegrationFamily> for IntegrationFamilyData {
    fn from(value: IntegrationFamily) -> Self {
        match value {
            IntegrationFamily::Calendar => Self::Calendar,
            IntegrationFamily::Tasks => Self::Tasks,
            IntegrationFamily::Activity => Self::Activity,
            IntegrationFamily::Git => Self::Git,
            IntegrationFamily::Messaging => Self::Messaging,
            IntegrationFamily::Notes => Self::Notes,
            IntegrationFamily::Transcripts => Self::Transcripts,
            IntegrationFamily::Documents => Self::Documents,
            IntegrationFamily::Health => Self::Health,
            IntegrationFamily::Gaming => Self::Gaming,
        }
    }
}

impl From<IntegrationFamilyData> for IntegrationFamily {
    fn from(value: IntegrationFamilyData) -> Self {
        match value {
            IntegrationFamilyData::Calendar => Self::Calendar,
            IntegrationFamilyData::Tasks => Self::Tasks,
            IntegrationFamilyData::Activity => Self::Activity,
            IntegrationFamilyData::Git => Self::Git,
            IntegrationFamilyData::Messaging => Self::Messaging,
            IntegrationFamilyData::Notes => Self::Notes,
            IntegrationFamilyData::Transcripts => Self::Transcripts,
            IntegrationFamilyData::Documents => Self::Documents,
            IntegrationFamilyData::Health => Self::Health,
            IntegrationFamilyData::Gaming => Self::Gaming,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSourceRefData {
    pub family: IntegrationFamilyData,
    pub provider_key: String,
    pub connection_id: IntegrationConnectionId,
    pub external_id: String,
}

impl From<vel_core::IntegrationSourceRef> for IntegrationSourceRefData {
    fn from(value: vel_core::IntegrationSourceRef) -> Self {
        Self {
            family: value.family.into(),
            provider_key: value.provider_key,
            connection_id: value.connection_id,
            external_id: value.external_id,
        }
    }
}

impl From<IntegrationSourceRefData> for vel_core::IntegrationSourceRef {
    fn from(value: IntegrationSourceRefData) -> Self {
        Self {
            family: value.family.into(),
            provider_key: value.provider_key,
            connection_id: value.connection_id,
            external_id: value.external_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectFamilyData {
    Personal,
    Creative,
    Work,
}

impl From<vel_core::ProjectFamily> for ProjectFamilyData {
    fn from(value: vel_core::ProjectFamily) -> Self {
        match value {
            vel_core::ProjectFamily::Personal => Self::Personal,
            vel_core::ProjectFamily::Creative => Self::Creative,
            vel_core::ProjectFamily::Work => Self::Work,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatusData {
    Active,
    Paused,
    Archived,
}

impl From<vel_core::ProjectStatus> for ProjectStatusData {
    fn from(value: vel_core::ProjectStatus) -> Self {
        match value {
            vel_core::ProjectStatus::Active => Self::Active,
            vel_core::ProjectStatus::Paused => Self::Paused,
            vel_core::ProjectStatus::Archived => Self::Archived,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRootRefData {
    pub path: String,
    pub label: String,
    pub kind: String,
}

impl From<vel_core::ProjectRootRef> for ProjectRootRefData {
    fn from(value: vel_core::ProjectRootRef) -> Self {
        Self {
            path: value.path,
            label: value.label,
            kind: value.kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectProvisionRequestData {
    #[serde(default)]
    pub create_repo: bool,
    #[serde(default)]
    pub create_notes_root: bool,
}

impl From<vel_core::ProjectProvisionRequest> for ProjectProvisionRequestData {
    fn from(value: vel_core::ProjectProvisionRequest) -> Self {
        Self {
            create_repo: value.create_repo,
            create_notes_root: value.create_notes_root,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRecordData {
    pub id: ProjectId,
    pub slug: String,
    pub name: String,
    pub family: ProjectFamilyData,
    pub status: ProjectStatusData,
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
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub archived_at: Option<OffsetDateTime>,
}

impl From<vel_core::ProjectRecord> for ProjectRecordData {
    fn from(value: vel_core::ProjectRecord) -> Self {
        Self {
            id: value.id,
            slug: value.slug,
            name: value.name,
            family: value.family.into(),
            status: value.status.into(),
            primary_repo: value.primary_repo.into(),
            primary_notes_root: value.primary_notes_root.into(),
            secondary_repos: value.secondary_repos.into_iter().map(Into::into).collect(),
            secondary_notes_roots: value
                .secondary_notes_roots
                .into_iter()
                .map(Into::into)
                .collect(),
            upstream_ids: value.upstream_ids,
            pending_provision: value.pending_provision.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            archived_at: value.archived_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionTaskKindData {
    Planning,
    Implementation,
    Debugging,
    Review,
    Research,
    Documentation,
}

impl From<vel_core::ExecutionTaskKind> for ExecutionTaskKindData {
    fn from(value: vel_core::ExecutionTaskKind) -> Self {
        match value {
            vel_core::ExecutionTaskKind::Planning => Self::Planning,
            vel_core::ExecutionTaskKind::Implementation => Self::Implementation,
            vel_core::ExecutionTaskKind::Debugging => Self::Debugging,
            vel_core::ExecutionTaskKind::Review => Self::Review,
            vel_core::ExecutionTaskKind::Research => Self::Research,
            vel_core::ExecutionTaskKind::Documentation => Self::Documentation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentProfileData {
    Budget,
    Balanced,
    Quality,
    Inherit,
}

impl From<vel_core::AgentProfile> for AgentProfileData {
    fn from(value: vel_core::AgentProfile) -> Self {
        match value {
            vel_core::AgentProfile::Budget => Self::Budget,
            vel_core::AgentProfile::Balanced => Self::Balanced,
            vel_core::AgentProfile::Quality => Self::Quality,
            vel_core::AgentProfile::Inherit => Self::Inherit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenBudgetClassData {
    Small,
    Medium,
    Large,
    Xlarge,
}

impl From<vel_core::TokenBudgetClass> for TokenBudgetClassData {
    fn from(value: vel_core::TokenBudgetClass) -> Self {
        match value {
            vel_core::TokenBudgetClass::Small => Self::Small,
            vel_core::TokenBudgetClass::Medium => Self::Medium,
            vel_core::TokenBudgetClass::Large => Self::Large,
            vel_core::TokenBudgetClass::Xlarge => Self::Xlarge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionReviewGateData {
    None,
    OperatorApproval,
    OperatorPreview,
    PostRunReview,
}

impl From<vel_core::ExecutionReviewGate> for ExecutionReviewGateData {
    fn from(value: vel_core::ExecutionReviewGate) -> Self {
        match value {
            vel_core::ExecutionReviewGate::None => Self::None,
            vel_core::ExecutionReviewGate::OperatorApproval => Self::OperatorApproval,
            vel_core::ExecutionReviewGate::OperatorPreview => Self::OperatorPreview,
            vel_core::ExecutionReviewGate::PostRunReview => Self::PostRunReview,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalRuntimeKindData {
    LocalCli,
    WasmGuest,
}

impl From<vel_core::LocalRuntimeKind> for LocalRuntimeKindData {
    fn from(value: vel_core::LocalRuntimeKind) -> Self {
        match value {
            vel_core::LocalRuntimeKind::LocalCli => Self::LocalCli,
            vel_core::LocalRuntimeKind::WasmGuest => Self::WasmGuest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoWorktreeRefData {
    pub path: String,
    pub label: String,
    pub branch: Option<String>,
    pub head_rev: Option<String>,
}

impl From<vel_core::RepoWorktreeRef> for RepoWorktreeRefData {
    fn from(value: vel_core::RepoWorktreeRef) -> Self {
        Self {
            path: value.path,
            label: value.label,
            branch: value.branch,
            head_rev: value.head_rev,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDescriptorData {
    pub scope: String,
    pub resource: Option<String>,
    pub action: String,
}

impl From<vel_core::CapabilityDescriptor> for CapabilityDescriptorData {
    fn from(value: vel_core::CapabilityDescriptor) -> Self {
        Self {
            scope: value.scope,
            resource: value.resource,
            action: value.action,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAgentManifestData {
    pub manifest_id: String,
    pub runtime_kind: LocalRuntimeKindData,
    pub entrypoint: String,
    pub working_directory: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env_keys: Vec<String>,
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<CapabilityDescriptorData>,
    pub review_gate: ExecutionReviewGateData,
}

impl From<vel_core::LocalAgentManifest> for LocalAgentManifestData {
    fn from(value: vel_core::LocalAgentManifest) -> Self {
        Self {
            manifest_id: value.manifest_id,
            runtime_kind: value.runtime_kind.into(),
            entrypoint: value.entrypoint,
            working_directory: value.working_directory,
            args: value.args,
            env_keys: value.env_keys,
            read_roots: value.read_roots,
            write_roots: value.write_roots,
            allowed_tools: value.allowed_tools,
            capabilities: value.capabilities.into_iter().map(Into::into).collect(),
            review_gate: value.review_gate.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPolicyInputData {
    pub task_kind: ExecutionTaskKindData,
    pub agent_profile: AgentProfileData,
    pub token_budget: TokenBudgetClassData,
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
    pub review_gate: ExecutionReviewGateData,
    #[serde(default)]
    pub requires_network: bool,
}

impl From<vel_core::ExecutionPolicyInput> for ExecutionPolicyInputData {
    fn from(value: vel_core::ExecutionPolicyInput) -> Self {
        Self {
            task_kind: value.task_kind.into(),
            agent_profile: value.agent_profile.into(),
            token_budget: value.token_budget.into(),
            read_roots: value.read_roots,
            write_roots: value.write_roots,
            review_gate: value.review_gate.into(),
            requires_network: value.requires_network,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectExecutionContextData {
    pub project_id: ProjectId,
    pub repo: RepoWorktreeRefData,
    pub notes_root: ProjectRootRefData,
    pub gsd_artifact_dir: String,
    pub default_task_kind: ExecutionTaskKindData,
    pub default_agent_profile: AgentProfileData,
    pub default_token_budget: TokenBudgetClassData,
    pub review_gate: ExecutionReviewGateData,
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
    #[serde(default)]
    pub local_manifests: Vec<LocalAgentManifestData>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<vel_core::ProjectExecutionContext> for ProjectExecutionContextData {
    fn from(value: vel_core::ProjectExecutionContext) -> Self {
        Self {
            project_id: value.project_id,
            repo: value.repo.into(),
            notes_root: value.notes_root.into(),
            gsd_artifact_dir: value.gsd_artifact_dir,
            default_task_kind: value.default_task_kind.into(),
            default_agent_profile: value.default_agent_profile.into(),
            default_token_budget: value.default_token_budget.into(),
            review_gate: value.review_gate.into(),
            read_roots: value.read_roots,
            write_roots: value.write_roots,
            local_manifests: value.local_manifests.into_iter().map(Into::into).collect(),
            metadata: value.metadata,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffEnvelopeData {
    pub task_id: String,
    pub trace_id: vel_core::TraceId,
    pub from_agent: String,
    pub to_agent: String,
    pub objective: String,
    #[serde(default)]
    pub inputs: JsonValue,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub project_id: Option<ProjectId>,
    #[serde(default)]
    pub task_kind: Option<ExecutionTaskKindData>,
    #[serde(default)]
    pub agent_profile: Option<AgentProfileData>,
    #[serde(default)]
    pub token_budget: Option<TokenBudgetClassData>,
    #[serde(default)]
    pub review_gate: Option<ExecutionReviewGateData>,
    #[serde(default)]
    pub repo_root: Option<RepoWorktreeRefData>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub capability_scope: JsonValue,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default)]
    pub expected_output_schema: JsonValue,
}

impl From<vel_core::HandoffEnvelope> for HandoffEnvelopeData {
    fn from(value: vel_core::HandoffEnvelope) -> Self {
        Self {
            task_id: value.task_id,
            trace_id: value.trace_id,
            from_agent: value.from_agent,
            to_agent: value.to_agent,
            objective: value.objective,
            inputs: value.inputs,
            constraints: value.constraints,
            read_scopes: value.read_scopes,
            write_scopes: value.write_scopes,
            project_id: value.project_id,
            task_kind: value.task_kind.map(Into::into),
            agent_profile: value.agent_profile.map(Into::into),
            token_budget: value.token_budget.map(Into::into),
            review_gate: value.review_gate.map(Into::into),
            repo_root: value.repo_root.map(Into::into),
            allowed_tools: value.allowed_tools,
            capability_scope: value.capability_scope,
            deadline: value.deadline,
            expected_output_schema: value.expected_output_schema,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHandoffData {
    pub handoff: HandoffEnvelopeData,
    pub project_id: ProjectId,
    pub task_kind: ExecutionTaskKindData,
    pub agent_profile: AgentProfileData,
    pub token_budget: TokenBudgetClassData,
    pub review_gate: ExecutionReviewGateData,
    pub repo: RepoWorktreeRefData,
    pub notes_root: ProjectRootRefData,
    #[serde(default)]
    pub manifest_id: Option<String>,
}

impl From<vel_core::ExecutionHandoff> for ExecutionHandoffData {
    fn from(value: vel_core::ExecutionHandoff) -> Self {
        Self {
            handoff: value.handoff.into(),
            project_id: value.project_id,
            task_kind: value.task_kind.into(),
            agent_profile: value.agent_profile.into(),
            token_budget: value.token_budget.into(),
            review_gate: value.review_gate.into(),
            repo: value.repo.into(),
            notes_root: value.notes_root.into(),
            manifest_id: value.manifest_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionHandoffOriginKindData {
    HumanToAgent,
    AgentToAgent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionHandoffReviewStateData {
    PendingReview,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRoutingReasonData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRoutingDecisionData {
    pub task_kind: ExecutionTaskKindData,
    pub agent_profile: AgentProfileData,
    pub token_budget: TokenBudgetClassData,
    pub review_gate: ExecutionReviewGateData,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub reasons: Vec<ExecutionRoutingReasonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHandoffRecordData {
    pub id: String,
    pub project_id: ProjectId,
    pub origin_kind: ExecutionHandoffOriginKindData,
    pub review_state: ExecutionHandoffReviewStateData,
    pub handoff: ExecutionHandoffData,
    pub routing: ExecutionRoutingDecisionData,
    #[serde(default)]
    pub manifest_id: Option<String>,
    pub requested_by: String,
    #[serde(default)]
    pub reviewed_by: Option<String>,
    #[serde(default)]
    pub decision_reason: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub reviewed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub launched_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCapabilityGroupKindData {
    ReadContext,
    ReviewActions,
    MutationActions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentBlockerData {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub escalation_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentCapabilityEntryData {
    pub key: String,
    pub label: String,
    pub summary: String,
    pub available: bool,
    #[serde(default)]
    pub blocked_reason: Option<AgentBlockerData>,
    #[serde(default)]
    pub requires_review_gate: Option<ExecutionReviewGateData>,
    #[serde(default)]
    pub requires_writeback_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentCapabilityGroupData {
    pub kind: AgentCapabilityGroupKindData,
    pub label: String,
    #[serde(default)]
    pub entries: Vec<AgentCapabilityEntryData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentCapabilitySummaryData {
    #[serde(default)]
    pub groups: Vec<AgentCapabilityGroupData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentContextRefData {
    pub computed_at: UnixSeconds,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub morning_state: Option<String>,
    pub current_context_path: String,
    pub explain_context_path: String,
    pub explain_drift_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReviewObligationsData {
    #[serde(default)]
    pub review_snapshot: ReviewSnapshotData,
    #[serde(default)]
    pub pending_writebacks: Vec<WritebackOperationData>,
    #[serde(default)]
    pub conflicts: Vec<ConflictCaseData>,
    #[serde(default)]
    pub pending_execution_handoffs: Vec<ExecutionHandoffRecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentGroundingPackData {
    pub generated_at: UnixSeconds,
    pub now: NowData,
    #[serde(default)]
    pub current_context: Option<AgentContextRefData>,
    #[serde(default)]
    pub projects: Vec<ProjectRecordData>,
    #[serde(default)]
    pub people: Vec<PersonRecordData>,
    #[serde(default)]
    pub commitments: Vec<CommitmentData>,
    pub review: AgentReviewObligationsData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentInspectExplainabilityData {
    #[serde(default)]
    pub persisted_record_kinds: Vec<String>,
    #[serde(default)]
    pub supporting_paths: Vec<String>,
    pub raw_context_json_supporting_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInspectData {
    pub grounding: AgentGroundingPackData,
    pub capabilities: AgentCapabilitySummaryData,
    #[serde(default)]
    pub blockers: Vec<AgentBlockerData>,
    pub explainability: AgentInspectExplainabilityData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateRequestData {
    pub slug: String,
    pub name: String,
    pub family: ProjectFamilyData,
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct ProjectCreateResponseData {
    pub project: ProjectRecordData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListResponseData {
    #[serde(default)]
    pub projects: Vec<ProjectRecordData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionSurfaceData {
    Now,
    Inbox,
}

impl From<vel_core::ActionSurface> for ActionSurfaceData {
    fn from(value: vel_core::ActionSurface) -> Self {
        match value {
            vel_core::ActionSurface::Now => Self::Now,
            vel_core::ActionSurface::Inbox => Self::Inbox,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKindData {
    NextStep,
    Recovery,
    Intervention,
    CheckIn,
    Review,
    Freshness,
    Blocked,
    Conflict,
    Linking,
}

impl From<vel_core::ActionKind> for ActionKindData {
    fn from(value: vel_core::ActionKind) -> Self {
        match value {
            vel_core::ActionKind::NextStep => Self::NextStep,
            vel_core::ActionKind::Recovery => Self::Recovery,
            vel_core::ActionKind::Intervention => Self::Intervention,
            vel_core::ActionKind::CheckIn => Self::CheckIn,
            vel_core::ActionKind::Review => Self::Review,
            vel_core::ActionKind::Freshness => Self::Freshness,
            vel_core::ActionKind::Blocked => Self::Blocked,
            vel_core::ActionKind::Conflict => Self::Conflict,
            vel_core::ActionKind::Linking => Self::Linking,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPermissionModeData {
    AutoAllowed,
    UserConfirm,
    Blocked,
    Unavailable,
}

impl From<vel_core::ActionPermissionMode> for ActionPermissionModeData {
    fn from(value: vel_core::ActionPermissionMode) -> Self {
        match value {
            vel_core::ActionPermissionMode::AutoAllowed => Self::AutoAllowed,
            vel_core::ActionPermissionMode::UserConfirm => Self::UserConfirm,
            vel_core::ActionPermissionMode::Blocked => Self::Blocked,
            vel_core::ActionPermissionMode::Unavailable => Self::Unavailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionScopeAffinityData {
    Global,
    Project,
    Thread,
    Connector,
    DailyLoop,
}

impl From<vel_core::ActionScopeAffinity> for ActionScopeAffinityData {
    fn from(value: vel_core::ActionScopeAffinity) -> Self {
        match value {
            vel_core::ActionScopeAffinity::Global => Self::Global,
            vel_core::ActionScopeAffinity::Project => Self::Project,
            vel_core::ActionScopeAffinity::Thread => Self::Thread,
            vel_core::ActionScopeAffinity::Connector => Self::Connector,
            vel_core::ActionScopeAffinity::DailyLoop => Self::DailyLoop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStateData {
    Active,
    Acknowledged,
    Resolved,
    Dismissed,
    Snoozed,
}

impl From<vel_core::ActionState> for ActionStateData {
    fn from(value: vel_core::ActionState) -> Self {
        match value {
            vel_core::ActionState::Active => Self::Active,
            vel_core::ActionState::Acknowledged => Self::Acknowledged,
            vel_core::ActionState::Resolved => Self::Resolved,
            vel_core::ActionState::Dismissed => Self::Dismissed,
            vel_core::ActionState::Snoozed => Self::Snoozed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailableActionData {
    Acknowledge,
    Resolve,
    Dismiss,
    Snooze,
    OpenThread,
    OpenProject,
    SyncNow,
    LinkNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEvidenceRefData {
    pub source_kind: String,
    pub source_id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl From<vel_core::ActionEvidenceRef> for ActionEvidenceRefData {
    fn from(value: vel_core::ActionEvidenceRef) -> Self {
        Self {
            source_kind: value.source_kind,
            source_id: value.source_id,
            label: value.label,
            detail: value.detail,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInSourceKindData {
    DailyLoop,
}

impl From<vel_core::CheckInSourceKind> for CheckInSourceKindData {
    fn from(value: vel_core::CheckInSourceKind) -> Self {
        match value {
            vel_core::CheckInSourceKind::DailyLoop => Self::DailyLoop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInSubmitTargetKindData {
    DailyLoopTurn,
}

impl From<vel_core::CheckInSubmitTargetKind> for CheckInSubmitTargetKindData {
    fn from(value: vel_core::CheckInSubmitTargetKind) -> Self {
        match value {
            vel_core::CheckInSubmitTargetKind::DailyLoopTurn => Self::DailyLoopTurn,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInSubmitTargetData {
    pub kind: CheckInSubmitTargetKindData,
    pub reference_id: String,
}

impl From<vel_core::CheckInSubmitTarget> for CheckInSubmitTargetData {
    fn from(value: vel_core::CheckInSubmitTarget) -> Self {
        Self {
            kind: value.kind.into(),
            reference_id: value.reference_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInEscalationTargetData {
    Threads,
}

impl From<vel_core::CheckInEscalationTarget> for CheckInEscalationTargetData {
    fn from(value: vel_core::CheckInEscalationTarget) -> Self {
        match value {
            vel_core::CheckInEscalationTarget::Threads => Self::Threads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInEscalationData {
    pub target: CheckInEscalationTargetData,
    pub label: String,
}

impl From<vel_core::CheckInEscalation> for CheckInEscalationData {
    fn from(value: vel_core::CheckInEscalation) -> Self {
        Self {
            target: value.target.into(),
            label: value.label,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInTransitionKindData {
    Submit,
    Bypass,
    Escalate,
}

impl From<vel_core::CheckInTransitionKind> for CheckInTransitionKindData {
    fn from(value: vel_core::CheckInTransitionKind) -> Self {
        match value {
            vel_core::CheckInTransitionKind::Submit => Self::Submit,
            vel_core::CheckInTransitionKind::Bypass => Self::Bypass,
            vel_core::CheckInTransitionKind::Escalate => Self::Escalate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInTransitionTargetKindData {
    DailyLoopTurn,
    Threads,
}

impl From<vel_core::CheckInTransitionTargetKind> for CheckInTransitionTargetKindData {
    fn from(value: vel_core::CheckInTransitionTargetKind) -> Self {
        match value {
            vel_core::CheckInTransitionTargetKind::DailyLoopTurn => Self::DailyLoopTurn,
            vel_core::CheckInTransitionTargetKind::Threads => Self::Threads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInTransitionData {
    pub kind: CheckInTransitionKindData,
    pub label: String,
    pub target: CheckInTransitionTargetKindData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub requires_response: bool,
    pub requires_note: bool,
}

impl From<vel_core::CheckInTransition> for CheckInTransitionData {
    fn from(value: vel_core::CheckInTransition) -> Self {
        Self {
            kind: value.kind.into(),
            label: value.label,
            target: value.target.into(),
            reference_id: value.reference_id,
            requires_response: value.requires_response,
            requires_note: value.requires_note,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInCardData {
    pub id: ActionItemId,
    pub source_kind: CheckInSourceKindData,
    pub phase: DailyLoopPhaseData,
    pub session_id: String,
    pub title: String,
    pub summary: String,
    pub prompt_id: String,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_response: Option<String>,
    pub allow_skip: bool,
    pub blocking: bool,
    pub submit_target: CheckInSubmitTargetData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation: Option<CheckInEscalationData>,
    #[serde(default)]
    pub transitions: Vec<CheckInTransitionData>,
}

impl From<vel_core::CheckInCard> for CheckInCardData {
    fn from(value: vel_core::CheckInCard) -> Self {
        Self {
            id: value.id,
            source_kind: value.source_kind.into(),
            phase: value.phase.into(),
            session_id: value.session_id,
            title: value.title,
            summary: value.summary,
            prompt_id: value.prompt_id,
            prompt_text: value.prompt_text,
            suggested_action_label: value.suggested_action_label,
            suggested_response: value.suggested_response,
            allow_skip: value.allow_skip,
            blocking: value.blocking,
            submit_target: value.submit_target.into(),
            escalation: value.escalation.map(Into::into),
            transitions: value.transitions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTriggerKindData {
    StaleSchedule,
    MissedEvent,
    SlippedPlannedBlock,
    MajorSyncChange,
    TaskNoLongerFits,
}

impl From<vel_core::ReflowTriggerKind> for ReflowTriggerKindData {
    fn from(value: vel_core::ReflowTriggerKind) -> Self {
        match value {
            vel_core::ReflowTriggerKind::StaleSchedule => Self::StaleSchedule,
            vel_core::ReflowTriggerKind::MissedEvent => Self::MissedEvent,
            vel_core::ReflowTriggerKind::SlippedPlannedBlock => Self::SlippedPlannedBlock,
            vel_core::ReflowTriggerKind::MajorSyncChange => Self::MajorSyncChange,
            vel_core::ReflowTriggerKind::TaskNoLongerFits => Self::TaskNoLongerFits,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowSeverityData {
    Medium,
    High,
    Critical,
}

impl From<vel_core::ReflowSeverity> for ReflowSeverityData {
    fn from(value: vel_core::ReflowSeverity) -> Self {
        match value {
            vel_core::ReflowSeverity::Medium => Self::Medium,
            vel_core::ReflowSeverity::High => Self::High,
            vel_core::ReflowSeverity::Critical => Self::Critical,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowAcceptModeData {
    DirectAccept,
    ConfirmRequired,
}

impl From<vel_core::ReflowAcceptMode> for ReflowAcceptModeData {
    fn from(value: vel_core::ReflowAcceptMode) -> Self {
        match value {
            vel_core::ReflowAcceptMode::DirectAccept => Self::DirectAccept,
            vel_core::ReflowAcceptMode::ConfirmRequired => Self::ConfirmRequired,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTransitionKindData {
    Accept,
    Edit,
}

impl From<vel_core::ReflowTransitionKind> for ReflowTransitionKindData {
    fn from(value: vel_core::ReflowTransitionKind) -> Self {
        match value {
            vel_core::ReflowTransitionKind::Accept => Self::Accept,
            vel_core::ReflowTransitionKind::Edit => Self::Edit,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTransitionTargetKindData {
    ApplySuggestion,
    Threads,
}

impl From<vel_core::ReflowTransitionTargetKind> for ReflowTransitionTargetKindData {
    fn from(value: vel_core::ReflowTransitionTargetKind) -> Self {
        match value {
            vel_core::ReflowTransitionTargetKind::ApplySuggestion => Self::ApplySuggestion,
            vel_core::ReflowTransitionTargetKind::Threads => Self::Threads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowTransitionData {
    pub kind: ReflowTransitionKindData,
    pub label: String,
    pub target: ReflowTransitionTargetKindData,
    pub confirm_required: bool,
}

impl From<vel_core::ReflowTransition> for ReflowTransitionData {
    fn from(value: vel_core::ReflowTransition) -> Self {
        Self {
            kind: value.kind.into(),
            label: value.label,
            target: value.target.into(),
            confirm_required: value.confirm_required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowEditTargetData {
    pub target: CheckInEscalationTargetData,
    pub label: String,
}

impl From<vel_core::ReflowEditTarget> for ReflowEditTargetData {
    fn from(value: vel_core::ReflowEditTarget) -> Self {
        Self {
            target: value.target.into(),
            label: value.label,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowCardData {
    pub id: ActionItemId,
    pub title: String,
    pub summary: String,
    pub trigger: ReflowTriggerKindData,
    pub severity: ReflowSeverityData,
    pub accept_mode: ReflowAcceptModeData,
    pub suggested_action_label: String,
    #[serde(default)]
    pub preview_lines: Vec<String>,
    pub edit_target: ReflowEditTargetData,
    #[serde(default)]
    pub transitions: Vec<ReflowTransitionData>,
}

impl From<vel_core::ReflowCard> for ReflowCardData {
    fn from(value: vel_core::ReflowCard) -> Self {
        Self {
            id: value.id,
            title: value.title,
            summary: value.summary,
            trigger: value.trigger.into(),
            severity: value.severity.into(),
            accept_mode: value.accept_mode.into(),
            suggested_action_label: value.suggested_action_label,
            preview_lines: value.preview_lines,
            edit_target: value.edit_target.into(),
            transitions: value.transitions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentContextReflowStatusKindData {
    Applied,
    Editing,
}

impl From<vel_core::CurrentContextReflowStatusKind> for CurrentContextReflowStatusKindData {
    fn from(value: vel_core::CurrentContextReflowStatusKind) -> Self {
        match value {
            vel_core::CurrentContextReflowStatusKind::Applied => Self::Applied,
            vel_core::CurrentContextReflowStatusKind::Editing => Self::Editing,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContextReflowStatusData {
    pub kind: CurrentContextReflowStatusKindData,
    pub trigger: ReflowTriggerKindData,
    pub severity: ReflowSeverityData,
    pub headline: String,
    pub detail: String,
    pub recorded_at: UnixSeconds,
    #[serde(default)]
    pub preview_lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl From<vel_core::CurrentContextReflowStatus> for CurrentContextReflowStatusData {
    fn from(value: vel_core::CurrentContextReflowStatus) -> Self {
        Self {
            kind: value.kind.into(),
            trigger: value.trigger.into(),
            severity: value.severity.into(),
            headline: value.headline,
            detail: value.detail,
            recorded_at: value.recorded_at,
            preview_lines: value.preview_lines,
            thread_id: value.thread_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItemData {
    pub id: ActionItemId,
    pub surface: ActionSurfaceData,
    pub kind: ActionKindData,
    pub permission_mode: ActionPermissionModeData,
    pub scope_affinity: ActionScopeAffinityData,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub project_label: Option<String>,
    pub project_family: Option<ProjectFamilyData>,
    pub state: ActionStateData,
    pub rank: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub surfaced_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub snoozed_until: Option<OffsetDateTime>,
    #[serde(default)]
    pub evidence: Vec<ActionEvidenceRefData>,
}

impl From<vel_core::ActionItem> for ActionItemData {
    fn from(value: vel_core::ActionItem) -> Self {
        Self {
            id: value.id,
            surface: value.surface.into(),
            kind: value.kind.into(),
            permission_mode: value.permission_mode.into(),
            scope_affinity: value.scope_affinity.into(),
            title: value.title,
            summary: value.summary,
            project_id: value.project_id,
            project_label: value.project_label,
            project_family: value.project_family.map(Into::into),
            state: value.state.into(),
            rank: value.rank,
            surfaced_at: value.surfaced_at,
            snoozed_until: value.snoozed_until,
            evidence: value.evidence.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReviewSnapshotData {
    #[serde(default)]
    pub open_action_count: u32,
    #[serde(default)]
    pub triage_count: u32,
    #[serde(default)]
    pub projects_needing_review: u32,
    #[serde(default)]
    pub pending_execution_reviews: u32,
}

impl From<vel_core::ReviewSnapshot> for ReviewSnapshotData {
    fn from(value: vel_core::ReviewSnapshot) -> Self {
        Self {
            open_action_count: value.open_action_count,
            triage_count: value.triage_count,
            projects_needing_review: value.projects_needing_review,
            pending_execution_reviews: value.pending_execution_reviews,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritebackTargetRefData {
    pub family: IntegrationFamilyData,
    pub provider_key: String,
    pub project_id: Option<ProjectId>,
    pub connection_id: Option<IntegrationConnectionId>,
    pub external_id: Option<String>,
}

impl From<vel_core::WritebackTargetRef> for WritebackTargetRefData {
    fn from(value: vel_core::WritebackTargetRef) -> Self {
        Self {
            family: value.family.into(),
            provider_key: value.provider_key,
            project_id: value.project_id,
            connection_id: value.connection_id,
            external_id: value.external_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackRiskData {
    Safe,
    ConfirmRequired,
    Blocked,
}

impl From<vel_core::WritebackRisk> for WritebackRiskData {
    fn from(value: vel_core::WritebackRisk) -> Self {
        match value {
            vel_core::WritebackRisk::Safe => Self::Safe,
            vel_core::WritebackRisk::ConfirmRequired => Self::ConfirmRequired,
            vel_core::WritebackRisk::Blocked => Self::Blocked,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackStatusData {
    Queued,
    InProgress,
    Applied,
    Conflicted,
    Denied,
    Failed,
    Cancelled,
}

impl From<vel_core::WritebackStatus> for WritebackStatusData {
    fn from(value: vel_core::WritebackStatus) -> Self {
        match value {
            vel_core::WritebackStatus::Queued => Self::Queued,
            vel_core::WritebackStatus::InProgress => Self::InProgress,
            vel_core::WritebackStatus::Applied => Self::Applied,
            vel_core::WritebackStatus::Conflicted => Self::Conflicted,
            vel_core::WritebackStatus::Denied => Self::Denied,
            vel_core::WritebackStatus::Failed => Self::Failed,
            vel_core::WritebackStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackOperationKindData {
    TodoistCreateTask,
    TodoistUpdateTask,
    TodoistCompleteTask,
    TodoistReopenTask,
    NotesCreateNote,
    NotesAppendNote,
    RemindersCreate,
    RemindersUpdate,
    RemindersComplete,
    GithubCreateIssue,
    GithubAddComment,
    GithubCloseIssue,
    GithubReopenIssue,
    EmailCreateDraftReply,
    EmailSendDraft,
}

impl From<vel_core::WritebackOperationKind> for WritebackOperationKindData {
    fn from(value: vel_core::WritebackOperationKind) -> Self {
        match value {
            vel_core::WritebackOperationKind::TodoistCreateTask => Self::TodoistCreateTask,
            vel_core::WritebackOperationKind::TodoistUpdateTask => Self::TodoistUpdateTask,
            vel_core::WritebackOperationKind::TodoistCompleteTask => Self::TodoistCompleteTask,
            vel_core::WritebackOperationKind::TodoistReopenTask => Self::TodoistReopenTask,
            vel_core::WritebackOperationKind::NotesCreateNote => Self::NotesCreateNote,
            vel_core::WritebackOperationKind::NotesAppendNote => Self::NotesAppendNote,
            vel_core::WritebackOperationKind::RemindersCreate => Self::RemindersCreate,
            vel_core::WritebackOperationKind::RemindersUpdate => Self::RemindersUpdate,
            vel_core::WritebackOperationKind::RemindersComplete => Self::RemindersComplete,
            vel_core::WritebackOperationKind::GithubCreateIssue => Self::GithubCreateIssue,
            vel_core::WritebackOperationKind::GithubAddComment => Self::GithubAddComment,
            vel_core::WritebackOperationKind::GithubCloseIssue => Self::GithubCloseIssue,
            vel_core::WritebackOperationKind::GithubReopenIssue => Self::GithubReopenIssue,
            vel_core::WritebackOperationKind::EmailCreateDraftReply => Self::EmailCreateDraftReply,
            vel_core::WritebackOperationKind::EmailSendDraft => Self::EmailSendDraft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritebackOperationData {
    pub id: WritebackOperationId,
    pub kind: WritebackOperationKindData,
    pub risk: WritebackRiskData,
    pub status: WritebackStatusData,
    pub target: WritebackTargetRefData,
    pub requested_payload: JsonValue,
    pub result_payload: Option<JsonValue>,
    #[serde(default)]
    pub provenance: Vec<IntegrationSourceRefData>,
    pub conflict_case_id: Option<String>,
    pub requested_by_node_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub requested_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub applied_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<vel_core::WritebackOperationRecord> for WritebackOperationData {
    fn from(value: vel_core::WritebackOperationRecord) -> Self {
        Self {
            id: value.id,
            kind: value.kind.into(),
            risk: value.risk.into(),
            status: value.status.into(),
            target: value.target.into(),
            requested_payload: value.requested_payload,
            result_payload: value.result_payload,
            provenance: value.provenance.into_iter().map(Into::into).collect(),
            conflict_case_id: value.conflict_case_id,
            requested_by_node_id: value.requested_by_node_id,
            requested_at: value.requested_at,
            applied_at: value.applied_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCaseKindData {
    UpstreamVsLocal,
    CrossClient,
    StaleWrite,
    ExecutorUnavailable,
}

impl From<vel_core::ConflictCaseKind> for ConflictCaseKindData {
    fn from(value: vel_core::ConflictCaseKind) -> Self {
        match value {
            vel_core::ConflictCaseKind::UpstreamVsLocal => Self::UpstreamVsLocal,
            vel_core::ConflictCaseKind::CrossClient => Self::CrossClient,
            vel_core::ConflictCaseKind::StaleWrite => Self::StaleWrite,
            vel_core::ConflictCaseKind::ExecutorUnavailable => Self::ExecutorUnavailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCaseStatusData {
    Open,
    Acknowledged,
    Resolved,
    Dismissed,
    Expired,
}

impl From<vel_core::ConflictCaseStatus> for ConflictCaseStatusData {
    fn from(value: vel_core::ConflictCaseStatus) -> Self {
        match value {
            vel_core::ConflictCaseStatus::Open => Self::Open,
            vel_core::ConflictCaseStatus::Acknowledged => Self::Acknowledged,
            vel_core::ConflictCaseStatus::Resolved => Self::Resolved,
            vel_core::ConflictCaseStatus::Dismissed => Self::Dismissed,
            vel_core::ConflictCaseStatus::Expired => Self::Expired,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictCaseData {
    pub id: ConflictCaseId,
    pub kind: ConflictCaseKindData,
    pub status: ConflictCaseStatusData,
    pub target: WritebackTargetRefData,
    pub summary: String,
    pub local_payload: JsonValue,
    pub upstream_payload: Option<JsonValue>,
    pub resolution_payload: Option<JsonValue>,
    #[serde(with = "time::serde::rfc3339")]
    pub opened_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub resolved_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<vel_core::ConflictCaseRecord> for ConflictCaseData {
    fn from(value: vel_core::ConflictCaseRecord) -> Self {
        Self {
            id: value.id,
            kind: value.kind.into(),
            status: value.status.into(),
            target: value.target.into(),
            summary: value.summary,
            local_payload: value.local_payload,
            upstream_payload: value.upstream_payload,
            resolution_payload: value.resolution_payload,
            opened_at: value.opened_at,
            resolved_at: value.resolved_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAliasData {
    pub platform: String,
    pub handle: String,
    pub display: String,
    pub source_ref: Option<IntegrationSourceRefData>,
}

impl From<vel_core::PersonAlias> for PersonAliasData {
    fn from(value: vel_core::PersonAlias) -> Self {
        Self {
            platform: value.platform,
            handle: value.handle,
            display: value.display,
            source_ref: value.source_ref.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonLinkRefData {
    pub kind: String,
    pub id: String,
    pub label: String,
}

impl From<vel_core::PersonLinkRef> for PersonLinkRefData {
    fn from(value: vel_core::PersonLinkRef) -> Self {
        Self {
            kind: value.kind,
            id: value.id,
            label: value.label,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRecordData {
    pub id: PersonId,
    pub display_name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub relationship_context: Option<String>,
    pub birthday: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_contacted_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub aliases: Vec<PersonAliasData>,
    #[serde(default)]
    pub links: Vec<PersonLinkRefData>,
}

impl From<vel_core::PersonRecord> for PersonRecordData {
    fn from(value: vel_core::PersonRecord) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            given_name: value.given_name,
            family_name: value.family_name,
            relationship_context: value.relationship_context,
            birthday: value.birthday,
            last_contacted_at: value.last_contacted_at,
            aliases: value.aliases.into_iter().map(Into::into).collect(),
            links: value.links.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAliasUpsertRequestData {
    pub platform: String,
    pub handle: String,
    pub display: Option<String>,
    pub source_ref: Option<IntegrationSourceRefData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStatusData {
    Pending,
    Linked,
    Revoked,
    Expired,
}

impl From<vel_core::LinkStatus> for LinkStatusData {
    fn from(value: vel_core::LinkStatus) -> Self {
        match value {
            vel_core::LinkStatus::Pending => Self::Pending,
            vel_core::LinkStatus::Linked => Self::Linked,
            vel_core::LinkStatus::Revoked => Self::Revoked,
            vel_core::LinkStatus::Expired => Self::Expired,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LinkScopeData {
    #[serde(default)]
    pub read_context: bool,
    #[serde(default)]
    pub write_safe_actions: bool,
    #[serde(default)]
    pub execute_repo_tasks: bool,
}

impl From<vel_core::LinkScope> for LinkScopeData {
    fn from(value: vel_core::LinkScope) -> Self {
        Self {
            read_context: value.read_context,
            write_safe_actions: value.write_safe_actions,
            execute_repo_tasks: value.execute_repo_tasks,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingTokenData {
    pub token_id: String,
    pub token_code: String,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub issued_by_node_id: String,
    pub scopes: LinkScopeData,
    #[serde(default)]
    pub suggested_targets: Vec<LinkTargetSuggestionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTargetSuggestionData {
    pub label: String,
    pub base_url: String,
    pub transport_hint: String,
    pub recommended: bool,
    pub redeem_command_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkingPromptData {
    pub target_node_id: String,
    pub target_node_display_name: Option<String>,
    pub issued_by_node_id: String,
    pub issued_by_node_display_name: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub scopes: LinkScopeData,
    #[serde(default)]
    pub issuer_sync_base_url: String,
    #[serde(default)]
    pub issuer_sync_transport: String,
    #[serde(default)]
    pub issuer_tailscale_base_url: Option<String>,
    #[serde(default)]
    pub issuer_lan_base_url: Option<String>,
    #[serde(default)]
    pub issuer_localhost_base_url: Option<String>,
    #[serde(default)]
    pub issuer_public_base_url: Option<String>,
}

impl From<vel_core::PairingTokenRecord> for PairingTokenData {
    fn from(value: vel_core::PairingTokenRecord) -> Self {
        Self {
            token_id: value.token_id,
            token_code: value.token_code,
            issued_at: value.issued_at,
            expires_at: value.expires_at,
            issued_by_node_id: value.issued_by_node_id,
            scopes: value.scopes.into(),
            suggested_targets: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedNodeData {
    pub node_id: String,
    pub node_display_name: String,
    pub status: LinkStatusData,
    pub scopes: LinkScopeData,
    #[serde(with = "time::serde::rfc3339")]
    pub linked_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_seen_at: Option<OffsetDateTime>,
    pub transport_hint: Option<String>,
    pub sync_base_url: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub public_base_url: Option<String>,
}

impl From<vel_core::LinkedNodeRecord> for LinkedNodeData {
    fn from(value: vel_core::LinkedNodeRecord) -> Self {
        Self {
            node_id: value.node_id,
            node_display_name: value.node_display_name,
            status: value.status.into(),
            scopes: value.scopes.into(),
            linked_at: value.linked_at,
            last_seen_at: value.last_seen_at,
            transport_hint: value.transport_hint,
            sync_base_url: value.sync_base_url,
            tailscale_base_url: value.tailscale_base_url,
            lan_base_url: value.lan_base_url,
            localhost_base_url: value.localhost_base_url,
            public_base_url: value.public_base_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchSyncCapabilityData {
    pub repo_root: String,
    pub default_remote: String,
    pub supports_fetch: bool,
    pub supports_pull: bool,
    pub supports_push: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationProfileData {
    pub profile_id: String,
    pub label: String,
    pub command_hint: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchSyncRequestData {
    pub repo_root: String,
    pub branch: String,
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub base_branch: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequestData {
    pub repo_root: String,
    pub profile_id: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueuedWorkRoutingKindData {
    BranchSync,
    Validation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedWorkRoutingData {
    pub work_request_id: String,
    pub request_type: QueuedWorkRoutingKindData,
    pub status: String,
    pub queued_signal_id: String,
    pub queued_signal_type: String,
    pub queued_at: UnixSeconds,
    pub queued_via: String,
    pub authority_node_id: String,
    pub authority_epoch: i64,
    pub target_node_id: String,
    pub target_worker_class: String,
    pub requested_capability: String,
    #[serde(default)]
    pub request_payload: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementRecommendationData {
    pub worker_id: String,
    pub node_id: String,
    pub capability: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkAssignmentStatusData {
    Assigned,
    Started,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentReceiptData {
    pub receipt_id: String,
    pub work_request_id: String,
    pub worker_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    pub status: WorkAssignmentStatusData,
    pub assigned_at: UnixSeconds,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    #[serde(default)]
    pub completed_at: Option<UnixSeconds>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
    pub last_updated: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimRequestData {
    pub work_request_id: String,
    pub worker_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentUpdateRequest {
    pub receipt_id: String,
    pub status: WorkAssignmentStatusData,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    #[serde(default)]
    pub completed_at: Option<UnixSeconds>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedWorkItemData {
    pub work_request_id: String,
    pub request_type: QueuedWorkRoutingKindData,
    pub queued_signal_id: String,
    pub queued_signal_type: String,
    pub queued_at: UnixSeconds,
    pub target_node_id: String,
    pub target_worker_class: String,
    pub requested_capability: String,
    pub request_payload: JsonValue,
    #[serde(default)]
    pub latest_receipt: Option<WorkAssignmentReceiptData>,
    pub is_stale: bool,
    pub attempt_count: u32,
    pub claimable_now: bool,
    #[serde(default)]
    pub claim_reason: Option<String>,
    #[serde(default)]
    pub next_retry_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimNextRequestData {
    pub node_id: String,
    pub worker_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimedWorkData {
    pub queue_item: QueuedWorkItemData,
    pub receipt: WorkAssignmentReceiptData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimNextResponseData {
    #[serde(default)]
    pub claim: Option<WorkAssignmentClaimedWorkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHeartbeatRequestData {
    pub node_id: String,
    #[serde(default)]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub client_kind: Option<String>,
    #[serde(default)]
    pub client_version: Option<String>,
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub build_id: Option<String>,
    pub worker_id: String,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub max_concurrency: Option<u32>,
    #[serde(default)]
    pub current_load: Option<u32>,
    #[serde(default)]
    pub queue_depth: Option<u32>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub latency_class: Option<String>,
    #[serde(default)]
    pub compute_class: Option<String>,
    #[serde(default)]
    pub power_class: Option<String>,
    #[serde(default)]
    pub recent_failure_rate: Option<f64>,
    #[serde(default)]
    pub tailscale_preferred: Option<bool>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub tailscale_base_url: Option<String>,
    #[serde(default)]
    pub preferred_tailnet_endpoint: Option<String>,
    #[serde(default)]
    pub tailscale_reachable: Option<bool>,
    #[serde(default)]
    pub lan_base_url: Option<String>,
    #[serde(default)]
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub ping_ms: Option<u32>,
    #[serde(default)]
    pub sync_status: Option<String>,
    #[serde(default)]
    pub last_upstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_downstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub last_heartbeat_at: Option<UnixSeconds>,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHeartbeatResponseData {
    pub accepted: bool,
    pub worker_id: String,
    pub expires_at: UnixSeconds,
    pub cluster_view_version: UnixSeconds,
    #[serde(default)]
    pub placement_hints: Vec<PlacementRecommendationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientActionKind {
    NudgeDone,
    NudgeSnooze,
    CommitmentDone,
    CommitmentCreate,
    CaptureCreate,
    BranchSyncRequest,
    ValidationRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionData {
    pub action_id: Option<String>,
    pub action_type: ClientActionKind,
    pub target_id: Option<String>,
    pub text: Option<String>,
    pub minutes: Option<u32>,
    #[serde(default)]
    pub payload: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionBatchRequest {
    pub actions: Vec<ClientActionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionResultData {
    pub action_id: Option<String>,
    pub action_type: ClientActionKind,
    pub target_id: Option<String>,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionBatchResultData {
    pub applied: u32,
    pub results: Vec<ClientActionResultData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeStateData {
    pub node_id: String,
    #[serde(default, alias = "display_name")]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub node_class: Option<String>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub tailscale_base_url: Option<String>,
    #[serde(default)]
    pub lan_base_url: Option<String>,
    #[serde(default)]
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub last_seen_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorkerStateData {
    #[serde(alias = "id")]
    pub worker_id: String,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub client_kind: Option<String>,
    #[serde(default)]
    pub client_version: Option<String>,
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub build_id: Option<String>,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub max_concurrency: Option<u32>,
    #[serde(default)]
    pub current_load: Option<u32>,
    #[serde(default)]
    pub queue_depth: Option<u32>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub latency_class: Option<String>,
    #[serde(default)]
    pub compute_class: Option<String>,
    #[serde(default)]
    pub power_class: Option<String>,
    #[serde(default)]
    pub recent_failure_rate: Option<f64>,
    #[serde(default)]
    pub tailscale_preferred: Option<bool>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub tailscale_base_url: Option<String>,
    #[serde(default)]
    pub preferred_tailnet_endpoint: Option<String>,
    #[serde(default)]
    pub tailscale_reachable: Option<bool>,
    #[serde(default)]
    pub lan_base_url: Option<String>,
    #[serde(default)]
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub ping_ms: Option<u32>,
    #[serde(default)]
    pub heartbeat_age_seconds: Option<UnixSeconds>,
    #[serde(default)]
    pub sync_status: Option<String>,
    #[serde(default)]
    pub last_upstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_downstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub last_heartbeat_at: Option<UnixSeconds>,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    #[serde(default)]
    pub available_concurrency: Option<u32>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub active_work: Vec<SwarmClientActiveWorkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncClusterStateData {
    #[serde(default)]
    pub cluster_view_version: Option<i64>,
    #[serde(default)]
    pub authority_node_id: Option<String>,
    #[serde(default)]
    pub authority_epoch: Option<i64>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub cluster: Option<ClusterBootstrapData>,
    #[serde(default)]
    pub nodes: Vec<ClusterNodeStateData>,
    #[serde(default)]
    pub workers: Vec<ClusterWorkerStateData>,
    #[serde(default)]
    pub clients: Vec<SwarmClientData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmClientActiveWorkData {
    pub receipt_id: String,
    pub work_request_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    pub status: String,
    pub assigned_at: UnixSeconds,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    pub last_updated: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmClientData {
    pub client_id: String,
    pub node_id: String,
    #[serde(default)]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub client_kind: Option<String>,
    #[serde(default)]
    pub client_version: Option<String>,
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub build_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub ping_ms: Option<u32>,
    #[serde(default)]
    pub heartbeat_age_seconds: Option<UnixSeconds>,
    #[serde(default)]
    pub last_heartbeat_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_upstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_downstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub sync_status: Option<String>,
    #[serde(default)]
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub max_concurrency: Option<u32>,
    #[serde(default)]
    pub current_load: Option<u32>,
    #[serde(default)]
    pub queue_depth: Option<u32>,
    #[serde(default)]
    pub active_work: Vec<SwarmClientActiveWorkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapacityData {
    pub max_concurrency: u32,
    pub current_load: u32,
    pub available_concurrency: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerPresenceData {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: String,
    pub client_kind: Option<String>,
    pub client_version: Option<String>,
    pub protocol_version: Option<String>,
    pub build_id: Option<String>,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub status: String,
    pub queue_depth: u32,
    pub reachability: String,
    pub latency_class: String,
    pub compute_class: String,
    pub power_class: String,
    pub recent_failure_rate: f64,
    pub tailscale_preferred: bool,
    pub last_heartbeat_at: UnixSeconds,
    pub started_at: UnixSeconds,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub ping_ms: Option<u32>,
    pub sync_status: Option<String>,
    pub last_upstream_sync_at: Option<UnixSeconds>,
    pub last_downstream_sync_at: Option<UnixSeconds>,
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub incoming_linking_prompt: Option<LinkingPromptData>,
    pub capacity: WorkerCapacityData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorkersData {
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub generated_at: UnixSeconds,
    pub workers: Vec<WorkerPresenceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmClientsData {
    pub generated_at: UnixSeconds,
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    #[serde(default)]
    pub clients: Vec<SwarmClientData>,
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
    pub backup: BackupTrustData,
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
pub struct MoodJournalCreateRequest {
    pub score: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_device: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainJournalCreateRequest {
    pub severity: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_device: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct AgentSpecData {
    pub id: String,
    pub kind: String,
    pub mission: String,
    pub ttl_seconds: u64,
    pub allowed_tools: Vec<String>,
    pub memory_scope: AgentMemoryScopeData,
    pub return_contract: String,
    #[serde(default)]
    pub budgets: Option<AgentBudgetsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpecListData {
    pub specs: Vec<AgentSpecData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryScopeData {
    pub constitution: bool,
    #[serde(default)]
    pub topic_pads: Vec<String>,
    #[serde(default)]
    pub event_query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBudgetsData {
    #[serde(default)]
    pub max_tool_calls: Option<u32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub max_memory_queries: Option<u32>,
    #[serde(default)]
    pub max_side_effects: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpawnRequestData {
    pub agent_id: String,
    pub mission_input: JsonValue,
    #[serde(default)]
    pub parent_run_id: Option<String>,
    #[serde(default)]
    pub deadline: Option<Rfc3339Timestamp>,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub requested_tools: Option<Vec<String>>,
    #[serde(default)]
    pub budgets: Option<AgentBudgetsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntimeViewData {
    pub run_id: String,
    pub agent_id: String,
    pub status: String,
    #[serde(default)]
    pub parent_run_id: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub started_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub finished_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub waiting_on: Option<JsonValue>,
    #[serde(default)]
    pub return_contract: Option<AgentReturnContractData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentReturnStatusData {
    Completed,
    Error,
    Blocked,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnContractData {
    pub status: AgentReturnStatusData,
    pub summary: String,
    #[serde(default)]
    pub evidence: Vec<AgentReturnEvidenceData>,
    pub confidence: f64,
    #[serde(default)]
    pub suggested_actions: Vec<AgentSuggestedActionData>,
    #[serde(default)]
    pub artifacts: Vec<AgentReturnedArtifactData>,
    #[serde(default)]
    pub errors: Vec<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnEvidenceData {
    pub kind: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSuggestedActionData {
    #[serde(rename = "type")]
    pub action_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnedArtifactData {
    pub artifact_type: String,
    pub location: String,
    #[serde(default)]
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct IntegrationCalendarData {
    pub id: String,
    pub summary: String,
    pub primary: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationGuidanceData {
    pub title: String,
    pub detail: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarIntegrationData {
    pub configured: bool,
    pub connected: bool,
    pub has_client_id: bool,
    pub has_client_secret: bool,
    pub calendars: Vec<IntegrationCalendarData>,
    pub all_calendars_selected: bool,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoistIntegrationData {
    pub configured: bool,
    pub connected: bool,
    pub has_api_token: bool,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalIntegrationData {
    pub configured: bool,
    pub source_path: Option<String>,
    #[serde(default)]
    pub selected_paths: Vec<String>,
    #[serde(default)]
    pub available_paths: Vec<String>,
    #[serde(default)]
    pub internal_paths: Vec<String>,
    #[serde(default)]
    pub suggested_paths: Vec<String>,
    pub source_kind: String,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalIntegrationPathSelectionData {
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsData {
    pub google_calendar: GoogleCalendarIntegrationData,
    pub todoist: TodoistIntegrationData,
    pub activity: LocalIntegrationData,
    pub health: LocalIntegrationData,
    pub git: LocalIntegrationData,
    pub messaging: LocalIntegrationData,
    pub reminders: LocalIntegrationData,
    pub notes: LocalIntegrationData,
    pub transcripts: LocalIntegrationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnectionSettingRefData {
    pub setting_key: String,
    pub setting_value: String,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnectionData {
    pub id: String,
    pub family: String,
    pub provider_key: String,
    pub status: String,
    pub display_name: String,
    pub account_ref: Option<String>,
    pub metadata: JsonValue,
    pub created_at: UnixSeconds,
    pub updated_at: UnixSeconds,
    #[serde(default)]
    pub setting_refs: Vec<IntegrationConnectionSettingRefData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnectionEventData {
    pub id: String,
    pub connection_id: String,
    pub event_type: String,
    pub payload: JsonValue,
    pub timestamp: UnixSeconds,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarAuthStartData {
    pub auth_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRuntimeCapabilityData {
    pub runtime_id: String,
    pub display_name: String,
    pub supports_launch: bool,
    pub supports_interactive_followup: bool,
    pub supports_native_open: bool,
    pub supports_host_agent_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectInstanceCapabilityManifestData {
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub launchable_runtimes: Vec<ConnectRuntimeCapabilityData>,
    pub supports_agent_launch: bool,
    pub supports_interactive_followup: bool,
    pub supports_native_open: bool,
    pub supports_host_agent_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectInstanceData {
    pub id: String,
    pub node_id: String,
    pub display_name: String,
    pub connection_id: Option<String>,
    pub status: String,
    pub reachability: String,
    pub sync_base_url: Option<String>,
    pub sync_transport: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub worker_ids: Vec<String>,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    pub last_seen_at: Option<UnixSeconds>,
    pub manifest: ConnectInstanceCapabilityManifestData,
    pub metadata: JsonValue,
}

impl From<vel_core::ConnectRuntimeCapability> for ConnectRuntimeCapabilityData {
    fn from(capability: vel_core::ConnectRuntimeCapability) -> Self {
        Self {
            runtime_id: capability.runtime_id,
            display_name: capability.display_name,
            supports_launch: capability.supports_launch,
            supports_interactive_followup: capability.supports_interactive_followup,
            supports_native_open: capability.supports_native_open,
            supports_host_agent_control: capability.supports_host_agent_control,
        }
    }
}

impl From<vel_core::ConnectInstanceCapabilityManifest> for ConnectInstanceCapabilityManifestData {
    fn from(manifest: vel_core::ConnectInstanceCapabilityManifest) -> Self {
        Self {
            worker_classes: manifest.worker_classes,
            capabilities: manifest.capabilities,
            launchable_runtimes: manifest
                .launchable_runtimes
                .into_iter()
                .map(ConnectRuntimeCapabilityData::from)
                .collect(),
            supports_agent_launch: manifest.supports_agent_launch,
            supports_interactive_followup: manifest.supports_interactive_followup,
            supports_native_open: manifest.supports_native_open,
            supports_host_agent_control: manifest.supports_host_agent_control,
        }
    }
}

impl From<vel_core::ConnectInstance> for ConnectInstanceData {
    fn from(instance: vel_core::ConnectInstance) -> Self {
        Self {
            id: instance.id,
            node_id: instance.node_id,
            display_name: instance.display_name,
            connection_id: instance.connection_id,
            status: instance.status.to_string(),
            reachability: instance.reachability,
            sync_base_url: instance.sync_base_url,
            sync_transport: instance.sync_transport,
            tailscale_base_url: instance.tailscale_base_url,
            lan_base_url: instance.lan_base_url,
            localhost_base_url: instance.localhost_base_url,
            worker_ids: instance.worker_ids,
            worker_classes: instance.worker_classes,
            last_seen_at: instance
                .last_seen_at
                .map(|seen_at| seen_at.unix_timestamp()),
            manifest: instance.manifest.into(),
            metadata: instance.metadata_json,
        }
    }
}

// --- Chat / Web surfaces ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: UnixSeconds,
    pub updated_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationCreateRequest {
    pub title: Option<String>,
    #[serde(default = "default_conversation_kind")]
    pub kind: String,
}

fn default_conversation_kind() -> String {
    "general".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationUpdateRequest {
    pub title: Option<String>,
    pub pinned: Option<bool>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content: JsonValue,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub created_at: UnixSeconds,
    pub updated_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageResponse {
    pub user_message: MessageData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message: Option<MessageData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateRequest {
    pub role: String,
    pub kind: String,
    pub content: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxItemData {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: UnixSeconds,
    pub snoozed_until: Option<UnixSeconds>,
    pub confidence: Option<f64>,
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub project_id: Option<ProjectId>,
    #[serde(default)]
    pub project_label: Option<String>,
    #[serde(default)]
    pub available_actions: Vec<AvailableActionData>,
    #[serde(default)]
    pub evidence: Vec<ActionEvidenceRefData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionActionData {
    pub id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceData {
    pub message_id: String,
    pub events: Vec<ProvenanceEvent>,
    pub signals: Vec<JsonValue>,
    pub policy_decisions: Vec<JsonValue>,
    pub linked_objects: Vec<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEvent {
    pub id: String,
    pub event_name: String,
    pub created_at: UnixSeconds,
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WsEventType {
    #[serde(rename = "messages:new")]
    MessagesNew,
    #[serde(rename = "interventions:new")]
    InterventionsNew,
    #[serde(rename = "interventions:updated")]
    InterventionsUpdated,
    #[serde(rename = "context:updated")]
    ContextUpdated,
    #[serde(rename = "runs:updated")]
    RunsUpdated,
    #[serde(rename = "components:updated")]
    ComponentsUpdated,
    #[serde(rename = "linking:updated")]
    LinkingUpdated,
}

impl std::fmt::Display for WsEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::MessagesNew => "messages:new",
            Self::InterventionsNew => "interventions:new",
            Self::InterventionsUpdated => "interventions:updated",
            Self::ContextUpdated => "context:updated",
            Self::RunsUpdated => "runs:updated",
            Self::ComponentsUpdated => "components:updated",
            Self::LinkingUpdated => "linking:updated",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for WsEventType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "messages:new" => Ok(Self::MessagesNew),
            "interventions:new" => Ok(Self::InterventionsNew),
            "interventions:updated" => Ok(Self::InterventionsUpdated),
            "context:updated" => Ok(Self::ContextUpdated),
            "runs:updated" => Ok(Self::RunsUpdated),
            "components:updated" => Ok(Self::ComponentsUpdated),
            "linking:updated" => Ok(Self::LinkingUpdated),
            other => Err(format!("unknown websocket event type: {}", other)),
        }
    }
}

impl From<&str> for WsEventType {
    fn from(value: &str) -> Self {
        value
            .parse()
            .unwrap_or_else(|_| panic!("invalid websocket event type: {}", value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsEnvelope {
    #[serde(rename = "type")]
    pub event_type: WsEventType,
    pub timestamp: Rfc3339Timestamp,
    pub payload: JsonValue,
}

impl WsEnvelope {
    pub fn new(event_type: impl Into<WsEventType>, payload: JsonValue) -> Self {
        Self {
            event_type: event_type.into(),
            timestamp: OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .expect("current timestamp should format as RFC3339"),
            payload,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub size_bytes: Option<i64>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// --- Runs (spec Section 15) ---

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
mod run_datetime_contract_tests {
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

#[cfg(test)]
mod linking_datetime_contract_tests {
    use super::*;

    #[test]
    fn pairing_and_linking_datetimes_serialize_as_rfc3339_strings() {
        let issued_at = OffsetDateTime::from_unix_timestamp(1_710_590_400).unwrap();
        let expires_at = OffsetDateTime::from_unix_timestamp(1_710_590_700).unwrap();

        let token = serde_json::to_value(PairingTokenData {
            token_id: "ptok_1".to_string(),
            token_code: "ABC123".to_string(),
            issued_at,
            expires_at,
            issued_by_node_id: "vel-node".to_string(),
            scopes: LinkScopeData {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            suggested_targets: Vec::new(),
        })
        .unwrap();
        assert!(token["issued_at"].is_string());
        assert!(token["expires_at"].is_string());

        let prompt = serde_json::to_value(LinkingPromptData {
            target_node_id: "node_remote".to_string(),
            target_node_display_name: Some("Remote".to_string()),
            issued_by_node_id: "vel-node".to_string(),
            issued_by_node_display_name: Some("Local".to_string()),
            issued_at,
            expires_at,
            scopes: LinkScopeData {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            issuer_sync_base_url: "http://vel-node.tailnet.ts.net:4130".to_string(),
            issuer_sync_transport: "tailscale".to_string(),
            issuer_tailscale_base_url: Some("http://vel-node.tailnet.ts.net:4130".to_string()),
            issuer_lan_base_url: Some("http://192.168.1.10:4130".to_string()),
            issuer_localhost_base_url: Some("http://127.0.0.1:4130".to_string()),
            issuer_public_base_url: None,
        })
        .unwrap();
        assert!(prompt["issued_at"].is_string());
        assert!(prompt["expires_at"].is_string());

        let linked = serde_json::to_value(LinkedNodeData {
            node_id: "node_remote".to_string(),
            node_display_name: "Remote".to_string(),
            status: LinkStatusData::Linked,
            scopes: LinkScopeData {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            linked_at: issued_at,
            last_seen_at: Some(expires_at),
            transport_hint: Some("tailscale".to_string()),
            sync_base_url: Some("http://node-remote.tailnet.ts.net:4130".to_string()),
            tailscale_base_url: Some("http://node-remote.tailnet.ts.net:4130".to_string()),
            lan_base_url: Some("http://192.168.1.20:4130".to_string()),
            localhost_base_url: None,
            public_base_url: None,
        })
        .unwrap();
        assert!(linked["linked_at"].is_string());
        assert!(linked["last_seen_at"].is_string());
    }
}

// --- Commitments ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentCreateRequest {
    pub text: String,
    #[serde(default = "default_commitment_source_type")]
    pub source_type: String,
    pub source_id: Option<String>,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    #[serde(default)]
    pub metadata: JsonValue,
}

fn default_commitment_source_type() -> String {
    "manual".to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitmentData {
    pub id: CommitmentId,
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub resolved_at: Option<OffsetDateTime>,
    pub metadata: JsonValue,
}

impl From<vel_core::Commitment> for CommitmentData {
    fn from(c: vel_core::Commitment) -> Self {
        Self {
            id: c.id,
            text: c.text,
            source_type: c.source_type,
            source_id: c.source_id,
            status: c.status.to_string(),
            due_at: c.due_at,
            project: c.project,
            commitment_kind: c.commitment_kind,
            created_at: c.created_at,
            resolved_at: c.resolved_at,
            metadata: c.metadata_json,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommitmentUpdateRequest {
    pub status: Option<String>,
    pub due_at: Option<Option<OffsetDateTime>>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentDependencyData {
    pub id: String,
    pub parent_commitment_id: String,
    pub child_commitment_id: String,
    pub dependency_type: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentDependencyCreateRequest {
    pub child_commitment_id: String,
    #[serde(default = "default_dependency_type")]
    pub dependency_type: String,
}

fn default_dependency_type() -> String {
    "blocks".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskData {
    pub commitment_id: String,
    pub risk_score: f64,
    pub risk_level: String,
    pub factors: RiskFactorsData,
    pub computed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactorsData {
    pub consequence: f64,
    pub proximity: f64,
    pub dependency_pressure: f64,
    pub external_anchor: f64,
    pub stale_open_age: f64,
    pub reasons: Vec<String>,
    pub dependency_ids: Vec<String>,
}

impl From<RiskFactors> for RiskFactorsData {
    fn from(value: RiskFactors) -> Self {
        Self {
            consequence: value.consequence,
            proximity: value.proximity,
            dependency_pressure: value.dependency_pressure,
            external_anchor: value.external_anchor,
            stale_open_age: value.stale_open_age,
            reasons: value.reasons,
            dependency_ids: value.dependency_ids,
        }
    }
}

impl From<RiskSnapshot> for RiskData {
    fn from(snapshot: RiskSnapshot) -> Self {
        let normalized_level = snapshot.normalized_level().to_string();
        Self {
            commitment_id: snapshot.commitment_id,
            risk_score: snapshot.risk_score,
            risk_level: normalized_level,
            factors: snapshot.factors.into(),
            computed_at: snapshot.computed_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionData {
    pub id: String,
    pub suggestion_type: String,
    pub state: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: i64,
    pub confidence: Option<String>,
    pub evidence_count: u32,
    pub decision_context_summary: Option<String>,
    pub decision_context: Option<JsonValue>,
    pub evidence: Option<Vec<SuggestionEvidenceData>>,
    #[serde(default)]
    pub latest_feedback_outcome: Option<String>,
    #[serde(default)]
    pub latest_feedback_notes: Option<String>,
    #[serde(default)]
    pub adaptive_policy: Option<SuggestionAdaptivePolicyData>,
    pub payload: JsonValue,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptivePolicyOverrideData {
    pub policy_key: String,
    pub value_minutes: u32,
    pub source_suggestion_id: Option<String>,
    pub source_title: Option<String>,
    pub source_accepted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionAdaptivePolicyData {
    pub policy_key: String,
    pub suggested_minutes: u32,
    pub current_minutes: Option<u32>,
    pub is_active_source: bool,
    pub active_override: Option<AdaptivePolicyOverrideData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEvidenceData {
    pub id: String,
    pub evidence_type: String,
    pub ref_id: String,
    pub evidence: Option<JsonValue>,
    pub weight: Option<f64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionUpdateRequest {
    pub state: Option<String>,
    pub payload: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuggestionActionRequest {
    pub reason: Option<String>,
}

// --- Signals (Phase B) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCreateRequest {
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalData {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload: JsonValue,
    pub created_at: i64,
}

// --- Nudges (Phase D) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeData {
    pub nudge_id: String,
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub related_commitment_id: Option<String>,
    pub message: String,
    pub created_at: i64,
    pub snoozed_until: Option<i64>,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeSnoozeRequest {
    #[serde(default = "default_snooze_minutes")]
    pub minutes: u32,
}

fn default_snooze_minutes() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResultData {
    pub source: String,
    pub signals_ingested: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopData {
    pub kind: String,
    pub enabled: bool,
    pub interval_seconds: i64,
    pub last_started_at: Option<UnixSeconds>,
    pub last_finished_at: Option<UnixSeconds>,
    pub last_status: Option<String>,
    pub last_error: Option<String>,
    pub next_due_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopUpdateRequest {
    pub enabled: Option<bool>,
    pub interval_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyData {
    pub id: String,
    pub subject_type: String,
    pub subject_id: Option<String>,
    pub decision_kind: String,
    pub confidence_band: String,
    pub confidence_score: Option<f64>,
    pub reasons: JsonValue,
    pub missing_evidence: Option<JsonValue>,
    pub resolution_mode: String,
    pub status: String,
    pub created_at: UnixSeconds,
    pub resolved_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub last_restarted_at: Option<i64>,
    pub last_error: Option<String>,
    pub restart_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLogEventData {
    pub id: String,
    pub component_id: String,
    pub event_name: String,
    pub status: String,
    pub message: String,
    pub payload: JsonValue,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationLogEventData {
    pub id: String,
    pub integration_id: String,
    pub event_name: String,
    pub status: String,
    pub message: String,
    pub payload: JsonValue,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateResultData {
    pub inferred_states: u32,
    pub nudges_created_or_updated: u32,
}

/// A single freshness entry for a data source tracked by the operator diagnostics endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessEntryData {
    /// Identifier for the data source (e.g. worker ID or source name).
    pub source: String,
    /// Unix timestamp of last successful heartbeat or sync for this source.
    pub last_seen_at: Option<UnixSeconds>,
    /// Freshness status: "fresh" | "stale" | "missing"
    pub status: String,
}

/// Operator diagnostics payload — surfaces currently available sync/capability state.
/// Returned by GET /api/diagnostics (operator-authenticated).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsData {
    /// Node ID of the authority node.
    pub node_id: String,
    /// Human-readable display name for the authority node.
    pub node_display_name: String,
    /// Unix timestamp when this diagnostics snapshot was generated.
    pub generated_at: UnixSeconds,
    /// Overall sync status: "ready" | "degraded" | "offline" | "unknown"
    pub sync_status: String,
    /// Count of currently active (registered) workers.
    pub active_workers: u32,
    /// Unique capability strings advertised across all active workers.
    pub capability_summary: Vec<String>,
    /// Per-source freshness entries derived from active worker heartbeat data.
    pub freshness_entries: Vec<FreshnessEntryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisWeekData {
    pub run_id: String,
    pub artifact_id: String,
}

/// Persistent current context singleton (computed by inference engine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContextData {
    pub computed_at: UnixSeconds,
    pub context: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBootstrapData {
    pub cluster: ClusterBootstrapData,
    pub current_context: Option<CurrentContextData>,
    pub nudges: Vec<NudgeData>,
    pub commitments: Vec<CommitmentData>,
    #[serde(default)]
    pub linked_nodes: Vec<LinkedNodeData>,
    #[serde(default)]
    pub projects: Vec<ProjectRecordData>,
    #[serde(default)]
    pub action_items: Vec<ActionItemData>,
    #[serde(default)]
    pub pending_writebacks: Vec<WritebackOperationData>,
    #[serde(default)]
    pub conflicts: Vec<ConflictCaseData>,
    #[serde(default)]
    pub people: Vec<PersonRecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowLabelData {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowRiskSummaryData {
    pub level: String,
    pub score: Option<f64>,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowSummaryData {
    pub mode: NowLabelData,
    pub phase: NowLabelData,
    pub meds: NowLabelData,
    pub risk: NowRiskSummaryData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowEventData {
    pub title: String,
    pub start_ts: UnixSeconds,
    pub end_ts: Option<UnixSeconds>,
    pub location: Option<String>,
    pub prep_minutes: Option<i64>,
    pub travel_minutes: Option<i64>,
    pub leave_by_ts: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTaskData {
    pub id: String,
    pub text: String,
    pub source_type: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowScheduleData {
    pub empty_message: Option<String>,
    pub next_event: Option<NowEventData>,
    pub upcoming_events: Vec<NowEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTasksData {
    pub todoist: Vec<NowTaskData>,
    pub other_open: Vec<NowTaskData>,
    pub next_commitment: Option<NowTaskData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowAttentionData {
    pub state: NowLabelData,
    pub drift: NowLabelData,
    pub severity: NowLabelData,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowSourceActivityData {
    pub label: String,
    pub timestamp: UnixSeconds,
    pub summary: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowSourcesData {
    pub git_activity: Option<NowSourceActivityData>,
    pub health: Option<NowSourceActivityData>,
    pub mood: Option<NowSourceActivityData>,
    pub pain: Option<NowSourceActivityData>,
    pub note_document: Option<NowSourceActivityData>,
    pub assistant_message: Option<NowSourceActivityData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowFreshnessEntryData {
    pub key: String,
    pub label: String,
    pub status: String,
    pub last_sync_at: Option<UnixSeconds>,
    pub age_seconds: Option<UnixSeconds>,
    pub guidance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowFreshnessData {
    pub overall_status: String,
    pub sources: Vec<NowFreshnessEntryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustReadinessFacetData {
    pub level: String,
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustReadinessReviewData {
    pub open_action_count: u32,
    pub pending_execution_reviews: u32,
    pub pending_writeback_count: u32,
    pub conflict_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustReadinessData {
    pub level: String,
    pub headline: String,
    pub summary: String,
    pub backup: TrustReadinessFacetData,
    pub freshness: TrustReadinessFacetData,
    pub review: TrustReadinessReviewData,
    #[serde(default)]
    pub guidance: Vec<String>,
    #[serde(default)]
    pub follow_through: Vec<ActionItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowDebugData {
    pub raw_context: JsonValue,
    pub signals_used: Vec<String>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowData {
    pub computed_at: UnixSeconds,
    pub timezone: String,
    pub summary: NowSummaryData,
    pub schedule: NowScheduleData,
    pub tasks: NowTasksData,
    pub attention: NowAttentionData,
    pub sources: NowSourcesData,
    pub freshness: NowFreshnessData,
    pub trust_readiness: TrustReadinessData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in: Option<CheckInCardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflow: Option<ReflowCardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflow_status: Option<CurrentContextReflowStatusData>,
    #[serde(default)]
    pub action_items: Vec<ActionItemData>,
    #[serde(default)]
    pub review_snapshot: ReviewSnapshotData,
    #[serde(default)]
    pub pending_writebacks: Vec<WritebackOperationData>,
    #[serde(default)]
    pub conflicts: Vec<ConflictCaseData>,
    #[serde(default)]
    pub people: Vec<PersonRecordData>,
    pub reasons: Vec<String>,
    pub debug: NowDebugData,
}

/// One entry in the context timeline (material context transitions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTimelineEntry {
    pub id: String,
    pub timestamp: i64,
    pub context: JsonValue,
}

/// Thread summary/list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadData {
    pub id: String,
    pub thread_type: String,
    pub title: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ThreadLinkData>>,
}

/// Thread link (entity linked to a thread).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLinkData {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCreateRequest {
    pub thread_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLinkRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadUpdateRequest {
    pub status: Option<String>,
}

/// Explain payload for current context (context + reasons + entity ids used).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalExplainSummary {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub timestamp: UnixSeconds,
    pub summary: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSourceSummaryData {
    pub timestamp: UnixSeconds,
    pub summary: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSourceSummariesData {
    pub git_activity: Option<ContextSourceSummaryData>,
    pub health: Option<ContextSourceSummaryData>,
    pub mood: Option<ContextSourceSummaryData>,
    pub pain: Option<ContextSourceSummaryData>,
    pub note_document: Option<ContextSourceSummaryData>,
    pub assistant_message: Option<ContextSourceSummaryData>,
}

/// Explain payload for current context (context + reasons + entity ids used).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextExplainData {
    pub computed_at: UnixSeconds,
    pub mode: Option<String>,
    pub morning_state: Option<String>,
    pub context: JsonValue,
    pub source_summaries: ContextSourceSummariesData,
    #[serde(default)]
    pub adaptive_policy_overrides: Vec<AdaptivePolicyOverrideData>,
    pub signals_used: Vec<String>,
    pub signal_summaries: Vec<SignalExplainSummary>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
    pub reasons: Vec<String>,
}

/// Explain payload for a commitment (commitment + risk snapshot + why in context).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentExplainData {
    pub commitment_id: String,
    pub commitment: JsonValue,
    pub risk: Option<JsonValue>,
    pub in_context_reasons: Vec<String>,
}

/// Explain payload for drift (attention/drift state from current context).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftExplainData {
    pub attention_state: Option<String>,
    pub drift_type: Option<String>,
    pub drift_severity: Option<String>,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
    pub signals_used: Vec<String>,
    pub signal_summaries: Vec<SignalExplainSummary>,
    pub commitments_used: Vec<String>,
}

/// Explain payload for a nudge (nudge + inference/signals snapshots for explainability).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeExplainData {
    pub nudge_id: String,
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub message: String,
    pub inference_snapshot: Option<JsonValue>,
    pub signals_snapshot: Option<JsonValue>,
    pub events: Vec<NudgeEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeEventData {
    pub id: String,
    pub event_type: String,
    pub payload: JsonValue,
    pub timestamp: i64,
    pub created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::{
        ActionEvidenceRefData, ActionItemData, ActionKindData, ActionPermissionModeData,
        ActionScopeAffinityData, ActionStateData, ActionSurfaceData, AgentBlockerData,
        AgentCapabilityEntryData, AgentCapabilityGroupKindData, AgentInspectData, AgentProfileData,
        AppleBehaviorMetricData, AppleBehaviorSummaryData, AppleBehaviorSummaryScopeData,
        AppleClientSurfaceData, AppleRequestedOperationData, AppleResponseEvidenceData,
        AppleResponseModeData, AppleScheduleEventData, AppleScheduleSnapshotData,
        AppleTurnProvenanceData, AppleVoiceIntentData, AppleVoiceTurnQueuedMutationSummaryData,
        AppleVoiceTurnRequestData, AppleVoiceTurnResponseData, DailyCommitmentDraftData,
        DailyDeferredTaskData, DailyFocusBlockProposalData, DailyLoopCheckInResolutionData,
        DailyLoopCheckInResolutionKindData, DailyLoopPhaseData, DailyLoopSessionData,
        DailyLoopSessionOutcomeData, DailyLoopStartMetadataData, DailyLoopStartRequestData,
        DailyLoopStartSourceData, DailyLoopSurfaceData, DailyLoopTurnActionData,
        DailyLoopTurnRequestData, DailyStandupBucketData, DailyStandupOutcomeData,
        ExecutionHandoffData, ExecutionHandoffReviewStateData, ExecutionReviewGateData,
        ExecutionTaskKindData, LocalRuntimeKindData, MorningIntentSignalData, NowTaskData,
        ProjectExecutionContextData, ProjectFamilyData, ProjectProvisionRequestData,
        ProjectRecordData, ProjectRootRefData, ProjectStatusData, ReviewSnapshotData,
        TokenBudgetClassData,
    };
    use std::collections::BTreeMap;
    use time::macros::datetime;
    use vel_core::{
        ActionItemId, AgentProfile, CapabilityDescriptor, DailyCommitmentDraft, DailyDeferredTask,
        DailyFocusBlockProposal, DailyLoopPhase, DailyLoopPrompt, DailyLoopPromptKind,
        DailyLoopSession, DailyLoopSessionId, DailyLoopSessionOutcome, DailyLoopStartMetadata,
        DailyLoopStartSource, DailyLoopStatus, DailyLoopSurface, DailyLoopTurnState,
        DailyStandupBucket, DailyStandupOutcome, ExecutionHandoff, ExecutionReviewGate,
        ExecutionTaskKind, HandoffEnvelope, LocalAgentManifest, LocalRuntimeKind,
        MorningFrictionCallout, MorningIntentSignal, MorningOverviewState, ProjectExecutionContext,
        ProjectId, ProjectRootRef, RepoWorktreeRef, TokenBudgetClass, TraceId,
    };

    #[test]
    fn now_task_due_at_serializes_as_rfc3339_string() {
        let task = NowTaskData {
            id: "commit_1".to_string(),
            text: "Reply to Dimitri".to_string(),
            source_type: "todoist".to_string(),
            due_at: Some(datetime!(2026-03-16 19:00:00 UTC)),
            project: None,
            commitment_kind: Some("todo".to_string()),
        };

        let value = serde_json::to_value(task).expect("now task should serialize");
        assert_eq!(value["due_at"], "2026-03-16T19:00:00Z");
    }

    #[test]
    fn now_task_none_due_at_serializes_as_null() {
        let task = NowTaskData {
            id: "commit_2".to_string(),
            text: "Inbox zero".to_string(),
            source_type: "manual".to_string(),
            due_at: None,
            project: None,
            commitment_kind: None,
        };

        let value = serde_json::to_value(task).expect("now task should serialize");
        assert!(value["due_at"].is_null());
    }

    #[test]
    fn review_snapshot_default_serializes_named_counts() {
        let value = serde_json::to_value(ReviewSnapshotData::default())
            .expect("review snapshot should serialize");

        assert_eq!(value["open_action_count"], 0);
        assert_eq!(value["triage_count"], 0);
        assert_eq!(value["projects_needing_review"], 0);
        assert_eq!(value["pending_execution_reviews"], 0);
    }

    #[test]
    fn action_item_timestamps_serialize_as_rfc3339_strings() {
        let item = ActionItemData {
            id: ActionItemId::from("act_1".to_string()),
            surface: ActionSurfaceData::Now,
            kind: ActionKindData::NextStep,
            permission_mode: ActionPermissionModeData::UserConfirm,
            scope_affinity: ActionScopeAffinityData::Global,
            title: "Ship patch".to_string(),
            summary: "Due soon".to_string(),
            project_id: None,
            project_label: None,
            project_family: None,
            state: ActionStateData::Active,
            rank: 70,
            surfaced_at: datetime!(2026-03-19 02:10:00 UTC),
            snoozed_until: Some(datetime!(2026-03-19 02:20:00 UTC)),
            evidence: vec![ActionEvidenceRefData {
                source_kind: "commitment".to_string(),
                source_id: "com_1".to_string(),
                label: "Ship patch".to_string(),
                detail: None,
            }],
        };

        let value = serde_json::to_value(item).expect("action item should serialize");
        assert_eq!(value["surfaced_at"], "2026-03-19T02:10:00Z");
        assert_eq!(value["snoozed_until"], "2026-03-19T02:20:00Z");
        assert_eq!(value["permission_mode"], "user_confirm");
        assert_eq!(value["scope_affinity"], "global");
    }

    #[test]
    fn project_record_timestamps_serialize_as_rfc3339_strings() {
        let project = ProjectRecordData {
            id: ProjectId::from("proj_1".to_string()),
            slug: "vel".to_string(),
            name: "Vel".to_string(),
            family: ProjectFamilyData::Work,
            status: ProjectStatusData::Active,
            primary_repo: ProjectRootRefData {
                path: "/tmp/vel".to_string(),
                label: "vel".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRefData {
                path: "/tmp/notes/vel".to_string(),
                label: "vel".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: BTreeMap::new(),
            pending_provision: ProjectProvisionRequestData {
                create_repo: false,
                create_notes_root: false,
            },
            created_at: datetime!(2026-03-19 02:10:00 UTC),
            updated_at: datetime!(2026-03-19 02:20:00 UTC),
            archived_at: None,
        };

        let value = serde_json::to_value(project).expect("project should serialize");
        assert_eq!(value["created_at"], "2026-03-19T02:10:00Z");
        assert_eq!(value["updated_at"], "2026-03-19T02:20:00Z");
        assert!(value["archived_at"].is_null());
    }

    #[test]
    fn project_execution_context_converts_from_core() {
        let context = ProjectExecutionContext {
            project_id: ProjectId::from("proj_velruntime".to_string()),
            repo: RepoWorktreeRef {
                path: "/home/jove/code/vel".to_string(),
                label: "vel".to_string(),
                branch: Some("main".to_string()),
                head_rev: Some("abc1234".to_string()),
            },
            notes_root: ProjectRootRef {
                path: "/home/jove/notes/vel".to_string(),
                label: "Vel Notes".to_string(),
                kind: "notes_root".to_string(),
            },
            gsd_artifact_dir: ".planning/vel".to_string(),
            default_task_kind: ExecutionTaskKind::Implementation,
            default_agent_profile: AgentProfile::Balanced,
            default_token_budget: TokenBudgetClass::Large,
            review_gate: ExecutionReviewGate::OperatorPreview,
            read_roots: vec!["/home/jove/code/vel".to_string()],
            write_roots: vec!["/home/jove/code/vel/.planning/vel".to_string()],
            local_manifests: vec![LocalAgentManifest {
                manifest_id: "manifest_local_cli".to_string(),
                runtime_kind: LocalRuntimeKind::LocalCli,
                entrypoint: "cargo".to_string(),
                working_directory: "/home/jove/code/vel".to_string(),
                args: vec!["run".to_string(), "-p".to_string(), "vel-cli".to_string()],
                env_keys: vec!["VEL_OPERATOR_TOKEN".to_string()],
                read_roots: vec!["/home/jove/code/vel".to_string()],
                write_roots: vec!["/home/jove/code/vel/.planning/vel".to_string()],
                allowed_tools: vec!["rg".to_string(), "cargo".to_string()],
                capabilities: vec![CapabilityDescriptor {
                    scope: "repo.read".to_string(),
                    resource: Some("/home/jove/code/vel".to_string()),
                    action: "read".to_string(),
                }],
                review_gate: ExecutionReviewGate::OperatorPreview,
            }],
            metadata: BTreeMap::from([("phase".to_string(), "08".to_string())]),
            created_at: datetime!(2026-03-19 10:00:00 UTC),
            updated_at: datetime!(2026-03-19 10:05:00 UTC),
        };

        let data = ProjectExecutionContextData::from(context);
        assert_eq!(
            data.project_id,
            ProjectId::from("proj_velruntime".to_string())
        );
        assert_eq!(
            data.default_task_kind,
            ExecutionTaskKindData::Implementation
        );
        assert_eq!(data.default_agent_profile, AgentProfileData::Balanced);
        assert_eq!(data.default_token_budget, TokenBudgetClassData::Large);
        assert_eq!(data.review_gate, ExecutionReviewGateData::OperatorPreview);
        assert_eq!(
            data.local_manifests[0].runtime_kind,
            LocalRuntimeKindData::LocalCli
        );
    }

    #[test]
    fn execution_handoff_converts_from_core() {
        let handoff = ExecutionHandoff {
            handoff: HandoffEnvelope {
                task_id: "task_1".to_string(),
                trace_id: TraceId::from("trace_1".to_string()),
                from_agent: "planner".to_string(),
                to_agent: "executor".to_string(),
                objective: "Implement Phase 08 contracts".to_string(),
                inputs: serde_json::json!({ "ticket": "08-01" }),
                constraints: vec!["stay within write scope".to_string()],
                read_scopes: vec!["docs/".to_string(), "crates/".to_string()],
                write_scopes: vec!["crates/vel-core/".to_string()],
                project_id: Some(ProjectId::from("proj_velruntime".to_string())),
                task_kind: Some(ExecutionTaskKind::Implementation),
                agent_profile: Some(AgentProfile::Balanced),
                token_budget: Some(TokenBudgetClass::Large),
                review_gate: Some(ExecutionReviewGate::OperatorPreview),
                repo_root: Some(RepoWorktreeRef {
                    path: "/home/jove/code/vel".to_string(),
                    label: "vel".to_string(),
                    branch: Some("main".to_string()),
                    head_rev: Some("abc1234".to_string()),
                }),
                allowed_tools: vec!["rg".to_string(), "cargo".to_string()],
                capability_scope: serde_json::json!({ "mode": "scoped" }),
                deadline: Some(datetime!(2026-03-19 12:00:00 UTC)),
                expected_output_schema: serde_json::json!({ "type": "object" }),
            },
            project_id: ProjectId::from("proj_velruntime".to_string()),
            task_kind: ExecutionTaskKind::Implementation,
            agent_profile: AgentProfile::Balanced,
            token_budget: TokenBudgetClass::Large,
            review_gate: ExecutionReviewGate::OperatorPreview,
            repo: RepoWorktreeRef {
                path: "/home/jove/code/vel".to_string(),
                label: "vel".to_string(),
                branch: Some("main".to_string()),
                head_rev: Some("abc1234".to_string()),
            },
            notes_root: ProjectRootRef {
                path: "/home/jove/notes/vel".to_string(),
                label: "Vel Notes".to_string(),
                kind: "notes_root".to_string(),
            },
            manifest_id: Some("manifest_local_cli".to_string()),
        };

        let data = ExecutionHandoffData::from(handoff);
        assert_eq!(data.task_kind, ExecutionTaskKindData::Implementation);
        assert_eq!(data.agent_profile, AgentProfileData::Balanced);
        assert_eq!(data.token_budget, TokenBudgetClassData::Large);
        assert_eq!(data.review_gate, ExecutionReviewGateData::OperatorPreview);
        assert_eq!(
            data.handoff.task_kind,
            Some(ExecutionTaskKindData::Implementation)
        );
        assert_eq!(data.handoff.repo_root.unwrap().label, "vel");
    }

    #[test]
    fn apple_voice_turn_request_round_trips_between_wire_and_core_types() {
        let request = AppleVoiceTurnRequestData {
            transcript: "what matters now".to_string(),
            surface: AppleClientSurfaceData::IosVoice,
            operation: AppleRequestedOperationData::QueryOnly,
            intents: vec![
                AppleVoiceIntentData::CurrentSchedule,
                AppleVoiceIntentData::ExplainWhy,
            ],
            provenance: Some(AppleTurnProvenanceData {
                source_device: Some("iphone".to_string()),
                locale: Some("en-US".to_string()),
                transcript_origin: Some("speech".to_string()),
                recorded_at: Some(datetime!(2026-03-19 07:10:00 UTC)),
                offline_captured_at: None,
                queued_at: None,
            }),
        };

        let core: vel_core::AppleVoiceTurnRequest = request.clone().into();
        let round_trip = AppleVoiceTurnRequestData::from(core);
        let value = serde_json::to_value(round_trip).expect("apple request should serialize");

        assert_eq!(value["surface"], "ios_voice");
        assert_eq!(value["operation"], "query_only");
        assert_eq!(value["provenance"]["recorded_at"], "2026-03-19T07:10:00Z");
    }

    #[test]
    fn apple_voice_turn_response_serializes_nested_schedule_and_behavior_summary() {
        let response = AppleVoiceTurnResponseData {
            operation: AppleRequestedOperationData::CaptureAndQuery,
            mode: AppleResponseModeData::SpokenSummary,
            summary: "You have standup in 20 minutes.".to_string(),
            capture_id: Some("cap_voice_1".to_string().into()),
            reasons: vec!["Standup starts at 09:00.".to_string()],
            evidence: vec![AppleResponseEvidenceData {
                kind: "event".to_string(),
                label: "Standup".to_string(),
                detail: "Starts at 09:00".to_string(),
                source_id: Some("evt_1".to_string()),
            }],
            queued_mutation: Some(AppleVoiceTurnQueuedMutationSummaryData {
                mutation_kind: "capture_create".to_string(),
                queued: false,
                summary: "Transcript stored as a voice note.".to_string(),
                action_reference_id: Some("act_1".to_string()),
            }),
            schedule: Some(AppleScheduleSnapshotData {
                generated_at: 1_763_661_000,
                timezone: "America/Denver".to_string(),
                focus_summary: Some("Morning execution block".to_string()),
                next_event: Some(AppleScheduleEventData {
                    title: "Standup".to_string(),
                    start_ts: 1_763_661_600,
                    end_ts: Some(1_763_662_200),
                    location: Some("Desk".to_string()),
                    leave_by_ts: Some(1_763_661_300),
                }),
                upcoming_events: vec![],
                reasons: vec!["Calendar synced 2 minutes ago.".to_string()],
            }),
            behavior_summary: Some(AppleBehaviorSummaryData {
                generated_at: 1_763_661_000,
                timezone: "America/Denver".to_string(),
                scope: AppleBehaviorSummaryScopeData::Daily,
                headline: "You are on track for movement today.".to_string(),
                metrics: vec![AppleBehaviorMetricData {
                    metric_key: "step_count".to_string(),
                    display_label: "Steps".to_string(),
                    value: 4200.0,
                    unit: "count".to_string(),
                    recorded_at: 1_763_660_900,
                    reasons: vec!["Above your same-time baseline.".to_string()],
                }],
                reasons: vec!["Health snapshot is fresh.".to_string()],
                freshness_seconds: Some(120),
            }),
        };

        let value = serde_json::to_value(response).expect("apple response should serialize");
        assert_eq!(value["mode"], "spoken_summary");
        assert_eq!(value["schedule"]["next_event"]["title"], "Standup");
        assert_eq!(
            value["behavior_summary"]["metrics"][0]["metric_key"],
            "step_count"
        );
    }

    #[test]
    fn daily_loop_session_data_round_trips_morning_and_standup_payloads() {
        let morning_session = DailyLoopSession {
            id: DailyLoopSessionId::from("dls_1".to_string()),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::MorningOverview,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::AppleVoice,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_morning_1".to_string(),
                kind: DailyLoopPromptKind::IntentQuestion,
                text: "What most needs to happen before noon?".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: MorningOverviewState {
                snapshot: "You have two meetings before lunch.".to_string(),
                friction_callouts: vec![MorningFrictionCallout {
                    label: "Prep debt".to_string(),
                    detail: "Design review starts in 45 minutes.".to_string(),
                }],
                signals: vec![MorningIntentSignal::MustDoHint {
                    text: "Finish review notes".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_morning_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Finish review notes".to_string()),
                    note_text: None,
                }],
            }
            .into(),
            outcome: Some(DailyLoopSessionOutcome::MorningOverview {
                signals: vec![MorningIntentSignal::FocusIntent {
                    text: "Protect a deep-work block".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_morning_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Protect a deep-work block".to_string()),
                    note_text: None,
                }],
            }),
        };

        let standup_session = DailyLoopSession {
            id: DailyLoopSessionId::from("dls_2".to_string()),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::Standup,
            status: DailyLoopStatus::Completed,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Cli,
            },
            turn_state: DailyLoopTurnState::Completed,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_standup_1".to_string(),
                kind: DailyLoopPromptKind::CommitmentReduction,
                text: "Reduce this to three commitments.".to_string(),
                ordinal: 2,
                allow_skip: false,
            }),
            state: DailyStandupOutcome {
                commitments: vec![DailyCommitmentDraft {
                    title: "Ship Phase 10 contract slice".to_string(),
                    bucket: DailyStandupBucket::Must,
                    source_ref: Some("ticket:10-01".to_string()),
                }],
                deferred_tasks: vec![DailyDeferredTask {
                    title: "Triage lower-priority inbox items".to_string(),
                    source_ref: Some("todoist:42".to_string()),
                    reason: "Not part of the top three".to_string(),
                }],
                confirmed_calendar: vec!["Design review at 10:00 remains on".to_string()],
                focus_blocks: vec![DailyFocusBlockProposal {
                    label: "Contract implementation".to_string(),
                    start_at: datetime!(2026-03-19 15:00:00 UTC),
                    end_at: datetime!(2026-03-19 16:00:00 UTC),
                    reason: "Best uninterrupted slot before review".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_standup_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Ship Phase 10 contract slice".to_string()),
                    note_text: None,
                }],
            }
            .into(),
            outcome: Some(DailyLoopSessionOutcome::Standup(DailyStandupOutcome {
                commitments: vec![DailyCommitmentDraft {
                    title: "Ship Phase 10 contract slice".to_string(),
                    bucket: DailyStandupBucket::Must,
                    source_ref: Some("ticket:10-01".to_string()),
                }],
                deferred_tasks: vec![DailyDeferredTask {
                    title: "Triage lower-priority inbox items".to_string(),
                    source_ref: Some("todoist:42".to_string()),
                    reason: "Not part of the top three".to_string(),
                }],
                confirmed_calendar: vec!["Design review at 10:00 remains on".to_string()],
                focus_blocks: vec![DailyFocusBlockProposal {
                    label: "Contract implementation".to_string(),
                    start_at: datetime!(2026-03-19 15:00:00 UTC),
                    end_at: datetime!(2026-03-19 16:00:00 UTC),
                    reason: "Best uninterrupted slot before review".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_standup_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Ship Phase 10 contract slice".to_string()),
                    note_text: None,
                }],
            })),
        };

        let morning_data = DailyLoopSessionData::from(morning_session.clone());
        let morning_json =
            serde_json::to_value(&morning_data).expect("morning session should serialize");
        assert_eq!(morning_json["phase"], "morning_overview");
        assert_eq!(morning_json["status"], "waiting_for_input");
        assert_eq!(morning_json["current_prompt"]["kind"], "intent_question");
        assert_eq!(morning_json["outcome"]["phase"], "morning_overview");

        let round_trip_morning: DailyLoopSession =
            DailyLoopSessionData::from(morning_session).into();
        assert_eq!(round_trip_morning.phase, DailyLoopPhase::MorningOverview);

        let standup_json = serde_json::to_value(DailyLoopSessionData::from(standup_session))
            .expect("standup session should serialize");
        assert_eq!(standup_json["phase"], "standup");
        assert_eq!(standup_json["outcome"]["phase"], "standup");
        assert_eq!(standup_json["outcome"]["commitments"][0]["bucket"], "must");
    }

    #[test]
    fn daily_loop_morning_signals_stay_distinct_from_standup_commitments() {
        let morning = DailyLoopSessionOutcomeData::MorningOverview {
            signals: vec![MorningIntentSignalData::MustDoHint {
                text: "Handle payroll first".to_string(),
            }],
            check_in_history: vec![DailyLoopCheckInResolutionData {
                prompt_id: "prompt_morning_1".to_string(),
                ordinal: 1,
                kind: DailyLoopCheckInResolutionKindData::Submitted,
                response_text: Some("Handle payroll first".to_string()),
                note_text: None,
            }],
        };
        let standup = DailyLoopSessionOutcomeData::Standup(DailyStandupOutcomeData {
            commitments: vec![DailyCommitmentDraftData {
                title: "Close payroll".to_string(),
                bucket: DailyStandupBucketData::Must,
                source_ref: Some("todoist:payroll".to_string()),
            }],
            deferred_tasks: vec![DailyDeferredTaskData {
                title: "Draft roadmap notes".to_string(),
                source_ref: None,
                reason: "Deferred until after payroll".to_string(),
            }],
            confirmed_calendar: vec!["11:00 payroll check-in".to_string()],
            focus_blocks: vec![DailyFocusBlockProposalData {
                label: "Payroll close".to_string(),
                start_at: datetime!(2026-03-19 16:00:00 UTC),
                end_at: datetime!(2026-03-19 16:30:00 UTC),
                reason: "Smallest uninterrupted slot".to_string(),
            }],
            check_in_history: vec![DailyLoopCheckInResolutionData {
                prompt_id: "prompt_standup_1".to_string(),
                ordinal: 1,
                kind: DailyLoopCheckInResolutionKindData::Submitted,
                response_text: Some("Close payroll".to_string()),
                note_text: None,
            }],
        });

        let morning_json = serde_json::to_value(morning).expect("morning outcome should serialize");
        let standup_json = serde_json::to_value(standup).expect("standup outcome should serialize");

        assert!(morning_json.get("commitments").is_none());
        assert_eq!(morning_json["phase"], "morning_overview");
        assert_eq!(morning_json["signals"][0]["kind"], "must_do_hint");
        assert_eq!(standup_json["phase"], "standup");
        assert_eq!(standup_json["commitments"][0]["title"], "Close payroll");
    }

    #[test]
    fn daily_loop_start_metadata_keeps_source_and_surface_for_manual_and_future_auto_starts() {
        let start = DailyLoopStartRequestData {
            phase: DailyLoopPhaseData::MorningOverview,
            session_date: "2026-03-19".to_string(),
            start: DailyLoopStartMetadataData {
                source: DailyLoopStartSourceData::Automatic,
                surface: DailyLoopSurfaceData::Web,
            },
        };
        let turn = DailyLoopTurnRequestData {
            session_id: "dls_3".to_string(),
            action: DailyLoopTurnActionData::Resume,
            response_text: None,
        };

        let core_start: vel_core::DailyLoopStartRequest = start.clone().into();
        let round_trip = DailyLoopStartRequestData::from(core_start);
        let start_json = serde_json::to_value(round_trip).expect("start request should serialize");
        let turn_json = serde_json::to_value(turn).expect("turn request should serialize");

        assert_eq!(start_json["start"]["source"], "automatic");
        assert_eq!(start_json["start"]["surface"], "web");
        assert_eq!(turn_json["action"], "resume");
    }

    #[test]
    fn agent_grounding_round_trips_typed_sections() {
        let data: AgentInspectData = serde_json::from_str(include_str!(
            "../../../config/examples/agent-inspect.example.json"
        ))
        .expect("agent inspect example should parse");

        assert_eq!(data.grounding.projects.len(), 1);
        assert_eq!(data.grounding.people.len(), 1);
        assert_eq!(data.grounding.commitments.len(), 1);
        assert_eq!(
            data.grounding.review.pending_execution_handoffs[0].review_state,
            ExecutionHandoffReviewStateData::PendingReview
        );
        assert_eq!(
            data.capabilities.groups[1].kind,
            AgentCapabilityGroupKindData::ReviewActions
        );
        assert!(data.explainability.raw_context_json_supporting_only);

        let value = serde_json::to_value(&data).expect("agent inspect should serialize");
        assert_eq!(
            value["grounding"]["review"]["pending_execution_handoffs"][0]["routing"]["task_kind"],
            "implementation"
        );
        assert_eq!(
            value["capabilities"]["groups"][2]["kind"],
            "mutation_actions"
        );
        assert_eq!(
            value["capabilities"]["groups"][2]["entries"][0]["blocked_reason"]["code"],
            "safe_mode_enabled"
        );
    }

    #[test]
    fn agent_grounding_capability_entries_preserve_explicit_blockers() {
        let entry = AgentCapabilityEntryData {
            key: "integration_writeback".to_string(),
            label: "Request integration writeback".to_string(),
            summary: "Can request bounded upstream mutations when writeback is enabled."
                .to_string(),
            available: false,
            blocked_reason: Some(AgentBlockerData {
                code: "safe_mode_enabled".to_string(),
                message: "SAFE MODE keeps writeback disabled.".to_string(),
                escalation_hint: Some("Enable writeback in Settings before retrying.".to_string()),
            }),
            requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
            requires_writeback_enabled: true,
        };

        let value = serde_json::to_value(entry).expect("capability entry should serialize");
        assert_eq!(value["available"], false);
        assert_eq!(value["blocked_reason"]["code"], "safe_mode_enabled");
        assert_eq!(value["requires_review_gate"], "operator_preview");
        assert_eq!(value["requires_writeback_enabled"], true);
    }

    #[test]
    fn agent_grounding_contract_assets_parse_and_register() {
        let pack: super::AgentGroundingPackData = serde_json::from_str(include_str!(
            "../../../config/examples/agent-grounding-pack.example.json"
        ))
        .expect("grounding pack example should parse");
        assert_eq!(pack.review.pending_execution_handoffs.len(), 1);

        let inspect: AgentInspectData = serde_json::from_str(include_str!(
            "../../../config/examples/agent-inspect.example.json"
        ))
        .expect("inspect example should parse");
        assert_eq!(inspect.blockers.len(), 1);

        let grounding_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/agent-grounding-pack.schema.json"
        ))
        .expect("grounding schema should parse");
        assert_eq!(grounding_schema["title"], "AgentGroundingPack");

        let inspect_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/agent-inspect.schema.json"
        ))
        .expect("inspect schema should parse");
        assert_eq!(inspect_schema["title"], "AgentInspect");

        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../config/contracts-manifest.json"))
                .expect("contracts manifest should parse");
        let examples = manifest["contract_examples"]
            .as_array()
            .expect("contract examples should be an array");
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/agent-grounding-pack.example.json"
                && entry["schema"] == "config/schemas/agent-grounding-pack.schema.json"
        }));
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/agent-inspect.example.json"
                && entry["schema"] == "config/schemas/agent-inspect.schema.json"
        }));

        let owner_doc = include_str!(
            "../../../docs/cognitive-agent-architecture/agents/agent-grounding-contracts.md"
        );
        assert!(owner_doc.contains("AgentInspectData"));
        assert!(owner_doc.contains("raw context JSON is supporting evidence"));
    }
}
