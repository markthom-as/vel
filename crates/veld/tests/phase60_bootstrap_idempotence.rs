use serde_json::json;
use sqlx::SqlitePool;
use vel_core::{
    CapabilityRequest, CoreBootstrapBundle, RegistryKind, RegistryManifest, RegistryStatus,
    SeededWorkflowMutability, SeededWorkflowSpec, SemanticRegistryId,
};
use vel_storage::{get_canonical_object, list_registry_objects, migrate_storage};
use veld::services::core_module_bootstrap::CoreModuleBootstrap;

fn source_bundle() -> CoreBootstrapBundle {
    CoreBootstrapBundle {
        registry_manifests: vec![RegistryManifest {
            registry_id: SemanticRegistryId::new(RegistryKind::Module, "core", "orientation"),
            display_name: "Orientation".to_string(),
            version: "0.5".to_string(),
            status: RegistryStatus::Active,
            manifest_ref: "modules/core/orientation/module.yaml".to_string(),
            capability_requests: vec![CapabilityRequest {
                capability: "workflow.invoke".to_string(),
                feature_gate: None,
            }],
        }],
        seeded_workflows: vec![SeededWorkflowSpec {
            workflow_id: "workflow_01seededbrief".to_string(),
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0".to_string(),
            mutability: SeededWorkflowMutability::Forkable,
            definition_json: json!({"step_types":["action","skill"]}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.22".to_string(),
            status: "active".to_string(),
        }],
    }
}

#[tokio::test]
async fn bootstrap_is_deterministic_and_idempotent_for_registry_and_seeded_workflows() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    let bootstrap = CoreModuleBootstrap::new(source_bundle());

    let first = bootstrap.run(&pool).await.unwrap();
    let second = bootstrap.run(&pool).await.unwrap();
    let registry_records = list_registry_objects(&pool).await.unwrap();
    let workflow = get_canonical_object(&pool, "workflow_01seededbrief")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(first.registry_registered, 1);
    assert_eq!(first.workflow_seeded, 1);
    assert_eq!(second.registry_registered, 1);
    assert_eq!(second.workflow_unchanged, 1);
    assert_eq!(registry_records.len(), 1);
    assert_eq!(workflow.revision, 1);
    assert_eq!(workflow.facets_json["seed_version"], "2026.03.22");
    assert_eq!(workflow.facets_json["reconciliation_state"], "unchanged");
}

#[tokio::test]
async fn bootstrap_preserves_forkable_local_state_and_marks_upstream_update_available() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    let bootstrap = CoreModuleBootstrap::new(source_bundle());
    bootstrap.run(&pool).await.unwrap();

    vel_storage::update_canonical_object(
        &pool,
        "workflow_01seededbrief",
        1,
        "active",
        &json!({
            "display_name": "Daily Brief",
            "version": "1.0.0-local",
            "mutability": "forkable",
            "forked_from_workflow_id": null,
            "definition": {"step_types":["action","skill"],"local":"yes"},
            "policy_ref": "policy.workflow.daily-brief",
            "seed_version": "2026.03.21",
            "local_modified_at": "2026-03-22T06:00:00Z",
            "local_modified_by": "operator",
            "upstream_update_available": false,
            "reconciliation_state": "updated"
        }),
        None,
        None,
    )
    .await
    .unwrap();

    let report = bootstrap.run(&pool).await.unwrap();
    let workflow = get_canonical_object(&pool, "workflow_01seededbrief")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(report.workflow_forked_local, 1);
    assert_eq!(workflow.facets_json["definition"]["local"], "yes");
    assert_eq!(workflow.facets_json["upstream_update_available"], true);
    assert_eq!(workflow.facets_json["reconciliation_state"], "forked_local");
}

#[tokio::test]
async fn editable_seeded_workflows_surface_drift_without_overwriting_local_definition() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    let mut bundle = source_bundle();
    bundle.seeded_workflows[0].mutability = SeededWorkflowMutability::Editable;
    let bootstrap = CoreModuleBootstrap::new(bundle);
    bootstrap.run(&pool).await.unwrap();

    vel_storage::update_canonical_object(
        &pool,
        "workflow_01seededbrief",
        1,
        "active",
        &json!({
            "display_name": "Daily Brief",
            "version": "1.0.0-local",
            "mutability": "editable",
            "forked_from_workflow_id": null,
            "definition": {"step_types":["action"],"local":"drift"},
            "policy_ref": "policy.workflow.daily-brief",
            "seed_version": "2026.03.21",
            "local_modified_at": "2026-03-22T06:00:00Z",
            "local_modified_by": "operator",
            "upstream_update_available": false,
            "reconciliation_state": "updated"
        }),
        None,
        None,
    )
    .await
    .unwrap();

    let report = bootstrap.run(&pool).await.unwrap();
    let workflow = get_canonical_object(&pool, "workflow_01seededbrief")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(report.workflow_drifted, 1);
    assert_eq!(workflow.facets_json["definition"]["local"], "drift");
    assert_eq!(workflow.facets_json["reconciliation_state"], "drifted");
    assert_eq!(workflow.facets_json["upstream_update_available"], true);
    assert_eq!(workflow.facets_json["seed_version"], "2026.03.21");
}
