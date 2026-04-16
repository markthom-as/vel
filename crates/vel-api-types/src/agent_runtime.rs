use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;

use crate::Rfc3339Timestamp;

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
