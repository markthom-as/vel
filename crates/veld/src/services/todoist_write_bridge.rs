use serde_json::{Value as JsonValue, json};
use sqlx::SqlitePool;
use vel_adapters_todoist::ownership_sync::{TaskEventRecord, TaskFieldChange};
use vel_core::{
    ConfirmationMode, ExplainBasis, PolicyDecisionKind, PolicyEvaluationInput, PolicyLayerKind,
    WriteIntentId, action_explain, generic_object_action_contracts, object_explain, policy_explain,
};

use crate::{
    errors::AppError,
    services::{
        conflict_classifier::ConflictClassifier,
        policy_evaluator::{PolicyEvaluator, PolicyEvaluatorError, default_layer},
        write_intent_dispatch::{
            DispatchDisposition, ExecutionDispatch, WriteIntentDispatchRequest,
            dispatch_write_intent,
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
    use super::{TodoistWriteBridgeRequest, bridge_todoist_write};
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
