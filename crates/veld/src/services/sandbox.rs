use time::OffsetDateTime;
use vel_core::{
    CapabilityDescriptor, RunEventType, RunId, RunKind, RunStatus, SandboxCapabilityPolicy,
    SandboxDecisionRecord, SandboxDecisionStatus, SandboxHostCall, SandboxHostCallEnvelope,
    SandboxPolicyMode,
};

use crate::{errors::AppError, services::broker::BrokerService, state::AppState};

#[derive(Debug)]
pub struct SandboxExecutionResult {
    pub run_id: RunId,
    pub decisions: Vec<SandboxDecisionRecord>,
    pub terminal_status: SandboxDecisionStatus,
}

pub async fn execute_host_calls(
    state: &AppState,
    policy: &SandboxCapabilityPolicy,
    allowlist: &[CapabilityDescriptor],
    calls: &[SandboxHostCallEnvelope],
) -> Result<SandboxExecutionResult, AppError> {
    let first = calls
        .first()
        .ok_or_else(|| AppError::bad_request("sandbox call list must not be empty"))?;
    if calls.iter().any(|call| {
        call.run_id != first.run_id
            || call.trace_id != first.trace_id
            || call.module_id != first.module_id
    }) {
        return Err(AppError::bad_request(
            "sandbox call batch must share run_id, trace_id, and module_id",
        ));
    }
    if first.abi_version != "sandbox_abi/v1" {
        return Err(AppError::bad_request("unsupported sandbox ABI version"));
    }

    let run_id = RunId::from(first.run_id.clone());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    if state
        .storage
        .get_run_by_id(run_id.as_ref())
        .await?
        .is_none()
    {
        state
            .storage
            .create_run(
                &run_id,
                RunKind::Agent,
                &serde_json::json!({
                    "trace_id": first.trace_id,
                    "module_id": first.module_id,
                    "sandbox_policy": policy,
                }),
            )
            .await?;
    }
    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Running,
            Some(now),
            None,
            None,
            None,
        )
        .await?;
    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunStarted,
            &serde_json::json!({
                "trace_id": first.trace_id,
                "module_id": first.module_id,
                "review_gate": policy.review_gate,
            }),
        )
        .await?;

    let broker = BrokerService::new(&state.storage);
    let mut decisions = Vec::new();

    for envelope in calls {
        let decision = execute_single_call(&broker, state, policy, allowlist, envelope).await;
        let status = decision.status;
        state
            .storage
            .append_run_event_auto(
                run_id.as_ref(),
                RunEventType::SandboxCallEvaluated,
                &serde_json::json!({
                    "trace_id": decision.trace_id,
                    "call_kind": decision.call_kind,
                    "status": decision.status,
                    "reason": decision.reason,
                }),
            )
            .await?;
        decisions.push(decision);
        if status != SandboxDecisionStatus::Approved {
            state
                .storage
                .update_run_status(
                    run_id.as_ref(),
                    RunStatus::Failed,
                    None,
                    Some(OffsetDateTime::now_utc().unix_timestamp()),
                    None,
                    Some(
                        &serde_json::json!({"reason": decisions.last().map(|d| d.reason.clone())}),
                    ),
                )
                .await?;
            state
                .storage
                .append_run_event_auto(
                    run_id.as_ref(),
                    RunEventType::SandboxRunCompleted,
                    &serde_json::json!({
                        "trace_id": first.trace_id,
                        "status": "failed",
                        "reason": decisions.last().map(|d| d.reason.clone()),
                    }),
                )
                .await?;
            return Ok(SandboxExecutionResult {
                run_id,
                decisions,
                terminal_status: status,
            });
        }
    }

    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Succeeded,
            None,
            Some(OffsetDateTime::now_utc().unix_timestamp()),
            Some(&serde_json::json!({
                "trace_id": first.trace_id,
                "approved_call_count": decisions.len(),
            })),
            None,
        )
        .await?;
    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::SandboxRunCompleted,
            &serde_json::json!({
                "trace_id": first.trace_id,
                "status": "approved",
                "approved_call_count": decisions.len(),
            }),
        )
        .await?;

    Ok(SandboxExecutionResult {
        run_id,
        decisions,
        terminal_status: SandboxDecisionStatus::Approved,
    })
}

