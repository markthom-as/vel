use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{writeback::WritebackTargetRef, VelCoreError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembraneConflictKind {
    StaleVersion,
    OwnershipConflict,
    PendingReconciliation,
    ProviderDivergence,
    TombstoneWriteRace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MembraneConflict {
    pub kind: MembraneConflictKind,
    pub field: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConflictCaseId(pub(crate) String);

impl ConflictCaseId {
    pub fn new() -> Self {
        Self(format!("conf_{}", Uuid::new_v4().simple()))
    }
}

impl Default for ConflictCaseId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ConflictCaseId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ConflictCaseId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ConflictCaseId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCaseKind {
    UpstreamVsLocal,
    CrossClient,
    StaleWrite,
    ExecutorUnavailable,
}

impl Display for ConflictCaseKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::UpstreamVsLocal => "upstream_vs_local",
            Self::CrossClient => "cross_client",
            Self::StaleWrite => "stale_write",
            Self::ExecutorUnavailable => "executor_unavailable",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for ConflictCaseKind {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "upstream_vs_local" => Ok(Self::UpstreamVsLocal),
            "cross_client" => Ok(Self::CrossClient),
            "stale_write" => Ok(Self::StaleWrite),
            "executor_unavailable" => Ok(Self::ExecutorUnavailable),
            _ => Err(VelCoreError::Validation(format!(
                "unknown conflict case kind: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCaseStatus {
    #[default]
    Open,
    Acknowledged,
    Resolved,
    Dismissed,
    Expired,
}

impl Display for ConflictCaseStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Open => "open",
            Self::Acknowledged => "acknowledged",
            Self::Resolved => "resolved",
            Self::Dismissed => "dismissed",
            Self::Expired => "expired",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for ConflictCaseStatus {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "open" => Ok(Self::Open),
            "acknowledged" => Ok(Self::Acknowledged),
            "resolved" => Ok(Self::Resolved),
            "dismissed" => Ok(Self::Dismissed),
            "expired" => Ok(Self::Expired),
            _ => Err(VelCoreError::Validation(format!(
                "unknown conflict case status: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictCaseRecord {
    pub id: ConflictCaseId,
    pub kind: ConflictCaseKind,
    pub status: ConflictCaseStatus,
    pub target: WritebackTargetRef,
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

#[cfg(test)]
mod tests {
    use super::{MembraneConflict, MembraneConflictKind};

    #[test]
    fn membrane_conflicts_keep_stale_and_ownership_states_distinct() {
        let stale = MembraneConflict {
            kind: MembraneConflictKind::StaleVersion,
            field: Some("revision".to_string()),
            reason: "revision mismatch".to_string(),
        };
        let ownership = MembraneConflict {
            kind: MembraneConflictKind::OwnershipConflict,
            field: Some("due".to_string()),
            reason: "provider owns due".to_string(),
        };

        assert_ne!(stale.kind, ownership.kind);
    }
}
