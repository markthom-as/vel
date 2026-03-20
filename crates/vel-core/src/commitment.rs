//! Commitment: actionable, reviewable, statusful object (distinct from raw capture).

use crate::CanonicalScheduleRules;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitmentId(pub(crate) String);

impl CommitmentId {
    pub fn new() -> Self {
        Self(format!("com_{}", Uuid::new_v4().simple()))
    }
}

impl Default for CommitmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CommitmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for CommitmentId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for CommitmentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentStatus {
    Open,
    Done,
    Cancelled,
}

impl Display for CommitmentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Open => "open",
            Self::Done => "done",
            Self::Cancelled => "cancelled",
        })
    }
}

impl std::str::FromStr for CommitmentStatus {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(Self::Open),
            "done" => Ok(Self::Done),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown commitment status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
    pub id: CommitmentId,
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: CommitmentStatus,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub created_at: OffsetDateTime,
    pub resolved_at: Option<OffsetDateTime>,
    pub metadata_json: serde_json::Value,
}

impl Commitment {
    pub fn scheduler_rules(&self) -> CanonicalScheduleRules {
        CanonicalScheduleRules::from_commitment_parts(&self.text, &self.metadata_json, self.due_at)
    }
}