async fn execute_single_call(
    broker: &BrokerService<'_>,
    state: &AppState,
    policy: &SandboxCapabilityPolicy,
    allowlist: &[CapabilityDescriptor],
    envelope: &SandboxHostCallEnvelope,
) -> SandboxDecisionRecord {
    let call_kind = call_kind(&envelope.call).to_string();
    if !policy_allows_call(policy, &call_kind) {
        return SandboxDecisionRecord {
            abi_version: envelope.abi_version.clone(),
            run_id: envelope.run_id.clone(),
            trace_id: envelope.trace_id.clone(),
            call_kind,
            status: SandboxDecisionStatus::Denied,
            reason: "call not allowed by sandbox policy".to_string(),
        };
    }

    match call_capability(&envelope.call) {
        Some(requested) => match broker
            .resolve_capability(&envelope.run_id, &requested, allowlist)
            .await
        {
            Ok(grant) => {
                let action_payload = action_payload(&envelope.call);
                match broker.execute_brokered(&grant, &action_payload).await {
                    Ok(_) => SandboxDecisionRecord {
                        abi_version: envelope.abi_version.clone(),
                        run_id: envelope.run_id.clone(),
                        trace_id: envelope.trace_id.clone(),
                        call_kind,
                        status: SandboxDecisionStatus::Approved,
                        reason: "approved".to_string(),
                    },
                    Err(error) => SandboxDecisionRecord {
                        abi_version: envelope.abi_version.clone(),
                        run_id: envelope.run_id.clone(),
                        trace_id: envelope.trace_id.clone(),
                        call_kind,
                        status: SandboxDecisionStatus::Failed,
                        reason: error.to_string(),
                    },
                }
            }
            Err(denial) => SandboxDecisionRecord {
                abi_version: envelope.abi_version.clone(),
                run_id: envelope.run_id.clone(),
                trace_id: envelope.trace_id.clone(),
                call_kind,
                status: SandboxDecisionStatus::Denied,
                reason: denial.reason,
            },
        },
        None => {
            let _ = state;
            SandboxDecisionRecord {
                abi_version: envelope.abi_version.clone(),
                run_id: envelope.run_id.clone(),
                trace_id: envelope.trace_id.clone(),
                call_kind,
                status: SandboxDecisionStatus::Denied,
                reason: "unsupported sandbox host call".to_string(),
            }
        }
    }
}

fn policy_allows_call(policy: &SandboxCapabilityPolicy, call_kind: &str) -> bool {
    if policy.allowed_calls.iter().any(|entry| entry == call_kind) {
        return true;
    }

    match policy.default_mode {
        SandboxPolicyMode::Allow => true,
        SandboxPolicyMode::Brokered | SandboxPolicyMode::Deny => false,
    }
}

fn call_kind(call: &SandboxHostCall) -> &'static str {
    match call {
        SandboxHostCall::ReadContext { .. } => "read_context",
        SandboxHostCall::RequestCapability { .. } => "request_capability",
        SandboxHostCall::SubmitActionBatch { .. } => "submit_action_batch",
        SandboxHostCall::ReadArtifact { .. } => "read_artifact",
    }
}

fn call_capability(call: &SandboxHostCall) -> Option<CapabilityDescriptor> {
    match call {
        SandboxHostCall::ReadContext { .. } => Some(CapabilityDescriptor {
            scope: "read:context".to_string(),
            resource: None,
            action: "read".to_string(),
        }),
        SandboxHostCall::RequestCapability { capability, .. } => {
            Some(capability_descriptor(capability, None))
        }
        SandboxHostCall::SubmitActionBatch { .. } => Some(CapabilityDescriptor {
            scope: "execute:action_batch".to_string(),
            resource: None,
            action: "execute".to_string(),
        }),
        SandboxHostCall::ReadArtifact { artifact_id } => Some(capability_descriptor(
            "read:artifact",
            Some(artifact_id.clone()),
        )),
    }
}

fn capability_descriptor(scope: &str, resource: Option<String>) -> CapabilityDescriptor {
    let action = scope
        .split(':')
        .next()
        .filter(|value| !value.is_empty())
        .unwrap_or("read");
    CapabilityDescriptor {
        scope: scope.to_string(),
        resource,
        action: action.to_string(),
    }
}

