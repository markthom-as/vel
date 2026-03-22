use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalRegistryRecord {
    pub id: String,
    pub registry_type: String,
    pub namespace: String,
    pub slug: String,
    pub display_name: String,
    pub version: String,
    pub status: String,
    pub manifest_ref: String,
    pub overlay_json: JsonValue,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn upsert_registry_object(
    pool: &SqlitePool,
    record: &CanonicalRegistryRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO canonical_registry_objects (
            id,
            registry_type,
            namespace,
            slug,
            display_name,
            version,
            status,
            manifest_ref,
            overlay_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            registry_type = excluded.registry_type,
            namespace = excluded.namespace,
            slug = excluded.slug,
            display_name = excluded.display_name,
            version = excluded.version,
            status = excluded.status,
            manifest_ref = excluded.manifest_ref,
            overlay_json = excluded.overlay_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&record.id)
    .bind(&record.registry_type)
    .bind(&record.namespace)
    .bind(&record.slug)
    .bind(&record.display_name)
    .bind(&record.version)
    .bind(&record.status)
    .bind(&record.manifest_ref)
    .bind(serde_json::to_string(&record.overlay_json)?)
    .bind(record.created_at.unix_timestamp())
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_registry_object(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<CanonicalRegistryRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            registry_type,
            namespace,
            slug,
            display_name,
            version,
            status,
            manifest_ref,
            overlay_json,
            created_at,
            updated_at
        FROM canonical_registry_objects
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_registry_row).transpose()
}

pub async fn list_registry_objects(
    pool: &SqlitePool,
) -> Result<Vec<CanonicalRegistryRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            registry_type,
            namespace,
            slug,
            display_name,
            version,
            status,
            manifest_ref,
            overlay_json,
            created_at,
            updated_at
        FROM canonical_registry_objects
        ORDER BY namespace ASC, slug ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_registry_row).collect()
}

fn map_registry_row(row: &sqlx::sqlite::SqliteRow) -> Result<CanonicalRegistryRecord, StorageError> {
    Ok(CanonicalRegistryRecord {
        id: row.try_get("id")?,
        registry_type: row.try_get("registry_type")?,
        namespace: row.try_get("namespace")?,
        slug: row.try_get("slug")?,
        display_name: row.try_get("display_name")?,
        version: row.try_get("version")?,
        status: row.try_get("status")?,
        manifest_ref: row.try_get("manifest_ref")?,
        overlay_json: serde_json::from_str(&row.try_get::<String, _>("overlay_json")?)?,
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
    async fn registry_repo_persists_module_registry_records() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();

        let todoist = CanonicalRegistryRecord {
            id: "module.integration.todoist".to_string(),
            registry_type: "module".to_string(),
            namespace: "integration".to_string(),
            slug: "todoist".to_string(),
            display_name: "Todoist".to_string(),
            version: "0.5".to_string(),
            status: "active".to_string(),
            manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
            overlay_json: json!({"preset":"read_only"}),
            created_at: now,
            updated_at: now,
        };

        let gcal = CanonicalRegistryRecord {
            id: "module.integration.google-calendar".to_string(),
            registry_type: "module".to_string(),
            namespace: "integration".to_string(),
            slug: "google-calendar".to_string(),
            display_name: "Google Calendar".to_string(),
            version: "0.5".to_string(),
            status: "active".to_string(),
            manifest_ref: "modules/integration/google-calendar/module.yaml".to_string(),
            overlay_json: json!({"preset":"conservative_sync"}),
            created_at: now,
            updated_at: now,
        };

        upsert_registry_object(&pool, &todoist).await.unwrap();
        upsert_registry_object(&pool, &gcal).await.unwrap();

        let fetched = get_registry_object(&pool, "module.integration.todoist")
            .await
            .unwrap()
            .expect("todoist registry record should exist");
        assert_eq!(fetched.namespace, "integration");

        let listed = list_registry_objects(&pool).await.unwrap();
        assert_eq!(listed.len(), 2);
        assert!(listed
            .iter()
            .any(|record| record.id == "module.integration.google-calendar"));
    }
}
