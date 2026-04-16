use serde::{Deserialize, Serialize};
use vel_core::ResolvedCommand;

use crate::{
    ArtifactData, CaptureCreateResponse, CommitmentData, CommitmentExplainData, ContextCapture,
    ContextExplainData, DriftExplainData, SynthesisWeekData, ThreadData,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPlanRequest {
    pub command: ResolvedCommand,
    #[serde(default)]
    pub persist_preview: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCompleteRequest {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecuteRequest {
    pub command: ResolvedCommand,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub approve: bool,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub write_scope: Vec<String>,
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
pub struct CommandParsedData {
    pub family: String,
    pub verb: String,
    #[serde(default)]
    pub target_tokens: Vec<String>,
    pub source_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRegistryEntryData {
    pub kind: String,
    pub aliases: Vec<String>,
    pub selectors: Vec<String>,
    pub operations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCompleteResponseData {
    pub input: Vec<String>,
    #[serde(default)]
    pub completion_hints: Vec<String>,
    #[serde(default)]
    pub registry: Vec<CommandRegistryEntryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<CommandParsedData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_command: Option<ResolvedCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_hints: Option<CommandIntentHintsData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_error: Option<String>,
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
