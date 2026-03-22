use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::actions::{
    OBJECT_CREATE, OBJECT_DELETE, OBJECT_EXPLAIN, OBJECT_GET, OBJECT_LINK, OBJECT_QUERY,
    OBJECT_UPDATE,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMode {
    Auto,
    Ask,
    AskIfDestructive,
    AskIfCrossSource,
    AskIfExternalWrite,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRequirement {
    Required,
    Optional,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionErrorKind {
    ValidationError,
    NotFound,
    PolicyDenied,
    ConfirmationRequired,
    ReadOnlyViolation,
    GrantMissing,
    StaleVersion,
    OwnershipConflict,
    PendingReconciliation,
    ExecutionDispatchFailed,
    AuditCaptureFailed,
    UnsupportedCapability,
    StorageFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionCapability {
    pub capability: String,
    pub provider_family: Option<String>,
    pub allows_external_write: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionContract {
    pub action_name: String,
    pub alias_of: Option<String>,
    pub input_schema: String,
    pub output_schema: String,
    pub capability: ActionCapability,
    pub confirmation: ConfirmationMode,
    pub audit: AuditRequirement,
    pub errors: Vec<ActionErrorKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionRequestEnvelope {
    pub action_name: String,
    pub target_object_refs: Vec<String>,
    pub dry_run: bool,
    pub input: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionResponseEnvelope {
    pub action_name: String,
    pub output: JsonValue,
    pub explain: Option<JsonValue>,
}

pub fn generic_object_action_contracts() -> Vec<ActionContract> {
    vec![
        ActionContract {
            action_name: OBJECT_GET.to_string(),
            alias_of: None,
            input_schema: "canonical-object-ref".to_string(),
            output_schema: "canonical-object-envelope".to_string(),
            capability: ActionCapability {
                capability: "object.read".to_string(),
                provider_family: None,
                allows_external_write: false,
            },
            confirmation: ConfirmationMode::Auto,
            audit: AuditRequirement::Optional,
            errors: vec![ActionErrorKind::NotFound, ActionErrorKind::StorageFailure],
        },
        ActionContract {
            action_name: OBJECT_QUERY.to_string(),
            alias_of: None,
            input_schema: "canonical-object-query".to_string(),
            output_schema: "canonical-object-page".to_string(),
            capability: ActionCapability {
                capability: "object.read".to_string(),
                provider_family: None,
                allows_external_write: false,
            },
            confirmation: ConfirmationMode::Auto,
            audit: AuditRequirement::Optional,
            errors: vec![ActionErrorKind::ValidationError, ActionErrorKind::StorageFailure],
        },
        ActionContract {
            action_name: OBJECT_CREATE.to_string(),
            alias_of: None,
            input_schema: "canonical-object-create".to_string(),
            output_schema: "canonical-object-envelope".to_string(),
            capability: ActionCapability {
                capability: "object.write".to_string(),
                provider_family: None,
                allows_external_write: false,
            },
            confirmation: ConfirmationMode::Ask,
            audit: AuditRequirement::Required,
            errors: vec![
                ActionErrorKind::ValidationError,
                ActionErrorKind::PolicyDenied,
                ActionErrorKind::AuditCaptureFailed,
                ActionErrorKind::StorageFailure,
            ],
        },
        ActionContract {
            action_name: OBJECT_UPDATE.to_string(),
            alias_of: None,
            input_schema: "canonical-object-update".to_string(),
            output_schema: "canonical-object-envelope".to_string(),
            capability: ActionCapability {
                capability: "object.write".to_string(),
                provider_family: None,
                allows_external_write: true,
            },
            confirmation: ConfirmationMode::AskIfExternalWrite,
            audit: AuditRequirement::Required,
            errors: vec![
                ActionErrorKind::ValidationError,
                ActionErrorKind::StaleVersion,
                ActionErrorKind::OwnershipConflict,
                ActionErrorKind::PendingReconciliation,
                ActionErrorKind::AuditCaptureFailed,
                ActionErrorKind::StorageFailure,
            ],
        },
        ActionContract {
            action_name: OBJECT_DELETE.to_string(),
            alias_of: None,
            input_schema: "canonical-object-delete".to_string(),
            output_schema: "write-intent".to_string(),
            capability: ActionCapability {
                capability: "object.write".to_string(),
                provider_family: None,
                allows_external_write: true,
            },
            confirmation: ConfirmationMode::AskIfDestructive,
            audit: AuditRequirement::Required,
            errors: vec![
                ActionErrorKind::ConfirmationRequired,
                ActionErrorKind::PolicyDenied,
                ActionErrorKind::ReadOnlyViolation,
                ActionErrorKind::StorageFailure,
            ],
        },
        ActionContract {
            action_name: OBJECT_LINK.to_string(),
            alias_of: None,
            input_schema: "canonical-object-link".to_string(),
            output_schema: "canonical-relation".to_string(),
            capability: ActionCapability {
                capability: "object.link".to_string(),
                provider_family: None,
                allows_external_write: false,
            },
            confirmation: ConfirmationMode::AskIfCrossSource,
            audit: AuditRequirement::Required,
            errors: vec![
                ActionErrorKind::ValidationError,
                ActionErrorKind::OwnershipConflict,
                ActionErrorKind::StorageFailure,
            ],
        },
        ActionContract {
            action_name: OBJECT_EXPLAIN.to_string(),
            alias_of: None,
            input_schema: "canonical-object-ref".to_string(),
            output_schema: "object-explain".to_string(),
            capability: ActionCapability {
                capability: "object.read".to_string(),
                provider_family: None,
                allows_external_write: false,
            },
            confirmation: ConfirmationMode::Auto,
            audit: AuditRequirement::Optional,
            errors: vec![ActionErrorKind::NotFound, ActionErrorKind::StorageFailure],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{generic_object_action_contracts, AuditRequirement, ConfirmationMode};

    #[test]
    fn generic_action_contracts_include_capability_confirmation_audit_and_error_metadata() {
        let contracts = generic_object_action_contracts();
        let update = contracts
            .iter()
            .find(|contract| contract.action_name == "object.update")
            .expect("object.update contract should exist");

        assert_eq!(update.capability.capability, "object.write");
        assert_eq!(update.confirmation, ConfirmationMode::AskIfExternalWrite);
        assert_eq!(update.audit, AuditRequirement::Required);
        assert!(
            update
                .errors
                .iter()
                .any(|error| format!("{error:?}") == "StaleVersion")
        );
    }
}

