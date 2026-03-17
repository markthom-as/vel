use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Planning-oriented command output kinds for higher-order intents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningKind {
    SpecDraft,
    ExecutionPlan,
    DelegationPlan,
}

impl Display for PlanningKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::SpecDraft => "spec_draft",
            Self::ExecutionPlan => "execution_plan",
            Self::DelegationPlan => "delegation_plan",
        };
        f.write_str(value)
    }
}
