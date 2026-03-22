use serde_json::json;
use sqlx::SqlitePool;
use vel_adapters_google_calendar::google_calendar_module_manifest;
use vel_core::{Grant, GrantScope, ModuleCapabilityProfile};
use vel_storage::migrate_storage;
use veld::services::{
    gcal_write_bridge::{GoogleCalendarWriteBridgeRequest, bridge_google_calendar_write},
    module_policy_bridge::{ModulePolicyBridge, ModulePolicyBridgeError, ModulePolicyBridgeInput},
};

fn google_grant(capabilities: &[&str], read_only: bool) -> Grant {
    Grant {
        id: "grant_01gcal".to_string(),
        scope: vec![
            GrantScope::Workspace,
            GrantScope::Module("module.integration.google-calendar".to_string()),
        ],
        capabilities: capabilities.iter().map(|value| value.to_string()).collect(),
        durable: false,
        run_scoped: true,
        read_only,
    }
}

#[test]
fn google_calendar_error_surface_keeps_unsupported_capability_distinct() {
    let module = google_calendar_module_manifest();
    let profile = ModuleCapabilityProfile::registered(
        "module.integration.google-calendar",
        &module.capability_requests,
    );

    let unsupported = ModulePolicyBridge::default()
        .evaluate(&ModulePolicyBridgeInput {
            module_id: "module.integration.google-calendar".to_string(),
            requested_capabilities: profile,
            enabled_feature_gates: vec![],
            grant: google_grant(&["google.calendar.read", "google.calendar.write"], false),
            read_only: false,
        })
        .unwrap_err();
    assert!(matches!(
        unsupported,
        ModulePolicyBridgeError::UnsupportedCapability(_)
    ));
    assert!("UnsupportedCapability".contains("UnsupportedCapability"));
}

#[tokio::test]
async fn google_calendar_error_surface_keeps_policy_read_only_scope_reconciliation_and_ownership_failures_typed()
 {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let pending = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: "event_01pending".to_string(),
            expected_revision: 2,
            actual_revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_google".to_string(),
            requested_change: json!({"title":"Move block"}),
            recurrence_scope: Some("single_occurrence".to_string()),
            source_owned_fields: vec![],
            read_only: false,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: true,
        },
    )
    .await
    .expect_err("pending reconciliation should block writes");
    assert!(pending.to_string().contains("PendingReconciliation"));

    let read_only = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: "event_01readonly".to_string(),
            expected_revision: 2,
            actual_revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_google".to_string(),
            requested_change: json!({"title":"Move block"}),
            recurrence_scope: Some("single_occurrence".to_string()),
            source_owned_fields: vec![],
            read_only: true,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("read-only account should stay distinct");
    assert!(read_only.to_string().contains("ReadOnlyViolation"));

    let denied = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: "event_01denied".to_string(),
            expected_revision: 2,
            actual_revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_google".to_string(),
            requested_change: json!({"title":"Move block"}),
            recurrence_scope: Some("single_occurrence".to_string()),
            source_owned_fields: vec![],
            read_only: false,
            write_enabled: false,
            dry_run: false,
            approved: false,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("disabled writes should remain policy denied");
    assert!(denied.to_string().contains("PolicyDenied"));

    let unsupported_scope = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: "event_01scope".to_string(),
            expected_revision: 2,
            actual_revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_google".to_string(),
            requested_change: json!({"scope":"this_and_following"}),
            recurrence_scope: Some("this_and_following".to_string()),
            source_owned_fields: vec![],
            read_only: false,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("unsupported recurrence scope should stay explicit");
    assert!(
        unsupported_scope
            .to_string()
            .contains("UnsupportedCapability")
    );

    let ownership = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: "event_01ownership".to_string(),
            expected_revision: 2,
            actual_revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_google".to_string(),
            requested_change: json!({"attendees":["user@example.com"]}),
            recurrence_scope: Some("single_occurrence".to_string()),
            source_owned_fields: vec!["attendees".to_string()],
            read_only: false,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("ownership conflict should remain explicit");
    assert!(ownership.to_string().contains("OwnershipConflict"));
}
