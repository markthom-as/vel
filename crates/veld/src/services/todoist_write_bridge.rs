use serde_json::{json, Value as JsonValue};
use sqlx::SqlitePool;
use vel_adapters_todoist::ownership_sync::{TaskEventRecord, TaskFieldChange};
use vel_config::AppConfig;
use vel_core::{
    action_explain, generic_object_action_contracts, object_explain, policy_explain,
    ConfirmationMode, ExplainBasis, PolicyDecisionKind, PolicyEvaluationInput, PolicyLayerKind,
    WritebackStatus, WriteIntentId,
};
use vel_storage::Storage;

use crate::{
    errors::AppError,
    services::{
        conflict_classifier::ConflictClassifier,
        policy_evaluator::{default_layer, PolicyEvaluator, PolicyEvaluatorError},
        writeback,
        write_intent_dispatch::{
            dispatch_write_intent, DispatchDisposition, ExecutionDispatch,
            WriteIntentDispatchRequest,
        },
    },
};

#[derive(Debug, Clone)]
pub struct TodoistWriteBridgeRequest {
    pub object_id: String,
    pub revision: i64,
    pub object_status: String,
    pub integration_account_id: String,
    pub requested_change: JsonValue,
    pub read_only: bool,
    pub write_enabled: bool,
    pub dry_run: bool,
    pub approved: bool,
    pub pending_reconciliation: bool,
}

#[derive(Debug, Clone)]
pub struct TodoistWriteBridgeOutcome {
    pub write_intent_id: String,
    pub explain: vel_core::ActionExplain,
    pub dispatch: Option<ExecutionDispatch>,
    pub task_events: Vec<TaskEventRecord>,
}

pub async fn bridge_todoist_write(
    pool: &SqlitePool,
    request: &TodoistWriteBridgeRequest,
) -> Result<TodoistWriteBridgeOutcome, AppError> {
    let classifier = ConflictClassifier;
    if let Some(conflict) =
        classifier.classify_tombstone_write_race(request.object_status == "deleted", true)
    {
        return Err(AppError::forbidden(conflict.reason));
    }
    if let Some(conflict) =
        classifier.classify_pending_reconciliation(if request.pending_reconciliation {
            "pending_reconciliation"
        } else {
            "reconciled"
        })
    {
        return Err(AppError::forbidden(conflict.reason));
    }

    let policy_input = PolicyEvaluationInput {
        action_name: "todoist.task.write".to_string(),
        allows_external_write: true,
        is_destructive: false,
        is_cross_source: false,
        workspace: default_layer(PolicyLayerKind::Workspace),
        module: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::Module,
            read_only: false,
            confirmation: if request.approved {
                ConfirmationMode::Auto
            } else if request.write_enabled {
                ConfirmationMode::AskIfExternalWrite
            } else {
                ConfirmationMode::Deny
            },
            reason: if request.write_enabled {
                "todoist module uses ask_if_external_write for outbound mutation".to_string()
            } else {
                "todoist outbound writes disabled".to_string()
            },
        },
        integration_account: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::IntegrationAccount,
            read_only: request.read_only,
            confirmation: ConfirmationMode::Auto,
            reason: if request.read_only {
                "integration account read_only posture".to_string()
            } else {
                "integration account allows configured writes".to_string()
            },
        },
        object: default_layer(PolicyLayerKind::Object),
        action: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::Action,
            read_only: false,
            confirmation: ConfirmationMode::Auto,
            reason: "todoist.task.write action".to_string(),
        },
        execution: default_layer(PolicyLayerKind::Execution),
    };

    let policy = PolicyEvaluator
        .evaluate(&policy_input)
        .map_err(map_policy_error)?;

    let contract = generic_object_action_contracts()
        .into_iter()
        .find(|contract| contract.action_name == "object.update")
        .ok_or_else(|| AppError::internal("missing object.update action contract"))?;
    let explain = action_explain(
        "todoist.task.write",
        contract.capability.capability,
        true,
        request.dry_run,
        policy_explain(
            "todoist.task.write",
            PolicyDecisionKind::Allowed,
            if request.approved {
                ConfirmationMode::Auto
            } else {
                ConfirmationMode::AskIfExternalWrite
            },
            request.read_only,
            policy.reasons.clone(),
        ),
        Some(object_explain(
            request.object_id.clone(),
            request.object_status.clone(),
            request.revision,
            None,
            1,
            ExplainBasis::Exact,
        )),
        vec![],
    );

    let task_events = vec![TaskEventRecord {
        id: format!("task_event_write_intent_{}", request.object_id),
        task_ref: request.object_id.clone(),
        event_type: "updated".to_string(),
        provenance: if request.dry_run {
            "local_write_intent".to_string()
        } else {
            "local_write_applied".to_string()
        },
        field_changes: vec![TaskFieldChange {
            field_name: "requested_change".to_string(),
            old_value: None,
            new_value: Some(request.requested_change.clone()),
        }],
    }];

    let write_intent_id = WriteIntentId::new().to_string();
    if request.dry_run {
        return Ok(TodoistWriteBridgeOutcome {
            write_intent_id,
            explain,
            dispatch: None,
            task_events,
        });
    }

    let dispatch = dispatch_write_intent(
        pool,
        &WriteIntentDispatchRequest {
            write_intent_id: write_intent_id.clone(),
            action_name: "todoist.task.write".to_string(),
            target_object_refs: vec![request.object_id.clone()],
            provider: Some("todoist".to_string()),
            integration_account_id: Some(request.integration_account_id.clone()),
            requested_change: request.requested_change.clone(),
            approved: request.approved,
            dry_run: false,
            downstream_operation_ref: format!("{write_intent_id}__todoist"),
            dispatch: DispatchDisposition::Succeeded {
                result: json!({"provider":"todoist","status":"accepted"}),
            },
        },
    )
    .await?;

    Ok(TodoistWriteBridgeOutcome {
        write_intent_id,
        explain,
        dispatch: Some(dispatch),
        task_events,
    })
}

