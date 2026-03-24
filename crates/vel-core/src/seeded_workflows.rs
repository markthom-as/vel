use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeededWorkflowOrigin {
    Seeded,
    UserAuthored,
    Imported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeededWorkflowMutability {
    Immutable,
    Forkable,
    Editable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeededWorkflowReconciliationState {
    New,
    Unchanged,
    Updated,
    Drifted,
    ForkedLocal,
    Disabled,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeededWorkflowSpec {
    pub workflow_id: String,
    pub source_module_id: String,
    pub manifest_ref: String,
    pub display_name: String,
    pub version: String,
    pub mutability: SeededWorkflowMutability,
    pub definition_json: JsonValue,
    pub policy_ref: Option<String>,
    pub seed_version: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeededWorkflowRecord {
    pub workflow_id: String,
    pub origin: SeededWorkflowOrigin,
    pub source_module_id: String,
    pub manifest_ref: String,
    pub display_name: String,
    pub version: String,
    pub mutability: SeededWorkflowMutability,
    pub forked_from_workflow_id: Option<String>,
    pub definition_json: JsonValue,
    pub policy_ref: Option<String>,
    pub seed_version: String,
    pub local_modified_at: Option<String>,
    pub local_modified_by: Option<String>,
    pub upstream_update_available: bool,
    pub reconciliation_state: SeededWorkflowReconciliationState,
    pub status: String,
}

pub fn reconcile_seeded_workflow(
    existing: Option<&SeededWorkflowRecord>,
    seed: &SeededWorkflowSpec,
) -> SeededWorkflowRecord {
    match existing {
        None => SeededWorkflowRecord {
            workflow_id: seed.workflow_id.clone(),
            origin: SeededWorkflowOrigin::Seeded,
            source_module_id: seed.source_module_id.clone(),
            manifest_ref: seed.manifest_ref.clone(),
            display_name: seed.display_name.clone(),
            version: seed.version.clone(),
            mutability: seed.mutability.clone(),
            forked_from_workflow_id: None,
            definition_json: seed.definition_json.clone(),
            policy_ref: seed.policy_ref.clone(),
            seed_version: seed.seed_version.clone(),
            local_modified_at: None,
            local_modified_by: None,
            upstream_update_available: false,
            reconciliation_state: SeededWorkflowReconciliationState::New,
            status: seed.status.clone(),
        },
        Some(existing)
            if existing.origin != SeededWorkflowOrigin::Seeded
                && existing.forked_from_workflow_id.is_none() =>
        {
            let mut preserved = existing.clone();
            preserved.reconciliation_state = SeededWorkflowReconciliationState::Invalid;
            preserved.upstream_update_available = true;
            preserved
        }
        Some(existing)
            if existing.mutability == SeededWorkflowMutability::Forkable
                && existing.local_modified_at.is_some() =>
        {
            let mut preserved = existing.clone();
            preserved.reconciliation_state = SeededWorkflowReconciliationState::ForkedLocal;
            preserved.upstream_update_available = workflow_seed_changed(existing, seed);
            preserved
        }
        Some(existing)
            if existing.mutability == SeededWorkflowMutability::Editable
                && existing.local_modified_at.is_some() =>
        {
            let mut preserved = existing.clone();
            preserved.reconciliation_state = SeededWorkflowReconciliationState::Drifted;
            preserved.upstream_update_available = workflow_seed_changed(existing, seed);
            preserved
        }
        Some(existing)
            if !workflow_seed_changed(existing, seed)
                && existing.reconciliation_state != SeededWorkflowReconciliationState::Disabled =>
        {
            let mut preserved = existing.clone();
            preserved.reconciliation_state = SeededWorkflowReconciliationState::Unchanged;
            preserved.upstream_update_available = false;
            preserved
        }
        Some(existing) if existing.status == "disabled" => {
            let mut preserved = existing.clone();
            preserved.reconciliation_state = SeededWorkflowReconciliationState::Disabled;
            preserved.upstream_update_available = workflow_seed_changed(existing, seed);
            preserved
        }
        Some(_) => SeededWorkflowRecord {
            workflow_id: seed.workflow_id.clone(),
            origin: SeededWorkflowOrigin::Seeded,
            source_module_id: seed.source_module_id.clone(),
            manifest_ref: seed.manifest_ref.clone(),
            display_name: seed.display_name.clone(),
            version: seed.version.clone(),
            mutability: seed.mutability.clone(),
            forked_from_workflow_id: None,
            definition_json: seed.definition_json.clone(),
            policy_ref: seed.policy_ref.clone(),
            seed_version: seed.seed_version.clone(),
            local_modified_at: None,
            local_modified_by: None,
            upstream_update_available: false,
            reconciliation_state: SeededWorkflowReconciliationState::Updated,
            status: seed.status.clone(),
        },
    }
}

fn workflow_seed_changed(existing: &SeededWorkflowRecord, seed: &SeededWorkflowSpec) -> bool {
    existing.seed_version != seed.seed_version
        || existing.version != seed.version
        || existing.definition_json != seed.definition_json
        || existing.manifest_ref != seed.manifest_ref
        || existing.display_name != seed.display_name
        || existing.policy_ref != seed.policy_ref
}

#[cfg(test)]
mod tests {
    use super::{
        reconcile_seeded_workflow, SeededWorkflowMutability, SeededWorkflowOrigin,
        SeededWorkflowReconciliationState, SeededWorkflowRecord, SeededWorkflowSpec,
    };

    fn spec() -> SeededWorkflowSpec {
        SeededWorkflowSpec {
            workflow_id: "workflow_01seededbrief".to_string(),
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0".to_string(),
            mutability: SeededWorkflowMutability::Forkable,
            definition_json: serde_json::json!({"step_types":["action","skill"]}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.22".to_string(),
            status: "active".to_string(),
        }
    }

    #[test]
    fn seeded_workflow_new_records_start_seeded_and_reconciled() {
        let record = reconcile_seeded_workflow(None, &spec());

        assert_eq!(record.origin, SeededWorkflowOrigin::Seeded);
        assert_eq!(
            record.reconciliation_state,
            SeededWorkflowReconciliationState::New
        );
        assert_eq!(record.forked_from_workflow_id, None);
    }

    #[test]
    fn forkable_local_modifications_preserve_local_state_and_mark_forked_from_workflow_id_rules() {
        let existing = SeededWorkflowRecord {
            workflow_id: "workflow_01seededbrief".to_string(),
            origin: SeededWorkflowOrigin::Seeded,
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0-local".to_string(),
            mutability: SeededWorkflowMutability::Forkable,
            forked_from_workflow_id: None,
            definition_json: serde_json::json!({"step_types":["action","skill"],"local":"yes"}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.21".to_string(),
            local_modified_at: Some("2026-03-22T06:00:00Z".to_string()),
            local_modified_by: Some("operator".to_string()),
            upstream_update_available: false,
            reconciliation_state: SeededWorkflowReconciliationState::Updated,
            status: "active".to_string(),
        };

        let reconciled = reconcile_seeded_workflow(Some(&existing), &spec());

        assert_eq!(
            reconciled.reconciliation_state,
            SeededWorkflowReconciliationState::ForkedLocal
        );
        assert!(reconciled.upstream_update_available);
        assert_eq!(reconciled.definition_json["local"], "yes");
    }

    #[test]
    fn editable_seeded_workflows_surface_drift_without_overwrite() {
        let existing = SeededWorkflowRecord {
            workflow_id: "workflow_01seededbrief".to_string(),
            origin: SeededWorkflowOrigin::Seeded,
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0-local".to_string(),
            mutability: SeededWorkflowMutability::Editable,
            forked_from_workflow_id: None,
            definition_json: serde_json::json!({"step_types":["action"],"local":"drift"}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.21".to_string(),
            local_modified_at: Some("2026-03-22T06:00:00Z".to_string()),
            local_modified_by: Some("operator".to_string()),
            upstream_update_available: false,
            reconciliation_state: SeededWorkflowReconciliationState::Updated,
            status: "active".to_string(),
        };

        let mut changed = spec();
        changed.mutability = SeededWorkflowMutability::Editable;

        let reconciled = reconcile_seeded_workflow(Some(&existing), &changed);

        assert_eq!(
            reconciled.reconciliation_state,
            SeededWorkflowReconciliationState::Drifted
        );
        assert_eq!(reconciled.definition_json["local"], "drift");
        assert!(reconciled.upstream_update_available);
    }
}
