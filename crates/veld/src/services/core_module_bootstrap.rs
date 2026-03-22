use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{
    reconcile_seeded_workflow, CoreBootstrapPolicy, CoreBootstrapReport, CoreBootstrapSource,
    SeededWorkflowRecord, SeededWorkflowReconciliationState,
};
use vel_storage::{
    get_canonical_object, insert_canonical_object, update_canonical_object, CanonicalObjectRecord,
    SqliteModuleRegistryStore,
};

use crate::errors::AppError;

use super::registry_loader::RegistryLoader;

pub struct CoreModuleBootstrap<S> {
    source: S,
    policy: CoreBootstrapPolicy,
}

impl<S> CoreModuleBootstrap<S> {
    pub fn new(source: S) -> Self {
        Self {
            source,
            policy: CoreBootstrapPolicy::default(),
        }
    }
}

impl<S> CoreModuleBootstrap<S>
where
    S: CoreBootstrapSource + Clone,
{
    pub async fn run(&self, pool: &SqlitePool) -> Result<CoreBootstrapReport, AppError> {
        let store = SqliteModuleRegistryStore::new(pool.clone());
        let loader = RegistryLoader::new(self.source.clone(), store.clone());
        let registry_results = loader.load_all().await?;

        let mut report = CoreBootstrapReport {
            registry_registered: registry_results.len(),
            registry_reconciled: registry_results.len(),
            ..Default::default()
        };

        for seed in self
            .source
            .seeded_workflows()
            .map_err(AppError::bad_request)?
        {
            let existing = get_canonical_object(pool, &seed.workflow_id)
                .await
                .map_err(|error| AppError::internal(error.to_string()))?
                .map(canonical_to_seeded_workflow)
                .transpose()
                .map_err(AppError::bad_request)?;
            let reconciled = reconcile_seeded_workflow(existing.as_ref(), &seed);

            apply_reconciled_workflow(pool, &reconciled, existing.as_ref()).await?;

            match reconciled.reconciliation_state {
                SeededWorkflowReconciliationState::New => report.workflow_seeded += 1,
                SeededWorkflowReconciliationState::Unchanged => report.workflow_unchanged += 1,
                SeededWorkflowReconciliationState::Updated => report.workflow_updated += 1,
                SeededWorkflowReconciliationState::Drifted => report.workflow_drifted += 1,
                SeededWorkflowReconciliationState::ForkedLocal => report.workflow_forked_local += 1,
                SeededWorkflowReconciliationState::Disabled
                | SeededWorkflowReconciliationState::Invalid => report.workflow_unchanged += 1,
            }
        }

        let _ = self.policy.idempotent;
        Ok(report)
    }
}

async fn apply_reconciled_workflow(
    pool: &SqlitePool,
    reconciled: &SeededWorkflowRecord,
    existing: Option<&SeededWorkflowRecord>,
) -> Result<(), AppError> {
    let now = OffsetDateTime::now_utc();
    let persisted = normalize_persisted_workflow(reconciled);
    match existing {
        None => {
            insert_canonical_object(pool, &seeded_workflow_to_canonical(&persisted, 1, now))
                .await
                .map_err(|error| AppError::internal(error.to_string()))?;
        }
        Some(existing) => {
            let stored = get_canonical_object(pool, &persisted.workflow_id)
                .await
                .map_err(|error| AppError::internal(error.to_string()))?
                .ok_or_else(|| AppError::not_found("seeded workflow missing during bootstrap"))?;

            let should_persist = existing != &persisted;
            if should_persist {
                update_canonical_object(
                    pool,
                    &persisted.workflow_id,
                    stored.revision,
                    &persisted.status,
                    &seeded_workflow_facets(&persisted),
                    stored.source_summary_json.as_ref(),
                    stored.archived_at,
                )
                .await
                .map_err(|error| AppError::internal(error.to_string()))?;
            }
        }
    }

    Ok(())
}

fn normalize_persisted_workflow(workflow: &SeededWorkflowRecord) -> SeededWorkflowRecord {
    let mut persisted = workflow.clone();
    if matches!(
        persisted.reconciliation_state,
        SeededWorkflowReconciliationState::New | SeededWorkflowReconciliationState::Updated
    ) {
        persisted.reconciliation_state = SeededWorkflowReconciliationState::Unchanged;
        persisted.upstream_update_available = false;
    }
    persisted
}

fn seeded_workflow_to_canonical(
    workflow: &SeededWorkflowRecord,
    revision: i64,
    now: OffsetDateTime,
) -> CanonicalObjectRecord {
    CanonicalObjectRecord {
        id: workflow.workflow_id.clone(),
        object_type: "workflow".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision,
        status: workflow.status.clone(),
        provenance_json: json!({
            "origin": workflow.origin,
            "source_module_id": workflow.source_module_id,
            "manifest_ref": workflow.manifest_ref,
        }),
        facets_json: seeded_workflow_facets(workflow),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: now,
        updated_at: now,
    }
}

