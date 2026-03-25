use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use vel_core::{ArtifactId, PrivacyClass, SyncClass};

use crate::db::{ArtifactInsert, ArtifactRecord, StorageError};

pub(crate) async fn create_artifact(
    pool: &SqlitePool,
    input: ArtifactInsert,
) -> Result<ArtifactId, StorageError> {
    let mut tx = pool.begin().await?;
    let artifact_id = create_artifact_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(artifact_id)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn create_artifact_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &ArtifactInsert,
) -> Result<ArtifactId, StorageError> {
    let artifact_id = ArtifactId::new();
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO artifacts (
            artifact_id,
            artifact_type,
            title,
            mime_type,
            storage_uri,
            storage_kind,
            privacy_class,
            sync_class,
            content_hash,
            size_bytes,
            created_at,
            updated_at,
            metadata_json
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(artifact_id.to_string())
    .bind(&input.artifact_type)
    .bind(&input.title)
    .bind(&input.mime_type)
    .bind(&input.storage_uri)
    .bind(input.storage_kind.to_string())
    .bind(input.privacy_class.to_string())
    .bind(input.sync_class.to_string())
    .bind(&input.content_hash)
    .bind(input.size_bytes)
    .bind(now)
    .bind(now)
    .bind(
        input
            .metadata_json
            .as_ref()
            .and_then(|v| serde_json::to_string(v).ok())
            .as_deref()
            .unwrap_or("{}"),
    )
    .execute(&mut **tx)
    .await?;
    Ok(artifact_id)
}

pub(crate) async fn get_artifact_by_id(
    pool: &SqlitePool,
    artifact_id: &str,
) -> Result<Option<ArtifactRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
               privacy_class, sync_class, content_hash, size_bytes, metadata_json, created_at, updated_at
        FROM artifacts
        WHERE artifact_id = ?
        "#,
    )
    .bind(artifact_id)
    .fetch_optional(pool)
    .await?;

    row.map(|row| map_artifact_row(&row)).transpose()
}

pub(crate) async fn get_latest_artifact_by_type(
    pool: &SqlitePool,
    artifact_type: &str,
) -> Result<Option<ArtifactRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
               privacy_class, sync_class, content_hash, size_bytes, metadata_json, created_at, updated_at
        FROM artifacts
        WHERE artifact_type = ?
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(artifact_type)
    .fetch_optional(pool)
    .await?;

    row.map(|row| map_artifact_row(&row)).transpose()
}

pub(crate) async fn list_artifacts(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<ArtifactRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
               privacy_class, sync_class, content_hash, size_bytes, metadata_json, created_at, updated_at
        FROM artifacts
        ORDER BY created_at DESC
        LIMIT ?
        "#,
    )
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|row| map_artifact_row(&row)).collect()
}

pub(crate) async fn list_artifacts_for_run(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<Vec<ArtifactRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT a.artifact_id, a.artifact_type, a.title, a.mime_type, a.storage_uri, a.storage_kind,
               a.privacy_class, a.sync_class, a.content_hash, a.size_bytes, a.metadata_json, a.created_at, a.updated_at
        FROM refs r
        JOIN artifacts a
          ON a.artifact_id = r.to_id
        WHERE r.from_type = 'run'
          AND r.from_id = ?
          AND r.to_type = 'artifact'
        ORDER BY r.created_at ASC, a.created_at ASC
        "#,
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| map_artifact_row(&row)).collect()
}

