use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{IntegrationConnectionId, ProjectId, WritebackOperationId};

use crate::{IntegrationFamilyData, IntegrationSourceRefData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritebackTargetRefData {
    pub family: IntegrationFamilyData,
    pub provider_key: String,
    pub project_id: Option<ProjectId>,
    pub connection_id: Option<IntegrationConnectionId>,
    pub external_id: Option<String>,
}

impl From<vel_core::WritebackTargetRef> for WritebackTargetRefData {
    fn from(value: vel_core::WritebackTargetRef) -> Self {
        Self {
            family: value.family.into(),
            provider_key: value.provider_key,
            project_id: value.project_id,
            connection_id: value.connection_id,
            external_id: value.external_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackRiskData {
    Safe,
    ConfirmRequired,
    Blocked,
}

impl From<vel_core::WritebackRisk> for WritebackRiskData {
    fn from(value: vel_core::WritebackRisk) -> Self {
        match value {
            vel_core::WritebackRisk::Safe => Self::Safe,
            vel_core::WritebackRisk::ConfirmRequired => Self::ConfirmRequired,
            vel_core::WritebackRisk::Blocked => Self::Blocked,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackStatusData {
    Queued,
    InProgress,
    Applied,
    Conflicted,
    Denied,
    Failed,
    Cancelled,
}

impl From<vel_core::WritebackStatus> for WritebackStatusData {
    fn from(value: vel_core::WritebackStatus) -> Self {
        match value {
            vel_core::WritebackStatus::Queued => Self::Queued,
            vel_core::WritebackStatus::InProgress => Self::InProgress,
            vel_core::WritebackStatus::Applied => Self::Applied,
            vel_core::WritebackStatus::Conflicted => Self::Conflicted,
            vel_core::WritebackStatus::Denied => Self::Denied,
            vel_core::WritebackStatus::Failed => Self::Failed,
            vel_core::WritebackStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritebackOperationKindData {
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

impl From<vel_core::WritebackOperationKind> for WritebackOperationKindData {
    fn from(value: vel_core::WritebackOperationKind) -> Self {
        match value {
            vel_core::WritebackOperationKind::TodoistCreateTask => Self::TodoistCreateTask,
            vel_core::WritebackOperationKind::TodoistUpdateTask => Self::TodoistUpdateTask,
            vel_core::WritebackOperationKind::TodoistCompleteTask => Self::TodoistCompleteTask,
            vel_core::WritebackOperationKind::TodoistReopenTask => Self::TodoistReopenTask,
            vel_core::WritebackOperationKind::NotesCreateNote => Self::NotesCreateNote,
            vel_core::WritebackOperationKind::NotesAppendNote => Self::NotesAppendNote,
            vel_core::WritebackOperationKind::RemindersCreate => Self::RemindersCreate,
            vel_core::WritebackOperationKind::RemindersUpdate => Self::RemindersUpdate,
            vel_core::WritebackOperationKind::RemindersComplete => Self::RemindersComplete,
            vel_core::WritebackOperationKind::GithubCreateIssue => Self::GithubCreateIssue,
            vel_core::WritebackOperationKind::GithubAddComment => Self::GithubAddComment,
            vel_core::WritebackOperationKind::GithubCloseIssue => Self::GithubCloseIssue,
            vel_core::WritebackOperationKind::GithubReopenIssue => Self::GithubReopenIssue,
            vel_core::WritebackOperationKind::EmailCreateDraftReply => Self::EmailCreateDraftReply,
            vel_core::WritebackOperationKind::EmailSendDraft => Self::EmailSendDraft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritebackOperationData {
    pub id: WritebackOperationId,
    pub kind: WritebackOperationKindData,
    pub risk: WritebackRiskData,
    pub status: WritebackStatusData,
    pub target: WritebackTargetRefData,
    pub requested_payload: JsonValue,
    pub result_payload: Option<JsonValue>,
    #[serde(default)]
    pub provenance: Vec<IntegrationSourceRefData>,
    pub conflict_case_id: Option<String>,
    pub requested_by_node_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub requested_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub applied_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<vel_core::WritebackOperationRecord> for WritebackOperationData {
    fn from(value: vel_core::WritebackOperationRecord) -> Self {
        Self {
            id: value.id,
            kind: value.kind.into(),
            risk: value.risk.into(),
            status: value.status.into(),
            target: value.target.into(),
            requested_payload: value.requested_payload,
            result_payload: value.result_payload,
            provenance: value.provenance.into_iter().map(Into::into).collect(),
            conflict_case_id: value.conflict_case_id,
            requested_by_node_id: value.requested_by_node_id,
            requested_at: value.requested_at,
            applied_at: value.applied_at,
            updated_at: value.updated_at,
        }
    }
}
