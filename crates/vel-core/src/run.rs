//! Run model: first-class execution records for context generation, synthesis, etc.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(pub(crate) String);

impl RunId {
    pub fn new() -> Self {
        Self(format!("run_{}", Uuid::new_v4().simple()))
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for RunId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for RunId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for RunId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub id: String,
    pub mission: String,
    #[serde(default)]
    pub kind: AgentKind,
    pub allowed_tools: Vec<String>,
    pub memory_scope: AgentMemoryScope,
    pub return_contract: String,
    pub ttl_seconds: u64,
    pub budgets: AgentBudgets,
    #[serde(default)]
    pub mission_input_schema: Option<Value>,
    #[serde(default)]
    pub side_effect_policy: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentKind {
    Subagent,
    Supervisor,
    Specialist,
}

impl Default for AgentKind {
    fn default() -> Self {
        Self::Subagent
    }
}

impl Display for AgentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Subagent => "subagent",
            Self::Supervisor => "supervisor",
            Self::Specialist => "specialist",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for AgentKind {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "subagent" => Ok(Self::Subagent),
            "supervisor" => Ok(Self::Supervisor),
            "specialist" => Ok(Self::Specialist),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown agent kind: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryScope {
    #[serde(default)]
    pub constitution: bool,
    #[serde(default)]
    pub topic_pads: Vec<String>,
    #[serde(default = "default_event_query")]
    pub event_query: String,
}

impl Default for AgentMemoryScope {
    fn default() -> Self {
        Self {
            constitution: false,
            topic_pads: Vec::new(),
            event_query: default_event_query(),
        }
    }
}

fn default_event_query() -> String {
    "limited".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBudgets {
    #[serde(default = "default_max_tool_calls")]
    pub max_tool_calls: u32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub max_memory_queries: Option<u32>,
    #[serde(default)]
    pub max_side_effects: Option<u32>,
}

impl Default for AgentBudgets {
    fn default() -> Self {
        Self {
            max_tool_calls: default_max_tool_calls(),
            max_tokens: default_max_tokens(),
            max_memory_queries: None,
            max_side_effects: None,
        }
    }
}

fn default_max_tool_calls() -> u32 {
    12
}

fn default_max_tokens() -> u32 {
    24_000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpawnRequest {
    pub agent_id: String,
    #[serde(default)]
    pub mission_input: Value,
    #[serde(default)]
    pub parent_run_id: Option<RunId>,
    #[serde(default)]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default)]
    pub priority: Option<AgentPriority>,
}

impl AgentSpawnRequest {
    pub fn validate_for_spec(&self, spec: &AgentSpec) -> Result<(), crate::VelCoreError> {
        if self.agent_id != spec.id {
            return Err(crate::VelCoreError::Validation(format!(
                "agent_id must match spec id, expected {}",
                spec.id
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Default for AgentPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnContract {
    pub status: AgentReturnStatus,
    pub summary: String,
    pub evidence: Vec<AgentReturnEvidence>,
    pub confidence: f64,
    pub suggested_actions: Vec<AgentSuggestedAction>,
    pub artifacts: Vec<AgentReturnedArtifact>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentReturnStatus {
    Waiting,
    Completed,
    Failed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnEvidence {
    pub kind: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSuggestedAction {
    pub action_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnedArtifact {
    pub artifact_type: String,
    pub location: String,
}

impl AgentSpec {
    pub fn validate(&self) -> Result<(), crate::VelCoreError> {
        if self.id.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(
                "agent id must be set".to_string(),
            ));
        }
        if self.mission.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(
                "agent mission must be set".to_string(),
            ));
        }
        if self.allowed_tools.is_empty() {
            return Err(crate::VelCoreError::Validation(
                "agent must allow at least one tool".to_string(),
            ));
        }
        if self.return_contract.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(
                "agent return_contract must be set".to_string(),
            ));
        }
        if self.ttl_seconds == 0 {
            return Err(crate::VelCoreError::Validation(
                "agent ttl_seconds must be greater than zero".to_string(),
            ));
        }
        if self.memory_scope.event_query.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(
                "agent memory_scope.event_query must be set".to_string(),
            ));
        }
        if !self.budgets.validate() {
            return Err(crate::VelCoreError::Validation(
                "agent budgets are invalid".to_string(),
            ));
        }

        let mut seen = HashSet::new();
        for tool in &self.allowed_tools {
            if tool.trim().is_empty() || !seen.insert(tool) {
                return Err(crate::VelCoreError::Validation(
                    "agent allowed_tools must be unique and non-empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl AgentBudgets {
    pub fn validate(&self) -> bool {
        self.max_tool_calls > 0 && self.max_tokens > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunKind {
    Search,
    ContextGeneration,
    ArtifactExtraction,
    Synthesis,
    Agent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunRetryPolicy {
    pub automatic_retry_supported: bool,
    pub automatic_retry_reason: Option<&'static str>,
}

impl Display for RunKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Search => "search",
            Self::ContextGeneration => "context_generation",
            Self::ArtifactExtraction => "artifact_extraction",
            Self::Synthesis => "synthesis",
            Self::Agent => "agent",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RunKind {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "search" => Ok(Self::Search),
            "context_generation" => Ok(Self::ContextGeneration),
            "artifact_extraction" => Ok(Self::ArtifactExtraction),
            "synthesis" => Ok(Self::Synthesis),
            "agent" => Ok(Self::Agent),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown run kind: {}",
                s
            ))),
        }
    }
}

impl RunKind {
    pub fn retry_policy(self) -> RunRetryPolicy {
        match self {
            Self::ContextGeneration | Self::Synthesis => RunRetryPolicy {
                automatic_retry_supported: true,
                automatic_retry_reason: Some("worker can re-execute the original run input"),
            },
            Self::Search => RunRetryPolicy {
                automatic_retry_supported: false,
                automatic_retry_reason: Some("search runs do not have an automatic retry executor"),
            },
            Self::ArtifactExtraction => RunRetryPolicy {
                automatic_retry_supported: false,
                automatic_retry_reason: Some(
                    "artifact extraction does not yet have a background retry executor",
                ),
            },
            Self::Agent => RunRetryPolicy {
                automatic_retry_supported: false,
                automatic_retry_reason: Some(
                    "agent runs do not yet have a background retry executor",
                ),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Waiting,
    Succeeded,
    Failed,
    Cancelled,
    /// Reserved for future use (e.g. retry-after-failure workflows).
    RetryScheduled,
    /// Reserved for future use (e.g. blocked on dependency).
    Blocked,
    /// Reserved for future use (e.g. runtime TTL expiry).
    Expired,
}

impl Display for RunStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Waiting => "waiting",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::RetryScheduled => "retry_scheduled",
            Self::Blocked => "blocked",
            Self::Expired => "expired",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RunStatus {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "waiting" => Ok(Self::Waiting),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "retry_scheduled" => Ok(Self::RetryScheduled),
            "blocked" => Ok(Self::Blocked),
            "expired" => Ok(Self::Expired),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown run status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: RunId,
    pub kind: RunKind,
    pub status: RunStatus,
    pub input_json: Value,
    pub output_json: Option<Value>,
    pub error_json: Option<Value>,
    pub created_at: OffsetDateTime,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
}

impl Run {
    /// Valid transition: Queued -> Running. Returns updated run.
    pub fn start(self, now: OffsetDateTime) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Queued {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot start run in status {}",
                self.status
            )));
        }
        Ok(Run {
            started_at: Some(now),
            status: RunStatus::Running,
            ..self
        })
    }

    pub fn wait(self, now: OffsetDateTime) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Running {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot wait for run in status {}",
                self.status
            )));
        }
        Ok(Run {
            status: RunStatus::Waiting,
            started_at: self.started_at.or(Some(now)),
            ..self
        })
    }

    pub fn resume(self) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Waiting {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot resume run in status {}",
                self.status
            )));
        }
        Ok(Run {
            status: RunStatus::Running,
            ..self
        })
    }

    pub fn complete(self, now: OffsetDateTime, output: Value) -> Result<Self, crate::VelCoreError> {
        self.succeed(now, output)
    }

    pub fn expire(self, now: OffsetDateTime) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Running && self.status != RunStatus::Waiting {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot expire run in status {}",
                self.status
            )));
        }
        Ok(Run {
            finished_at: Some(now),
            status: RunStatus::Expired,
            output_json: self.output_json,
            error_json: self.error_json,
            ..self
        })
    }

    /// Valid transition: Running -> Succeeded. Returns updated run.
    pub fn succeed(self, now: OffsetDateTime, output: Value) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Running {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot succeed run in status {}",
                self.status
            )));
        }
        Ok(Run {
            finished_at: Some(now),
            output_json: Some(output),
            error_json: None,
            status: RunStatus::Succeeded,
            ..self
        })
    }

    /// Valid transition: Queued | Running -> Failed. Returns updated run.
    pub fn fail(self, now: OffsetDateTime, error: Value) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Queued && self.status != RunStatus::Running {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot fail run in status {}",
                self.status
            )));
        }
        Ok(Run {
            finished_at: Some(now),
            error_json: Some(error),
            output_json: None,
            status: RunStatus::Failed,
            ..self
        })
    }

    /// Valid transition: Queued | Running -> Cancelled. Returns updated run.
    pub fn cancel(self, now: OffsetDateTime) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Queued && self.status != RunStatus::Running {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot cancel run in status {}",
                self.status
            )));
        }
        Ok(Run {
            finished_at: Some(now),
            status: RunStatus::Cancelled,
            ..self
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunEventType {
    RunCreated,
    RunStarted,
    RunSucceeded,
    RunFailed,
    RunCancelled,
    RunRetryScheduled,
    RunRequeued,
    RunRetryBlocked,
    ArtifactWritten,
    SearchExecuted,
    ContextGenerated,
    RefsCreated,
}

impl Display for RunEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::RunCreated => "run_created",
            Self::RunStarted => "run_started",
            Self::RunSucceeded => "run_succeeded",
            Self::RunFailed => "run_failed",
            Self::RunCancelled => "run_cancelled",
            Self::RunRetryScheduled => "run_retry_scheduled",
            Self::RunRequeued => "run_requeued",
            Self::RunRetryBlocked => "run_retry_blocked",
            Self::ArtifactWritten => "artifact_written",
            Self::SearchExecuted => "search_executed",
            Self::ContextGenerated => "context_generated",
            Self::RefsCreated => "refs_created",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RunEventType {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "run_created" => Ok(Self::RunCreated),
            "run_started" => Ok(Self::RunStarted),
            "run_succeeded" => Ok(Self::RunSucceeded),
            "run_failed" => Ok(Self::RunFailed),
            "run_cancelled" => Ok(Self::RunCancelled),
            "run_retry_scheduled" => Ok(Self::RunRetryScheduled),
            "run_requeued" => Ok(Self::RunRequeued),
            "run_retry_blocked" => Ok(Self::RunRetryBlocked),
            "artifact_written" => Ok(Self::ArtifactWritten),
            "search_executed" => Ok(Self::SearchExecuted),
            "context_generated" => Ok(Self::ContextGenerated),
            "refs_created" => Ok(Self::RefsCreated),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown run event type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEvent {
    pub id: String,
    pub run_id: RunId,
    pub seq: u32,
    pub event_type: RunEventType,
    pub payload_json: Value,
    pub created_at: OffsetDateTime,
}
