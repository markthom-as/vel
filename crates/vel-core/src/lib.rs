pub mod apple;
pub mod action_contracts;
pub mod actions;
pub mod capability;
pub mod command;
pub mod commitment;
pub mod conflicts;
pub mod connect;
pub mod context;
pub mod daily_loop;
pub mod execution;
pub mod integration;
pub mod intervention;
pub mod ids;
pub mod linking;
pub mod loops;
pub mod message;
pub mod node_identity;
pub mod object_envelope;
pub mod operator_queue;
pub mod ordering;
pub mod people;
pub mod planning;
pub mod project;
pub mod protocol;
pub mod provenance;
pub mod risk;
pub mod run;
pub mod sandbox;
pub mod scheduler;
pub mod semantic;
pub mod time;
pub mod types;
pub mod uncertainty;
pub mod vocabulary;
pub mod writeback;

pub use apple::{
    AppleBehaviorMetric, AppleBehaviorSummary, AppleBehaviorSummaryScope, AppleClientSurface,
    AppleRequestedOperation, AppleResponseEvidence, AppleResponseMode, AppleScheduleEvent,
    AppleScheduleSnapshot, AppleTurnProvenance, AppleVoiceIntent,
    AppleVoiceTurnQueuedMutationSummary, AppleVoiceTurnRequest, AppleVoiceTurnResponse,
};
pub use action_contracts::{
    generic_object_action_contracts, ActionCapability, ActionContract, ActionErrorKind,
    ActionRequestEnvelope, ActionResponseEnvelope, AuditRequirement, ConfirmationMode,
};
pub use actions::{
    generic_object_action_names, OBJECT_CREATE, OBJECT_DELETE, OBJECT_EXPLAIN, OBJECT_GET,
    OBJECT_LINK, OBJECT_QUERY, OBJECT_UPDATE,
};
pub use capability::{CapabilityDenial, CapabilityDescriptor, CapabilityGrant};
pub use command::{
    CommandConfidenceBand, DomainKind, DomainOperation, IntentResolution, ParseMode, PlanningKind,
    RelationOperation, ResolutionConfidence, ResolutionMeta, ResolvedCommand, TargetSelector,
    TypedTarget,
};
pub use commitment::{Commitment, CommitmentId, CommitmentStatus};
pub use conflicts::{ConflictCaseId, ConflictCaseKind, ConflictCaseRecord, ConflictCaseStatus};
pub use connect::{
    ConnectInstance, ConnectInstanceCapabilityManifest, ConnectInstanceStatus,
    ConnectRuntimeCapability,
};
pub use context::{
    ContextCapture, ContextMigrator, CurrentContextReflowStatus, CurrentContextReflowStatusKind,
    CurrentContextV1, OrientationSnapshot, SearchResult,
};
pub use daily_loop::{
    DailyCommitmentDraft, DailyDeferredTask, DailyFocusBlockProposal, DailyLoopCheckInResolution,
    DailyLoopCheckInResolutionKind, DailyLoopCommitmentLimit, DailyLoopPhase, DailyLoopPrompt,
    DailyLoopPromptKind, DailyLoopQuestionBudget, DailyLoopSession, DailyLoopSessionId,
    DailyLoopSessionOutcome, DailyLoopSessionState, DailyLoopStartMetadata, DailyLoopStartRequest,
    DailyLoopStartSource, DailyLoopStatus, DailyLoopSurface, DailyLoopTurnAction,
    DailyLoopTurnRequest, DailyLoopTurnState, DailyStandupBucket, DailyStandupOutcome,
    MorningFrictionCallout, MorningIntentSignal, MorningOverviewState, DAILY_LOOP_MAX_COMMITMENTS,
    DAILY_LOOP_MAX_QUESTIONS,
};
pub use execution::{
    AgentProfile, ExecutionHandoff, ExecutionPolicyInput, ExecutionReviewGate, ExecutionTaskKind,
    LocalAgentManifest, LocalRuntimeKind, ProjectExecutionContext, RepoWorktreeRef,
    TokenBudgetClass,
};
pub use ids::{
    IntegrationAccountId, ModuleId, SkillId, SyncLinkId, TaskId, ToolId, WorkflowId,
    WriteIntentId,
};
pub use integration::{
    IntegrationConnection, IntegrationConnectionEvent, IntegrationConnectionEventType,
    IntegrationConnectionSettingRef, IntegrationConnectionStatus, IntegrationFamily,
    IntegrationProvider, IntegrationSourceRef,
};
pub use intervention::{Intervention, InterventionState};
pub use linking::{LinkScope, LinkStatus, LinkedNodeRecord, PairingTokenRecord};
pub use loops::LoopKind;
pub use message::{
    Message, MessageAction, MessageBody, MessageImportance, MessageRole, MessageStatus,
    ProvenanceRef, ReminderCard, RiskCard, SuggestionCard, SummaryCard, SystemNotice, TextMessage,
};
pub use node_identity::NodeIdentity;
pub use object_envelope::{
    CanonicalObjectEnvelope, DurableStatus, ObjectClass, ObjectProvenance, SourceSummary,
    TaskEnvelope,
};
pub use operator_queue::{
    ActionEvidenceRef, ActionItem, ActionItemId, ActionKind, ActionPermissionMode,
    ActionScopeAffinity, ActionState, ActionSurface, ActionThreadRoute, ActionThreadRouteTarget,
    AssistantActionProposal, AssistantProposalState, CheckInCard, CheckInEscalation,
    CheckInEscalationTarget, CheckInSourceKind, CheckInSubmitTarget, CheckInSubmitTargetKind,
    CheckInTransition, CheckInTransitionKind, CheckInTransitionTargetKind,
    CommitmentSchedulingContinuity, CommitmentSchedulingMutation, CommitmentSchedulingMutationKind,
    CommitmentSchedulingProposal, CommitmentSchedulingSourceKind, DayPlanChange, DayPlanChangeKind,
    DayPlanProposal, ReflowAcceptMode, ReflowCard, ReflowChange, ReflowChangeKind,
    ReflowEditTarget, ReflowProposal, ReflowSeverity, ReflowTransition, ReflowTransitionKind,
    ReflowTransitionTargetKind, ReflowTriggerKind, ReviewSnapshot, RoutineBlock,
    RoutineBlockSourceKind, ScheduleRuleFacet, ScheduleRuleFacetKind,
};
pub use ordering::OrderingStamp;
pub use people::{PersonAlias, PersonId, PersonLinkRef, PersonRecord};
pub use planning::{
    DurableRoutineBlock, PlanningConstraint, PlanningConstraintKind, PlanningProfileContinuity,
    PlanningProfileEditProposal, PlanningProfileMutation, PlanningProfileMutationKind,
    PlanningProfileRemoveTarget, PlanningProfileSurface, RoutinePlanningProfile,
};
pub use project::{
    ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord, ProjectRootRef, ProjectStatus,
};
pub use protocol::{
    CapabilityRequest as ProtocolCapabilityRequest, ProtocolEnvelope, ProtocolPayload,
    ProtocolSender, ProtocolTraceContext,
};
pub use provenance::{Ref, RefRelationType};
pub use risk::{normalize_risk_level, sort_snapshots_by_priority_desc, RiskFactors, RiskSnapshot};
pub use run::{
    HandoffEnvelope, Run, RunEvent, RunEventType, RunId, RunKind, RunStatus, TraceId, TraceLink,
};
pub use sandbox::{
    FilesystemAccessPolicy, NetworkAccessPolicy, SandboxCapabilityPolicy, SandboxDecisionRecord,
    SandboxDecisionStatus, SandboxHostCall, SandboxHostCallEnvelope, SandboxPolicyMode,
    SandboxResourceLimits,
};
pub use scheduler::{CanonicalScheduleRules, ScheduleTimeWindow};
pub use semantic::{
    HybridRetrievalPolicy, RecallContextHit, RecallContextPack, RecallContextSourceCount,
    RetrievalStrategy, SemanticHit, SemanticMemoryRecord, SemanticProvenance, SemanticQuery,
    SemanticQueryFilters, SemanticRecordId, SemanticSourceKind,
};
pub use time::{Clock, FixedClock, SystemClock};
pub use types::{ConversationId, EventId, IntegrationConnectionId, InterventionId, MessageId};
pub use uncertainty::{ResolutionMode, UncertaintyStatus};
pub use vocabulary::{
    dsl_registry_entries, glossary_entries, glossary_entry, glossary_entry_for_kind,
    normalize_should_command_verb, should_command_verb_entries, GlossaryCategory, GlossaryEntry,
    SHOULD_COMMAND_VERBS,
};
pub use writeback::{
    WritebackOperationId, WritebackOperationKind, WritebackOperationRecord, WritebackRisk,
    WritebackStatus, WritebackTargetRef,
};

