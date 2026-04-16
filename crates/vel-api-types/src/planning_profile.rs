use serde::{Deserialize, Serialize};

use crate::{
    AssistantProposalStateData, DurableRoutineBlockData, PlanningConstraintKindData, UnixSeconds,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleTimeWindowData {
    Prenoon,
    Afternoon,
    Evening,
    Night,
    Day,
}

impl From<vel_core::ScheduleTimeWindow> for ScheduleTimeWindowData {
    fn from(value: vel_core::ScheduleTimeWindow) -> Self {
        match value {
            vel_core::ScheduleTimeWindow::Prenoon => Self::Prenoon,
            vel_core::ScheduleTimeWindow::Afternoon => Self::Afternoon,
            vel_core::ScheduleTimeWindow::Evening => Self::Evening,
            vel_core::ScheduleTimeWindow::Night => Self::Night,
            vel_core::ScheduleTimeWindow::Day => Self::Day,
        }
    }
}

impl From<ScheduleTimeWindowData> for vel_core::ScheduleTimeWindow {
    fn from(value: ScheduleTimeWindowData) -> Self {
        match value {
            ScheduleTimeWindowData::Prenoon => Self::Prenoon,
            ScheduleTimeWindowData::Afternoon => Self::Afternoon,
            ScheduleTimeWindowData::Evening => Self::Evening,
            ScheduleTimeWindowData::Night => Self::Night,
            ScheduleTimeWindowData::Day => Self::Day,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningConstraintData {
    pub id: String,
    pub label: String,
    pub kind: PlanningConstraintKindData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<ScheduleTimeWindowData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u32>,
    #[serde(default)]
    pub active: bool,
}

impl From<vel_core::PlanningConstraint> for PlanningConstraintData {
    fn from(value: vel_core::PlanningConstraint) -> Self {
        Self {
            id: value.id,
            label: value.label,
            kind: value.kind.into(),
            detail: value.detail,
            time_window: value.time_window.map(Into::into),
            minutes: value.minutes,
            max_items: value.max_items,
            active: value.active,
        }
    }
}

impl From<PlanningConstraintData> for vel_core::PlanningConstraint {
    fn from(value: PlanningConstraintData) -> Self {
        Self {
            id: value.id,
            label: value.label,
            kind: value.kind.into(),
            detail: value.detail,
            time_window: value.time_window.map(Into::into),
            minutes: value.minutes,
            max_items: value.max_items,
            active: value.active,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutinePlanningProfileData {
    #[serde(default)]
    pub routine_blocks: Vec<DurableRoutineBlockData>,
    #[serde(default)]
    pub planning_constraints: Vec<PlanningConstraintData>,
}

impl From<vel_core::RoutinePlanningProfile> for RoutinePlanningProfileData {
    fn from(value: vel_core::RoutinePlanningProfile) -> Self {
        Self {
            routine_blocks: value.routine_blocks.into_iter().map(Into::into).collect(),
            planning_constraints: value
                .planning_constraints
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<RoutinePlanningProfileData> for vel_core::RoutinePlanningProfile {
    fn from(value: RoutinePlanningProfileData) -> Self {
        Self {
            routine_blocks: value.routine_blocks.into_iter().map(Into::into).collect(),
            planning_constraints: value
                .planning_constraints
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileRemoveTargetData {
    pub id: String,
}

impl From<vel_core::PlanningProfileRemoveTarget> for PlanningProfileRemoveTargetData {
    fn from(value: vel_core::PlanningProfileRemoveTarget) -> Self {
        Self { id: value.id }
    }
}

impl From<PlanningProfileRemoveTargetData> for vel_core::PlanningProfileRemoveTarget {
    fn from(value: PlanningProfileRemoveTargetData) -> Self {
        Self { id: value.id }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum PlanningProfileMutationData {
    UpsertRoutineBlock(DurableRoutineBlockData),
    RemoveRoutineBlock(PlanningProfileRemoveTargetData),
    UpsertPlanningConstraint(PlanningConstraintData),
    RemovePlanningConstraint(PlanningProfileRemoveTargetData),
}

impl From<vel_core::PlanningProfileMutation> for PlanningProfileMutationData {
    fn from(value: vel_core::PlanningProfileMutation) -> Self {
        match value {
            vel_core::PlanningProfileMutation::UpsertRoutineBlock(block) => {
                Self::UpsertRoutineBlock(block.into())
            }
            vel_core::PlanningProfileMutation::RemoveRoutineBlock(target) => {
                Self::RemoveRoutineBlock(target.into())
            }
            vel_core::PlanningProfileMutation::UpsertPlanningConstraint(constraint) => {
                Self::UpsertPlanningConstraint(constraint.into())
            }
            vel_core::PlanningProfileMutation::RemovePlanningConstraint(target) => {
                Self::RemovePlanningConstraint(target.into())
            }
        }
    }
}

impl From<PlanningProfileMutationData> for vel_core::PlanningProfileMutation {
    fn from(value: PlanningProfileMutationData) -> Self {
        match value {
            PlanningProfileMutationData::UpsertRoutineBlock(block) => {
                Self::UpsertRoutineBlock(block.into())
            }
            PlanningProfileMutationData::RemoveRoutineBlock(target) => {
                Self::RemoveRoutineBlock(target.into())
            }
            PlanningProfileMutationData::UpsertPlanningConstraint(constraint) => {
                Self::UpsertPlanningConstraint(constraint.into())
            }
            PlanningProfileMutationData::RemovePlanningConstraint(target) => {
                Self::RemovePlanningConstraint(target.into())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileMutationRequestData {
    pub mutation: PlanningProfileMutationData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileResponseData {
    pub profile: RoutinePlanningProfileData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposal_summary: Option<PlanningProfileProposalSummaryData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileProposalApplyResponseData {
    pub profile: RoutinePlanningProfileData,
    pub proposal: PlanningProfileEditProposalData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentSchedulingSourceKindData {
    DayPlan,
    Reflow,
}

impl From<vel_core::CommitmentSchedulingSourceKind> for CommitmentSchedulingSourceKindData {
    fn from(value: vel_core::CommitmentSchedulingSourceKind) -> Self {
        match value {
            vel_core::CommitmentSchedulingSourceKind::DayPlan => Self::DayPlan,
            vel_core::CommitmentSchedulingSourceKind::Reflow => Self::Reflow,
        }
    }
}

impl From<CommitmentSchedulingSourceKindData> for vel_core::CommitmentSchedulingSourceKind {
    fn from(value: CommitmentSchedulingSourceKindData) -> Self {
        match value {
            CommitmentSchedulingSourceKindData::DayPlan => Self::DayPlan,
            CommitmentSchedulingSourceKindData::Reflow => Self::Reflow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentSchedulingContinuityData {
    #[default]
    Inline,
    Thread,
}

impl From<vel_core::CommitmentSchedulingContinuity> for CommitmentSchedulingContinuityData {
    fn from(value: vel_core::CommitmentSchedulingContinuity) -> Self {
        match value {
            vel_core::CommitmentSchedulingContinuity::Inline => Self::Inline,
            vel_core::CommitmentSchedulingContinuity::Thread => Self::Thread,
        }
    }
}

impl From<CommitmentSchedulingContinuityData> for vel_core::CommitmentSchedulingContinuity {
    fn from(value: CommitmentSchedulingContinuityData) -> Self {
        match value {
            CommitmentSchedulingContinuityData::Inline => Self::Inline,
            CommitmentSchedulingContinuityData::Thread => Self::Thread,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentSchedulingMutationKindData {
    SetDueAt,
    ClearDueAt,
}

impl From<vel_core::CommitmentSchedulingMutationKind> for CommitmentSchedulingMutationKindData {
    fn from(value: vel_core::CommitmentSchedulingMutationKind) -> Self {
        match value {
            vel_core::CommitmentSchedulingMutationKind::SetDueAt => Self::SetDueAt,
            vel_core::CommitmentSchedulingMutationKind::ClearDueAt => Self::ClearDueAt,
        }
    }
}

impl From<CommitmentSchedulingMutationKindData> for vel_core::CommitmentSchedulingMutationKind {
    fn from(value: CommitmentSchedulingMutationKindData) -> Self {
        match value {
            CommitmentSchedulingMutationKindData::SetDueAt => Self::SetDueAt,
            CommitmentSchedulingMutationKindData::ClearDueAt => Self::ClearDueAt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentSchedulingMutationData {
    pub commitment_id: String,
    pub kind: CommitmentSchedulingMutationKindData,
    pub title: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_due_at_ts: Option<UnixSeconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_due_at_ts: Option<UnixSeconds>,
}

impl From<vel_core::CommitmentSchedulingMutation> for CommitmentSchedulingMutationData {
    fn from(value: vel_core::CommitmentSchedulingMutation) -> Self {
        Self {
            commitment_id: value.commitment_id,
            kind: value.kind.into(),
            title: value.title,
            summary: value.summary,
            project_label: value.project_label,
            previous_due_at_ts: value.previous_due_at_ts,
            next_due_at_ts: value.next_due_at_ts,
        }
    }
}

impl From<CommitmentSchedulingMutationData> for vel_core::CommitmentSchedulingMutation {
    fn from(value: CommitmentSchedulingMutationData) -> Self {
        Self {
            commitment_id: value.commitment_id,
            kind: value.kind.into(),
            title: value.title,
            summary: value.summary,
            project_label: value.project_label,
            previous_due_at_ts: value.previous_due_at_ts,
            next_due_at_ts: value.next_due_at_ts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentSchedulingProposalData {
    pub source_kind: CommitmentSchedulingSourceKindData,
    pub state: AssistantProposalStateData,
    pub summary: String,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default)]
    pub continuity: CommitmentSchedulingContinuityData,
    #[serde(default)]
    pub mutations: Vec<CommitmentSchedulingMutationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentSchedulingProposalApplyResponseData {
    pub proposal: CommitmentSchedulingProposalData,
}

impl From<vel_core::CommitmentSchedulingProposal> for CommitmentSchedulingProposalData {
    fn from(value: vel_core::CommitmentSchedulingProposal) -> Self {
        Self {
            source_kind: value.source_kind.into(),
            state: value.state.into(),
            summary: value.summary,
            requires_confirmation: value.requires_confirmation,
            continuity: value.continuity.into(),
            mutations: value.mutations.into_iter().map(Into::into).collect(),
            outcome_summary: value.outcome_summary,
            thread_id: value.thread_id,
            thread_type: value.thread_type,
        }
    }
}

impl From<CommitmentSchedulingProposalData> for vel_core::CommitmentSchedulingProposal {
    fn from(value: CommitmentSchedulingProposalData) -> Self {
        Self {
            source_kind: value.source_kind.into(),
            state: match value.state {
                AssistantProposalStateData::Staged => vel_core::AssistantProposalState::Staged,
                AssistantProposalStateData::Approved => vel_core::AssistantProposalState::Approved,
                AssistantProposalStateData::Applied => vel_core::AssistantProposalState::Applied,
                AssistantProposalStateData::Failed => vel_core::AssistantProposalState::Failed,
                AssistantProposalStateData::Reversed => vel_core::AssistantProposalState::Reversed,
            },
            summary: value.summary,
            requires_confirmation: value.requires_confirmation,
            continuity: value.continuity.into(),
            mutations: value.mutations.into_iter().map(Into::into).collect(),
            outcome_summary: value.outcome_summary,
            thread_id: value.thread_id,
            thread_type: value.thread_type,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningProfileSurfaceData {
    WebSettings,
    Cli,
    Apple,
    Assistant,
    Voice,
}

impl From<vel_core::PlanningProfileSurface> for PlanningProfileSurfaceData {
    fn from(value: vel_core::PlanningProfileSurface) -> Self {
        match value {
            vel_core::PlanningProfileSurface::WebSettings => Self::WebSettings,
            vel_core::PlanningProfileSurface::Cli => Self::Cli,
            vel_core::PlanningProfileSurface::Apple => Self::Apple,
            vel_core::PlanningProfileSurface::Assistant => Self::Assistant,
            vel_core::PlanningProfileSurface::Voice => Self::Voice,
        }
    }
}

impl From<PlanningProfileSurfaceData> for vel_core::PlanningProfileSurface {
    fn from(value: PlanningProfileSurfaceData) -> Self {
        match value {
            PlanningProfileSurfaceData::WebSettings => Self::WebSettings,
            PlanningProfileSurfaceData::Cli => Self::Cli,
            PlanningProfileSurfaceData::Apple => Self::Apple,
            PlanningProfileSurfaceData::Assistant => Self::Assistant,
            PlanningProfileSurfaceData::Voice => Self::Voice,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PlanningProfileContinuityData {
    #[default]
    Inline,
    Thread,
}

impl From<vel_core::PlanningProfileContinuity> for PlanningProfileContinuityData {
    fn from(value: vel_core::PlanningProfileContinuity) -> Self {
        match value {
            vel_core::PlanningProfileContinuity::Inline => Self::Inline,
            vel_core::PlanningProfileContinuity::Thread => Self::Thread,
        }
    }
}

impl From<PlanningProfileContinuityData> for vel_core::PlanningProfileContinuity {
    fn from(value: PlanningProfileContinuityData) -> Self {
        match value {
            PlanningProfileContinuityData::Inline => Self::Inline,
            PlanningProfileContinuityData::Thread => Self::Thread,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileEditProposalData {
    pub source_surface: PlanningProfileSurfaceData,
    pub state: AssistantProposalStateData,
    pub mutation: PlanningProfileMutationData,
    pub summary: String,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default)]
    pub continuity: PlanningProfileContinuityData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileProposalSummaryItemData {
    pub thread_id: String,
    pub state: AssistantProposalStateData,
    pub title: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_summary: Option<String>,
    pub updated_at: UnixSeconds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileProposalSummaryData {
    pub pending_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_pending: Option<PlanningProfileProposalSummaryItemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_applied: Option<PlanningProfileProposalSummaryItemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_failed: Option<PlanningProfileProposalSummaryItemData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentSchedulingProposalSummaryItemData {
    pub thread_id: String,
    pub state: AssistantProposalStateData,
    pub title: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_summary: Option<String>,
    pub updated_at: UnixSeconds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentSchedulingProposalSummaryData {
    pub pending_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_pending: Option<CommitmentSchedulingProposalSummaryItemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_applied: Option<CommitmentSchedulingProposalSummaryItemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_failed: Option<CommitmentSchedulingProposalSummaryItemData>,
}

impl From<vel_core::PlanningProfileEditProposal> for PlanningProfileEditProposalData {
    fn from(value: vel_core::PlanningProfileEditProposal) -> Self {
        Self {
            source_surface: value.source_surface.into(),
            state: value.state.into(),
            mutation: value.mutation.into(),
            summary: value.summary,
            requires_confirmation: value.requires_confirmation,
            continuity: value.continuity.into(),
            outcome_summary: value.outcome_summary,
            thread_id: value.thread_id,
            thread_type: value.thread_type,
        }
    }
}

impl From<PlanningProfileEditProposalData> for vel_core::PlanningProfileEditProposal {
    fn from(value: PlanningProfileEditProposalData) -> Self {
        Self {
            source_surface: value.source_surface.into(),
            state: match value.state {
                AssistantProposalStateData::Staged => vel_core::AssistantProposalState::Staged,
                AssistantProposalStateData::Approved => vel_core::AssistantProposalState::Approved,
                AssistantProposalStateData::Applied => vel_core::AssistantProposalState::Applied,
                AssistantProposalStateData::Failed => vel_core::AssistantProposalState::Failed,
                AssistantProposalStateData::Reversed => vel_core::AssistantProposalState::Reversed,
            },
            mutation: value.mutation.into(),
            summary: value.summary,
            requires_confirmation: value.requires_confirmation,
            continuity: value.continuity.into(),
            outcome_summary: value.outcome_summary,
            thread_id: value.thread_id,
            thread_type: value.thread_type,
        }
    }
}
