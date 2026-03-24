use sqlx::SqlitePool;
use vel_adapters_google_calendar::google_calendar_module_manifest;
use vel_adapters_todoist::todoist_module_manifest;
use vel_core::{
    CapabilityRequest, Grant, GrantScope, RegistryKind, RegistryManifest, RegistryStatus,
    SemanticRegistryId,
};
use vel_storage::{list_registry_objects, migrate_storage};
use veld::services::provider_module_registration::ProviderModuleRegistrationService;

fn grant_for(capabilities: &[&str]) -> Grant {
    Grant {
        id: "grant_01providermodules".to_string(),
        scope: vec![GrantScope::Workspace],
        capabilities: capabilities.iter().map(|value| value.to_string()).collect(),
        durable: false,
        run_scoped: true,
        read_only: false,
    }
}

fn core_manifest() -> RegistryManifest {
    RegistryManifest {
        registry_id: SemanticRegistryId::new(RegistryKind::Module, "core", "orientation"),
        display_name: "Orientation".to_string(),
        version: "0.5".to_string(),
        status: RegistryStatus::Active,
        manifest_ref: "modules/core/orientation/module.yaml".to_string(),
        capability_requests: vec![CapabilityRequest {
            capability: "object.read".to_string(),
            feature_gate: None,
        }],
    }
}

#[tokio::test]
async fn core_and_provider_modules_register_through_one_governed_seam() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    let service = ProviderModuleRegistrationService::new(pool.clone());

    let core = service
        .register_provider_module(core_manifest(), grant_for(&["object.read"]), vec![])
        .await
        .unwrap();
    let todoist = service
        .register_provider_module(
            todoist_module_manifest(),
            grant_for(&["todoist.read", "todoist.write"]),
            vec!["todoist".to_string()],
        )
        .await
        .unwrap();
    let google = service
        .register_provider_module(
            google_calendar_module_manifest(),
            grant_for(&["google.calendar.read", "google.calendar.write"]),
            vec!["google-calendar".to_string()],
        )
        .await
        .unwrap();
    let registry = list_registry_objects(&pool).await.unwrap();

    assert_eq!(core.reconciliation.object.id, "module.core.orientation");
    assert_eq!(
        todoist.reconciliation.object.id,
        "module.integration.todoist"
    );
    assert_eq!(
        google.reconciliation.object.id,
        "module.integration.google-calendar"
    );
    assert_eq!(registry.len(), 3);
    assert!(core.activation.activated);
    assert!(todoist.activation.activated);
    assert!(google.activation.activated);
}

#[tokio::test]
async fn provider_registration_does_not_imply_sync_or_runtime_behavior() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    let service = ProviderModuleRegistrationService::new(pool);

    let todoist = service
        .register_provider_module(
            todoist_module_manifest(),
            grant_for(&["todoist.read", "todoist.write"]),
            vec!["todoist".to_string()],
        )
        .await
        .unwrap();

    assert_eq!(
        todoist.reconciliation.object.id,
        "module.integration.todoist"
    );
    assert!(todoist.activation.invokable);
    assert!(!todoist.runtime_behavior_implemented);
}