pub async fn bridge_todoist_write_with_services(
    storage: &Storage,
    config: &AppConfig,
    request: &TodoistWriteBridgeRequest,
) -> Result<TodoistWriteBridgeOutcome, AppError> {
    let classifier = ConflictClassifier;
    if let Some(conflict) =
        classifier.classify_tombstone_write_race(request.object_status == "deleted", true)
    {
        return Err(AppError::forbidden(conflict.reason));
    }
    if let Some(conflict) =
        classifier.classify_pending_reconciliation(if request.pending_reconciliation {
            "pending_reconciliation"
        } else {
            "reconciled"
        })
    {
        return Err(AppError::forbidden(conflict.reason));
    }

    let policy_input = PolicyEvaluationInput {
        action_name: "todoist.task.write".to_string(),
        allows_external_write: true,
        is_destructive: false,
        is_cross_source: false,
        workspace: default_layer(PolicyLayerKind::Workspace),
        module: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::Module,
            read_only: false,
            confirmation: if request.approved {
                ConfirmationMode::Auto
            } else if request.write_enabled {
                ConfirmationMode::AskIfExternalWrite
            } else {
                ConfirmationMode::Deny
            },
            reason: if request.write_enabled {
                "todoist module uses ask_if_external_write for outbound mutation".to_string()
            } else {
                "todoist outbound writes disabled".to_string()
            },
        },
        integration_account: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::IntegrationAccount,
            read_only: request.read_only,
            confirmation: ConfirmationMode::Auto,
            reason: if request.read_only {
                "integration account read_only posture".to_string()
            } else {
                "integration account allows configured writes".to_string()
            },
        },
        object: default_layer(PolicyLayerKind::Object),
        action: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::Action,
            read_only: false,
            confirmation: ConfirmationMode::Auto,
            reason: "todoist.task.write action".to_string(),
        },
        execution: default_layer(PolicyLayerKind::Execution),
    };

    let policy = PolicyEvaluator
        .evaluate(&policy_input)
        .map_err(map_policy_error)?;

    let contract = generic_object_action_contracts()
        .into_iter()
        .find(|contract| contract.action_name == "object.update")
        .ok_or_else(|| AppError::internal("missing object.update action contract"))?;
    let explain = action_explain(
        "todoist.task.write",
        contract.capability.capability,
        true,
        request.dry_run,
        policy_explain(
            "todoist.task.write",
            PolicyDecisionKind::Allowed,
            if request.approved {
                ConfirmationMode::Auto
            } else {
                ConfirmationMode::AskIfExternalWrite
            },
            request.read_only,
            policy.reasons.clone(),
        ),
        Some(object_explain(
            request.object_id.clone(),
            request.object_status.clone(),
            request.revision,
            None,
            1,
            ExplainBasis::Exact,
        )),
        vec![],
    );

    let task_events = vec![TaskEventRecord {
        id: format!("task_event_write_intent_{}", request.object_id),
        task_ref: request.object_id.clone(),
        event_type: "updated".to_string(),
        provenance: if request.dry_run {
            "local_write_intent".to_string()
        } else {
            "local_write_applied".to_string()
        },
        field_changes: vec![TaskFieldChange {
            field_name: "requested_change".to_string(),
            old_value: None,
            new_value: Some(request.requested_change.clone()),
        }],
    }];

    let parsed_change = parse_todoist_write_intent(&request.requested_change)?;
    let write_intent_id = WriteIntentId::new().to_string();
    if request.dry_run {
        return Ok(TodoistWriteBridgeOutcome {
            write_intent_id,
            explain,
            dispatch: None,
            task_events,
        });
    }

    let operation = match parsed_change {
        TodoistWriteIntent::Complete => writeback::todoist_complete_task(
            storage,
            config,
            "vel-local",
            &request.object_id,
        )
        .await?,
        TodoistWriteIntent::Reopen => writeback::todoist_reopen_task(
            storage,
            config,
            "vel-local",
            &request.object_id,
        )
        .await?,
        TodoistWriteIntent::Update(mutation) => writeback::todoist_update_task(
            storage,
            config,
            "vel-local",
            &request.object_id,
            mutation,
        )
        .await?,
    };

    let dispatch = dispatch_write_intent(
        storage.sql_pool(),
        &WriteIntentDispatchRequest {
            write_intent_id: write_intent_id.clone(),
            action_name: "todoist.task.write".to_string(),
            target_object_refs: vec![request.object_id.clone()],
            provider: Some("todoist".to_string()),
            integration_account_id: Some(request.integration_account_id.clone()),
            requested_change: request.requested_change.clone(),
            approved: request.approved,
            dry_run: false,
            downstream_operation_ref: format!("{write_intent_id}__todoist"),
            dispatch: map_todoist_write_disposition(&operation),
        },
    )
    .await?;

    Ok(TodoistWriteBridgeOutcome {
        write_intent_id,
        explain,
        dispatch: Some(dispatch),
        task_events,
    })
}

