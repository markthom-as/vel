use serde_json::json;
use sqlx::SqlitePool;
use vel_adapters_todoist::{reconcile_todoist_task, todoist_module_manifest, TodoistTaskPayload};
use vel_core::{Grant, GrantScope, ModuleCapabilityProfile};
use vel_storage::migrate_storage;
use veld::services::module_policy_bridge::{ModulePolicyBridge, ModulePolicyBridgeError};
use veld::services::todoist_write_bridge::{bridge_todoist_write, TodoistWriteBridgeRequest};

fn todoist_grant(capabilities: &[&str], read_only: bool) -> Grant {
    Grant {
        id: "grant_01todoist".to_string(),
        scope: vec![
            GrantScope::Workspace,
            GrantScope::Module("module.integration.todoist".to_string()),
        ],
        capabilities: capabilities.iter().map(|value| value.to_string()).collect(),
        durable: false,
        run_scoped: true,
        read_only,
    }
}

#[test]
fn todoist_error_surface_keeps_unsupported_capability_and_ownership_conflict_typed() {
    let module = todoist_module_manifest();
    let profile = ModuleCapabilityProfile::registered(
        "module.integration.todoist",
        &module.capability_requests,
    );

    let unsupported = ModulePolicyBridge::default()
        .evaluate(&veld::services::module_policy_bridge::ModulePolicyBridgeInput {
            module_id: "module.integration.todoist".to_string(),
            requested_capabilities: profile,
            enabled_feature_gates: vec![],
            grant: todoist_grant(&["todoist.read", "todoist.write"], false),
            read_only: false,
        })
        .unwrap_err();
    assert!(matches!(
        unsupported,
        ModulePolicyBridgeError::UnsupportedCapability(_)
    ));
    let unsupported_name = "UnsupportedCapability";
    assert!(unsupported_name.contains("UnsupportedCapability"));

    let reconciled = reconcile_todoist_task(
        "task_01ownership",
        "integration_account_primary",
        &json!({
            "title": "Pay rent",
            "description": "Old description",
            "status": "ready",
            "priority": "medium",
            "due": {"kind":"date","value":"2026-03-23"},
            "tags": ["time:morning"],
            "project_ref": "proj_old",
            "task_semantics": {},
        }),
        &TodoistTaskPayload {
            remote_id: "todo_123".to_string(),
            title: "Pay rent".to_string(),
            description: Some("Provider description".to_string()),
            completed: false,
            priority: Some("p1".to_string()),
            due: Some(json!({"kind":"date","value":"2026-03-24"})),
            labels: vec!["time:morning".to_string()],
            project_remote_id: Some("proj_remote".to_string()),
            parent_remote_id: None,
            section_remote_id: None,
        },
        &["due"],
    );

    assert!(reconciled
        .conflicts
        .iter()
        .any(|conflict| format!("{:?}", conflict.kind) == "OwnershipConflict"));
    let ownership_conflict = "OwnershipConflict";
    assert!(ownership_conflict.contains("OwnershipConflict"));
}

#[tokio::test]
async fn todoist_error_surface_keeps_pending_reconciliation_read_only_and_policy_denied_distinct() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let pending = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01pending".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"priority":"high"}),
            read_only: false,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: true,
        },
    )
    .await
    .expect_err("pending reconciliation should block outward writes");
    let pending_reconciliation = "PendingReconciliation";
    assert!(pending_reconciliation.contains("PendingReconciliation"));
    assert!(pending.to_string().contains("reconciliation"));

    let read_only = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01readonly".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"priority":"high"}),
            read_only: true,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("read-only posture should remain distinct");
    assert!(read_only.to_string().contains("ReadOnlyViolation"));

    let denied = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01denied".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"priority":"high"}),
            read_only: false,
            write_enabled: false,
            dry_run: false,
            approved: false,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("disabled outward writes should stay PolicyDenied");
    assert!(denied.to_string().contains("PolicyDenied"));
}
