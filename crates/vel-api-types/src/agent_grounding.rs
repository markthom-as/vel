use serde::{Deserialize, Serialize};

use crate::{
    CommitmentData, ConflictCaseData, ExecutionHandoffRecordData, ExecutionReviewGateData, NowData,
    PersonRecordData, ProjectRecordData, ReviewSnapshotData, UnixSeconds, WritebackOperationData,
};

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
