use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{ConfirmationMode, OwnershipClass, PolicyDecisionKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainBasis {
    Exact,
    Inferred,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyExplain {
    pub action_name: String,
    pub decision: PolicyDecisionKind,
    pub confirmation: ConfirmationMode,
    pub read_only: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectExplain {
    pub object_ref: String,
    pub status: String,
    pub revision: i64,
    pub source_summary: Option<JsonValue>,
    pub linked_provider_count: usize,
    pub basis: ExplainBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipExplain {
    pub field: String,
    pub owner: OwnershipClass,
    pub overlay_applied: bool,
    pub source_favored: bool,
    pub pending_write_intent: bool,
    pub confirmation_required: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionExplain {
    pub action_name: String,
    pub capability: String,
    pub allows_external_write: bool,
    pub dry_run: bool,
    pub policy_explain: PolicyExplain,
    pub object_explain: Option<ObjectExplain>,
    pub ownership_explain: Vec<OwnershipExplain>,
}

pub fn policy_explain(
    action_name: impl Into<String>,
    decision: PolicyDecisionKind,
    confirmation: ConfirmationMode,
    read_only: bool,
    reasons: Vec<String>,
) -> PolicyExplain {
    PolicyExplain {
        action_name: action_name.into(),
        decision,
        confirmation,
        read_only,
        reasons,
    }
}

pub fn object_explain(
    object_ref: impl Into<String>,
    status: impl Into<String>,
    revision: i64,
    source_summary: Option<JsonValue>,
    linked_provider_count: usize,
    basis: ExplainBasis,
) -> ObjectExplain {
    ObjectExplain {
        object_ref: object_ref.into(),
        status: status.into(),
        revision,
        source_summary,
        linked_provider_count,
        basis,
    }
}

pub fn ownership_explain(
    field: impl Into<String>,
    owner: OwnershipClass,
    overlay_applied: bool,
    source_favored: bool,
    pending_write_intent: bool,
    confirmation_required: bool,
    reason: impl Into<String>,
) -> OwnershipExplain {
    OwnershipExplain {
        field: field.into(),
        owner,
        overlay_applied,
        source_favored,
        pending_write_intent,
        confirmation_required,
        reason: reason.into(),
    }
}

pub fn action_explain(
    action_name: impl Into<String>,
    capability: impl Into<String>,
    allows_external_write: bool,
    dry_run: bool,
    policy_explain: PolicyExplain,
    object_explain: Option<ObjectExplain>,
    ownership_explain: Vec<OwnershipExplain>,
) -> ActionExplain {
    ActionExplain {
        action_name: action_name.into(),
        capability: capability.into(),
        allows_external_write,
        dry_run,
        policy_explain,
        object_explain,
        ownership_explain,
    }
}

#[cfg(test)]
mod tests {
    use super::{action_explain, object_explain, ownership_explain, policy_explain, ExplainBasis};
    use crate::{ConfirmationMode, OwnershipClass, PolicyDecisionKind};
    use serde_json::json;

    #[test]
    fn explain_payloads_cover_policy_object_ownership_and_action_views() {
        let policy = policy_explain(
            "object.update",
            PolicyDecisionKind::Allowed,
            ConfirmationMode::AskIfExternalWrite,
            false,
            vec!["module requires confirmation".to_string()],
        );
        let object = object_explain(
            "task_01explain",
            "active",
            3,
            Some(json!({"active_link_count": 1})),
            1,
            ExplainBasis::Exact,
        );
        let ownership = ownership_explain(
            "due",
            OwnershipClass::SourceOwned,
            true,
            true,
            true,
            true,
            "provider owns due while local edit requires write intent",
        );
        let action = action_explain(
            "object.update",
            "object.write",
            true,
            true,
            policy,
            Some(object),
            vec![ownership],
        );

        assert!(action.dry_run);
        assert_eq!(action.policy_explain.reasons.len(), 1);
        assert_eq!(action.ownership_explain[0].field, "due");
    }
}
