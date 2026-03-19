use crate::{daily_loop::DailyLoopPhase, project::ProjectFamily, project::ProjectId};
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
    Recovery,
    Intervention,
    CheckIn,
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
            Self::Recovery => "recovery",
            Self::Intervention => "intervention",
            Self::CheckIn => "check_in",
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
pub enum ActionPermissionMode {
    AutoAllowed,
    UserConfirm,
    Blocked,
    Unavailable,
}

impl Display for ActionPermissionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::AutoAllowed => "auto_allowed",
            Self::UserConfirm => "user_confirm",
            Self::Blocked => "blocked",
            Self::Unavailable => "unavailable",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionScopeAffinity {
    Global,
    Project,
    Thread,
    Connector,
    DailyLoop,
}

impl Display for ActionScopeAffinity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Global => "global",
            Self::Project => "project",
            Self::Thread => "thread",
            Self::Connector => "connector",
            Self::DailyLoop => "daily_loop",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionThreadRouteTarget {
    ExistingThread,
    FilteredThreads,
}

impl Display for ActionThreadRouteTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::ExistingThread => "existing_thread",
            Self::FilteredThreads => "filtered_threads",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionThreadRoute {
    pub target: ActionThreadRouteTarget,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<ProjectId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInSourceKind {
    DailyLoop,
}

impl Display for CheckInSourceKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::DailyLoop => "daily_loop",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInSubmitTargetKind {
    DailyLoopTurn,
}

impl Display for CheckInSubmitTargetKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::DailyLoopTurn => "daily_loop_turn",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckInSubmitTarget {
    pub kind: CheckInSubmitTargetKind,
    pub reference_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInEscalationTarget {
    Threads,
}

impl Display for CheckInEscalationTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Threads => "threads",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckInEscalation {
    pub target: CheckInEscalationTarget,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInTransitionKind {
    Submit,
    Bypass,
    Escalate,
}

impl Display for CheckInTransitionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Submit => "submit",
            Self::Bypass => "bypass",
            Self::Escalate => "escalate",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInTransitionTargetKind {
    DailyLoopTurn,
    Threads,
}

impl Display for CheckInTransitionTargetKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::DailyLoopTurn => "daily_loop_turn",
            Self::Threads => "threads",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckInTransition {
    pub kind: CheckInTransitionKind,
    pub label: String,
    pub target: CheckInTransitionTargetKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub requires_response: bool,
    pub requires_note: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckInCard {
    pub id: ActionItemId,
    pub source_kind: CheckInSourceKind,
    pub phase: DailyLoopPhase,
    pub session_id: String,
    pub title: String,
    pub summary: String,
    pub prompt_id: String,
    pub prompt_text: String,
    pub suggested_action_label: Option<String>,
    pub suggested_response: Option<String>,
    pub allow_skip: bool,
    pub blocking: bool,
    pub submit_target: CheckInSubmitTarget,
    pub escalation: Option<CheckInEscalation>,
    #[serde(default)]
    pub transitions: Vec<CheckInTransition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTriggerKind {
    StaleSchedule,
    MissedEvent,
    SlippedPlannedBlock,
    MajorSyncChange,
    TaskNoLongerFits,
}

impl Display for ReflowTriggerKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::StaleSchedule => "stale_schedule",
            Self::MissedEvent => "missed_event",
            Self::SlippedPlannedBlock => "slipped_planned_block",
            Self::MajorSyncChange => "major_sync_change",
            Self::TaskNoLongerFits => "task_no_longer_fits",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowSeverity {
    Medium,
    High,
    Critical,
}

impl Display for ReflowSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowAcceptMode {
    DirectAccept,
    ConfirmRequired,
}

impl Display for ReflowAcceptMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::DirectAccept => "direct_accept",
            Self::ConfirmRequired => "confirm_required",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflowEditTarget {
    pub target: CheckInEscalationTarget,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTransitionKind {
    Accept,
    Edit,
}

impl Display for ReflowTransitionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Accept => "accept",
            Self::Edit => "edit",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTransitionTargetKind {
    ApplySuggestion,
    Threads,
}

impl Display for ReflowTransitionTargetKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::ApplySuggestion => "apply_suggestion",
            Self::Threads => "threads",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflowTransition {
    pub kind: ReflowTransitionKind,
    pub label: String,
    pub target: ReflowTransitionTargetKind,
    pub confirm_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflowCard {
    pub id: ActionItemId,
    pub title: String,
    pub summary: String,
    pub trigger: ReflowTriggerKind,
    pub severity: ReflowSeverity,
    pub accept_mode: ReflowAcceptMode,
    pub suggested_action_label: String,
    pub preview_lines: Vec<String>,
    pub edit_target: ReflowEditTarget,
    #[serde(default)]
    pub transitions: Vec<ReflowTransition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionItem {
    pub id: ActionItemId,
    pub surface: ActionSurface,
    pub kind: ActionKind,
    pub permission_mode: ActionPermissionMode,
    pub scope_affinity: ActionScopeAffinity,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub project_label: Option<String>,
    pub project_family: Option<ProjectFamily>,
    pub state: ActionState,
    pub rank: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub surfaced_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub snoozed_until: Option<OffsetDateTime>,
    #[serde(default)]
    pub evidence: Vec<ActionEvidenceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_route: Option<ActionThreadRoute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReviewSnapshot {
    #[serde(default)]
    pub open_action_count: u32,
    #[serde(default)]
    pub triage_count: u32,
    #[serde(default)]
    pub projects_needing_review: u32,
    #[serde(default)]
    pub pending_execution_reviews: u32,
}

#[cfg(test)]
mod tests {
    use super::{
        ActionItem, ActionKind, ActionPermissionMode, ActionScopeAffinity, ActionThreadRouteTarget,
        CheckInCard, CheckInEscalationTarget, CheckInSourceKind, CheckInSubmitTargetKind,
        CheckInTransitionKind, ReflowCard, ReflowTransitionKind,
    };

    #[test]
    fn operator_action_item_example_parses() {
        let item: ActionItem = serde_json::from_str(include_str!(
            "../../../config/examples/operator-action-item.example.json"
        ))
        .expect("operator action item example should parse");

        assert_eq!(item.kind, ActionKind::Intervention);
        assert_eq!(item.permission_mode, ActionPermissionMode::UserConfirm);
        assert_eq!(item.scope_affinity, ActionScopeAffinity::Project);
        assert_eq!(item.evidence.len(), 2);
        assert_eq!(
            item.thread_route.as_ref().map(|route| route.target),
            Some(ActionThreadRouteTarget::FilteredThreads)
        );
        assert_eq!(item.rank, 10);
    }

    #[test]
    fn check_in_card_round_trips_as_json() {
        let value = serde_json::json!({
            "id": "act_check_in_1",
            "source_kind": "daily_loop",
            "phase": "standup",
            "session_id": "dls_123",
            "title": "Standup check-in",
            "summary": "Vel needs one short answer before the standup can continue.",
            "prompt_id": "standup_prompt_1",
            "prompt_text": "Name the one to three commitments that matter most today.",
            "suggested_action_label": "Continue standup",
            "suggested_response": null,
            "allow_skip": true,
            "blocking": true,
            "submit_target": {
                "kind": "daily_loop_turn",
                "reference_id": "dls_123"
            },
            "escalation": {
                "target": "threads",
                "label": "Continue in Threads"
            },
            "transitions": [
                {
                    "kind": "submit",
                    "label": "Continue standup",
                    "target": "daily_loop_turn",
                    "reference_id": "dls_123",
                    "requires_response": true,
                    "requires_note": false
                },
                {
                    "kind": "bypass",
                    "label": "Skip for now",
                    "target": "daily_loop_turn",
                    "reference_id": "dls_123",
                    "requires_response": false,
                    "requires_note": true
                },
                {
                    "kind": "escalate",
                    "label": "Continue in Threads",
                    "target": "threads",
                    "reference_id": "dls_123",
                    "requires_response": false,
                    "requires_note": false
                }
            ]
        });

        let card: CheckInCard = serde_json::from_value(value).expect("check-in card should parse");

        assert_eq!(card.source_kind, CheckInSourceKind::DailyLoop);
        assert_eq!(
            card.submit_target.kind,
            CheckInSubmitTargetKind::DailyLoopTurn
        );
        assert_eq!(card.prompt_id, "standup_prompt_1");
        assert!(card.blocking);
        assert_eq!(card.transitions.len(), 3);
        assert_eq!(card.transitions[0].kind, CheckInTransitionKind::Submit);
        assert_eq!(
            card.escalation.as_ref().map(|value| &value.target),
            Some(&CheckInEscalationTarget::Threads)
        );
    }

    #[test]
    fn reflow_card_round_trips_as_json() {
        let value = serde_json::json!({
            "id": "act_reflow_1",
            "title": "Day changed",
            "summary": "A scheduled event appears to have slipped past without the plan being updated.",
            "trigger": "missed_event",
            "severity": "critical",
            "accept_mode": "confirm_required",
            "suggested_action_label": "Accept",
            "preview_lines": [
                "Next scheduled event started 20 minutes ago."
            ],
            "edit_target": {
                "target": "threads",
                "label": "Edit"
            },
            "transitions": [
                {
                    "kind": "accept",
                    "label": "Accept",
                    "target": "apply_suggestion",
                    "confirm_required": true
                },
                {
                    "kind": "edit",
                    "label": "Edit",
                    "target": "threads",
                    "confirm_required": false
                }
            ]
        });

        let card: ReflowCard = serde_json::from_value(value).expect("reflow card should parse");

        assert_eq!(card.transitions.len(), 2);
        assert_eq!(card.transitions[0].kind, ReflowTransitionKind::Accept);
        assert_eq!(card.transitions[1].kind, ReflowTransitionKind::Edit);
        assert_eq!(card.edit_target.target, CheckInEscalationTarget::Threads);
    }
}
