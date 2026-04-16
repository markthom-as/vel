use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::ProjectId;

use crate::ProjectRootRefData;

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
