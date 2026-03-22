use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_adapters_todoist::{
    apply_upstream_delete, import_todoist_backlog, link_todoist_account, map_todoist_comment,
    map_todoist_project, restore_from_tombstone, todoist_module_manifest,
    AttachedCommentRecord, TodoistAccountLinkRequest, TodoistBacklogImportRequest,
    TodoistBacklogTask, TodoistCheckpointState, TodoistCommentAuthorStub, TodoistCommentPayload,
    TodoistProjectPayload, TodoistSectionFacet,
};
use vel_storage::{
    get_canonical_object, list_sync_links_for_object, migrate_storage, query_canonical_objects,
    CanonicalObjectQuery,
};
use veld::services::todoist_write_bridge::{bridge_todoist_write, TodoistWriteBridgeRequest};

#[tokio::test]
async fn todoist_black_box_proves_account_import_task_project_tag_attached_comment_tombstone_and_write_flow()
{
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let module = todoist_module_manifest();
    assert_eq!(module.registry_id.as_string(), "module.integration.todoist");

    let account = link_todoist_account(
        &pool,
        &TodoistAccountLinkRequest {
            external_account_ref: "todo_primary".to_string(),
            display_name: "Todoist Primary".to_string(),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "full_backlog".to_string(),
            metadata_json: json!({"workspace":"main"}),
            checkpoints: TodoistCheckpointState {
                sync_cursor: Some("sync_primary".to_string()),
                activity_cursor: Some("activity_primary".to_string()),
            },
        },
    )
    .await
    .unwrap();

    let imported_at = OffsetDateTime::now_utc();
    let report = import_todoist_backlog(
        &pool,
        &TodoistBacklogImportRequest {
            integration_account: account.clone(),
            tasks: vec![TodoistBacklogTask {
                remote_id: "todo_black_box".to_string(),
                title: "Morning review".to_string(),
                project_remote_id: Some("proj_personal".to_string()),
                parent_remote_id: None,
                section_remote_id: Some("sec_morning".to_string()),
                labels: vec![
                    "maintain".to_string(),
                    "time:morning".to_string(),
                    "duration:15m".to_string(),
                ],
                priority: Some("p1".to_string()),
                due: Some(json!({"kind":"date","value":"2026-03-23"})),
                remote_version: Some("v1".to_string()),
            }],
            checkpoints: TodoistCheckpointState {
                sync_cursor: Some("sync_primary_v1".to_string()),
                activity_cursor: Some("activity_primary_v1".to_string()),
            },
            imported_at,
        },
    )
    .await
    .unwrap();

    let task_id = &report.imported[0].task_id;
    let task = get_canonical_object(&pool, task_id)
        .await
        .unwrap()
        .expect("imported task should persist canonically");
    let links = list_sync_links_for_object(&pool, task_id).await.unwrap();
    let tasks = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            object_type: Some("task".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(tasks.len(), 1);
    assert_eq!(task.object_type, "task");
    assert_eq!(task.provenance_json["source_refs"][0], "module.integration.todoist");
    assert_eq!(task.facets_json["task_type"], "maintain");
    assert_eq!(task.facets_json["tags"][1], "time:morning");
    assert_eq!(
        task.facets_json["task_semantics"]["estimated_duration_minutes"],
        15
    );
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].provider, "todoist");

    let project = map_todoist_project(
        &account.id,
        &TodoistProjectPayload {
            remote_id: "proj_personal".to_string(),
            name: "Personal".to_string(),
            color: Some("berry_red".to_string()),
            is_inbox_project: false,
            sections: vec![TodoistSectionFacet {
                remote_id: "sec_morning".to_string(),
                name: "Morning".to_string(),
            }],
        },
        imported_at,
    );
    assert_eq!(project.object_type, "project");
    assert_eq!(
        project.facets_json["provider_facets"]["todoist"]["section_posture"],
        "non-first-class"
    );

    let comment = map_todoist_comment(
        &account.id,
        task_id,
        &TodoistCommentPayload {
            remote_id: "comment_black_box".to_string(),
            parent_remote_task_id: "todo_black_box".to_string(),
            body: "Need to reschedule this if morning slips".to_string(),
            author: TodoistCommentAuthorStub {
                remote_id: Some("user_123".to_string()),
                display_name: Some("Jove".to_string()),
            },
            created_at: imported_at,
            updated_at: imported_at,
        },
    );
    let _: AttachedCommentRecord = comment.clone();
    assert_eq!(comment.parent_object_ref, task_id.as_str());
    assert_eq!(
        comment.provider_facets["todoist"]["comment_id"],
        "comment_black_box"
    );

    let deleted = apply_upstream_delete(&task, &links[0], imported_at);
    assert_eq!(deleted.object.status, "deleted");
    assert_eq!(deleted.sync_link_state, "deleted_upstream");
    assert_eq!(
        deleted.object.facets_json["provider_facets"]["todoist"]["tombstone_state"],
        "pending_reconcile"
    );

    let restored = restore_from_tombstone(&deleted.object, &links[0], imported_at);
    assert_eq!(restored.object.status, "active");
    assert_eq!(restored.sync_link_state, "restored");

    let dry_run = bridge_todoist_write(
        &pool,
        &TodoistWriteBridgeRequest {
            object_id: task_id.clone(),
            revision: task.revision,
            object_status: task.status.clone(),
            integration_account_id: account.id.clone(),
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
            object_id: task_id.clone(),
            revision: task.revision,
            object_status: task.status.clone(),
            integration_account_id: account.id.clone(),
            requested_change: json!({"due":{"kind":"date","value":"2026-03-24"}}),
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
