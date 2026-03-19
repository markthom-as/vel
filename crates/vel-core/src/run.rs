//! Run model: first-class execution records for context generation, synthesis, etc.

use crate::{
    AgentProfile, ExecutionReviewGate, ExecutionTaskKind, ProjectId, RepoWorktreeRef,
    TokenBudgetClass,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId(pub(crate) String);

impl TraceId {
    pub fn new() -> Self {
        Self(format!("trace_{}", Uuid::new_v4().simple()))
    }
}

impl Default for TraceId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for TraceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for TraceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for TraceId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceLink {
    pub trace_id: TraceId,
    #[serde(default)]
    pub parent_run_id: Option<RunId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandoffEnvelope {
    pub task_id: String,
    pub trace_id: TraceId,
    pub from_agent: String,
    pub to_agent: String,
    pub objective: String,
    #[serde(default)]
    pub inputs: Value,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub project_id: Option<ProjectId>,
    #[serde(default)]
    pub task_kind: Option<ExecutionTaskKind>,
    #[serde(default)]
    pub agent_profile: Option<AgentProfile>,
    #[serde(default)]
    pub token_budget: Option<TokenBudgetClass>,
    #[serde(default)]
    pub review_gate: Option<ExecutionReviewGate>,
    #[serde(default)]
    pub repo_root: Option<RepoWorktreeRef>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub capability_scope: Value,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default)]
    pub expected_output_schema: Value,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentKind {
    Subagent,
    Supervisor,
    Specialist,
    Custom(String),
}

impl Serialize for AgentKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Self::Subagent => "subagent",
            Self::Supervisor => "supervisor",
            Self::Specialist => "specialist",
            Self::Custom(value) => value.as_str(),
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for AgentKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "subagent" => Self::Subagent,
            "supervisor" => Self::Supervisor,
            "specialist" => Self::Specialist,
            _ => Self::Custom(value),
        })
    }
}