fn seeded_workflow_facets(workflow: &SeededWorkflowRecord) -> serde_json::Value {
    json!({
        "display_name": workflow.display_name,
        "version": workflow.version,
        "mutability": workflow.mutability,
        "forked_from_workflow_id": workflow.forked_from_workflow_id,
        "definition": workflow.definition_json,
        "policy_ref": workflow.policy_ref,
        "seed_version": workflow.seed_version,
        "local_modified_at": workflow.local_modified_at,
        "local_modified_by": workflow.local_modified_by,
        "upstream_update_available": workflow.upstream_update_available,
        "reconciliation_state": workflow.reconciliation_state,
    })
}

fn canonical_to_seeded_workflow(record: CanonicalObjectRecord) -> Result<SeededWorkflowRecord, String> {
    let facets = &record.facets_json;
    Ok(SeededWorkflowRecord {
        workflow_id: record.id,
        origin: serde_json::from_value(record.provenance_json["origin"].clone())
            .map_err(|error| error.to_string())?,
        source_module_id: record.provenance_json["source_module_id"]
            .as_str()
            .ok_or_else(|| "workflow provenance missing source_module_id".to_string())?
            .to_string(),
        manifest_ref: record.provenance_json["manifest_ref"]
            .as_str()
            .ok_or_else(|| "workflow provenance missing manifest_ref".to_string())?
            .to_string(),
        display_name: facets["display_name"]
            .as_str()
            .ok_or_else(|| "workflow facets missing display_name".to_string())?
            .to_string(),
        version: facets["version"]
            .as_str()
            .ok_or_else(|| "workflow facets missing version".to_string())?
            .to_string(),
        mutability: serde_json::from_value(facets["mutability"].clone())
            .map_err(|error| error.to_string())?,
        forked_from_workflow_id: facets
            .get("forked_from_workflow_id")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned),
        definition_json: facets["definition"].clone(),
        policy_ref: facets
            .get("policy_ref")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned),
        seed_version: facets["seed_version"]
            .as_str()
            .ok_or_else(|| "workflow facets missing seed_version".to_string())?
            .to_string(),
        local_modified_at: facets
            .get("local_modified_at")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned),
        local_modified_by: facets
            .get("local_modified_by")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned),
        upstream_update_available: facets
            .get("upstream_update_available")
            .and_then(|value| value.as_bool())
            .unwrap_or(false),
        reconciliation_state: serde_json::from_value(facets["reconciliation_state"].clone())
            .map_err(|error| error.to_string())?,
        status: record.status,
    })
}

#[cfg(test)]
mod tests {
    use super::{canonical_to_seeded_workflow, normalize_persisted_workflow, seeded_workflow_facets};
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_core::{
        SeededWorkflowMutability, SeededWorkflowOrigin, SeededWorkflowReconciliationState,
        SeededWorkflowRecord,
    };
    use vel_storage::CanonicalObjectRecord;

    #[test]
    fn workflow_facet_helpers_preserve_seed_version_and_reconciliation_state() {
        let workflow = SeededWorkflowRecord {
            workflow_id: "workflow_01seededbrief".to_string(),
            origin: SeededWorkflowOrigin::Seeded,
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0".to_string(),
            mutability: SeededWorkflowMutability::Forkable,
            forked_from_workflow_id: None,
            definition_json: json!({"step_types":["action"]}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.22".to_string(),
            local_modified_at: None,
            local_modified_by: None,
            upstream_update_available: false,
            reconciliation_state: SeededWorkflowReconciliationState::Unchanged,
            status: "active".to_string(),
        };
        let facets = seeded_workflow_facets(&workflow);
        let roundtrip = canonical_to_seeded_workflow(CanonicalObjectRecord {
            id: workflow.workflow_id.clone(),
            object_type: "workflow".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({
                "origin": workflow.origin,
                "source_module_id": workflow.source_module_id,
                "manifest_ref": workflow.manifest_ref,
            }),
            facets_json: facets,
            source_summary_json: None,
            deleted_at: None,
            archived_at: None,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        })
        .unwrap();

        assert_eq!(roundtrip.seed_version, "2026.03.22");
        assert_eq!(
            roundtrip.reconciliation_state,
            SeededWorkflowReconciliationState::Unchanged
        );
    }

    #[test]
    fn persisted_workflows_normalize_transient_bootstrap_states() {
        let workflow = SeededWorkflowRecord {
            workflow_id: "workflow_01seededbrief".to_string(),
            origin: SeededWorkflowOrigin::Seeded,
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0".to_string(),
            mutability: SeededWorkflowMutability::Forkable,
            forked_from_workflow_id: None,
            definition_json: json!({"step_types":["action"]}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.22".to_string(),
            local_modified_at: None,
            local_modified_by: None,
            upstream_update_available: true,
            reconciliation_state: SeededWorkflowReconciliationState::Updated,
            status: "active".to_string(),
        };

        let normalized = normalize_persisted_workflow(&workflow);

        assert_eq!(
            normalized.reconciliation_state,
            SeededWorkflowReconciliationState::Unchanged
        );
        assert!(!normalized.upstream_update_available);
    }
}
