use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionRecord {
    pub id: String,
    pub projection_type: String,
    pub object_id: Option<String>,
    pub source_summary_json: Option<JsonValue>,
    pub projection_json: JsonValue,
    pub rebuild_token: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn upsert_projection(
    pool: &SqlitePool,
    record: &ProjectionRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO projections (
            id,
            projection_type,
            object_id,
            source_summary_json,
            projection_json,
            rebuild_token,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            projection_type = excluded.projection_type,
            object_id = excluded.object_id,
            source_summary_json = excluded.source_summary_json,
            projection_json = excluded.projection_json,
            rebuild_token = excluded.rebuild_token,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&record.id)
    .bind(&record.projection_type)
    .bind(&record.object_id)
    .bind(
        record
            .source_summary_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?,
    )
    .bind(serde_json::to_string(&record.projection_json)?)
    .bind(&record.rebuild_token)
    .bind(record.created_at.unix_timestamp())
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_projection(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<ProjectionRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            projection_type,
            object_id,
            source_summary_json,
            projection_json,
            rebuild_token,
            created_at,
            updated_at
        FROM projections
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_projection_row).transpose()
}

pub async fn rebuild_projection(
    pool: &SqlitePool,
    id: &str,
    source_summary_json: Option<&JsonValue>,
    projection_json: &JsonValue,
    rebuild_token: &str,
) -> Result<ProjectionRecord, StorageError> {
    sqlx::query(
        r#"
        UPDATE projections
        SET source_summary_json = ?, projection_json = ?, rebuild_token = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(source_summary_json.map(serde_json::to_string).transpose()?)
    .bind(serde_json::to_string(projection_json)?)
    .bind(rebuild_token)
    .bind(OffsetDateTime::now_utc().unix_timestamp())
    .bind(id)
    .execute(pool)
    .await?;

    get_projection(pool, id)
        .await?
        .ok_or_else(|| StorageError::NotFound(format!("projection {id} missing after rebuild")))
}

fn map_projection_row(row: &sqlx::sqlite::SqliteRow) -> Result<ProjectionRecord, StorageError> {
    Ok(ProjectionRecord {
        id: row.try_get("id")?,
        projection_type: row.try_get("projection_type")?,
        object_id: row.try_get("object_id")?,
        source_summary_json: row
            .try_get::<Option<String>, _>("source_summary_json")?
            .map(|value| serde_json::from_str(&value))
            .transpose()?,
        projection_json: serde_json::from_str(&row.try_get::<String, _>("projection_json")?)?,
        rebuild_token: row.try_get("rebuild_token")?,
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

    #[tokio::test]
    async fn projections_repo_rebuilds_source_summary_without_becoming_truth() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();
        let record = ProjectionRecord {
            id: "projection_01source_summary".to_string(),
            projection_type: "source_summary".to_string(),
            object_id: Some("task_01phase58".to_string()),
            source_summary_json: Some(json!({"active_link_count": 0})),
            projection_json: json!({"providers":[]}),
            rebuild_token: Some("initial".to_string()),
            created_at: now,
            updated_at: now,
        };

        upsert_projection(&pool, &record).await.unwrap();
        let rebuilt = rebuild_projection(
            &pool,
            &record.id,
            Some(&json!({"active_link_count": 1})),
            &json!({"providers":["todoist"]}),
            "rebuild-2",
        )
        .await
        .unwrap();

        assert_eq!(rebuilt.projection_type, "source_summary");
        assert_eq!(
            rebuilt.source_summary_json,
            Some(json!({"active_link_count": 1}))
        );
        assert_eq!(rebuilt.rebuild_token.as_deref(), Some("rebuild-2"));
    }
}