#[derive(Debug)]
enum TodoistWriteIntent {
    Update(crate::services::integrations_todoist::TodoistTaskMutation),
    Complete,
    Reopen,
}

fn parse_todoist_write_intent(
    requested_change: &JsonValue,
) -> Result<TodoistWriteIntent, AppError> {
    let action = requested_change
        .get("action")
        .and_then(JsonValue::as_str)
        .map(str::to_ascii_lowercase)
        .or_else(|| {
            requested_change
                .get("status")
                .and_then(JsonValue::as_str)
                .map(|value| format!("status:{}", value.to_ascii_lowercase()))
        });

    if let Some(action) = action.as_deref() {
        match action {
            "complete" | "done" | "complete_task" | "close" | "close_task" => {
                return Ok(TodoistWriteIntent::Complete);
            }
            "reopen" | "reopen_task" | "uncomplete" | "status:open" | "status:ready"
            | "status:active" => {
                return Ok(TodoistWriteIntent::Reopen);
            }
            "update" => {
                return Ok(TodoistWriteIntent::Update(parse_todoist_mutation(
                    requested_change,
                )?));
            }
            _ => {}
        }
    }

    let mutation = parse_todoist_mutation(requested_change)?;
    if mutation.content.is_none()
        && mutation.project_id.is_none()
        && mutation.scheduled_for.is_none()
        && mutation.priority.is_none()
        && mutation.waiting_on.is_none()
        && mutation.review_state.is_none()
        && mutation.tags.is_none()
    {
        return Err(AppError::bad_request(
            "todoist write requires either a supported field change or explicit action",
        ));
    }

    Ok(TodoistWriteIntent::Update(mutation))
}

