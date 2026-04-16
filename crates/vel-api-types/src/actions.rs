use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_core::{ActionItemId, ProjectId};

use crate::ProjectFamilyData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionSurfaceData {
    Now,
    Inbox,
}

impl From<vel_core::ActionSurface> for ActionSurfaceData {
    fn from(value: vel_core::ActionSurface) -> Self {
        match value {
            vel_core::ActionSurface::Now => Self::Now,
            vel_core::ActionSurface::Inbox => Self::Inbox,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKindData {
    NextStep,
    Recovery,
    Intervention,
    CheckIn,
    Review,
    Freshness,
    Blocked,
    Conflict,
    Linking,
}

impl From<vel_core::ActionKind> for ActionKindData {
    fn from(value: vel_core::ActionKind) -> Self {
        match value {
            vel_core::ActionKind::NextStep => Self::NextStep,
            vel_core::ActionKind::Recovery => Self::Recovery,
            vel_core::ActionKind::Intervention => Self::Intervention,
            vel_core::ActionKind::CheckIn => Self::CheckIn,
            vel_core::ActionKind::Review => Self::Review,
            vel_core::ActionKind::Freshness => Self::Freshness,
            vel_core::ActionKind::Blocked => Self::Blocked,
            vel_core::ActionKind::Conflict => Self::Conflict,
            vel_core::ActionKind::Linking => Self::Linking,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPermissionModeData {
    AutoAllowed,
    UserConfirm,
    Blocked,
    Unavailable,
}

impl From<vel_core::ActionPermissionMode> for ActionPermissionModeData {
    fn from(value: vel_core::ActionPermissionMode) -> Self {
        match value {
            vel_core::ActionPermissionMode::AutoAllowed => Self::AutoAllowed,
            vel_core::ActionPermissionMode::UserConfirm => Self::UserConfirm,
            vel_core::ActionPermissionMode::Blocked => Self::Blocked,
            vel_core::ActionPermissionMode::Unavailable => Self::Unavailable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionScopeAffinityData {
    Global,
    Project,
    Thread,
    Connector,
    DailyLoop,
}

impl From<vel_core::ActionScopeAffinity> for ActionScopeAffinityData {
    fn from(value: vel_core::ActionScopeAffinity) -> Self {
        match value {
            vel_core::ActionScopeAffinity::Global => Self::Global,
            vel_core::ActionScopeAffinity::Project => Self::Project,
            vel_core::ActionScopeAffinity::Thread => Self::Thread,
            vel_core::ActionScopeAffinity::Connector => Self::Connector,
            vel_core::ActionScopeAffinity::DailyLoop => Self::DailyLoop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStateData {
    Active,
    Acknowledged,
    Resolved,
    Dismissed,
    Snoozed,
}

impl From<vel_core::ActionState> for ActionStateData {
    fn from(value: vel_core::ActionState) -> Self {
        match value {
            vel_core::ActionState::Active => Self::Active,
            vel_core::ActionState::Acknowledged => Self::Acknowledged,
            vel_core::ActionState::Resolved => Self::Resolved,
            vel_core::ActionState::Dismissed => Self::Dismissed,
            vel_core::ActionState::Snoozed => Self::Snoozed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailableActionData {
    Acknowledge,
    Resolve,
    Dismiss,
    Snooze,
    OpenThread,
    OpenProject,
    SyncNow,
    LinkNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEvidenceRefData {
    pub source_kind: String,
    pub source_id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl From<vel_core::ActionEvidenceRef> for ActionEvidenceRefData {
    fn from(value: vel_core::ActionEvidenceRef) -> Self {
        Self {
            source_kind: value.source_kind,
            source_id: value.source_id,
            label: value.label,
            detail: value.detail,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionThreadRouteTargetData {
    ExistingThread,
    FilteredThreads,
}

impl From<vel_core::ActionThreadRouteTarget> for ActionThreadRouteTargetData {
    fn from(value: vel_core::ActionThreadRouteTarget) -> Self {
        match value {
            vel_core::ActionThreadRouteTarget::ExistingThread => Self::ExistingThread,
            vel_core::ActionThreadRouteTarget::FilteredThreads => Self::FilteredThreads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionThreadRouteData {
    pub target: ActionThreadRouteTargetData,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<ProjectId>,
}

impl From<vel_core::ActionThreadRoute> for ActionThreadRouteData {
    fn from(value: vel_core::ActionThreadRoute) -> Self {
        Self {
            target: value.target.into(),
            label: value.label,
            thread_id: value.thread_id,
            thread_type: value.thread_type,
            project_id: value.project_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItemData {
    pub id: ActionItemId,
    pub surface: ActionSurfaceData,
    pub kind: ActionKindData,
    pub permission_mode: ActionPermissionModeData,
    pub scope_affinity: ActionScopeAffinityData,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub project_label: Option<String>,
    pub project_family: Option<ProjectFamilyData>,
    pub state: ActionStateData,
    pub rank: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub surfaced_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub snoozed_until: Option<OffsetDateTime>,
    #[serde(default)]
    pub evidence: Vec<ActionEvidenceRefData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_route: Option<ActionThreadRouteData>,
}

impl From<vel_core::ActionItem> for ActionItemData {
    fn from(value: vel_core::ActionItem) -> Self {
        Self {
            id: value.id,
            surface: value.surface.into(),
            kind: value.kind.into(),
            permission_mode: value.permission_mode.into(),
            scope_affinity: value.scope_affinity.into(),
            title: value.title,
            summary: value.summary,
            project_id: value.project_id,
            project_label: value.project_label,
            project_family: value.project_family.map(Into::into),
            state: value.state.into(),
            rank: value.rank,
            surfaced_at: value.surfaced_at,
            snoozed_until: value.snoozed_until,
            evidence: value.evidence.into_iter().map(Into::into).collect(),
            thread_route: value.thread_route.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn action_item_timestamps_serialize_as_rfc3339_strings() {
        let item = ActionItemData {
            id: ActionItemId::from("act_1".to_string()),
            surface: ActionSurfaceData::Now,
            kind: ActionKindData::NextStep,
            permission_mode: ActionPermissionModeData::UserConfirm,
            scope_affinity: ActionScopeAffinityData::Global,
            title: "Ship patch".to_string(),
            summary: "Due soon".to_string(),
            project_id: None,
            project_label: None,
            project_family: None,
            state: ActionStateData::Active,
            rank: 70,
            surfaced_at: datetime!(2026-03-19 02:10:00 UTC),
            snoozed_until: Some(datetime!(2026-03-19 02:20:00 UTC)),
            evidence: vec![ActionEvidenceRefData {
                source_kind: "commitment".to_string(),
                source_id: "com_1".to_string(),
                label: "Ship patch".to_string(),
                detail: None,
            }],
            thread_route: Some(ActionThreadRouteData {
                target: ActionThreadRouteTargetData::FilteredThreads,
                label: "Open related threads".to_string(),
                thread_id: None,
                thread_type: Some("action_resolution".to_string()),
                project_id: Some(ProjectId::from("proj_1".to_string())),
            }),
        };

        let value = serde_json::to_value(item).expect("action item should serialize");
        assert_eq!(value["surfaced_at"], "2026-03-19T02:10:00Z");
        assert_eq!(value["snoozed_until"], "2026-03-19T02:20:00Z");
        assert_eq!(value["permission_mode"], "user_confirm");
        assert_eq!(value["scope_affinity"], "global");
        assert_eq!(value["thread_route"]["target"], "filtered_threads");
        assert_eq!(value["thread_route"]["thread_type"], "action_resolution");
    }
}
