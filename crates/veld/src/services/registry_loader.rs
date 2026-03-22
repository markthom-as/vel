use serde_json::Value as JsonValue;
use vel_core::{
    DefaultRegistryReconciler, ManifestSource, PersistedOverlay, RegistryLoader as RegistryLoaderRole,
    RegistryObject, RegistryReconciler, ReconciliationResult,
};
use vel_storage::{RegistryStore, StoredRecord};

use crate::errors::AppError;

pub struct RegistryLoader<S, T, R = DefaultRegistryReconciler> {
    source: S,
    store: T,
    reconciler: R,
}

impl<S, T> RegistryLoader<S, T, DefaultRegistryReconciler> {
    pub fn new(source: S, store: T) -> Self {
        Self {
            source,
            store,
            reconciler: DefaultRegistryReconciler,
        }
    }
}

impl<S, T, R> RegistryLoader<S, T, R> {
    pub fn with_reconciler(source: S, store: T, reconciler: R) -> Self {
        Self {
            source,
            store,
            reconciler,
        }
    }
}

impl<S, T, R> RegistryLoaderRole for RegistryLoader<S, T, R> {}

impl<S, T, R> RegistryLoader<S, T, R>
where
    S: ManifestSource,
    T: RegistryStore,
    R: RegistryReconciler,
{
    pub async fn load_all(&self) -> Result<Vec<ReconciliationResult>, AppError> {
        let manifests = self
            .source
            .load_manifests()
            .map_err(AppError::bad_request)?;
        let mut results = Vec::with_capacity(manifests.len());

        for manifest in manifests {
            let existing_record = self
                .store
                .get_registry_object(&manifest.registry_id.as_string())
                .await
                .map_err(|error| AppError::internal(error.to_string()))?;
            let existing_object = existing_record
                .as_ref()
                .map(stored_record_to_registry_object)
                .transpose()
                .map_err(AppError::bad_request)?;
            let persisted_overlay = existing_record
                .as_ref()
                .and_then(|record| record.payload.get("persisted_overlay"))
                .cloned()
                .map(json_to_overlay)
                .transpose()
                .map_err(AppError::bad_request)?;

            let result = self
                .reconciler
                .reconcile(&manifest, persisted_overlay, existing_object.as_ref());

            self.store
                .put_registry_object(registry_object_to_stored_record(&result.object))
                .await
                .map_err(|error| AppError::internal(error.to_string()))?;
            results.push(result);
        }

        Ok(results)
    }
}

fn stored_record_to_registry_object(record: &StoredRecord) -> Result<RegistryObject, String> {
    Ok(RegistryObject {
        id: record.id.clone(),
        registry_kind: serde_json::from_value(record.payload["registry_kind"].clone())
            .map_err(|error| error.to_string())?,
        namespace: required_string(&record.payload, "namespace")?,
        slug: required_string(&record.payload, "slug")?,
        display_name: required_string(&record.payload, "display_name")?,
        version: record.version.clone(),
        status: serde_json::from_value(record.payload["status"].clone())
            .map_err(|error| error.to_string())?,
        manifest_ref: required_string(&record.payload, "manifest_ref")?,
        capability_requests: record
            .payload
            .get("capability_requests")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .map_err(|error| error.to_string())?
            .unwrap_or_default(),
        persisted_overlay: record
            .payload
            .get("persisted_overlay")
            .cloned()
            .map(json_to_overlay)
            .transpose()?
            .unwrap_or(PersistedOverlay {
                enabled: None,
                notes: None,
                metadata: serde_json::json!({}),
            }),
    })
}

fn registry_object_to_stored_record(object: &RegistryObject) -> StoredRecord {
    StoredRecord {
        id: object.id.clone(),
        version: object.version.clone(),
        payload: serde_json::json!({
            "registry_kind": object.registry_kind,
            "namespace": object.namespace,
            "slug": object.slug,
            "display_name": object.display_name,
            "status": object.status,
            "manifest_ref": object.manifest_ref,
            "capability_requests": object.capability_requests,
            "persisted_overlay": object.persisted_overlay,
        }),
    }
}

fn required_string(payload: &JsonValue, field: &str) -> Result<String, String> {
    payload
        .get(field)
        .and_then(JsonValue::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("registry payload missing {field}"))
}

fn json_to_overlay(value: JsonValue) -> Result<PersistedOverlay, String> {
    serde_json::from_value(value).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use vel_core::{
        CapabilityRequest, ManifestSource, MODULE_INTEGRATION_GOOGLE_CALENDAR,
        MODULE_INTEGRATION_TODOIST, RegistryKind, RegistryManifest, RegistryStatus,
        SemanticRegistryId,
    };
    use vel_storage::{RegistryStore, StorageContractError, StoredRecord};

    use super::RegistryLoader;

    struct StaticManifestSource;

    impl ManifestSource for StaticManifestSource {
        fn load_manifests(&self) -> Result<Vec<RegistryManifest>, String> {
            Ok(vec![
                RegistryManifest {
                    registry_id: SemanticRegistryId::new(
                        RegistryKind::Module,
                        "integration",
                        "todoist",
                    ),
                    display_name: "Todoist".to_string(),
                    version: "0.5".to_string(),
                    status: RegistryStatus::Active,
                    manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
                    capability_requests: vec![CapabilityRequest {
                        capability: "todoist.read".to_string(),
                        feature_gate: Some("todoist".to_string()),
                    }],
                },
                RegistryManifest {
                    registry_id: SemanticRegistryId::new(
                        RegistryKind::Module,
                        "integration",
                        "google-calendar",
                    ),
                    display_name: "Google Calendar".to_string(),
                    version: "0.5".to_string(),
                    status: RegistryStatus::Active,
                    manifest_ref: "modules/integration/google-calendar/module.yaml".to_string(),
                    capability_requests: vec![CapabilityRequest {
                        capability: "google.calendar.read".to_string(),
                        feature_gate: Some("google-calendar".to_string()),
                    }],
                },
            ])
        }
    }

    #[derive(Clone, Default)]
    struct MemoryRegistryStore {
        records: Arc<Mutex<HashMap<String, StoredRecord>>>,
    }

    #[async_trait]
    impl RegistryStore for MemoryRegistryStore {
        async fn put_registry_object(
            &self,
            record: StoredRecord,
        ) -> Result<(), StorageContractError> {
            self.records.lock().unwrap().insert(record.id.clone(), record);
            Ok(())
        }

        async fn get_registry_object(
            &self,
            id: &str,
        ) -> Result<Option<StoredRecord>, StorageContractError> {
            Ok(self.records.lock().unwrap().get(id).cloned())
        }
    }

    #[tokio::test]
    async fn registry_loader_materializes_manifests_into_canonical_registry_entities() {
        let store = MemoryRegistryStore::default();
        let loader = RegistryLoader::new(StaticManifestSource, store.clone());

        let results = loader.load_all().await.unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|result| result.object.id == MODULE_INTEGRATION_TODOIST));
        assert!(results
            .iter()
            .any(|result| result.object.id == MODULE_INTEGRATION_GOOGLE_CALENDAR));
        assert!(store
            .records
            .lock()
            .unwrap()
            .contains_key(MODULE_INTEGRATION_TODOIST));
    }
}
