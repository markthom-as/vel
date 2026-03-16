//! Intervention model for proactive surfacing. See docs/tickets/vel-agent-ticket-pack/006-implement-intervention-model.md

use crate::types::{InterventionId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Intervention state. Transitions: active -> snoozed | resolved | dismissed; snoozed -> active | resolved | dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterventionState {
    Active,
    Snoozed,
    Resolved,
    Dismissed,
}

impl Display for InterventionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Active => "active",
            Self::Snoozed => "snoozed",
            Self::Resolved => "resolved",
            Self::Dismissed => "dismissed",
        };
        f.write_str(s)
    }
}

/// Intervention: a surfaced reminder/risk/suggestion tied to a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intervention {
    pub id: InterventionId,
    pub message_id: MessageId,
    pub kind: String,
    pub state: InterventionState,
    pub confidence: Option<f64>,
    pub surfaced_at: i64,
    pub resolved_at: Option<i64>,
    pub snoozed_until: Option<i64>,
}

impl Intervention {
    /// Valid transition from current state. Resolved and Dismissed are terminal.
    pub fn can_transition_to(&self, next: InterventionState) -> bool {
        use InterventionState::*;
        if self.state == next {
            return true;
        }
        match (self.state, next) {
            (Active, Snoozed | Resolved | Dismissed) => true,
            (Snoozed, Active | Resolved | Dismissed) => true,
            (Resolved | Dismissed, _) => false,
            _ => false,
        }
    }
}