use ::time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyClass {
    #[default]
    Private,
    Work,
    Sensitive,
    DoNotRecord,
}

impl Display for PrivacyClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Private => "private",
            Self::Work => "work",
            Self::Sensitive => "sensitive",
            Self::DoNotRecord => "do_not_record",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SyncClass {
    Hot,
    #[default]
    Warm,
    Cold,
}

impl Display for SyncClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Hot => "hot",
            Self::Warm => "warm",
            Self::Cold => "cold",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkAssignmentStatus {
    Assigned,
    Started,
    Completed,
    Failed,
    Cancelled,
}

impl Display for WorkAssignmentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Assigned => "assigned",
            Self::Started => "started",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for WorkAssignmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "assigned" => Ok(Self::Assigned),
            "started" => Ok(Self::Started),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("unknown work assignment status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionFeedback {
    Dismiss,
    Correct,
    NeverShowAgain,
    Train,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionConfidence {
    Low,
    Medium,
    High,
}

impl Display for SuggestionConfidence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        };
        f.write_str(value)
    }
}

pub type ConfidenceBand = SuggestionConfidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    IncreaseCommuteBuffer,
    IncreasePrepWindow,
    AddStartRoutine,
    AddFollowupBlock,
}

impl Display for SuggestionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::IncreaseCommuteBuffer => "increase_commute_buffer",
            Self::IncreasePrepWindow => "increase_prep_window",
            Self::AddStartRoutine => "add_start_routine",
            Self::AddFollowupBlock => "add_followup_block",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaptureId(String);

impl CaptureId {
    pub fn new() -> Self {
        Self(format!("cap_{}", Uuid::new_v4().simple()))
    }
}

impl Default for CaptureId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CaptureId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for CaptureId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for CaptureId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(String);

impl JobId {
    pub fn new() -> Self {
        Self(format!("job_{}", Uuid::new_v4().simple()))
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for JobId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Whether Vel manages the artifact file (writes, checksum, size) or only references an external location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStorageKind {
    Managed,
    #[default]
    External,
}

impl std::fmt::Display for ArtifactStorageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Managed => "managed",
            Self::External => "external",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for ArtifactStorageKind {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "managed" => Ok(Self::Managed),
            "external" => Ok(Self::External),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown storage kind: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactId(String);

impl ArtifactId {
    pub fn new() -> Self {
        Self(format!("art_{}", Uuid::new_v4().simple()))
    }
}

impl Default for ArtifactId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ArtifactId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ArtifactId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ArtifactId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRecord {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub content_text: String,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub source_device: Option<String>,
    pub privacy_class: PrivacyClass,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum VelCoreError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("invalid run transition: {0}")]
    InvalidTransition(String),
}
