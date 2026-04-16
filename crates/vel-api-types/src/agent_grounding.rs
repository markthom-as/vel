use serde::{Deserialize, Serialize};

use crate::ExecutionReviewGateData;

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