fn action_payload(call: &SandboxHostCall) -> serde_json::Value {
    match call {
        SandboxHostCall::ReadContext { query } => serde_json::json!({ "query": query }),
        SandboxHostCall::RequestCapability { capability, reason } => {
            serde_json::json!({ "capability": capability, "reason": reason })
        }
        SandboxHostCall::SubmitActionBatch { actions } => serde_json::json!({ "actions": actions }),
        SandboxHostCall::ReadArtifact { artifact_id } => {
            serde_json::json!({ "artifact_id": artifact_id })
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{
        FilesystemAccessPolicy, NetworkAccessPolicy, RunEventType, SandboxCapabilityPolicy,
        SandboxHostCallEnvelope, SandboxPolicyMode, SandboxResourceLimits,
    };
    use vel_storage::Storage;

    use super::*;
    use crate::{policy_config::PolicyConfig, state::AppState};

    async fn test_state() -> AppState {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = broadcast::channel(16);
        let config = AppConfig {
            artifact_root: std::env::temp_dir()
                .join(format!("vel_sandbox_{}", uuid::Uuid::new_v4().simple()))
                .to_string_lossy()
                .to_string(),
            ..Default::default()
        };
        AppState::new(
            storage,
            config,
            PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    fn base_policy() -> SandboxCapabilityPolicy {
        SandboxCapabilityPolicy {
            default_mode: SandboxPolicyMode::Deny,
            allowed_calls: vec![
                "read_context".to_string(),
                "submit_action_batch".to_string(),
            ],
            filesystem: FilesystemAccessPolicy {
                read_roots: Vec::new(),
                write_roots: Vec::new(),
            },
            network: NetworkAccessPolicy {
                allowed_hosts: Vec::new(),
            },
            resources: SandboxResourceLimits {
                max_fuel: 10_000,
                max_memory_bytes: 4 * 1024 * 1024,
                wall_timeout_ms: 5_000,
            },
            review_gate: "operator".to_string(),
        }
    }

    #[tokio::test]
    async fn sandbox_denies_calls_not_in_policy_allowlist() {
        let state = test_state().await;
        let result = execute_host_calls(
            &state,
            &base_policy(),
            &[CapabilityDescriptor {
                scope: "read:artifact".to_string(),
                resource: Some("art_1".to_string()),
                action: "read".to_string(),
            }],
            &[SandboxHostCallEnvelope {
                abi_version: "sandbox_abi/v1".to_string(),
                module_id: "mod_demo".to_string(),
                run_id: "run_sandbox_deny".to_string(),
                trace_id: "trace_sandbox_deny".to_string(),
                call: SandboxHostCall::ReadArtifact {
                    artifact_id: "art_1".to_string(),
                },
            }],
        )
        .await
        .unwrap();

        assert_eq!(result.terminal_status, SandboxDecisionStatus::Denied);
        assert_eq!(result.decisions.len(), 1);
        assert_eq!(
            result.decisions[0].reason,
            "call not allowed by sandbox policy"
        );

        let events = state
            .storage
            .list_run_events("run_sandbox_deny")
            .await
            .unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::SandboxCallEvaluated));
    }

    #[tokio::test]
    async fn sandbox_executes_brokered_calls_and_records_completion() {
        let state = test_state().await;
        let result = execute_host_calls(
            &state,
            &base_policy(),
            &[
                CapabilityDescriptor {
                    scope: "read:context".to_string(),
                    resource: None,
                    action: "read".to_string(),
                },
                CapabilityDescriptor {
                    scope: "execute:action_batch".to_string(),
                    resource: None,
                    action: "execute".to_string(),
                },
            ],
            &[
                SandboxHostCallEnvelope {
                    abi_version: "sandbox_abi/v1".to_string(),
                    module_id: "mod_demo".to_string(),
                    run_id: "run_sandbox_ok".to_string(),
                    trace_id: "trace_sandbox_ok".to_string(),
                    call: SandboxHostCall::ReadContext {
                        query: "current_context".to_string(),
                    },
                },
                SandboxHostCallEnvelope {
                    abi_version: "sandbox_abi/v1".to_string(),
                    module_id: "mod_demo".to_string(),
                    run_id: "run_sandbox_ok".to_string(),
                    trace_id: "trace_sandbox_ok".to_string(),
                    call: SandboxHostCall::SubmitActionBatch {
                        actions: vec![serde_json::json!({"kind": "capture.create"})],
                    },
                },
            ],
        )
        .await
        .unwrap();

        assert_eq!(result.terminal_status, SandboxDecisionStatus::Approved);
        assert!(result
            .decisions
            .iter()
            .all(|decision| decision.status == SandboxDecisionStatus::Approved));

        let events = state
            .storage
            .list_run_events("run_sandbox_ok")
            .await
            .unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::SandboxRunCompleted));

        let broker_events = state
            .storage
            .list_broker_events("run_sandbox_ok")
            .await
            .unwrap();
        assert!(broker_events
            .iter()
            .any(|event| event.event_type == "grant"));
        assert!(broker_events
            .iter()
            .any(|event| event.event_type == "execute"));
    }
}
