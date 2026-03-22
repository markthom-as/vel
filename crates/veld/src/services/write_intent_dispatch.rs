use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_storage::{insert_runtime_record, RuntimeRecord};

use crate::errors::AppError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownstreamOperation {
    pub downstream_operation_ref: String,
    pub status: String,
    pub result: Option<JsonValue>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DispatchDisposition {
    Succeeded { result: JsonValue },
    Failed { error: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteIntentDispatchRequest {
    pub write_intent_id: String,
    pub action_name: String,
    pub target_object_refs: Vec<String>,
    pub provider: Option<String>,
    pub integration_account_id: Option<String>,
    pub requested_change: JsonValue,
    pub approved: bool,
    pub dry_run: bool,
    pub downstream_operation_ref: String,
    pub dispatch: DispatchDisposition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionDispatch {
    pub write_intent_id: String,
    pub approved_record_id: String,
    pub executing_record_id: String,
    pub terminal_record_id: String,
    pub downstream: DownstreamOperation,
}

pub async fn dispatch_write_intent(
    pool: &SqlitePool,
    request: &WriteIntentDispatchRequest,
) -> Result<ExecutionDispatch, AppError> {
    if !request.approved {
        return Err(AppError::bad_request(
            "write intent dispatch requires approved state before execution",
        ));
    }
    if request.dry_run {
        return Err(AppError::bad_request(
            "dry_run write intents must not enter execution dispatch",
        ));
    }

    let now = OffsetDateTime::now_utc();
    let object_ref = request.target_object_refs.first().cloned();
    let approved_record_id = format!("{}__approved", request.write_intent_id);
    let executing_record_id = format!("{}__executing", request.write_intent_id);
    let terminal_record_id = match &request.dispatch {
        DispatchDisposition::Succeeded { .. } => format!("{}__succeeded", request.write_intent_id),
        DispatchDisposition::Failed { .. } => format!("{}__failed", request.write_intent_id),
    };

    let downstream_operation_refs = vec![request.downstream_operation_ref.clone()];

    let approved_record = RuntimeRecord {
        id: approved_record_id.clone(),
        record_type: "write_intent".to_string(),
        object_ref: object_ref.clone(),
        status: "approved".to_string(),
        payload_json: serde_json::json!({
            "write_intent_id": request.write_intent_id,
            "action_name": request.action_name,
            "provider": request.provider,
            "integration_account_id": request.integration_account_id,
            "requested_change": request.requested_change,
            "target_object_refs": request.target_object_refs,
            "downstream_operation_refs": downstream_operation_refs,
        }),
        created_at: now,
        updated_at: now,
    };
    insert_runtime_record(pool, &approved_record).await?;

    let executing_record = RuntimeRecord {
        id: executing_record_id.clone(),
        record_type: "write_intent".to_string(),
        object_ref: object_ref.clone(),
        status: "executing".to_string(),
        payload_json: serde_json::json!({
            "write_intent_id": request.write_intent_id,
            "dispatch": "downstream",
            "downstream_operation_refs": downstream_operation_refs,
        }),
        created_at: now,
        updated_at: now,
    };
    insert_runtime_record(pool, &executing_record).await?;

    let downstream = match &request.dispatch {
        DispatchDisposition::Succeeded { result } => DownstreamOperation {
            downstream_operation_ref: request.downstream_operation_ref.clone(),
            status: "succeeded".to_string(),
            result: Some(result.clone()),
            error: None,
        },
        DispatchDisposition::Failed { error } => DownstreamOperation {
            downstream_operation_ref: request.downstream_operation_ref.clone(),
            status: "failed".to_string(),
            result: None,
            error: Some(error.clone()),
        },
    };

    let downstream_record = RuntimeRecord {
        id: request.downstream_operation_ref.clone(),
        record_type: "write_intent_downstream".to_string(),
        object_ref: object_ref.clone(),
        status: downstream.status.clone(),
        payload_json: serde_json::to_value(&downstream)
            .map_err(|error| AppError::internal(error.to_string()))?,
        created_at: now,
        updated_at: now,
    };
    insert_runtime_record(pool, &downstream_record).await?;

    let terminal_status = downstream.status.clone();
    let terminal_record = RuntimeRecord {
        id: terminal_record_id.clone(),
        record_type: "write_intent".to_string(),
        object_ref,
        status: terminal_status,
        payload_json: serde_json::json!({
            "write_intent_id": request.write_intent_id,
            "action_name": request.action_name,
            "downstream_operation_refs": downstream_operation_refs,
            "downstream": downstream,
        }),
        created_at: now,
        updated_at: now,
    };
    insert_runtime_record(pool, &terminal_record).await?;

    Ok(ExecutionDispatch {
        write_intent_id: request.write_intent_id.clone(),
        approved_record_id,
        executing_record_id,
        terminal_record_id,
        downstream,
    })
}

#[cfg(test)]
mod tests {
    use super::{dispatch_write_intent, DispatchDisposition, WriteIntentDispatchRequest};
    use serde_json::json;
    use sqlx::SqlitePool;
    use vel_storage::{list_runtime_records, migrate_storage};

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn write_intent_dispatch_records_approved_executing_and_succeeded_downstream_state() {
        let pool = test_pool().await;
        let outcome = dispatch_write_intent(
            &pool,
            &WriteIntentDispatchRequest {
                write_intent_id: "write_intent_01dispatch".to_string(),
                action_name: "object.update".to_string(),
                target_object_refs: vec!["task_01dispatch".to_string()],
                provider: Some("todoist".to_string()),
                integration_account_id: Some("integration_account_01dispatch".to_string()),
                requested_change: json!({"field":"due","to":"2026-03-23"}),
                approved: true,
                dry_run: false,
                downstream_operation_ref: "downstream_01dispatch".to_string(),
                dispatch: DispatchDisposition::Succeeded {
                    result: json!({"remote_version":"v2"}),
                },
            },
        )
        .await
        .unwrap();

        let write_intents = list_runtime_records(&pool, "write_intent").await.unwrap();
        let downstream = list_runtime_records(&pool, "write_intent_downstream")
            .await
            .unwrap();

        assert_eq!(write_intents.len(), 3);
        assert_eq!(downstream.len(), 1);
        assert_eq!(outcome.downstream.status, "succeeded");
        assert!(write_intents.iter().any(|record| record.status == "approved"));
        assert!(write_intents.iter().any(|record| record.status == "executing"));
        assert!(write_intents.iter().any(|record| record.status == "succeeded"));
    }

    #[tokio::test]
    async fn write_intent_dispatch_records_failed_downstream_error_state() {
        let pool = test_pool().await;
        let outcome = dispatch_write_intent(
            &pool,
            &WriteIntentDispatchRequest {
                write_intent_id: "write_intent_01dispatchfail".to_string(),
                action_name: "object.update".to_string(),
                target_object_refs: vec!["task_01dispatchfail".to_string()],
                provider: Some("google-calendar".to_string()),
                integration_account_id: Some("integration_account_01dispatchfail".to_string()),
                requested_change: json!({"field":"status","to":"cancelled"}),
                approved: true,
                dry_run: false,
                downstream_operation_ref: "downstream_01dispatchfail".to_string(),
                dispatch: DispatchDisposition::Failed {
                    error: "provider unavailable".to_string(),
                },
            },
        )
        .await
        .unwrap();

        let write_intents = list_runtime_records(&pool, "write_intent").await.unwrap();
        let downstream = list_runtime_records(&pool, "write_intent_downstream")
            .await
            .unwrap();

        assert_eq!(outcome.downstream.status, "failed");
        assert_eq!(downstream[0].payload_json["error"], json!("provider unavailable"));
        assert!(write_intents.iter().any(|record| record.status == "failed"));
    }
}
