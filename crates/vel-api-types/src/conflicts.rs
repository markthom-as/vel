use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::ConflictCaseId;

use crate::WritebackTargetRefData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCaseKindData {
    UpstreamVsLocal,
    CrossClient,
    StaleWrite,
    ExecutorUnavailable,
}

impl From<vel_core::ConflictCaseKind> for ConflictCaseKindData {
    fn from(value: vel_core::ConflictCaseKind) -> Self {
        match value {
            vel_core::ConflictCaseKind::UpstreamVsLocal => Self::UpstreamVsLocal,
            vel_core::ConflictCaseKind::CrossClient => Self::CrossClient,
            vel_core::ConflictCaseKind::StaleWrite => Self::StaleWrite,
            vel_core::ConflictCaseKind::ExecutorUnavailable => Self::ExecutorUnavailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCaseStatusData {
    Open,
    Acknowledged,
    Resolved,
    Dismissed,
    Expired,
}

impl From<vel_core::ConflictCaseStatus> for ConflictCaseStatusData {
    fn from(value: vel_core::ConflictCaseStatus) -> Self {
        match value {
            vel_core::ConflictCaseStatus::Open => Self::Open,
            vel_core::ConflictCaseStatus::Acknowledged => Self::Acknowledged,
            vel_core::ConflictCaseStatus::Resolved => Self::Resolved,
            vel_core::ConflictCaseStatus::Dismissed => Self::Dismissed,
            vel_core::ConflictCaseStatus::Expired => Self::Expired,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictCaseData {
    pub id: ConflictCaseId,
    pub kind: ConflictCaseKindData,
    pub status: ConflictCaseStatusData,
    pub target: WritebackTargetRefData,
    pub summary: String,
    pub local_payload: JsonValue,
    pub upstream_payload: Option<JsonValue>,
    pub resolution_payload: Option<JsonValue>,
    #[serde(with = "time::serde::rfc3339")]
    pub opened_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub resolved_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<vel_core::ConflictCaseRecord> for ConflictCaseData {
    fn from(value: vel_core::ConflictCaseRecord) -> Self {
        Self {
            id: value.id,
            kind: value.kind.into(),
            status: value.status.into(),
            target: value.target.into(),
            summary: value.summary,
            local_payload: value.local_payload,
            upstream_payload: value.upstream_payload,
            resolution_payload: value.resolution_payload,
            opened_at: value.opened_at,
            resolved_at: value.resolved_at,
            updated_at: value.updated_at,
        }
    }
}
