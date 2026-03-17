use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Domain kinds addressable by typed command resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainKind {
    Capture,
    Commitment,
    Artifact,
    Run,
    RunEvent,
    Ref,
    Signal,
    Nudge,
    Suggestion,
    Thread,
    Context,
    SpecDraft,
    ExecutionPlan,
    DelegationPlan,
}

impl Display for DomainKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Capture => "capture",
            Self::Commitment => "commitment",
            Self::Artifact => "artifact",
            Self::Run => "run",
            Self::RunEvent => "run_event",
            Self::Ref => "ref",
            Self::Signal => "signal",
            Self::Nudge => "nudge",
            Self::Suggestion => "suggestion",
            Self::Thread => "thread",
            Self::Context => "context",
            Self::SpecDraft => "spec_draft",
            Self::ExecutionPlan => "execution_plan",
            Self::DelegationPlan => "delegation_plan",
        };
        f.write_str(value)
    }
}
