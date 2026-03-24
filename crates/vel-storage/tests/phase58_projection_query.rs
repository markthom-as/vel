use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_storage::{
    insert_canonical_object, query_canonical_objects, traverse_relations, CanonicalObjectQuery,
    CanonicalObjectRecord, CanonicalObjectSort, CanonicalObjectSortField, CanonicalRelationRecord,
    QuerySortDirection, RelationTraversal,
};

#[tokio::test]
async fn phase58_query_respects_include_deleted_and_include_archived() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    vel_storage::migrate_storage(&pool).await.unwrap();
    let now = OffsetDateTime::now_utc();

    for record in [
        CanonicalObjectRecord {
            id: "task_01phase58_active".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"user"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: None,
            deleted_at: None,
            archived_at: None,
            created_at: now,
            updated_at: now,
        },
        CanonicalObjectRecord {
            id: "task_01phase58_tombstone".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "deleted".to_string(),
            provenance_json: json!({"origin":"imported"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: None,
            deleted_at: Some(now),
            archived_at: None,
            created_at: now,
            updated_at: now,
        },
        CanonicalObjectRecord {
            id: "task_01phase58_archived".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "archived".to_string(),
            provenance_json: json!({"origin":"workflow"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: None,
            deleted_at: None,
            archived_at: Some(now),
            created_at: now,
            updated_at: now,
        },
    ] {
        insert_canonical_object(&pool, &record).await.unwrap();
    }

    let default_query = CanonicalObjectQuery {
        object_type: Some("task".to_string()),
        sort: CanonicalObjectSort {
            field: CanonicalObjectSortField::Id,
            direction: QuerySortDirection::Asc,
        },
        ..Default::default()
    };
    let visible = query_canonical_objects(&pool, &default_query)
        .await
        .unwrap();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, "task_01phase58_active");

    let include_deleted = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            include_deleted: true,
            ..default_query.clone()
        },
    )
    .await
    .unwrap();
    assert_eq!(include_deleted.len(), 2);

    let include_deleted_and_archived = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            include_deleted: true,
            include_archived: true,
            ..default_query
        },
    )
    .await
    .unwrap();
    assert_eq!(include_deleted_and_archived.len(), 3);
}

#[tokio::test]
async fn phase58_relation_traversal_is_typed_and_paged() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    vel_storage::migrate_storage(&pool).await.unwrap();
    let now = OffsetDateTime::now_utc();

    for relation in [
        CanonicalRelationRecord {
            id: "rel_01phase58_project".to_string(),
            relation_type: "task -> project".to_string(),
            from_id: "task_01phase58".to_string(),
            to_id: "project_01phase58".to_string(),
            direction: "outbound".to_string(),
            active: true,
            source_json: json!({"basis":"exact"}),
            confidence: Some(1.0),
            revision: 1,
            created_at: now,
            updated_at: now,
        },
        CanonicalRelationRecord {
            id: "rel_01phase58_parent".to_string(),
            relation_type: "task -> parent".to_string(),
            from_id: "task_01phase58".to_string(),
            to_id: "task_01phase58_parent".to_string(),
            direction: "outbound".to_string(),
            active: false,
            source_json: json!({"basis":"historical"}),
            confidence: Some(0.8),
            revision: 1,
            created_at: now,
            updated_at: now,
        },
    ] {
        vel_storage::upsert_relation(&pool, &relation)
            .await
            .unwrap();
    }

    let active_only = traverse_relations(
        &pool,
        &RelationTraversal {
            from_id: "task_01phase58".to_string(),
            limit: Some(10),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(active_only.len(), 1);
    assert_eq!(active_only[0].relation_type, "task -> project");

    let include_inactive = traverse_relations(
        &pool,
        &RelationTraversal {
            from_id: "task_01phase58".to_string(),
            active_only: false,
            limit: Some(1),
            sort: QuerySortDirection::Asc,
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(include_inactive.len(), 1);
}
