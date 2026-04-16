use serde::{Deserialize, Serialize};
use vel_core::ProjectId;

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
