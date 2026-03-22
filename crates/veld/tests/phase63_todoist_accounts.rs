use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_adapters_todoist::{
    import_todoist_backlog, link_todoist_account, TodoistAccountLinkRequest,
    TodoistBacklogImportRequest, TodoistBacklogTask, TodoistCheckpointState,
};
use vel_storage::{
    get_canonical_object, get_integration_account, list_sync_links_for_object, migrate_storage,
    query_canonical_objects, CanonicalObjectQuery,
};

#[tokio::test]
async fn todoist_multi_account_backlog_import_uses_canonical_account_and_synclink_substrate() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let primary = link_todoist_account(
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
    let secondary = link_todoist_account(
        &pool,
        &TodoistAccountLinkRequest {
            external_account_ref: "todo_secondary".to_string(),
            display_name: "Todoist Secondary".to_string(),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "full_backlog".to_string(),
            metadata_json: json!({"workspace":"sidecar"}),
            checkpoints: TodoistCheckpointState {
                sync_cursor: Some("sync_secondary".to_string()),
                activity_cursor: Some("activity_secondary".to_string()),
            },
        },
    )
    .await
    .unwrap();

    assert_ne!(
        primary.id, secondary.id,
        "distinct Todoist accounts must coexist"
    );

    let imported_at = OffsetDateTime::now_utc();
    let primary_import = import_todoist_backlog(
        &pool,
        &TodoistBacklogImportRequest {
            integration_account: primary.clone(),
            tasks: vec![TodoistBacklogTask {
                remote_id: "todo_same_remote".to_string(),
                title: "Morning review".to_string(),
                project_remote_id: Some("proj_personal".to_string()),
                parent_remote_id: None,
                section_remote_id: Some("sec_routine".to_string()),
                labels: vec!["time:morning".to_string(), "duration:15m".to_string()],
                priority: Some("p2".to_string()),
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
    let secondary_import = import_todoist_backlog(
        &pool,
        &TodoistBacklogImportRequest {
            integration_account: secondary.clone(),
            tasks: vec![TodoistBacklogTask {
                remote_id: "todo_same_remote".to_string(),
                title: "Evening review".to_string(),
                project_remote_id: Some("proj_work".to_string()),
                parent_remote_id: None,
                section_remote_id: Some("sec_focus".to_string()),
                labels: vec!["time:evening".to_string()],
                priority: Some("p1".to_string()),
                due: None,
                remote_version: Some("v9".to_string()),
            }],
            checkpoints: TodoistCheckpointState {
                sync_cursor: Some("sync_secondary_v1".to_string()),
                activity_cursor: Some("activity_secondary_v1".to_string()),
            },
            imported_at,
        },
    )
    .await
    .unwrap();

    let objects = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            object_type: Some("task".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(
        objects.len(),
        2,
        "same remote ids across two accounts must not collide into one canonical object"
    );

    let primary_task = get_canonical_object(&pool, &primary_import.imported[0].task_id)
        .await
        .unwrap()
        .expect("primary canonical task should exist");
    let secondary_task = get_canonical_object(&pool, &secondary_import.imported[0].task_id)
        .await
        .unwrap()
        .expect("secondary canonical task should exist");

    assert_eq!(primary_task.object_type, "task");
    assert_eq!(primary_task.provenance_json["origin"], "imported");
    assert_eq!(
        primary_task.provenance_json["source_refs"][0],
        "module.integration.todoist"
    );
    assert_eq!(primary_task.facets_json["title"], "Morning review");
    assert_eq!(primary_task.facets_json["task_type"], "generic");
    assert_eq!(
        primary_task.facets_json["provider_facets"]["todoist"]["section_id"],
        "sec_routine"
    );
    assert_eq!(
        secondary_task.facets_json["provider_facets"]["todoist"]["project_id"],
        "proj_work"
    );

    let primary_links = list_sync_links_for_object(&pool, &primary_import.imported[0].task_id)
        .await
        .unwrap();
    let secondary_links = list_sync_links_for_object(&pool, &secondary_import.imported[0].task_id)
        .await
        .unwrap();
    assert_eq!(primary_links.len(), 1);
    assert_eq!(secondary_links.len(), 1);
    assert_eq!(primary_links[0].integration_account_id, primary.id);
    assert_eq!(secondary_links[0].integration_account_id, secondary.id);
    assert_eq!(
        primary_links[0].metadata_json["history_layers"]["provider_activity"],
        "pending_ingestion"
    );

    let stored_primary = get_integration_account(&pool, &primary.id)
        .await
        .unwrap()
        .expect("primary account should persist");
    assert_eq!(
        stored_primary.metadata_json["checkpoints"]["sync_cursor"],
        "sync_primary_v1"
    );
    assert_eq!(
        stored_primary.metadata_json["module_id"],
        "module.integration.todoist"
    );
}