fn parse_todoist_mutation(
    requested_change: &JsonValue,
) -> Result<crate::services::integrations_todoist::TodoistTaskMutation, AppError> {
    let content = requested_change
        .get("content")
        .or_else(|| requested_change.get("title"))
        .and_then(JsonValue::as_str)
        .map(normalize_text)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            requested_change
                .get("description")
                .and_then(JsonValue::as_str)
                .map(normalize_text)
                .filter(|value| !value.is_empty())
        });
    let project_id = requested_change
        .get("project_id")
        .or_else(|| requested_change.get("project_ref"))
        .and_then(JsonValue::as_str)
        .map(normalize_text)
        .filter(|value| !value.is_empty())
        .map(Some)
        .unwrap_or_default();
    let scheduled_for = requested_change
        .get("scheduled_for")
        .and_then(JsonValue::as_str)
        .map(normalize_text)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            requested_change
                .get("due")
                .and_then(extract_scheduled_from_due)
                .filter(|value| !value.is_empty())
        });
    let priority = requested_change
        .get("priority")
        .map(parse_todoist_priority)
        .transpose()?
        .flatten();
    let waiting_on = requested_change
        .get("waiting_on")
        .and_then(JsonValue::as_str)
        .map(normalize_text)
        .filter(|value| !value.is_empty());
    let review_state = requested_change
        .get("review_state")
        .and_then(JsonValue::as_str)
        .map(normalize_text)
        .filter(|value| !value.is_empty());
    let tags = parse_todoist_tags(requested_change.get("tags"))?;

    Ok(crate::services::integrations_todoist::TodoistTaskMutation {
        content,
        project_id,
        scheduled_for,
        priority,
        waiting_on,
        review_state,
        tags,
    })
}

fn extract_scheduled_from_due(due: &JsonValue) -> Option<String> {
    if let Some(value) = due.as_str().map(normalize_text) {
        if !value.is_empty() {
            return Some(value);
        }
    }
    due.get("value")
        .and_then(JsonValue::as_str)
        .map(normalize_text)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            due.get("date")
                .and_then(JsonValue::as_str)
                .map(normalize_text)
                .filter(|value| !value.is_empty())
        })
        .or_else(|| {
            due.get("datetime")
                .and_then(JsonValue::as_str)
                .map(normalize_text)
                .filter(|value| !value.is_empty())
        })
}

fn parse_todoist_tags(
    value: Option<&JsonValue>,
) -> Result<Option<Vec<String>>, AppError> {
    match value {
        Some(JsonValue::Array(values)) => {
            let tags = values
                .iter()
                .filter_map(|value| value.as_str().map(normalize_text))
                .filter(|value| !value.is_empty())
                .collect::<Vec<_>>();
            Ok(Some(tags))
        }
        Some(other) if other.is_null() => Ok(Some(Vec::new())),
        Some(other) => {
            Err(AppError::bad_request(format!(
                "unsupported todoist tags payload: {other}"
            )))
        }
        None => Ok(None),
    }
}

fn parse_todoist_priority(value: &JsonValue) -> Result<Option<u8>, AppError> {
    let priority = match value {
        JsonValue::Number(number) => {
            if let Some(raw) = number.as_u64() {
                if (1..=4).contains(&raw) {
                    Some(raw as u8)
                } else {
                    return Err(AppError::bad_request("todoist priority must be between 1 and 4"));
                }
            } else if let Some(raw) = number.as_i64() {
                if (1..=4).contains(&raw) {
                    Some(raw as u8)
                } else {
                    return Err(AppError::bad_request("todoist priority must be between 1 and 4"));
                }
            } else {
                return Err(AppError::bad_request("todoist priority must be between 1 and 4"));
            }
        }
        JsonValue::String(value) => {
            let priority = value.trim().to_ascii_lowercase();
            if priority.is_empty() {
                None
            } else if let Some(value) = priority.strip_prefix('p') {
                let parsed = value.parse::<u8>().map_err(|_| {
                    AppError::bad_request(format!(
                        "todoist priority string {priority} is not recognized",
                    ))
                })?;
                if (1..=4).contains(&parsed) {
                    Some(parsed)
                } else {
                    return Err(AppError::bad_request("todoist priority must be between 1 and 4"));
                }
            } else {
                match priority.as_str() {
                    "critical" | "urgent" | "high" => Some(1),
                    "medium" => Some(2),
                    "low" => Some(3),
                    "lowest" => Some(4),
                    _ => {
                        let parsed = priority.parse::<u8>().map_err(|_| {
                            AppError::bad_request(format!(
                                "todoist priority string {priority} is not recognized",
                            ))
                        })?;
                        if (1..=4).contains(&parsed) {
                            Some(parsed)
                        } else {
                            return Err(AppError::bad_request(
                                "todoist priority must be between 1 and 4",
                            ));
                        }
                    }
                }
            }
        }
        other => {
            return Err(AppError::bad_request(format!(
                "todoist priority must be a number or string, got {other}"
            )));
        }
    };

    Ok(priority)
}

