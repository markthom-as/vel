use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    IntegrationConnectionId, IntegrationFamily, IntegrationSourceRef, ProjectId, VelCoreError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WritebackOperationId(pub(crate) String);

impl WritebackOperationId {
    pub fn new() -> Self {
        Self(format!("wb_{}", Uuid::new_v4().simple()))
    }
}

impl Default for WritebackOperationId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for WritebackOperationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for WritebackOperationId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for WritebackOperationId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WritebackRisk {
    #[default]
    Safe,
    ConfirmRequired,
    Blocked,
}

impl Display for WritebackRisk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Safe => "safe",
            Self::ConfirmRequired => "confirm_required",
            Self::Blocked => "blocked",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for WritebackRisk {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "safe" => Ok(Self::Safe),
            "confirm_required" => Ok(Self::ConfirmRequired),
            "blocked" => Ok(Self::Blocked),
            _ => Err(VelCoreError::Validation(format!(
                "unknown writeback risk: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackStatus {
    Queued,
    InProgress,
    Applied,
    Conflicted,
    Denied,
    Failed,
    Cancelled,
}

impl Display for WritebackStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Queued => "queued",
            Self::InProgress => "in_progress",
            Self::Applied => "applied",
            Self::Conflicted => "conflicted",
            Self::Denied => "denied",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for WritebackStatus {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "queued" => Ok(Self::Queued),
            "in_progress" => Ok(Self::InProgress),
            "applied" => Ok(Self::Applied),
            "conflicted" => Ok(Self::Conflicted),
            "denied" => Ok(Self::Denied),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(VelCoreError::Validation(format!(
                "unknown writeback status: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackOperationKind {
    TodoistCreateTask,
    TodoistUpdateTask,
    TodoistCompleteTask,
    TodoistReopenTask,
    NotesCreateNote,
    NotesAppendNote,
    RemindersCreate,
    RemindersUpdate,
    RemindersComplete,
    GithubCreateIssue,
    GithubAddComment,
    GithubCloseIssue,
    GithubReopenIssue,
    EmailCreateDraftReply,
    EmailSendDraft,
}

impl Display for WritebackOperationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::TodoistCreateTask => "todoist_create_task",
            Self::TodoistUpdateTask => "todoist_update_task",
            Self::TodoistCompleteTask => "todoist_complete_task",
            Self::TodoistReopenTask => "todoist_reopen_task",
            Self::NotesCreateNote => "notes_create_note",
            Self::NotesAppendNote => "notes_append_note",
            Self::RemindersCreate => "reminders_create",
            Self::RemindersUpdate => "reminders_update",
            Self::RemindersComplete => "reminders_complete",
            Self::GithubCreateIssue => "github_create_issue",
            Self::GithubAddComment => "github_add_comment",
            Self::GithubCloseIssue => "github_close_issue",
            Self::GithubReopenIssue => "github_reopen_issue",
            Self::EmailCreateDraftReply => "email_create_draft_reply",
            Self::EmailSendDraft => "email_send_draft",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for WritebackOperationKind {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "todoist_create_task" => Ok(Self::TodoistCreateTask),
            "todoist_update_task" => Ok(Self::TodoistUpdateTask),
            "todoist_complete_task" => Ok(Self::TodoistCompleteTask),
            "todoist_reopen_task" => Ok(Self::TodoistReopenTask),
            "notes_create_note" => Ok(Self::NotesCreateNote),
            "notes_append_note" => Ok(Self::NotesAppendNote),
            "reminders_create" => Ok(Self::RemindersCreate),
            "reminders_update" => Ok(Self::RemindersUpdate),
            "reminders_complete" => Ok(Self::RemindersComplete),
            "github_create_issue" => Ok(Self::GithubCreateIssue),
            "github_add_comment" => Ok(Self::GithubAddComment),
            "github_close_issue" => Ok(Self::GithubCloseIssue),
            "github_reopen_issue" => Ok(Self::GithubReopenIssue),
            "email_create_draft_reply" => Ok(Self::EmailCreateDraftReply),
            "email_send_draft" => Ok(Self::EmailSendDraft),
            _ => Err(VelCoreError::Validation(format!(
                "unknown writeback operation kind: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WritebackTargetRef {
    pub family: IntegrationFamily,
    pub provider_key: String,
    pub project_id: Option<ProjectId>,
    pub connection_id: Option<IntegrationConnectionId>,
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WritebackOperationRecord {
    pub id: WritebackOperationId,
    pub kind: WritebackOperationKind,
    pub risk: WritebackRisk,
    pub status: WritebackStatus,
    pub target: WritebackTargetRef,
    pub requested_payload: JsonValue,
    pub result_payload: Option<JsonValue>,
    #[serde(default)]
    pub provenance: Vec<IntegrationSourceRef>,
    pub conflict_case_id: Option<String>,
    pub requested_by_node_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub requested_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub applied_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::{WritebackOperationKind, WritebackOperationRecord, WritebackRisk};

    #[test]
    fn writeback_operation_example_parses() {
        let record: WritebackOperationRecord = serde_json::from_str(include_str!(
            "../../../config/examples/writeback-operation.example.json"
        ))
        .expect("writeback operation example should parse");

        assert_eq!(record.kind, WritebackOperationKind::TodoistCreateTask);
        assert_eq!(record.risk, WritebackRisk::Safe);
        assert_eq!(record.provenance.len(), 1);
    }
}