fn map_artifact_row(row: &sqlx::sqlite::SqliteRow) -> Result<ArtifactRecord, StorageError> {
    let storage_kind_str: String = row.try_get("storage_kind")?;
    let storage_kind = storage_kind_str
        .parse()
        .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?;
    let privacy_class_str: String = row.try_get("privacy_class")?;
    let privacy_class = parse_privacy_class(&privacy_class_str)?;
    let sync_class_str: String = row.try_get("sync_class")?;
    let sync_class = parse_sync_class(&sync_class_str)?;
    let metadata_str: String = row.try_get("metadata_json")?;

    Ok(ArtifactRecord {
        artifact_id: ArtifactId::from(row.try_get::<String, _>("artifact_id")?),
        artifact_type: row.try_get("artifact_type")?,
        title: row.try_get("title")?,
        mime_type: row.try_get("mime_type")?,
        storage_uri: row.try_get("storage_uri")?,
        storage_kind,
        privacy_class,
        sync_class,
        content_hash: row.try_get("content_hash")?,
        size_bytes: row.try_get("size_bytes")?,
        metadata_json: serde_json::from_str(&metadata_str)
            .unwrap_or_else(|_| serde_json::json!({})),
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn parse_privacy_class(value: &str) -> Result<PrivacyClass, StorageError> {
    match value {
        "private" => Ok(PrivacyClass::Private),
        "work" => Ok(PrivacyClass::Work),
        "sensitive" => Ok(PrivacyClass::Sensitive),
        "do_not_record" => Ok(PrivacyClass::DoNotRecord),
        other => Err(StorageError::Validation(format!(
            "invalid privacy_class: {other}"
        ))),
    }
}

fn parse_sync_class(value: &str) -> Result<SyncClass, StorageError> {
    match value {
        "hot" => Ok(SyncClass::Hot),
        "warm" => Ok(SyncClass::Warm),
        "cold" => Ok(SyncClass::Cold),
        other => Err(StorageError::Validation(format!(
            "invalid sync_class: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vel_core::{ArtifactStorageKind, PrivacyClass, Ref, RefRelationType, SyncClass};

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_artifact_input(artifact_type: &str, storage_uri: &str) -> ArtifactInsert {
        ArtifactInsert {
            artifact_type: artifact_type.to_string(),
            title: Some(format!("{artifact_type} title")),
            mime_type: Some("application/json".to_string()),
            storage_uri: storage_uri.to_string(),
            storage_kind: ArtifactStorageKind::External,
            privacy_class: PrivacyClass::Private,
            sync_class: SyncClass::Warm,
            content_hash: Some("sha256:test".to_string()),
            size_bytes: Some(42),
            metadata_json: Some(json!({"origin":"test"})),
        }
    }

    #[tokio::test]
    async fn create_get_latest_and_list_artifacts() {
        let pool = test_pool().await;
        let first = create_artifact(
            &pool,
            sample_artifact_input("transcript", "file:///tmp/transcript-1.json"),
        )
        .await
        .unwrap();
        let second = create_artifact(
            &pool,
            sample_artifact_input("transcript", "file:///tmp/transcript-2.json"),
        )
        .await
        .unwrap();

        sqlx::query("UPDATE artifacts SET created_at = 100 WHERE artifact_id = ?")
            .bind(first.to_string())
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("UPDATE artifacts SET created_at = 200 WHERE artifact_id = ?")
            .bind(second.to_string())
            .execute(&pool)
            .await
            .unwrap();

        let fetched = get_artifact_by_id(&pool, first.as_ref())
            .await
            .unwrap()
            .expect("artifact should exist");
        assert_eq!(fetched.artifact_id, first);
        assert_eq!(fetched.storage_uri, "file:///tmp/transcript-1.json");

        let latest = get_latest_artifact_by_type(&pool, "transcript")
            .await
            .unwrap()
            .expect("latest artifact should exist");
        assert_eq!(latest.artifact_id, second);

        let listed = list_artifacts(&pool, 1).await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].artifact_id, second);
    }

    #[tokio::test]
    async fn create_artifact_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;
        let artifact_id = {
            let mut tx = pool.begin().await.unwrap();
            let id = create_artifact_in_tx(
                &mut tx,
                &sample_artifact_input("snapshot", "file:///tmp/snapshot.json"),
            )
            .await
            .unwrap();
            tx.rollback().await.unwrap();
            id
        };

        let fetched = get_artifact_by_id(&pool, artifact_id.as_ref())
            .await
            .unwrap();
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn list_artifacts_for_run_returns_linked_records_in_ref_order() {
        let pool = test_pool().await;
        let run_id = "run_artifact_link_test";
        let first = create_artifact(
            &pool,
            sample_artifact_input("snapshot", "file:///tmp/run-artifact-1.json"),
        )
        .await
        .unwrap();
        let second = create_artifact(
            &pool,
            sample_artifact_input("snapshot", "file:///tmp/run-artifact-2.json"),
        )
        .await
        .unwrap();

        let ref1 = Ref::new(
            "run",
            run_id,
            "artifact",
            first.to_string(),
            RefRelationType::GeneratedFrom,
        );
        let ref2 = Ref::new(
            "run",
            run_id,
            "artifact",
            second.to_string(),
            RefRelationType::GeneratedFrom,
        );
        sqlx::query(
            r#"
            INSERT INTO refs (ref_id, from_type, from_id, to_type, to_id, relation_type, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&ref1.id)
        .bind(&ref1.from_type)
        .bind(&ref1.from_id)
        .bind(&ref1.to_type)
        .bind(&ref1.to_id)
        .bind(ref1.relation_type.to_string())
        .bind(100_i64)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO refs (ref_id, from_type, from_id, to_type, to_id, relation_type, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&ref2.id)
        .bind(&ref2.from_type)
        .bind(&ref2.from_id)
        .bind(&ref2.to_type)
        .bind(&ref2.to_id)
        .bind(ref2.relation_type.to_string())
        .bind(200_i64)
        .execute(&pool)
        .await
        .unwrap();

        let linked = list_artifacts_for_run(&pool, run_id).await.unwrap();
        assert_eq!(linked.len(), 2);
        assert_eq!(linked[0].artifact_id, first);
        assert_eq!(linked[1].artifact_id, second);
    }
}
