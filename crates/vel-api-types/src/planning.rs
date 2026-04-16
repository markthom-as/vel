use serde::{Deserialize, Serialize};
use vel_core::ActionItemId;

use crate::{CheckInEscalationTargetData, UnixSeconds};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTriggerKindData {
    StaleSchedule,
    MissedEvent,
    SlippedPlannedBlock,
    MajorSyncChange,
    TaskNoLongerFits,
}

impl From<vel_core::ReflowTriggerKind> for ReflowTriggerKindData {
    fn from(value: vel_core::ReflowTriggerKind) -> Self {
        match value {
            vel_core::ReflowTriggerKind::StaleSchedule => Self::StaleSchedule,
            vel_core::ReflowTriggerKind::MissedEvent => Self::MissedEvent,
            vel_core::ReflowTriggerKind::SlippedPlannedBlock => Self::SlippedPlannedBlock,
            vel_core::ReflowTriggerKind::MajorSyncChange => Self::MajorSyncChange,
            vel_core::ReflowTriggerKind::TaskNoLongerFits => Self::TaskNoLongerFits,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowSeverityData {
    Medium,
    High,
    Critical,
}

impl From<vel_core::ReflowSeverity> for ReflowSeverityData {
    fn from(value: vel_core::ReflowSeverity) -> Self {
        match value {
            vel_core::ReflowSeverity::Medium => Self::Medium,
            vel_core::ReflowSeverity::High => Self::High,
            vel_core::ReflowSeverity::Critical => Self::Critical,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowAcceptModeData {
    DirectAccept,
    ConfirmRequired,
}

impl From<vel_core::ReflowAcceptMode> for ReflowAcceptModeData {
    fn from(value: vel_core::ReflowAcceptMode) -> Self {
        match value {
            vel_core::ReflowAcceptMode::DirectAccept => Self::DirectAccept,
            vel_core::ReflowAcceptMode::ConfirmRequired => Self::ConfirmRequired,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTransitionKindData {
    Accept,
    Edit,
}

impl From<vel_core::ReflowTransitionKind> for ReflowTransitionKindData {
    fn from(value: vel_core::ReflowTransitionKind) -> Self {
        match value {
            vel_core::ReflowTransitionKind::Accept => Self::Accept,
            vel_core::ReflowTransitionKind::Edit => Self::Edit,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowTransitionTargetKindData {
    ApplySuggestion,
    Threads,
}

impl From<vel_core::ReflowTransitionTargetKind> for ReflowTransitionTargetKindData {
    fn from(value: vel_core::ReflowTransitionTargetKind) -> Self {
        match value {
            vel_core::ReflowTransitionTargetKind::ApplySuggestion => Self::ApplySuggestion,
            vel_core::ReflowTransitionTargetKind::Threads => Self::Threads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowTransitionData {
    pub kind: ReflowTransitionKindData,
    pub label: String,
    pub target: ReflowTransitionTargetKindData,
    pub confirm_required: bool,
}

impl From<vel_core::ReflowTransition> for ReflowTransitionData {
    fn from(value: vel_core::ReflowTransition) -> Self {
        Self {
            kind: value.kind.into(),
            label: value.label,
            target: value.target.into(),
            confirm_required: value.confirm_required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReflowChangeKindData {
    Moved,
    Unscheduled,
    NeedsJudgment,
}

impl From<vel_core::ReflowChangeKind> for ReflowChangeKindData {
    fn from(value: vel_core::ReflowChangeKind) -> Self {
        match value {
            vel_core::ReflowChangeKind::Moved => Self::Moved,
            vel_core::ReflowChangeKind::Unscheduled => Self::Unscheduled,
            vel_core::ReflowChangeKind::NeedsJudgment => Self::NeedsJudgment,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleRuleFacetKindData {
    BlockTarget,
    Duration,
    CalendarFree,
    FixedStart,
    TimeWindow,
    LocalUrgency,
    LocalDefer,
}

impl From<vel_core::ScheduleRuleFacetKind> for ScheduleRuleFacetKindData {
    fn from(value: vel_core::ScheduleRuleFacetKind) -> Self {
        match value {
            vel_core::ScheduleRuleFacetKind::BlockTarget => Self::BlockTarget,
            vel_core::ScheduleRuleFacetKind::Duration => Self::Duration,
            vel_core::ScheduleRuleFacetKind::CalendarFree => Self::CalendarFree,
            vel_core::ScheduleRuleFacetKind::FixedStart => Self::FixedStart,
            vel_core::ScheduleRuleFacetKind::TimeWindow => Self::TimeWindow,
            vel_core::ScheduleRuleFacetKind::LocalUrgency => Self::LocalUrgency,
            vel_core::ScheduleRuleFacetKind::LocalDefer => Self::LocalDefer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleRuleFacetData {
    pub kind: ScheduleRuleFacetKindData,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl From<vel_core::ScheduleRuleFacet> for ScheduleRuleFacetData {
    fn from(value: vel_core::ScheduleRuleFacet) -> Self {
        Self {
            kind: value.kind.into(),
            label: value.label,
            detail: value.detail,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowChangeData {
    pub kind: ReflowChangeKindData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_id: Option<String>,
    pub title: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_ts: Option<UnixSeconds>,
}

impl From<vel_core::ReflowChange> for ReflowChangeData {
    fn from(value: vel_core::ReflowChange) -> Self {
        Self {
            kind: value.kind.into(),
            commitment_id: value.commitment_id,
            title: value.title,
            detail: value.detail,
            project_label: value.project_label,
            scheduled_start_ts: value.scheduled_start_ts,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowProposalData {
    pub headline: String,
    pub summary: String,
    pub moved_count: u32,
    pub unscheduled_count: u32,
    pub needs_judgment_count: u32,
    #[serde(default)]
    pub changes: Vec<ReflowChangeData>,
    #[serde(default)]
    pub rule_facets: Vec<ScheduleRuleFacetData>,
}

impl From<vel_core::ReflowProposal> for ReflowProposalData {
    fn from(value: vel_core::ReflowProposal) -> Self {
        Self {
            headline: value.headline,
            summary: value.summary,
            moved_count: value.moved_count,
            unscheduled_count: value.unscheduled_count,
            needs_judgment_count: value.needs_judgment_count,
            changes: value.changes.into_iter().map(Into::into).collect(),
            rule_facets: value.rule_facets.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayPlanChangeKindData {
    Scheduled,
    Deferred,
    DidNotFit,
    NeedsJudgment,
}

impl From<vel_core::DayPlanChangeKind> for DayPlanChangeKindData {
    fn from(value: vel_core::DayPlanChangeKind) -> Self {
        match value {
            vel_core::DayPlanChangeKind::Scheduled => Self::Scheduled,
            vel_core::DayPlanChangeKind::Deferred => Self::Deferred,
            vel_core::DayPlanChangeKind::DidNotFit => Self::DidNotFit,
            vel_core::DayPlanChangeKind::NeedsJudgment => Self::NeedsJudgment,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutineBlockSourceKindData {
    OperatorDeclared,
    Inferred,
    Imported,
}

impl From<vel_core::RoutineBlockSourceKind> for RoutineBlockSourceKindData {
    fn from(value: vel_core::RoutineBlockSourceKind) -> Self {
        match value {
            vel_core::RoutineBlockSourceKind::OperatorDeclared => Self::OperatorDeclared,
            vel_core::RoutineBlockSourceKind::Inferred => Self::Inferred,
            vel_core::RoutineBlockSourceKind::Imported => Self::Imported,
        }
    }
}

impl From<RoutineBlockSourceKindData> for vel_core::RoutineBlockSourceKind {
    fn from(value: RoutineBlockSourceKindData) -> Self {
        match value {
            RoutineBlockSourceKindData::OperatorDeclared => Self::OperatorDeclared,
            RoutineBlockSourceKindData::Inferred => Self::Inferred,
            RoutineBlockSourceKindData::Imported => Self::Imported,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineBlockData {
    pub id: String,
    pub label: String,
    pub source: RoutineBlockSourceKindData,
    pub start_ts: UnixSeconds,
    pub end_ts: UnixSeconds,
    pub protected: bool,
}

impl From<vel_core::RoutineBlock> for RoutineBlockData {
    fn from(value: vel_core::RoutineBlock) -> Self {
        Self {
            id: value.id,
            label: value.label,
            source: value.source.into(),
            start_ts: value.start_ts,
            end_ts: value.end_ts,
            protected: value.protected,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableRoutineBlockData {
    pub id: String,
    pub label: String,
    pub source: RoutineBlockSourceKindData,
    pub local_timezone: String,
    pub start_local_time: String,
    pub end_local_time: String,
    #[serde(default)]
    pub days_of_week: Vec<u8>,
    #[serde(default)]
    pub protected: bool,
    #[serde(default)]
    pub active: bool,
}

impl From<vel_core::DurableRoutineBlock> for DurableRoutineBlockData {
    fn from(value: vel_core::DurableRoutineBlock) -> Self {
        Self {
            id: value.id,
            label: value.label,
            source: value.source.into(),
            local_timezone: value.local_timezone,
            start_local_time: value.start_local_time,
            end_local_time: value.end_local_time,
            days_of_week: value.days_of_week,
            protected: value.protected,
            active: value.active,
        }
    }
}

impl From<DurableRoutineBlockData> for vel_core::DurableRoutineBlock {
    fn from(value: DurableRoutineBlockData) -> Self {
        Self {
            id: value.id,
            label: value.label,
            source: value.source.into(),
            local_timezone: value.local_timezone,
            start_local_time: value.start_local_time,
            end_local_time: value.end_local_time,
            days_of_week: value.days_of_week,
            protected: value.protected,
            active: value.active,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningConstraintKindData {
    MaxScheduledItems,
    ReserveBufferBeforeCalendar,
    ReserveBufferAfterCalendar,
    DefaultTimeWindow,
    RequireJudgmentForOverflow,
}

impl From<vel_core::PlanningConstraintKind> for PlanningConstraintKindData {
    fn from(value: vel_core::PlanningConstraintKind) -> Self {
        match value {
            vel_core::PlanningConstraintKind::MaxScheduledItems => Self::MaxScheduledItems,
            vel_core::PlanningConstraintKind::ReserveBufferBeforeCalendar => {
                Self::ReserveBufferBeforeCalendar
            }
            vel_core::PlanningConstraintKind::ReserveBufferAfterCalendar => {
                Self::ReserveBufferAfterCalendar
            }
            vel_core::PlanningConstraintKind::DefaultTimeWindow => Self::DefaultTimeWindow,
            vel_core::PlanningConstraintKind::RequireJudgmentForOverflow => {
                Self::RequireJudgmentForOverflow
            }
        }
    }
}

impl From<PlanningConstraintKindData> for vel_core::PlanningConstraintKind {
    fn from(value: PlanningConstraintKindData) -> Self {
        match value {
            PlanningConstraintKindData::MaxScheduledItems => Self::MaxScheduledItems,
            PlanningConstraintKindData::ReserveBufferBeforeCalendar => {
                Self::ReserveBufferBeforeCalendar
            }
            PlanningConstraintKindData::ReserveBufferAfterCalendar => {
                Self::ReserveBufferAfterCalendar
            }
            PlanningConstraintKindData::DefaultTimeWindow => Self::DefaultTimeWindow,
            PlanningConstraintKindData::RequireJudgmentForOverflow => {
                Self::RequireJudgmentForOverflow
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlanChangeData {
    pub kind: DayPlanChangeKindData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_id: Option<String>,
    pub title: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_ts: Option<UnixSeconds>,
    #[serde(default)]
    pub rule_facets: Vec<ScheduleRuleFacetData>,
}

impl From<vel_core::DayPlanChange> for DayPlanChangeData {
    fn from(value: vel_core::DayPlanChange) -> Self {
        Self {
            kind: value.kind.into(),
            commitment_id: value.commitment_id,
            title: value.title,
            detail: value.detail,
            project_label: value.project_label,
            scheduled_start_ts: value.scheduled_start_ts,
            rule_facets: value.rule_facets.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlanProposalData {
    pub headline: String,
    pub summary: String,
    pub scheduled_count: u32,
    pub deferred_count: u32,
    pub did_not_fit_count: u32,
    pub needs_judgment_count: u32,
    #[serde(default)]
    pub changes: Vec<DayPlanChangeData>,
    #[serde(default)]
    pub routine_blocks: Vec<RoutineBlockData>,
}

impl From<vel_core::DayPlanProposal> for DayPlanProposalData {
    fn from(value: vel_core::DayPlanProposal) -> Self {
        Self {
            headline: value.headline,
            summary: value.summary,
            scheduled_count: value.scheduled_count,
            deferred_count: value.deferred_count,
            did_not_fit_count: value.did_not_fit_count,
            needs_judgment_count: value.needs_judgment_count,
            changes: value.changes.into_iter().map(Into::into).collect(),
            routine_blocks: value.routine_blocks.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowEditTargetData {
    pub target: CheckInEscalationTargetData,
    pub label: String,
}

impl From<vel_core::ReflowEditTarget> for ReflowEditTargetData {
    fn from(value: vel_core::ReflowEditTarget) -> Self {
        Self {
            target: value.target.into(),
            label: value.label,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflowCardData {
    pub id: ActionItemId,
    pub title: String,
    pub summary: String,
    pub trigger: ReflowTriggerKindData,
    pub severity: ReflowSeverityData,
    pub accept_mode: ReflowAcceptModeData,
    pub suggested_action_label: String,
    #[serde(default)]
    pub preview_lines: Vec<String>,
    pub edit_target: ReflowEditTargetData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal: Option<ReflowProposalData>,
    #[serde(default)]
    pub transitions: Vec<ReflowTransitionData>,
}

impl From<vel_core::ReflowCard> for ReflowCardData {
    fn from(value: vel_core::ReflowCard) -> Self {
        Self {
            id: value.id,
            title: value.title,
            summary: value.summary,
            trigger: value.trigger.into(),
            severity: value.severity.into(),
            accept_mode: value.accept_mode.into(),
            suggested_action_label: value.suggested_action_label,
            preview_lines: value.preview_lines,
            edit_target: value.edit_target.into(),
            proposal: value.proposal.map(Into::into),
            transitions: value.transitions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentContextReflowStatusKindData {
    Applied,
    Editing,
}

impl From<vel_core::CurrentContextReflowStatusKind> for CurrentContextReflowStatusKindData {
    fn from(value: vel_core::CurrentContextReflowStatusKind) -> Self {
        match value {
            vel_core::CurrentContextReflowStatusKind::Applied => Self::Applied,
            vel_core::CurrentContextReflowStatusKind::Editing => Self::Editing,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContextReflowStatusData {
    pub kind: CurrentContextReflowStatusKindData,
    pub trigger: ReflowTriggerKindData,
    pub severity: ReflowSeverityData,
    pub headline: String,
    pub detail: String,
    pub recorded_at: UnixSeconds,
    #[serde(default)]
    pub preview_lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl From<vel_core::CurrentContextReflowStatus> for CurrentContextReflowStatusData {
    fn from(value: vel_core::CurrentContextReflowStatus) -> Self {
        Self {
            kind: value.kind.into(),
            trigger: value.trigger.into(),
            severity: value.severity.into(),
            headline: value.headline,
            detail: value.detail,
            recorded_at: value.recorded_at,
            preview_lines: value.preview_lines,
            thread_id: value.thread_id,
        }
    }
}
