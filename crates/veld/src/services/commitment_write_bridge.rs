use serde_json::{Value as JsonValue, json};
use sqlx::SqlitePool;
use vel_core::{
    ConfirmationMode, ExplainBasis, PolicyDecisionKind, PolicyEvaluationInput, PolicyLayerKind,
    WriteIntentId, action_explain, generic_object_action_contracts, object_explain, policy_explain,
};

use crate::{
    errors::AppError,
    services::{
        policy_evaluator::{PolicyEvaluator, PolicyEvaluatorError, default_layer},
        write_intent_dispatch::{
            DispatchDisposition, ExecutionDispatch, WriteIntentDispatchRequest,
            dispatch_write_intent,
        },
    },
};

#[derive(Debug, Clone)]
pub struct CommitmentWriteBridgeRequest {
    pub object_id: String,
    pub object_status: String,
    pub requested_change: JsonValue,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct CommitmentWriteBridgeOutcome {
    pub write_intent_id: String,
    pub explain: vel_core::ActionExplain,
    pub dispatch: Option<ExecutionDispatch>,
}

pub async fn bridge_commitment_write(
    pool: &SqlitePool,
    request: &CommitmentWriteBridgeRequest,
) -> Result<CommitmentWriteBridgeOutcome, AppError> {
    let policy_input = PolicyEvaluationInput {
        action_name: "commitment.write".to_string(),
        allows_external_write: false,
        is_destructive: false,
        is_cross_source: false,
        workspace: default_layer(PolicyLayerKind::Workspace),
        module: default_layer(PolicyLayerKind::Module),
        integration_account: default_layer(PolicyLayerKind::IntegrationAccount),
        object: default_layer(PolicyLayerKind::Object),
        action: vel_core::PolicyLayerDecision {
            layer: PolicyLayerKind::Action,
            read_only: false,
            confirmation: ConfirmationMode::Auto,
            reason: "commitment.write action".to_string(),
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
        "commitment.write",
        contract.capability.capability,
        false,
        request.dry_run,
        policy_explain(
            "commitment.write",
            PolicyDecisionKind::Allowed,
            ConfirmationMode::Auto,
            false,
            policy.reasons.clone(),
        ),
        Some(object_explain(
            request.object_id.clone(),
            request.object_status.clone(),
            0,
            None,
            0,
            ExplainBasis::Exact,
        )),
        vec![],
    );

    let write_intent_id = WriteIntentId::new().to_string();
    if request.dry_run {
        return Ok(CommitmentWriteBridgeOutcome {
            write_intent_id,
            explain,
            dispatch: None,
        });
    }

    let dispatch = dispatch_write_intent(
        pool,
        &WriteIntentDispatchRequest {
            write_intent_id: write_intent_id.clone(),
            action_name: "commitment.write".to_string(),
            target_object_refs: vec![request.object_id.clone()],
            provider: None,
            integration_account_id: None,
            requested_change: request.requested_change.clone(),
            approved: true,
            dry_run: false,
            downstream_operation_ref: format!("{write_intent_id}__local_commitment"),
            dispatch: DispatchDisposition::Succeeded {
                result: json!({"provider":"local","status":"accepted"}),
            },
        },
    )
    .await?;

    Ok(CommitmentWriteBridgeOutcome {
        write_intent_id,
        explain,
        dispatch: Some(dispatch),
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
    use super::{CommitmentWriteBridgeRequest, bridge_commitment_write};
    use serde_json::json;
    use sqlx::SqlitePool;
    use vel_storage::{list_runtime_records, migrate_storage};

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn commitment_write_bridge_dispatches_runtime_write_intent_records() {
        let pool = test_pool().await;
        let outcome = bridge_commitment_write(
            &pool,
            &CommitmentWriteBridgeRequest {
                object_id: "commitment_01bridge".to_string(),
                object_status: "open".to_string(),
                requested_change: json!({"status":"done"}),
                dry_run: false,
            },
        )
        .await
        .unwrap();

        assert!(outcome.dispatch.is_some());
        assert_eq!(
            list_runtime_records(&pool, "write_intent")
                .await
                .unwrap()
                .len(),
            3
        );
    }

    #[tokio::test]
    async fn commitment_write_bridge_skips_dispatch_for_dry_runs() {
        let pool = test_pool().await;
        let outcome = bridge_commitment_write(
            &pool,
            &CommitmentWriteBridgeRequest {
                object_id: "commitment_01dry".to_string(),
                object_status: "open".to_string(),
                requested_change: json!({"status":"done"}),
                dry_run: true,
            },
        )
        .await
        .unwrap();

        assert!(outcome.dispatch.is_none());
        assert!(
            list_runtime_records(&pool, "write_intent")
                .await
                .unwrap()
                .is_empty()
        );
    }
}
