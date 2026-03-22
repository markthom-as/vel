use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalRelationRecord {
    pub id: String,
    pub relation_type: String,
    pub from_id: String,
    pub to_id: String,
    pub direction: String,
    pub active: bool,
    pub source_json: JsonValue,
    pub confidence: Option<f64>,
    pub revision: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn upsert_relation(
    pool: &SqlitePool,
    record: &CanonicalRelationRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO canonical_relations (
            id,
            relation_type,
            from_id,
            to_id,
            direction,
            active,
            source_json,
            confidence,
            revision,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(from_id, relation_type, to_id, direction) DO UPDATE SET
            active = excluded.active,
            source_json = excluded.source_json,
            confidence = excluded.confidence,
            revision = canonical_relations.revision + 1,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&record.id)
    .bind(&record.relation_type)
    .bind(&record.from_id)
    .bind(&record.to_id)
    .bind(&record.direction)
    .bind(record.active)
    .bind(serde_json::to_string(&record.source_json)?)
    .bind(record.confidence)
    .bind(record.revision)
    .bind(record.created_at.unix_timestamp())
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_relations_from(
    pool: &SqlitePool,
    from_id: &str,
) -> Result<Vec<CanonicalRelationRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            relation_type,
            from_id,
            to_id,
            direction,
            active,
            source_json,
            confidence,
            revision,
            created_at,
            updated_at
        FROM canonical_relations
        WHERE from_id = ?
        ORDER BY relation_type ASC, to_id ASC
        "#,
    )
    .bind(from_id)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_relation_row).collect()
}

pub(crate) fn map_relation_row_for_query(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<CanonicalRelationRecord, StorageError> {
    Ok(CanonicalRelationRecord {
        id: row.try_get("id")?,
        relation_type: row.try_get("relation_type")?,
        from_id: row.try_get("from_id")?,
        to_id: row.try_get("to_id")?,
        direction: row.try_get("direction")?,
        active: row.try_get("active")?,
        source_json: serde_json::from_str(&row.try_get::<String, _>("source_json")?)?,
        confidence: row.try_get("confidence")?,
        revision: row.try_get("revision")?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
    })
}

fn map_relation_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<CanonicalRelationRecord, StorageError> {
    map_relation_row_for_query(row)
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
    async fn relations_repo_persists_directional_typed_relations() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();
        let relation = CanonicalRelationRecord {
            id: "rel_01phase58".to_string(),
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
        };

        upsert_relation(&pool, &relation).await.unwrap();
        let listed = list_relations_from(&pool, "task_01phase58").await.unwrap();

        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].relation_type, "task -> project");
        assert_eq!(listed[0].from_id, "task_01phase58");
        assert_eq!(listed[0].to_id, "project_01phase58");
        assert!(listed[0].active);
    }
}
