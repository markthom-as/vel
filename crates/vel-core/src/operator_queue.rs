use crate::project::ProjectId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionItemId(pub(crate) String);

impl ActionItemId {
    pub fn new() -> Self {
        Self(format!("act_{}", Uuid::new_v4().simple()))
    }
}

impl Default for ActionItemId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ActionItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ActionItemId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ActionItemId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionSurface {
    Now,
    Inbox,
}

impl Display for ActionSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Now => "now",
            Self::Inbox => "inbox",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    NextStep,
    Intervention,
    Review,
    Freshness,
    Blocked,
    Conflict,
    Linking,
}

impl Display for ActionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::NextStep => "next_step",
            Self::Intervention => "intervention",
            Self::Review => "review",
            Self::Freshness => "freshness",
            Self::Blocked => "blocked",
            Self::Conflict => "conflict",
            Self::Linking => "linking",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionState {
    Active,
    Acknowledged,
    Resolved,
    Dismissed,
    Snoozed,
}

impl Display for ActionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Active => "active",
            Self::Acknowledged => "acknowledged",
            Self::Resolved => "resolved",
            Self::Dismissed => "dismissed",
            Self::Snoozed => "snoozed",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionEvidenceRef {
    pub source_kind: String,
    pub source_id: String,
    pub label: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionItem {
    pub id: ActionItemId,
    pub surface: ActionSurface,
    pub kind: ActionKind,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub state: ActionState,
    pub rank: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub surfaced_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub snoozed_until: Option<OffsetDateTime>,
    #[serde(default)]
    pub evidence: Vec<ActionEvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReviewSnapshot {
    #[serde(default)]
    pub open_action_count: u32,
    #[serde(default)]
    pub triage_count: u32,
    #[serde(default)]
    pub projects_needing_review: u32,
}

#[cfg(test)]
mod tests {
    use super::{ActionItem, ActionKind};

    #[test]
    fn operator_action_item_example_parses() {
        let item: ActionItem = serde_json::from_str(include_str!(
            "../../../config/examples/operator-action-item.example.json"
        ))
        .expect("operator action item example should parse");

        assert_eq!(item.kind, ActionKind::Intervention);
        assert_eq!(item.evidence.len(), 2);
        assert_eq!(item.rank, 10);
    }
}