impl Display for AgentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Subagent => "subagent",
            Self::Supervisor => "supervisor",
            Self::Specialist => "specialist",
            Self::Custom(value) => value,
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for AgentKind {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "subagent" => Self::Subagent,
            "supervisor" => Self::Supervisor,
            "specialist" => Self::Specialist,
            _ => Self::Custom(s.to_string()),
        };
        Ok(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryScope {
    pub constitution: bool,
    #[serde(default)]
    pub topic_pads: Vec<String>,
    #[serde(default)]
    pub event_query: Option<String>,
}

impl AgentMemoryScope {
    pub fn validate(&self) -> Result<(), crate::VelCoreError> {
        if !self.constitution && self.topic_pads.is_empty() && self.event_query.is_none() {
            return Err(crate::VelCoreError::Validation(
                "memory scope must include at least one access mode".to_string(),
            ));
        }
        if self.topic_pads.iter().any(|topic| topic.trim().is_empty()) {
            return Err(crate::VelCoreError::Validation(
                "memory scope topic pad must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBudgets {
    #[serde(default)]
    pub max_tool_calls: Option<u32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub max_memory_queries: Option<u32>,
    #[serde(default)]
    pub max_side_effects: Option<u32>,
}

impl AgentBudgets {
    pub fn validate(&self) -> Result<(), crate::VelCoreError> {
        if let Some(max_tool_calls) = self.max_tool_calls {
            if max_tool_calls == 0 {
                return Err(crate::VelCoreError::Validation(
                    "max_tool_calls must be greater than zero".to_string(),
                ));
            }
        }
        if let Some(max_tokens) = self.max_tokens {
            if max_tokens == 0 {
                return Err(crate::VelCoreError::Validation(
                    "max_tokens must be greater than zero".to_string(),
                ));
            }
        }
        if let Some(max_memory_queries) = self.max_memory_queries {
            if max_memory_queries == 0 {
                return Err(crate::VelCoreError::Validation(
                    "max_memory_queries must be greater than zero".to_string(),
                ));
            }
        }
        if let Some(max_side_effects) = self.max_side_effects {
            if max_side_effects == 0 {
                return Err(crate::VelCoreError::Validation(
                    "max_side_effects must be greater than zero".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub id: String,
    pub kind: AgentKind,
    pub mission: String,
    pub allowed_tools: Vec<String>,
    pub memory_scope: AgentMemoryScope,
    pub return_contract: String,
    pub ttl_seconds: u32,
    #[serde(default)]
    pub budgets: Option<AgentBudgets>,
    #[serde(default)]
    pub side_effect_policy: Option<String>,
}

impl AgentSpec {
    pub fn validate(&self) -> Result<(), crate::VelCoreError> {
        if self.id.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(
                "agent spec id is required".to_string(),
            ));
        }
        if self.mission.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(format!(
                "agent spec {} mission is required",
                self.id
            )));
        }
        if self.allowed_tools.is_empty() {
            return Err(crate::VelCoreError::Validation(format!(
                "agent spec {} requires at least one allowed tool",
                self.id
            )));
        }
        if self.allowed_tools.iter().any(|tool| tool.trim().is_empty()) {
            return Err(crate::VelCoreError::Validation(format!(
                "agent spec {} has empty allowed_tool entry",
                self.id
            )));
        }
        if self.return_contract.trim().is_empty() {
            return Err(crate::VelCoreError::Validation(format!(
                "agent spec {} return_contract is required",
                self.id
            )));
        }
        if self.ttl_seconds == 0 {
            return Err(crate::VelCoreError::Validation(format!(
                "agent spec {} ttl_seconds must be greater than zero",
                self.id
            )));
        }
        if let Some(budgets) = &self.budgets {
            budgets.validate()?;
        }
        self.memory_scope.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AgentPriority {
    Low,
    #[default]
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpawnRequest {
    pub agent_id: String,
    pub mission_input: Value,
    #[serde(default)]
    pub parent_run_id: Option<RunId>,
    #[serde(default)]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default)]
    pub priority: AgentPriority,
    #[serde(default)]
    pub requested_tools: Option<Vec<String>>,
    #[serde(default)]
    pub budgets: Option<AgentBudgets>,
}

impl AgentSpawnRequest {
    pub fn validate_for_spec(&self, spec: &AgentSpec) -> Result<(), crate::VelCoreError> {
        if self.agent_id != spec.id {
            return Err(crate::VelCoreError::Validation(format!(
                "agent_id {} does not match spec {}",
                self.agent_id, spec.id
            )));
        }
        if !matches!(
            self.mission_input,
            Value::Object(_) | Value::Array(_) | Value::String(_)
        ) {
            return Err(crate::VelCoreError::Validation(
                "mission_input must be structured JSON".to_string(),
            ));
        }
        if let Some(requested_tools) = &self.requested_tools {
            let allowed: HashSet<&str> = spec.allowed_tools.iter().map(String::as_str).collect();
            for tool in requested_tools {
                if !allowed.contains(tool.as_str()) {
                    return Err(crate::VelCoreError::Validation(format!(
                        "tool {} is not allowed by spec {}",
                        tool, spec.id
                    )));
                }
            }
        }
        if let Some(requested_budgets) = &self.budgets {
            requested_budgets.validate()?;
            if let Some(spec_budgets) = &spec.budgets {
                if let (Some(max_tool_calls), Some(spec_max)) = (
                    requested_budgets.max_tool_calls,
                    spec_budgets.max_tool_calls,
                ) {
                    if max_tool_calls > spec_max {
                        return Err(crate::VelCoreError::Validation(
                            "requested max_tool_calls exceeds spec budget".to_string(),
                        ));
                    }
                }
                if let (Some(max_tokens), Some(spec_max)) =
                    (requested_budgets.max_tokens, spec_budgets.max_tokens)
                {
                    if max_tokens > spec_max {
                        return Err(crate::VelCoreError::Validation(
                            "requested max_tokens exceeds spec budget".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentReturnStatus {
    Completed,
    Error,
    Blocked,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnEvidence {
    pub kind: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSuggestedAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnedArtifact {
    pub artifact_type: String,
    pub location: String,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReturnContract {
    pub status: AgentReturnStatus,
    pub summary: String,
    #[serde(default)]
    pub evidence: Vec<AgentReturnEvidence>,
    pub confidence: f64,
    #[serde(default)]
    pub suggested_actions: Vec<AgentSuggestedAction>,
    #[serde(default)]
    pub artifacts: Vec<AgentReturnedArtifact>,
    #[serde(default)]
    pub errors: Vec<Value>,
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
    Succeeded,
    Failed,
    Cancelled,
    /// Blocked waiting on tools, approvals, or external dependency.
    Waiting,
    /// Run exceeded ttl_seconds and timed out.
    Expired,
    /// Reserved for future use (e.g. retry-after-failure workflows).
    RetryScheduled,
    /// Reserved for future use (e.g. blocked on dependency).
    Blocked,
}

impl Display for RunStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Waiting => "waiting",
            Self::Expired => "expired",
            Self::RetryScheduled => "retry_scheduled",
            Self::Blocked => "blocked",
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
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "waiting" => Ok(Self::Waiting),
            "expired" => Ok(Self::Expired),
            "retry_scheduled" => Ok(Self::RetryScheduled),
            "blocked" => Ok(Self::Blocked),
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

    /// Valid transition: Running -> Waiting.
    pub fn wait(self) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Running {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot set run to waiting from status {}",
                self.status
            )));
        }
        Ok(Run {
            status: RunStatus::Waiting,
            ..self
        })
    }

    /// Valid transition: Waiting -> Running.
    pub fn resume(self) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Waiting {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot resume run from status {}",
                self.status
            )));
        }
        Ok(Run {
            status: RunStatus::Running,
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

    /// Alias for API-facing completed status.
    pub fn complete(self, now: OffsetDateTime, output: Value) -> Result<Self, crate::VelCoreError> {
        self.succeed(now, output)
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

    /// Valid transition: Running -> Expired.
    pub fn expire(self, now: OffsetDateTime) -> Result<Self, crate::VelCoreError> {
        if self.status != RunStatus::Running {
            return Err(crate::VelCoreError::InvalidTransition(format!(
                "cannot expire run in status {}",
                self.status
            )));
        }
        Ok(Run {
            finished_at: Some(now),
            status: RunStatus::Expired,
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
    SandboxCallEvaluated,
    SandboxRunCompleted,
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
            Self::SandboxCallEvaluated => "sandbox_call_evaluated",
            Self::SandboxRunCompleted => "sandbox_run_completed",
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
            "sandbox_call_evaluated" => Ok(Self::SandboxCallEvaluated),
            "sandbox_run_completed" => Ok(Self::SandboxRunCompleted),
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

#[cfg(test)]
mod tests {
    use super::{HandoffEnvelope, TraceId, TraceLink};
    use crate::{
        AgentProfile, ExecutionReviewGate, ExecutionTaskKind, ProjectId, RepoWorktreeRef,
        TokenBudgetClass,
    };
    use serde_json::json;
    use time::OffsetDateTime;

    #[test]
    fn trace_link_serializes_parent_run_id() {
        let link = TraceLink {
            trace_id: TraceId::from("trace_demo".to_string()),
            parent_run_id: Some("run_parent".to_string().into()),
        };

        let json = serde_json::to_value(link).expect("trace link should serialize");
        assert_eq!(json["trace_id"], "trace_demo");
        assert_eq!(json["parent_run_id"], "run_parent");
    }

    #[test]
    fn handoff_envelope_round_trips_with_trace_id() {
        let deadline = OffsetDateTime::from_unix_timestamp(1_742_273_600).unwrap();
        let envelope = HandoffEnvelope {
            task_id: "task_1".to_string(),
            trace_id: TraceId::from("trace_1".to_string()),
            from_agent: "planner".to_string(),
            to_agent: "risk_evaluator".to_string(),
            objective: "Evaluate the next step".to_string(),
            inputs: json!({ "run_id": "run_1" }),
            constraints: vec!["stay deterministic".to_string()],
            read_scopes: vec!["docs/".to_string()],
            write_scopes: vec![".planning/".to_string()],
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
            allowed_tools: vec!["search".to_string()],
            capability_scope: json!({ "mode": "read_only" }),
            deadline: Some(deadline),
            expected_output_schema: json!({ "type": "object" }),
        };

        let value = serde_json::to_value(&envelope).expect("handoff envelope should serialize");
        assert_eq!(value["trace_id"], "trace_1");
        let decoded: HandoffEnvelope =
            serde_json::from_value(value).expect("handoff envelope should deserialize");
        assert_eq!(decoded.trace_id.as_ref(), "trace_1");
        assert_eq!(decoded.to_agent, "risk_evaluator");
        assert_eq!(
            decoded.project_id,
            Some(ProjectId::from("proj_velruntime".to_string()))
        );
    }
}
