use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalObjectRecord {
    pub id: String,
    pub object_type: String,
    pub object_class: String,
    pub schema_version: String,
    pub revision: i64,
    pub status: String,
    pub provenance_json: JsonValue,
    pub facets_json: JsonValue,
    pub source_summary_json: Option<JsonValue>,
    pub deleted_at: Option<OffsetDateTime>,
    pub archived_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn insert_canonical_object(
    pool: &SqlitePool,
    record: &CanonicalObjectRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO canonical_objects (
            id,
            object_type,
            object_class,
            schema_version,
            revision,
            status,
            provenance_json,
            facets_json,
            source_summary_json,
            deleted_at,
            archived_at,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&record.id)
    .bind(&record.object_type)
    .bind(&record.object_class)
    .bind(&record.schema_version)
    .bind(record.revision)
    .bind(&record.status)
    .bind(serde_json::to_string(&record.provenance_json)?)
    .bind(serde_json::to_string(&record.facets_json)?)
    .bind(
        record
            .source_summary_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?,
    )
    .bind(record.deleted_at.map(|value| value.unix_timestamp()))
    .bind(record.archived_at.map(|value| value.unix_timestamp()))
    .bind(record.created_at.unix_timestamp())
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_canonical_object(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<CanonicalObjectRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            object_type,
            object_class,
            schema_version,
            revision,
            status,
            provenance_json,
            facets_json,
            source_summary_json,
            deleted_at,
            archived_at,
            created_at,
            updated_at
        FROM canonical_objects
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_canonical_object_row).transpose()
}

pub async fn update_canonical_object(
    pool: &SqlitePool,
    id: &str,
    expected_revision: i64,
    status: &str,
    facets_json: &JsonValue,
    source_summary_json: Option<&JsonValue>,
    archived_at: Option<OffsetDateTime>,
) -> Result<CanonicalObjectRecord, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let result = sqlx::query(
        r#"
        UPDATE canonical_objects
        SET
            revision = revision + 1,
            status = ?,
            facets_json = ?,
            source_summary_json = ?,
            archived_at = ?,
            updated_at = ?
        WHERE id = ? AND revision = ?
        "#,
    )
    .bind(status)
    .bind(serde_json::to_string(facets_json)?)
    .bind(source_summary_json.map(serde_json::to_string).transpose()?)
    .bind(archived_at.map(|value| value.unix_timestamp()))
    .bind(now)
    .bind(id)
    .bind(expected_revision)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(StorageError::Validation(format!(
            "canonical object stale revision conflict for {id} at revision {expected_revision}"
        )));
    }

    get_canonical_object(pool, id).await?.ok_or_else(|| {
        StorageError::NotFound(format!("canonical object {id} missing after update"))
    })
}

fn map_canonical_object_row(row: &sqlx::sqlite::SqliteRow) -> Result<CanonicalObjectRecord, StorageError> {
    Ok(CanonicalObjectRecord {
        id: row.try_get("id")?,
        object_type: row.try_get("object_type")?,
        object_class: row.try_get("object_class")?,
        schema_version: row.try_get("schema_version")?,
        revision: row.try_get("revision")?,
        status: row.try_get("status")?,
        provenance_json: serde_json::from_str(&row.try_get::<String, _>("provenance_json")?)?,
        facets_json: serde_json::from_str(&row.try_get::<String, _>("facets_json")?)?,
        source_summary_json: row
            .try_get::<Option<String>, _>("source_summary_json")?
            .map(|value| serde_json::from_str(&value))
            .transpose()?,
        deleted_at: row
            .try_get::<Option<i64>, _>("deleted_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        archived_at: row
            .try_get::<Option<i64>, _>("archived_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::migrate::Migrator;

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_record() -> CanonicalObjectRecord {
        let now = OffsetDateTime::now_utc();
        CanonicalObjectRecord {
            id: "task_01phase58".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"seeded"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: Some(json!({"active_link_count": 0})),
            deleted_at: None,
            archived_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn canonical_object_repo_persists_revisioned_objects() {
        let pool = test_pool().await;
        let record = sample_record();

        insert_canonical_object(&pool, &record).await.unwrap();
        let fetched = get_canonical_object(&pool, &record.id)
            .await
            .unwrap()
            .expect("canonical object should exist");

        assert_eq!(fetched.revision, 1);
        assert_eq!(fetched.object_class, "content");
        assert_eq!(fetched.schema_version, "0.5");
        assert!(fetched.archived_at.is_none());
    }

    #[tokio::test]
    async fn canonical_object_repo_enforces_revision_ready_updates() {
        let pool = test_pool().await;
        let record = sample_record();
        insert_canonical_object(&pool, &record).await.unwrap();

        let updated = update_canonical_object(
            &pool,
            &record.id,
            1,
            "archived",
            &json!({"task_type":"generic","archived_reason":"test"}),
            Some(&json!({"active_link_count": 1})),
            Some(OffsetDateTime::now_utc()),
        )
        .await
        .unwrap();

        assert_eq!(updated.revision, 2);
        assert_eq!(updated.status, "archived");

        let stale = update_canonical_object(
            &pool,
            &record.id,
            1,
            "deleted",
            &json!({}),
            None,
            None,
        )
        .await;

        assert!(stale.is_err(), "stale revision should fail");
    }
}
