use serde_json::{json, Value as JsonValue};
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::TaskId;
use vel_storage::{
    get_sync_link, insert_canonical_object, upsert_integration_account, upsert_sync_link,
    CanonicalObjectRecord, IntegrationAccountRecord, StorageError, SyncLinkRecord,
};

use crate::{
    account_linking::TodoistCheckpointState,
    todoist_ids::{
        todoist_provider_object_ref, todoist_sync_link_id, TODOIST_MODULE_ID, TODOIST_PROVIDER,
        TODOIST_TASK_REMOTE_TYPE,
    },
};

#[derive(Debug, Clone)]
pub struct TodoistBacklogTask {
    pub remote_id: String,
    pub title: String,
    pub project_remote_id: Option<String>,
    pub parent_remote_id: Option<String>,
    pub section_remote_id: Option<String>,
    pub labels: Vec<String>,
    pub priority: Option<String>,
    pub due: Option<JsonValue>,
    pub remote_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TodoistBacklogImportRequest {
    pub integration_account: IntegrationAccountRecord,
    pub tasks: Vec<TodoistBacklogTask>,
    pub checkpoints: TodoistCheckpointState,
    pub imported_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedTodoistTask {
    pub task_id: String,
    pub sync_link_id: String,
    pub created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoistBacklogImportReport {
    pub imported: Vec<ImportedTodoistTask>,
}

pub async fn import_todoist_backlog(
    pool: &SqlitePool,
    request: &TodoistBacklogImportRequest,
) -> Result<TodoistBacklogImportReport, StorageError> {
    let mut imported = Vec::with_capacity(request.tasks.len());

    for task in &request.tasks {
        let sync_link_id = todoist_sync_link_id(
            &request.integration_account.id,
            TODOIST_TASK_REMOTE_TYPE,
            &task.remote_id,
        )
        .to_string();

        let existing_link = get_sync_link(pool, &sync_link_id).await?;
        let (task_id, created) = match existing_link {
            Some(link) => (link.object_id, false),
            None => {
                let task_id = TaskId::new().to_string();
                insert_canonical_object(
                    pool,
                    &CanonicalObjectRecord {
                        id: task_id.clone(),
                        object_type: "task".to_string(),
                        object_class: "content".to_string(),
                        schema_version: "0.5".to_string(),
                        revision: 1,
                        status: "active".to_string(),
                        provenance_json: json!({
                            "origin": "imported",
                            "basis": "provider_backlog_import",
                            "source_refs": [
                                TODOIST_MODULE_ID,
                                todoist_provider_object_ref(TODOIST_TASK_REMOTE_TYPE, &task.remote_id)
                            ],
                        }),
                        facets_json: canonical_task_facets(task),
                        source_summary_json: None,
                        deleted_at: None,
                        archived_at: None,
                        created_at: request.imported_at,
                        updated_at: request.imported_at,
                    },
                )
                .await?;
                (task_id, true)
            }
        };

        upsert_sync_link(
            pool,
            &SyncLinkRecord {
                id: sync_link_id.clone(),
                provider: TODOIST_PROVIDER.to_string(),
                integration_account_id: request.integration_account.id.clone(),
                object_id: task_id.clone(),
                remote_id: task.remote_id.clone(),
                remote_type: TODOIST_TASK_REMOTE_TYPE.to_string(),
                state: "reconciled".to_string(),
                authority_mode: "shared".to_string(),
                remote_version: task.remote_version.clone(),
                metadata_json: json!({
                    "module_id": TODOIST_MODULE_ID,
                    "import_mode": "backlog",
                    "history_layers": {
                        "current_state": true,
                        "sync_linkage": true,
                        "provider_activity": "pending_ingestion",
                    },
                    "checkpoints": {
                        "sync_cursor": request.checkpoints.sync_cursor,
                        "activity_cursor": request.checkpoints.activity_cursor,
                    },
                }),
                linked_at: request.imported_at,
                last_seen_at: request.imported_at,
            },
        )
        .await?;

        imported.push(ImportedTodoistTask {
            task_id,
            sync_link_id,
            created,
        });
    }

    let updated_account = stamp_account_checkpoints(
        request.integration_account.clone(),
        &request.checkpoints,
        request.imported_at,
    );
    upsert_integration_account(pool, &updated_account).await?;

    Ok(TodoistBacklogImportReport { imported })
}

fn canonical_task_facets(task: &TodoistBacklogTask) -> JsonValue {
    json!({
        "title": task.title,
        "description": JsonValue::Null,
        "status": "ready",
        "priority": "medium",
        "task_type": "generic",
        "tags": task.labels,
        "provider_facets": {
            "todoist": {
                "project_id": task.project_remote_id,
                "section_id": task.section_remote_id,
                "section_name_snapshot": JsonValue::Null,
                "parent_task_id": task.parent_remote_id,
                "labels": task.labels,
                "priority": task.priority,
                "due": task.due,
                "is_deleted_upstream": false,
            }
        }
    })
}

fn stamp_account_checkpoints(
    mut account: IntegrationAccountRecord,
    checkpoints: &TodoistCheckpointState,
    imported_at: OffsetDateTime,
) -> IntegrationAccountRecord {
    let mut metadata = account.metadata_json;
    if !metadata.is_object() {
        metadata = json!({});
    }

    let JsonValue::Object(ref mut map) = metadata else {
        account.metadata_json = metadata;
        account.updated_at = imported_at;
        return account;
    };

    map.insert(
        "checkpoints".to_string(),
        json!({
            "sync_cursor": checkpoints.sync_cursor,
            "activity_cursor": checkpoints.activity_cursor,
        }),
    );
    map.insert(
        "history_layers".to_string(),
        json!({
            "current_state": true,
            "sync_linkage": true,
            "provider_activity": "pending_ingestion",
        }),
    );

    account.metadata_json = metadata;
    account.updated_at = imported_at;
    account
}

#[cfg(test)]
mod tests {
    use super::{
        import_todoist_backlog, TodoistBacklogImportRequest, TodoistBacklogTask,
        TodoistCheckpointState,
    };
    use serde_json::json;
    use sqlx::{migrate::Migrator, SqlitePool};
    use time::OffsetDateTime;
    use vel_storage::{
        get_canonical_object, get_integration_account, list_sync_links_for_object,
        upsert_integration_account, IntegrationAccountRecord,
    };

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn integration_account(id: &str, external_account_ref: &str) -> IntegrationAccountRecord {
        let now = OffsetDateTime::now_utc();
        IntegrationAccountRecord {
            id: id.to_string(),
            provider: "todoist".to_string(),
            display_name: "Todoist".to_string(),
            external_account_ref: Some(external_account_ref.to_string()),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "full_backlog".to_string(),
            metadata_json: json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn backlog_import_creates_canonical_task_and_synclink_once() {
        let pool = test_pool().await;
        let account = integration_account("integration_account_test", "todo_primary");
        upsert_integration_account(&pool, &account).await.unwrap();
        let imported_at = OffsetDateTime::now_utc();

        let first = import_todoist_backlog(
            &pool,
            &TodoistBacklogImportRequest {
                integration_account: account.clone(),
                tasks: vec![TodoistBacklogTask {
                    remote_id: "todo_123".to_string(),
                    title: "Pay rent".to_string(),
                    project_remote_id: Some("proj_home".to_string()),
                    parent_remote_id: None,
                    section_remote_id: None,
                    labels: vec!["time:morning".to_string()],
                    priority: Some("p2".to_string()),
                    due: Some(json!({"kind":"date","value":"2026-03-25"})),
                    remote_version: Some("sync_v1".to_string()),
                }],
                checkpoints: TodoistCheckpointState {
                    sync_cursor: Some("sync_v1".to_string()),
                    activity_cursor: Some("activity_v1".to_string()),
                },
                imported_at,
            },
        )
        .await
        .unwrap();

        let second = import_todoist_backlog(
            &pool,
            &TodoistBacklogImportRequest {
                integration_account: account.clone(),
                tasks: vec![TodoistBacklogTask {
                    remote_id: "todo_123".to_string(),
                    title: "Pay rent".to_string(),
                    project_remote_id: Some("proj_home".to_string()),
                    parent_remote_id: None,
                    section_remote_id: None,
                    labels: vec!["time:morning".to_string()],
                    priority: Some("p2".to_string()),
                    due: Some(json!({"kind":"date","value":"2026-03-25"})),
                    remote_version: Some("sync_v2".to_string()),
                }],
                checkpoints: TodoistCheckpointState {
                    sync_cursor: Some("sync_v2".to_string()),
                    activity_cursor: Some("activity_v2".to_string()),
                },
                imported_at,
            },
        )
        .await
        .unwrap();

        assert!(first.imported[0].created);
        assert!(!second.imported[0].created);
        assert_eq!(first.imported[0].task_id, second.imported[0].task_id);

        let stored = get_canonical_object(&pool, &first.imported[0].task_id)
            .await
            .unwrap()
            .expect("canonical task should exist");
        assert_eq!(stored.object_type, "task");
        assert_eq!(
            stored.facets_json["provider_facets"]["todoist"]["project_id"],
            "proj_home"
        );
        assert_eq!(
            list_sync_links_for_object(&pool, &stored.id)
                .await
                .unwrap()
                .len(),
            1
        );

        let account = get_integration_account(&pool, "integration_account_test")
            .await
            .unwrap()
            .expect("account should exist");
        assert_eq!(
            account.metadata_json["checkpoints"]["sync_cursor"],
            "sync_v2"
        );
        assert_eq!(
            account.metadata_json["history_layers"]["provider_activity"],
            "pending_ingestion"
        );
    }
}
