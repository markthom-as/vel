use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{ActionResponseEnvelope, OBJECT_EXPLAIN, OBJECT_GET, OBJECT_QUERY, OBJECT_UPDATE};
use vel_storage::{
    get_canonical_object, list_sync_links_for_object, query_canonical_objects,
    update_canonical_object, CanonicalObjectQuery, CanonicalObjectSort, CanonicalObjectSortField,
    QuerySortDirection, StorageError,
};

use crate::errors::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectGetInput {
    pub object_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ObjectQueryInput {
    pub object_class: Option<String>,
    pub object_type: Option<String>,
    pub include_archived: bool,
    pub include_deleted: bool,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectUpdateInput {
    pub object_id: String,
    pub expected_revision: i64,
    pub status: String,
    pub facets_json: JsonValue,
    pub source_summary_json: Option<JsonValue>,
    pub archived_at: Option<OffsetDateTime>,
}

pub async fn execute_object_get(
    pool: &SqlitePool,
    input: &ObjectGetInput,
) -> Result<ActionResponseEnvelope, AppError> {
    let object = get_canonical_object(pool, &input.object_id)
        .await
        .map_err(map_storage_error)?
        .ok_or_else(|| AppError::not_found(format!("object {} not found", input.object_id)))?;

    Ok(ActionResponseEnvelope {
        action_name: OBJECT_GET.to_string(),
        output: serde_json::to_value(object)
            .map_err(|error| AppError::internal(error.to_string()))?,
        explain: None,
    })
}

pub async fn execute_object_query(
    pool: &SqlitePool,
    input: &ObjectQueryInput,
) -> Result<ActionResponseEnvelope, AppError> {
    let results = query_canonical_objects(
        pool,
        &CanonicalObjectQuery {
            object_class: input.object_class.clone(),
            object_type: input.object_type.clone(),
            include_archived: input.include_archived,
            include_deleted: input.include_deleted,
            limit: input.limit,
            sort: CanonicalObjectSort {
                field: CanonicalObjectSortField::UpdatedAt,
                direction: QuerySortDirection::Desc,
            },
            ..Default::default()
        },
    )
    .await
    .map_err(map_storage_error)?;

    Ok(ActionResponseEnvelope {
        action_name: OBJECT_QUERY.to_string(),
        output: serde_json::to_value(results)
            .map_err(|error| AppError::internal(error.to_string()))?,
        explain: None,
    })
}

pub async fn execute_object_update(
    pool: &SqlitePool,
    input: &ObjectUpdateInput,
) -> Result<ActionResponseEnvelope, AppError> {
    let updated = update_canonical_object(
        pool,
        &input.object_id,
        input.expected_revision,
        &input.status,
        &input.facets_json,
        input.source_summary_json.as_ref(),
        input.archived_at,
    )
    .await
    .map_err(map_storage_error)?;

    Ok(ActionResponseEnvelope {
        action_name: OBJECT_UPDATE.to_string(),
        output: serde_json::to_value(updated)
            .map_err(|error| AppError::internal(error.to_string()))?,
        explain: None,
    })
}

pub async fn execute_object_explain(
    pool: &SqlitePool,
    input: &ObjectGetInput,
) -> Result<ActionResponseEnvelope, AppError> {
    let object = get_canonical_object(pool, &input.object_id)
        .await
        .map_err(map_storage_error)?
        .ok_or_else(|| AppError::not_found(format!("object {} not found", input.object_id)))?;
    let sync_links = list_sync_links_for_object(pool, &input.object_id)
        .await
        .map_err(map_storage_error)?;

    Ok(ActionResponseEnvelope {
        action_name: OBJECT_EXPLAIN.to_string(),
        output: json!({
            "object_id": object.id,
            "status": object.status,
            "revision": object.revision,
            "source_summary": object.source_summary_json,
            "link_count": sync_links.len(),
        }),
        explain: Some(json!({
            "basis": "exact",
            "policy_relevant_status": object.status,
            "source_summary": object.source_summary_json,
            "linked_provider_count": sync_links.len(),
        })),
    })
}

fn map_storage_error(error: StorageError) -> AppError {
    match error {
        StorageError::NotFound(message) => AppError::not_found(message),
        StorageError::Validation(message) => AppError::bad_request(message),
        other => AppError::internal(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        execute_object_explain, execute_object_get, execute_object_query, execute_object_update,
        ObjectGetInput, ObjectQueryInput, ObjectUpdateInput,
    };
    use serde_json::json;
    use sqlx::SqlitePool;
    use time::OffsetDateTime;
    use vel_storage::{insert_canonical_object, migrate_storage, CanonicalObjectRecord};

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        pool
    }

    async fn seed_object(pool: &SqlitePool) -> CanonicalObjectRecord {
        let now = OffsetDateTime::now_utc();
        let object = CanonicalObjectRecord {
            id: "task_01action".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"user"}),
            facets_json: json!({"task_type":"generic"}),
            source_summary_json: Some(json!({"active_link_count": 0})),
            deleted_at: None,
            archived_at: None,
            created_at: now,
            updated_at: now,
        };
        insert_canonical_object(pool, &object).await.unwrap();
        object
    }

    #[tokio::test]
    async fn object_get_and_query_return_serialized_payloads() {
        let pool = test_pool().await;
        let object = seed_object(&pool).await;

        let get = execute_object_get(
            &pool,
            &ObjectGetInput {
                object_id: object.id.clone(),
            },
        )
        .await
        .unwrap();
        let query = execute_object_query(
            &pool,
            &ObjectQueryInput {
                object_type: Some("task".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(get.action_name, "object.get");
        assert!(query.output.as_array().is_some());
    }

    #[tokio::test]
    async fn object_update_and_explain_work_over_phase58_substrate() {
        let pool = test_pool().await;
        let object = seed_object(&pool).await;

        let updated = execute_object_update(
            &pool,
            &ObjectUpdateInput {
                object_id: object.id.clone(),
                expected_revision: 1,
                status: "archived".to_string(),
                facets_json: json!({"task_type":"generic","note":"updated"}),
                source_summary_json: Some(json!({"active_link_count": 1})),
                archived_at: Some(OffsetDateTime::now_utc()),
            },
        )
        .await
        .unwrap();
        let explain = execute_object_explain(
            &pool,
            &ObjectGetInput {
                object_id: object.id,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.action_name, "object.update");
        assert_eq!(explain.action_name, "object.explain");
        assert!(explain.explain.is_some());
    }
}
