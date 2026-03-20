use crate::{
    operator_queue::{AssistantProposalState, RoutineBlockSourceKind},
    scheduler::ScheduleTimeWindow,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableRoutineBlock {
    pub id: String,
    pub label: String,
    pub source: RoutineBlockSourceKind,
    pub local_timezone: String,
    pub start_local_time: String,
    pub end_local_time: String,
    #[serde(default)]
    pub days_of_week: Vec<u8>,
    #[serde(default)]
    pub protected: bool,
    #[serde(default = "default_true")]
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningConstraintKind {
    MaxScheduledItems,
    ReserveBufferBeforeCalendar,
    ReserveBufferAfterCalendar,
    DefaultTimeWindow,
    RequireJudgmentForOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningConstraint {
    pub id: String,
    pub label: String,
    pub kind: PlanningConstraintKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<ScheduleTimeWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u32>,
    #[serde(default = "default_true")]
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutinePlanningProfile {
    #[serde(default)]
    pub routine_blocks: Vec<DurableRoutineBlock>,
    #[serde(default)]
    pub planning_constraints: Vec<PlanningConstraint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningProfileSurface {
    WebSettings,
    Cli,
    Apple,
    Assistant,
    Voice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PlanningProfileContinuity {
    #[default]
    Inline,
    Thread,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningProfileMutationKind {
    UpsertRoutineBlock,
    RemoveRoutineBlock,
    UpsertPlanningConstraint,
    RemovePlanningConstraint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileRemoveTarget {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum PlanningProfileMutation {
    UpsertRoutineBlock(DurableRoutineBlock),
    RemoveRoutineBlock(PlanningProfileRemoveTarget),
    UpsertPlanningConstraint(PlanningConstraint),
    RemovePlanningConstraint(PlanningProfileRemoveTarget),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProfileEditProposal {
    pub source_surface: PlanningProfileSurface,
    pub state: AssistantProposalState,
    pub mutation: PlanningProfileMutation,
    pub summary: String,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default = "default_inline_continuity")]
    pub continuity: PlanningProfileContinuity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_type: Option<String>,
}

impl PlanningProfileMutation {
    pub fn kind(&self) -> PlanningProfileMutationKind {
        match self {
            Self::UpsertRoutineBlock(_) => PlanningProfileMutationKind::UpsertRoutineBlock,
            Self::RemoveRoutineBlock(_) => PlanningProfileMutationKind::RemoveRoutineBlock,
            Self::UpsertPlanningConstraint(_) => {
                PlanningProfileMutationKind::UpsertPlanningConstraint
            }
            Self::RemovePlanningConstraint(_) => {
                PlanningProfileMutationKind::RemovePlanningConstraint
            }
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_inline_continuity() -> PlanningProfileContinuity {
    PlanningProfileContinuity::default()
}

#[cfg(test)]
mod tests {
    use super::{
        PlanningConstraintKind, PlanningProfileContinuity, PlanningProfileEditProposal,
        PlanningProfileMutation, PlanningProfileMutationKind, PlanningProfileSurface,
        RoutinePlanningProfile,
    };
    use crate::AssistantProposalState;

    #[test]
    fn durable_routine_planning_profile_example_parses() {
        let profile: RoutinePlanningProfile = serde_json::from_str(include_str!(
            "../../../config/examples/routine-planning-profile.example.json"
        ))
        .expect("routine planning profile example should parse");

        assert_eq!(profile.routine_blocks.len(), 2);
        assert_eq!(profile.planning_constraints.len(), 3);
        assert!(profile.routine_blocks[0].protected);
        assert_eq!(
            profile.planning_constraints[0].kind,
            PlanningConstraintKind::ReserveBufferBeforeCalendar
        );
    }

    #[test]
    fn planning_profile_mutation_example_parses() {
        let request: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/examples/planning-profile-mutation.example.json"
        ))
        .expect("planning profile mutation request example should parse");
        let mutation: PlanningProfileMutation = serde_json::from_value(request["mutation"].clone())
            .expect("planning profile mutation payload should parse");

        assert_eq!(
            mutation.kind(),
            PlanningProfileMutationKind::UpsertRoutineBlock
        );
    }

    #[test]
    fn planning_profile_edit_proposal_example_parses() {
        let proposal: PlanningProfileEditProposal = serde_json::from_str(include_str!(
            "../../../config/examples/planning-profile-edit-proposal.example.json"
        ))
        .expect("planning profile edit proposal example should parse");

        assert_eq!(proposal.source_surface, PlanningProfileSurface::Assistant);
        assert_eq!(proposal.state, AssistantProposalState::Staged);
        assert!(proposal.requires_confirmation);
        assert_eq!(proposal.continuity, PlanningProfileContinuity::Thread);
        assert_eq!(proposal.outcome_summary, None);
        assert_eq!(
            proposal.mutation.kind(),
            PlanningProfileMutationKind::UpsertRoutineBlock
        );
    }
}
