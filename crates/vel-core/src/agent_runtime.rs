use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::{AgentMemoryScope, AgentReturnContract};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentRunStatus {
    Created,
    Queued,
    Running,
    Waiting,
    Completed,
    Failed,
    Expired,
    Cancelled,
}

impl Display for AgentRunStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Created => "created",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Waiting => "waiting",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
        };
        f.write_str(value)
    }
}

impl FromStr for AgentRunStatus {
    type Err = crate::VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "created" => Ok(Self::Created),
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "waiting" => Ok(Self::Waiting),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "expired" => Ok(Self::Expired),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown agent run status: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRunRecord {
    pub run_id: String,
    pub agent_id: String,
    pub parent_run_id: Option<String>,
    pub status: AgentRunStatus,
    pub mission_input: JsonValue,
    pub deadline_ts: Option<i64>,
    pub ttl_seconds: u64,
    pub expires_at: i64,
    pub waiting_reason: Option<String>,
    pub return_contract: String,
    pub max_tool_calls: u32,
    pub max_tokens: u32,
    pub allowed_tools: Vec<String>,
    pub memory_scope: AgentMemoryScope,
    pub summary: Option<String>,
    pub confidence: Option<f64>,
    pub structured_output: Option<AgentReturnContract>,
    pub created_at: i64,
    pub updated_at: i64,
}
