use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_storage::{
    bootstrap_canonical_registry, get_canonical_object, list_registry_objects, migrate_storage,
    replay_migration_artifact, validate_migration_artifact, CanonicalObjectRecord,
    MigrationArtifactRecord,
};

#[tokio::test]
async fn phase58_bootstrap_is_idempotent() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let first = bootstrap_canonical_registry(&pool).await.unwrap();
    let second = bootstrap_canonical_registry(&pool).await.unwrap();

    assert_eq!(first.seeded, 2);
    assert_eq!(second.unchanged, 2);
    assert_eq!(list_registry_objects(&pool).await.unwrap().len(), 2);
}

#[tokio::test]
async fn phase58_migration_artifact_replay_is_idempotent() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let now = OffsetDateTime::now_utc();
    let artifact = MigrationArtifactRecord {
        id: "migration_artifact_01phase58".to_string(),
        version: "0.5".to_string(),
        snapshot_ref: "snapshots/test.json".to_string(),
        validation: json!({"status":"passed"}),
        objects: vec![CanonicalObjectRecord {
            id: "task_01phase58_migrated".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"imported"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: Some(json!({"active_link_count": 0})),
            deleted_at: None,
            archived_at: None,
            created_at: now,
            updated_at: now,
        }],
    };

    let validation = validate_migration_artifact(&artifact);
    assert!(validation.valid, "migration artifact should validate");

    let first = replay_migration_artifact(&pool, &artifact).await.unwrap();
    let second = replay_migration_artifact(&pool, &artifact).await.unwrap();

    assert!(first.idempotent);
    assert_eq!(first.inserted, 1);
    assert_eq!(second.unchanged, 1);
    assert!(get_canonical_object(&pool, "task_01phase58_migrated")
        .await
        .unwrap()
        .is_some());
}
