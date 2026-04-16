use serde::{Deserialize, Serialize};
use vel_core::ActionItemId;

use crate::{DailyLoopCommitmentActionData, DailyLoopPhaseData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInSourceKindData {
    DailyLoop,
}

impl From<vel_core::CheckInSourceKind> for CheckInSourceKindData {
    fn from(value: vel_core::CheckInSourceKind) -> Self {
        match value {
            vel_core::CheckInSourceKind::DailyLoop => Self::DailyLoop,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInSubmitTargetKindData {
    DailyLoopTurn,
}

impl From<vel_core::CheckInSubmitTargetKind> for CheckInSubmitTargetKindData {
    fn from(value: vel_core::CheckInSubmitTargetKind) -> Self {
        match value {
            vel_core::CheckInSubmitTargetKind::DailyLoopTurn => Self::DailyLoopTurn,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInSubmitTargetData {
    pub kind: CheckInSubmitTargetKindData,
    pub reference_id: String,
}

impl From<vel_core::CheckInSubmitTarget> for CheckInSubmitTargetData {
    fn from(value: vel_core::CheckInSubmitTarget) -> Self {
        Self {
            kind: value.kind.into(),
            reference_id: value.reference_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInEscalationTargetData {
    Threads,
}

impl From<vel_core::CheckInEscalationTarget> for CheckInEscalationTargetData {
    fn from(value: vel_core::CheckInEscalationTarget) -> Self {
        match value {
            vel_core::CheckInEscalationTarget::Threads => Self::Threads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInEscalationData {
    pub target: CheckInEscalationTargetData,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

impl From<vel_core::CheckInEscalation> for CheckInEscalationData {
    fn from(value: vel_core::CheckInEscalation) -> Self {
        Self {
            target: value.target.into(),
            label: value.label,
            thread_id: value.thread_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInTransitionKindData {
    Submit,
    Bypass,
    Escalate,
}

impl From<vel_core::CheckInTransitionKind> for CheckInTransitionKindData {
    fn from(value: vel_core::CheckInTransitionKind) -> Self {
        match value {
            vel_core::CheckInTransitionKind::Submit => Self::Submit,
            vel_core::CheckInTransitionKind::Bypass => Self::Bypass,
            vel_core::CheckInTransitionKind::Escalate => Self::Escalate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInTransitionTargetKindData {
    DailyLoopTurn,
    Threads,
}

impl From<vel_core::CheckInTransitionTargetKind> for CheckInTransitionTargetKindData {
    fn from(value: vel_core::CheckInTransitionTargetKind) -> Self {
        match value {
            vel_core::CheckInTransitionTargetKind::DailyLoopTurn => Self::DailyLoopTurn,
            vel_core::CheckInTransitionTargetKind::Threads => Self::Threads,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInTransitionData {
    pub kind: CheckInTransitionKindData,
    pub label: String,
    pub target: CheckInTransitionTargetKindData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub requires_response: bool,
    pub requires_note: bool,
}

impl From<vel_core::CheckInTransition> for CheckInTransitionData {
    fn from(value: vel_core::CheckInTransition) -> Self {
        Self {
            kind: value.kind.into(),
            label: value.label,
            target: value.target.into(),
            reference_id: value.reference_id,
            requires_response: value.requires_response,
            requires_note: value.requires_note,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInCardData {
    pub id: ActionItemId,
    pub source_kind: CheckInSourceKindData,
    pub phase: DailyLoopPhaseData,
    pub session_id: String,
    pub title: String,
    pub summary: String,
    pub prompt_id: String,
    pub prompt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_response: Option<String>,
    pub allow_skip: bool,
    pub blocking: bool,
    pub submit_target: CheckInSubmitTargetData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation: Option<CheckInEscalationData>,
    #[serde(default)]
    pub commitment_actions: Vec<DailyLoopCommitmentActionData>,
    #[serde(default)]
    pub transitions: Vec<CheckInTransitionData>,
}

impl From<vel_core::CheckInCard> for CheckInCardData {
    fn from(value: vel_core::CheckInCard) -> Self {
        let commitment_actions = check_in_commitment_actions(&value);
        Self {
            id: value.id,
            source_kind: value.source_kind.into(),
            phase: value.phase.into(),
            session_id: value.session_id,
            title: value.title,
            summary: value.summary,
            prompt_id: value.prompt_id,
            prompt_text: value.prompt_text,
            suggested_action_label: value.suggested_action_label,
            suggested_response: value.suggested_response,
            allow_skip: value.allow_skip,
            blocking: value.blocking,
            submit_target: value.submit_target.into(),
            escalation: value.escalation.map(Into::into),
            commitment_actions,
            transitions: value.transitions.into_iter().map(Into::into).collect(),
        }
    }
}

fn check_in_commitment_actions(
    value: &vel_core::CheckInCard,
) -> Vec<DailyLoopCommitmentActionData> {
    let mut actions = vec![DailyLoopCommitmentActionData::Accept];
    if value.allow_skip {
        actions.push(DailyLoopCommitmentActionData::Defer);
    }
    if value.escalation.is_some() {
        actions.push(DailyLoopCommitmentActionData::Choose);
    }
    actions.push(DailyLoopCommitmentActionData::Close);
    actions
}
