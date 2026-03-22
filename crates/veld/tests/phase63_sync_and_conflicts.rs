use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_adapters_todoist::{
    TodoistTaskPayload, apply_upstream_delete, reconcile_todoist_task, restore_from_tombstone,
};
use vel_storage::{CanonicalObjectRecord, SyncLinkRecord, migrate_storage};
use veld::services::todoist_write_bridge::{TodoistWriteBridgeRequest, bridge_todoist_write};

#[test]
fn source_owned_fields_win_and_local_and_provider_task_history_stay_continuous() {
    let result = reconcile_todoist_task(
        "task_01sync",
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

    assert_eq!(result.merged_facets["due"]["value"], "2026-03-24");
    assert!(
        result
            .conflicts
            .iter()
            .any(|conflict| conflict.reason.contains("source-owned field due"))
    );
    assert!(
        result
            .task_events
            .iter()
            .any(|event| event.provenance == "provider_event")
    );
    assert!(
        result
            .task_events
            .iter()
            .any(|event| event.provenance == "local_write_intent")
    );
}

#[test]
fn upstream_delete_becomes_tombstone_and_restore_is_explicit() {
    let object = CanonicalObjectRecord {
        id: "task_01tombstone".to_string(),
        object_type: "task".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({"origin":"imported"}),
        facets_json: json!({"provider_facets":{"todoist":{"is_deleted_upstream":false}}}),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    };
    let sync_link = SyncLinkRecord {
        id: "sync_link_01todoist".to_string(),
        provider: "todoist".to_string(),
        integration_account_id: "integration_account_primary".to_string(),
        object_id: object.id.clone(),
        remote_id: "todo_123".to_string(),
        remote_type: "task".to_string(),
        state: "reconciled".to_string(),
        authority_mode: "shared".to_string(),
        remote_version: Some("v1".to_string()),
        metadata_json: json!({}),
        linked_at: OffsetDateTime::UNIX_EPOCH,
        last_seen_at: OffsetDateTime::UNIX_EPOCH,
    };

    let deleted = apply_upstream_delete(&object, &sync_link, OffsetDateTime::UNIX_EPOCH);
    assert_eq!(deleted.object.status, "deleted");
    assert_eq!(deleted.sync_link_state, "deleted_upstream");
    assert_eq!(
        deleted.object.facets_json["provider_facets"]["todoist"]["tombstone_state"],
        "pending_reconcile"
    );

    let restored = restore_from_tombstone(&deleted.object, &sync_link, OffsetDateTime::UNIX_EPOCH);
    assert_eq!(restored.object.status, "active");
    assert_eq!(restored.sync_link_state, "restored");
}

#[tokio::test]
async fn todoist_write_bridge_enforces_read_only_and_policy_denied_paths_and_dispatches_write_intent()
 {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let read_only = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01readonly".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"due":{"kind":"date","value":"2026-03-24"}}),
            read_only: true,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("read_only external write should fail");
    assert!(read_only.to_string().contains("ReadOnlyViolation"));

    let denied = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01denied".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"due":{"kind":"date","value":"2026-03-24"}}),
            read_only: false,
            write_enabled: false,
            dry_run: false,
            approved: false,
            pending_reconciliation: false,
        },
    )
    .await
    .expect_err("disabled writes should fail");
    assert!(denied.to_string().contains("PolicyDenied"));

    let dry_run = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01dryrun".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"priority":"high"}),
            read_only: false,
            write_enabled: true,
            dry_run: true,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .unwrap();
    assert!(dry_run.dispatch.is_none());
    assert_eq!(dry_run.task_events[0].provenance, "local_write_intent");

    let executed = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: "task_01execute".to_string(),
            revision: 2,
            object_status: "active".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            requested_change: json!({"priority":"high"}),
            read_only: false,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .unwrap();
    assert!(executed.dispatch.is_some());
    assert_eq!(executed.task_events[0].provenance, "local_write_applied");
}
