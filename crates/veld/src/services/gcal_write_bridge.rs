use serde_json::{Value as JsonValue, json};
use sqlx::SqlitePool;
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
pub struct GoogleCalendarWriteBridgeRequest {
    pub object_id: String,
    pub expected_revision: i64,
    pub actual_revision: i64,
    pub object_status: String,
    pub integration_account_id: String,
    pub requested_change: JsonValue,
    pub recurrence_scope: Option<String>,
    pub source_owned_fields: Vec<String>,
    pub read_only: bool,
    pub write_enabled: bool,
    pub dry_run: bool,
    pub approved: bool,
    pub pending_reconciliation: bool,
}

#[derive(Debug, Clone)]
pub struct GoogleCalendarWriteBridgeOutcome {
    pub write_intent_id: String,
    pub explain: vel_core::ActionExplain,
    pub dispatch: Option<ExecutionDispatch>,
}

pub async fn bridge_google_calendar_write(
    pool: &SqlitePool,
    request: &GoogleCalendarWriteBridgeRequest,
) -> Result<GoogleCalendarWriteBridgeOutcome, AppError> {
    let classifier = ConflictClassifier;
    if let Some(conflict) =
        classifier.classify_stale_version(request.expected_revision, request.actual_revision)
    {
        return Err(AppError::forbidden(format!(
            "StaleVersion {}",
            conflict.reason
        )));
    }
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
        return Err(AppError::forbidden(format!(
            "PendingReconciliation {}",
            conflict.reason
        )));
    }
    if let Some(scope) = request.recurrence_scope.as_deref() {
        if scope == "this_and_following" {
            return Err(AppError::forbidden(
                "UnsupportedCapability recurrence scope this_and_following is not supported in 0.5",
            ));
        }
    }
    if let Some(field) = request.source_owned_fields.first() {
        if let Some(conflict) = classifier.classify_ownership_conflict(field, true, true) {
            return Err(AppError::forbidden(format!(
                "OwnershipConflict {}",
                conflict.reason
            )));
        }
    }

    let policy_input = PolicyEvaluationInput {
        action_name: "google.calendar.write".to_string(),
        allows_external_write: true,
        is_destructive: request
            .requested_change
            .get("action")
            .and_then(JsonValue::as_str)
            == Some("delete"),
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
                "google calendar module uses ask_if_external_write for outbound mutation"
                    .to_string()
            } else {
                "google calendar outbound writes disabled".to_string()
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
            reason: "google.calendar.write action".to_string(),
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
        "google.calendar.write",
        contract.capability.capability,
        true,
        request.dry_run,
        policy_explain(
            "google.calendar.write",
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
            request.actual_revision,
            None,
            1,
            ExplainBasis::Exact,
        )),
        vec![],
    );

    let write_intent_id = WriteIntentId::new().to_string();
    if request.dry_run {
        return Ok(GoogleCalendarWriteBridgeOutcome {
            write_intent_id,
            explain,
            dispatch: None,
        });
    }

    let dispatch = dispatch_write_intent(
        pool,
        &WriteIntentDispatchRequest {
            write_intent_id: write_intent_id.clone(),
            action_name: "google.calendar.write".to_string(),
            target_object_refs: vec![request.object_id.clone()],
            provider: Some("google-calendar".to_string()),
            integration_account_id: Some(request.integration_account_id.clone()),
            requested_change: request.requested_change.clone(),
            approved: request.approved,
            dry_run: false,
            downstream_operation_ref: format!("{write_intent_id}__google_calendar"),
            dispatch: DispatchDisposition::Succeeded {
                result: json!({
                    "provider":"google-calendar",
                    "status":"accepted",
                    "scope": request.recurrence_scope.as_deref().unwrap_or("single_event"),
                }),
            },
        },
    )
    .await?;

    Ok(GoogleCalendarWriteBridgeOutcome {
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
    use super::{GoogleCalendarWriteBridgeRequest, bridge_google_calendar_write};
    use serde_json::json;
    use sqlx::SqlitePool;
    use vel_storage::{list_runtime_records, migrate_storage};

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn google_calendar_write_bridge_dispatches_approved_writes_and_skips_dry_runs() {
        let pool = test_pool().await;

        let dry_run = bridge_google_calendar_write(
            &pool,
            &GoogleCalendarWriteBridgeRequest {
                object_id: "event_01dryrun".to_string(),
                expected_revision: 2,
                actual_revision: 2,
                object_status: "active".to_string(),
                integration_account_id: "integration_account_google".to_string(),
                requested_change: json!({"title":"Moved block"}),
                recurrence_scope: Some("single_occurrence".to_string()),
                source_owned_fields: vec![],
                read_only: false,
                write_enabled: true,
                dry_run: true,
                approved: true,
                pending_reconciliation: false,
            },
        )
        .await
        .unwrap();
        assert!(dry_run.dispatch.is_none());

        let executed = bridge_google_calendar_write(
            &pool,
            &GoogleCalendarWriteBridgeRequest {
                object_id: "event_01dispatch".to_string(),
                expected_revision: 2,
                actual_revision: 2,
                object_status: "active".to_string(),
                integration_account_id: "integration_account_google".to_string(),
                requested_change: json!({"title":"Moved block"}),
                recurrence_scope: Some("single_occurrence".to_string()),
                source_owned_fields: vec![],
                read_only: false,
                write_enabled: true,
                dry_run: false,
                approved: true,
                pending_reconciliation: false,
            },
        )
        .await
        .unwrap();

        assert!(executed.dispatch.is_some());
        assert_eq!(
            list_runtime_records(&pool, "write_intent")
                .await
                .unwrap()
                .len(),
            3
        );
    }
}
