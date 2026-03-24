use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
use time::OffsetDateTime;

use crate::storage_backend::StoredRecord;
use crate::{
    get_registry_object, upsert_registry_object, CanonicalRegistryRecord, RegistryStore,
    StorageContractError,
};

#[derive(Clone)]
pub struct SqliteModuleRegistryStore {
    pool: SqlitePool,
}

impl SqliteModuleRegistryStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegistryStore for SqliteModuleRegistryStore {
    async fn put_registry_object(&self, record: StoredRecord) -> Result<(), StorageContractError> {
        let payload = record.payload;
        let now = OffsetDateTime::now_utc();
        let registry = CanonicalRegistryRecord {
            id: record.id,
            registry_type: payload_value(&payload, "registry_kind")?,
            namespace: payload_value(&payload, "namespace")?,
            slug: payload_value(&payload, "slug")?,
            display_name: payload_value(&payload, "display_name")?,
            version: record.version,
            status: payload_value(&payload, "status")?,
            manifest_ref: payload_value(&payload, "manifest_ref")?,
            overlay_json: payload
                .get("persisted_overlay")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({})),
            created_at: now,
            updated_at: now,
        };

        upsert_registry_object(&self.pool, &registry)
            .await
            .map_err(|error| StorageContractError::StorageFailure(error.to_string()))
    }

    async fn get_registry_object(
        &self,
        id: &str,
    ) -> Result<Option<StoredRecord>, StorageContractError> {
        let record = get_registry_object(&self.pool, id)
            .await
            .map_err(|error| StorageContractError::StorageFailure(error.to_string()))?;

        record
            .map(|record| StoredRecord {
                id: record.id,
                version: record.version,
                payload: serde_json::json!({
                    "registry_kind": record.registry_type,
                    "namespace": record.namespace,
                    "slug": record.slug,
                    "display_name": record.display_name,
                    "status": record.status,
                    "manifest_ref": record.manifest_ref,
                    "persisted_overlay": record.overlay_json,
                }),
            })
            .map(Some)
            .ok_or_else(|| {
                StorageContractError::NotFound(format!("module registry {id} not found"))
            })
            .or(Ok(None))
    }
}

fn payload_value(payload: &JsonValue, field: &str) -> Result<String, StorageContractError> {
    payload
        .get(field)
        .and_then(JsonValue::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| StorageContractError::Validation(format!("module registry missing {field}")))
}

#[cfg(test)]
mod tests {
    use super::SqliteModuleRegistryStore;
    use crate::storage_backend::StoredRecord;
    use crate::{migrate_storage, RegistryStore};
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn registry_store_round_trips_module_registry_records() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        let store = SqliteModuleRegistryStore::new(pool);

        store
            .put_registry_object(StoredRecord {
                id: "module.integration.todoist".to_string(),
                version: "0.5".to_string(),
                payload: serde_json::json!({
                    "registry_kind": "module",
                    "namespace": "integration",
                    "slug": "todoist",
                    "display_name": "Todoist",
                    "status": "active",
                    "manifest_ref": "modules/integration/todoist/module.yaml",
                    "persisted_overlay": {"enabled": true}
                }),
            })
            .await
            .unwrap();

        let stored = store
            .get_registry_object("module.integration.todoist")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(stored.version, "0.5");
        assert_eq!(stored.payload["namespace"], "integration");
        assert_eq!(stored.payload["persisted_overlay"]["enabled"], true);
    }
}
