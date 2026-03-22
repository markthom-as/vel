use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_storage::{
    get_projection, insert_canonical_object, list_relations_from, list_sync_links_for_object,
    rebuild_source_summary_projection, upsert_integration_account, upsert_relation,
    upsert_sync_link, CanonicalObjectRecord, CanonicalRelationRecord, IntegrationAccountRecord,
    ProjectionRecord, SyncLinkId, SyncLinkRecord, TaskId,
};

#[tokio::test]
async fn phase58_storage_roundtrip_rebuilds_source_summary_from_synclink_state() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    vel_storage::migrate_storage(&pool).await.unwrap();
    let now = OffsetDateTime::now_utc();
    let task_id = TaskId::new().to_string();
    let sync_link_id = SyncLinkId::new().to_string();

    insert_canonical_object(
        &pool,
        &CanonicalObjectRecord {
            id: task_id.clone(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"imported"}),
            facets_json: json!({"task_type":"maintain"}),
            source_summary_json: None,
            deleted_at: None,
            archived_at: None,
            created_at: now,
            updated_at: now,
        },
    )
    .await
    .unwrap();

    upsert_relation(
        &pool,
        &CanonicalRelationRecord {
            id: "rel_01roundtrip".to_string(),
            relation_type: "task -> project".to_string(),
            from_id: task_id.clone(),
            to_id: "project_01roundtrip".to_string(),
            direction: "outbound".to_string(),
            active: true,
            source_json: json!({"basis":"exact"}),
            confidence: Some(1.0),
            revision: 1,
            created_at: now,
            updated_at: now,
        },
    )
    .await
    .unwrap();

    upsert_integration_account(
        &pool,
        &IntegrationAccountRecord {
            id: "integration_account_01roundtrip".to_string(),
            provider: "todoist".to_string(),
            display_name: "Todoist Primary".to_string(),
            external_account_ref: Some("todoist_primary".to_string()),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "incremental".to_string(),
            metadata_json: json!({"scopes":["todoist.tasks.read"]}),
            created_at: now,
            updated_at: now,
        },
    )
    .await
    .unwrap();

    upsert_sync_link(
        &pool,
        &SyncLinkRecord {
            id: sync_link_id.clone(),
            provider: "todoist".to_string(),
            integration_account_id: "integration_account_01roundtrip".to_string(),
            object_id: task_id.clone(),
            remote_id: "todo_123".to_string(),
            remote_type: "task".to_string(),
            state: "reconciled".to_string(),
            authority_mode: "shared".to_string(),
            remote_version: Some("v1".to_string()),
            metadata_json: json!({"source":"roundtrip"}),
            linked_at: now,
            last_seen_at: now,
        },
    )
    .await
    .unwrap();

    let projection = rebuild_source_summary_projection(&pool, &task_id)
        .await
        .unwrap();
    let stored_projection = get_projection(&pool, &projection.id)
        .await
        .unwrap()
        .expect("source_summary projection should exist");
    let source_summary = stored_projection
        .source_summary_json
        .clone()
        .expect("source_summary should be materialized");

    assert_eq!(
        source_summary.get("active_link_count").and_then(|value| value.as_u64()),
        Some(1)
    );
    assert_eq!(
        source_summary.get("providers").and_then(|value| value.as_array()).map(|items| items.len()),
        Some(1)
    );
    assert_eq!(
        source_summary.get("primary_provider").and_then(|value| value.as_str()),
        Some("todoist")
    );
    assert_eq!(list_relations_from(&pool, &task_id).await.unwrap().len(), 1);
    assert_eq!(list_sync_links_for_object(&pool, &task_id).await.unwrap().len(), 1);
}

#[tokio::test]
async fn phase58_projection_roundtrip_stays_rebuildable_and_non_authoritative() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    vel_storage::migrate_storage(&pool).await.unwrap();
    let now = OffsetDateTime::now_utc();
    let task_id = TaskId::new().to_string();

    insert_canonical_object(
        &pool,
        &CanonicalObjectRecord {
            id: task_id.clone(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"user"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: Some(json!({"active_link_count": 99})),
            deleted_at: None,
            archived_at: None,
            created_at: now,
            updated_at: now,
        },
    )
    .await
    .unwrap();

    vel_storage::upsert_projection(
        &pool,
        &ProjectionRecord {
            id: format!("projection.source_summary.{task_id}"),
            projection_type: "source_summary".to_string(),
            object_id: Some(task_id.clone()),
            source_summary_json: Some(json!({"active_link_count": 99})),
            projection_json: json!({"source_summary":{"active_link_count":99}}),
            rebuild_token: Some("seed".to_string()),
            created_at: now,
            updated_at: now,
        },
    )
    .await
    .unwrap();

    let rebuilt = rebuild_source_summary_projection(&pool, &task_id)
        .await
        .unwrap();
    let source_summary = rebuilt
        .source_summary_json
        .expect("rebuild should produce source_summary");

    assert_eq!(
        source_summary.get("active_link_count").and_then(|value| value.as_u64()),
        Some(0)
    );
}
