use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Storage-agnostic, serde-backed, version-aware store seams for the `0.5` substrate.

#[derive(thiserror::Error, Debug)]
pub enum StorageContractError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict detected: {0}")]
    Conflict(String),
    #[error("unsupported capability: {0}")]
    UnsupportedCapability(String),
    #[error("storage failure: {0}")]
    StorageFailure(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StoreQuery {
    pub object_class: Option<String>,
    pub object_type: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub include_archived: bool,
    pub include_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionToken {
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredRecord {
    pub id: String,
    pub version: String,
    pub payload: JsonValue,
}

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn put_object(&self, record: StoredRecord) -> Result<(), StorageContractError>;
    async fn get_object(&self, id: &str) -> Result<Option<StoredRecord>, StorageContractError>;
    async fn query_objects(
        &self,
        query: &StoreQuery,
    ) -> Result<Vec<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait RegistryStore: Send + Sync {
    async fn put_registry_object(&self, record: StoredRecord) -> Result<(), StorageContractError>;
    async fn get_registry_object(
        &self,
        id: &str,
    ) -> Result<Option<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait RelationStore: Send + Sync {
    async fn put_relation(&self, record: StoredRecord) -> Result<(), StorageContractError>;
    async fn query_relations(
        &self,
        query: &StoreQuery,
    ) -> Result<Vec<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait SyncLinkStore: Send + Sync {
    async fn put_sync_link(&self, record: StoredRecord) -> Result<(), StorageContractError>;
    async fn get_sync_link(&self, id: &str) -> Result<Option<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait RuntimeStore: Send + Sync {
    async fn put_runtime_record(&self, record: StoredRecord) -> Result<(), StorageContractError>;
    async fn get_runtime_record(
        &self,
        id: &str,
    ) -> Result<Option<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait AuditStore: Send + Sync {
    async fn append_audit_entry(&self, entry: StoredRecord) -> Result<(), StorageContractError>;
    async fn list_audit_entries(
        &self,
        query: &StoreQuery,
    ) -> Result<Vec<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait ProjectionStore: Send + Sync {
    async fn put_projection(&self, record: StoredRecord) -> Result<(), StorageContractError>;
    async fn get_projection(&self, id: &str) -> Result<Option<StoredRecord>, StorageContractError>;
}

#[async_trait]
pub trait StorageTransaction: Send + Sync {
    async fn commit(self: Box<Self>) -> Result<(), StorageContractError>;
    async fn rollback(self: Box<Self>) -> Result<(), StorageContractError>;
}

#[async_trait]
pub trait TransactionManager: Send + Sync {
    async fn begin_transaction(&self) -> Result<Box<dyn StorageTransaction>, StorageContractError>;
}

#[cfg(test)]
mod tests {
    use super::{RevisionToken, StoreQuery};

    #[test]
    fn store_query_defaults_to_safe_visibility_flags() {
        let query = StoreQuery::default();
        assert!(!query.include_archived);
        assert!(!query.include_deleted);
    }

    #[test]
    fn revision_token_is_version_shaped() {
        let token = RevisionToken {
            version: "0.5".to_string(),
        };
        assert_eq!(token.version, "0.5");
    }
}
