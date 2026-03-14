//! Run model: first-class execution records for context generation, synthesis, etc.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;
use serde_json::Value;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunKind {
    Search,
    ContextGeneration,
    ArtifactExtraction,
    Synthesis,
    Agent,
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
            _ => Err(crate::VelCoreError::Validation(format!("unknown run kind: {}", s))),
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
            "retry_scheduled" => Ok(Self::RetryScheduled),
            "blocked" => Ok(Self::Blocked),
            _ => Err(crate::VelCoreError::Validation(format!("unknown run status: {}", s))),
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
            "artifact_written" => Ok(Self::ArtifactWritten),
            "search_executed" => Ok(Self::SearchExecuted),
            "context_generated" => Ok(Self::ContextGenerated),
            "refs_created" => Ok(Self::RefsCreated),
            _ => Err(crate::VelCoreError::Validation(format!("unknown run event type: {}", s))),
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