fn map_todoist_write_disposition(
    operation: &vel_core::WritebackOperationRecord,
) -> DispatchDisposition {
    match operation.status {
        WritebackStatus::Applied => DispatchDisposition::Succeeded {
            result: operation.result_payload.clone().unwrap_or_else(|| {
                json!({
                    "provider": "todoist",
                    "status": "applied",
                    "writeback_id": operation.id,
                    "kind": operation.kind.to_string(),
                })
            }),
        },
        status => DispatchDisposition::Failed {
            error: format!(
                "todoist write {status} for {}: {}",
                operation.kind,
                operation
                    .result_payload
                    .as_ref()
                    .map(|payload| payload.to_string())
                    .unwrap_or_else(|| "no result payload".to_string())
            ),
        },
    }
}

fn normalize_text(value: &str) -> String {
    value.trim().to_string()
}

fn map_policy_error(error: PolicyEvaluatorError) -> AppError {
    match error {
        PolicyEvaluatorError::PolicyDenied(message) => {
            AppError::forbidden(format!("PolicyDenied {message}"))
        }
        PolicyEvaluatorError::ConfirmationRequired(message) => {
            AppError::forbidden(format!("ConfirmationRequired {message}"))
        }
        PolicyEvaluatorError::ReadOnlyViolation(message) => {
            AppError::forbidden(format!("ReadOnlyViolation {message}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{bridge_todoist_write, TodoistWriteBridgeRequest};
    use serde_json::json;
    use sqlx::SqlitePool;
    use vel_storage::{list_runtime_records, migrate_storage};

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn todoist_write_bridge_uses_write_intent_dispatch_for_approved_writes() {
        let pool = test_pool().await;
        let outcome = bridge_todoist_write(
            &pool,
            &TodoistWriteBridgeRequest {
                object_id: "task_01bridge".to_string(),
                revision: 3,
                object_status: "active".to_string(),
                integration_account_id: "integration_account_primary".to_string(),
                requested_change: json!({"due":{"kind":"date","value":"2026-03-24"}}),
                read_only: false,
                write_enabled: true,
                dry_run: false,
                approved: true,
                pending_reconciliation: false,
            },
        )
        .await
        .unwrap();

        assert!(outcome.dispatch.is_some());
        assert_eq!(outcome.task_events[0].provenance, "local_write_applied");
        assert_eq!(
            list_runtime_records(&pool, "write_intent")
                .await
                .unwrap()
                .len(),
            3
        );
    }

    #[tokio::test]
    async fn todoist_write_bridge_returns_dry_run_without_dispatch() {
        let pool = test_pool().await;
        let outcome = bridge_todoist_write(
            &pool,
            &TodoistWriteBridgeRequest {
                object_id: "task_01dryrun".to_string(),
                revision: 1,
                object_status: "active".to_string(),
                integration_account_id: "integration_account_primary".to_string(),
                requested_change: json!({"priority":"high"}),
                read_only: false,
                write_enabled: true,
                dry_run: true,
                approved: true,
                pending_reconciliation: false,
            },
        )
        .await
        .unwrap();

        assert!(outcome.dispatch.is_none());
        assert_eq!(outcome.task_events[0].provenance, "local_write_intent");
    }
}
