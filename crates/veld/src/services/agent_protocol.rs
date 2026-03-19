use time::OffsetDateTime;
use vel_core::{
    CapabilityDescriptor, SandboxCapabilityPolicy, SandboxHostCall, SandboxHostCallEnvelope,
};
use vel_protocol::{
    CapabilityRequest, ProtocolEnvelope, ProtocolPayload, ProtocolTraceContext,
    CURRENT_PROTOCOL_VERSION,
};

use crate::{
    errors::AppError,
    services::{broker::BrokerService, sandbox},
    state::AppState,
};

pub(crate) const CONNECT_LEASE_SECONDS: i64 = 300;

pub(crate) fn default_connect_lease_seconds() -> i64 {
    CONNECT_LEASE_SECONDS
}

pub async fn handle_envelope(
    state: &AppState,
    envelope: &ProtocolEnvelope,
    sandbox_policy: &SandboxCapabilityPolicy,
    runtime_allowlist: &[CapabilityDescriptor],
) -> Result<ProtocolEnvelope, AppError> {
    envelope
        .validate()
        .map_err(|error| AppError::bad_request(error.to_string()))?;

    match &envelope.payload {
        ProtocolPayload::Handshake {
            requested_capabilities,
            ..
        } => handle_handshake(state, envelope, requested_capabilities, runtime_allowlist).await,
        ProtocolPayload::Heartbeat { lease_id, status } => {
            handle_heartbeat(state, envelope, lease_id, status).await
        }
        ProtocolPayload::CapabilityRequest { requests } => {
            handle_capability_request(state, envelope, requests, runtime_allowlist).await
        }
        ProtocolPayload::ActionBatchSubmit { batch_id, actions } => {
            handle_action_batch(
                state,
                envelope,
                sandbox_policy,
                runtime_allowlist,
                batch_id,
                actions,
            )
            .await
        }
        ProtocolPayload::ActionResult { .. } => Err(AppError::bad_request(
            "action_result is runtime-emitted only",
        )),
    }
}

async fn handle_handshake(
    state: &AppState,
    envelope: &ProtocolEnvelope,
    requested_capabilities: &[CapabilityRequest],
    runtime_allowlist: &[CapabilityDescriptor],
) -> Result<ProtocolEnvelope, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let lease_id = envelope.trace.run_id.clone();
    let broker = BrokerService::new(&state.storage);
    let granted = resolve_requests(
        &broker,
        &envelope.trace.run_id,
        requested_capabilities,
        runtime_allowlist,
    )
    .await;

    if state.storage.get_connect_run(&lease_id).await?.is_none() {
        state
            .storage
            .insert_connect_run(
                &lease_id,
                &envelope.sender.actor_id,
                &envelope.sender.node_id,
                &serde_json::to_string(&granted).map_err(|e| AppError::internal(e.to_string()))?,
                now + CONNECT_LEASE_SECONDS,
                now,
            )
            .await?;
    }

    Ok(action_result(
        &envelope.trace,
        "handshake",
        serde_json::json!({
            "lease_id": lease_id,
            "granted_capabilities": granted,
        }),
    ))
}

async fn handle_heartbeat(
    state: &AppState,
    envelope: &ProtocolEnvelope,
    lease_id: &str,
    status: &str,
) -> Result<ProtocolEnvelope, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    state
        .storage
        .update_connect_heartbeat(lease_id, now + CONNECT_LEASE_SECONDS)
        .await?;

    Ok(action_result(
        &envelope.trace,
        "heartbeat_ack",
        serde_json::json!({
            "lease_id": lease_id,
            "status": status,
            "lease_expires_at": now + CONNECT_LEASE_SECONDS,
        }),
    ))
}

async fn handle_capability_request(
    state: &AppState,
    envelope: &ProtocolEnvelope,
    requests: &[CapabilityRequest],
    runtime_allowlist: &[CapabilityDescriptor],
) -> Result<ProtocolEnvelope, AppError> {
    let broker = BrokerService::new(&state.storage);
    let granted =
        resolve_requests(&broker, &envelope.trace.run_id, requests, runtime_allowlist).await;

    Ok(action_result(
        &envelope.trace,
        "capability_negotiated",
        serde_json::json!({ "granted_capabilities": granted }),
    ))
}

async fn handle_action_batch(
    state: &AppState,
    envelope: &ProtocolEnvelope,
    sandbox_policy: &SandboxCapabilityPolicy,
    runtime_allowlist: &[CapabilityDescriptor],
    batch_id: &str,
    actions: &[serde_json::Value],
) -> Result<ProtocolEnvelope, AppError> {
    let record = state
        .storage
        .get_connect_run(&envelope.trace.run_id)
        .await?
        .ok_or_else(|| AppError::not_found("connect run not found"))?;
    let granted_allowlist = deserialize_granted_capabilities(&record.capabilities_json)
        .unwrap_or_else(|_| runtime_allowlist.to_vec());
    let calls = actions
        .iter()
        .map(|action| action_to_host_call(envelope, action))
        .collect::<Result<Vec<_>, _>>()?;

    let result =
        sandbox::execute_host_calls(state, sandbox_policy, &granted_allowlist, &calls).await?;

    Ok(action_result(
        &envelope.trace,
        "action_batch_processed",
        serde_json::json!({
            "batch_id": batch_id,
            "sandbox_run_id": result.run_id,
            "terminal_status": result.terminal_status,
            "decision_count": result.decisions.len(),
        }),
    ))
}

fn action_result(
    trace: &ProtocolTraceContext,
    outcome: &str,
    details: serde_json::Value,
) -> ProtocolEnvelope {
    ProtocolEnvelope {
        protocol_version: CURRENT_PROTOCOL_VERSION.to_string(),
        message_id: format!("msg_{}_{}", outcome, trace.trace_id),
        sent_at: OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string()),
        sender: vel_protocol::ProtocolSender {
            node_id: "veld".to_string(),
            actor_id: "authority_runtime".to_string(),
            actor_kind: "authority".to_string(),
        },
        trace: trace.clone(),
        payload: ProtocolPayload::ActionResult {
            batch_id: trace.run_id.clone(),
            outcome: outcome.to_string(),
            details,
        },
    }
}

async fn resolve_requests(
    broker: &BrokerService<'_>,
    run_id: &str,
    requests: &[CapabilityRequest],
    runtime_allowlist: &[CapabilityDescriptor],
) -> Vec<CapabilityDescriptor> {
    let mut granted = Vec::new();
    for request in requests {
        let descriptor = capability_request_to_descriptor(request);
        if let Ok(grant) = broker
            .resolve_capability(run_id, &descriptor, runtime_allowlist)
            .await
        {
            granted.push(grant.descriptor);
        }
    }
    granted
}

fn capability_request_to_descriptor(request: &CapabilityRequest) -> CapabilityDescriptor {
    match request.name.as_str() {
        "context.read" => CapabilityDescriptor {
            scope: "read:context".to_string(),
            resource: None,
            action: "read".to_string(),
        },
        "artifact.read" => CapabilityDescriptor {
            scope: "read:artifact".to_string(),
            resource: Some(request.scope.clone()),
            action: "read".to_string(),
        },
        "action.execute" => CapabilityDescriptor {
            scope: "execute:action_batch".to_string(),
            resource: None,
            action: "execute".to_string(),
        },
        _ => CapabilityDescriptor {
            scope: request.name.replace('.', ":"),
            resource: Some(request.scope.clone()),
            action: request.name.split('.').nth(1).unwrap_or("read").to_string(),
        },
    }
}

fn action_to_host_call(
    envelope: &ProtocolEnvelope,
    action: &serde_json::Value,
) -> Result<SandboxHostCallEnvelope, AppError> {
    let kind = action
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| AppError::bad_request("action kind is required"))?;
    let call = match kind {
        "read_context" => SandboxHostCall::ReadContext {
            query: action
                .get("query")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("current_context")
                .to_string(),
        },
        "request_capability" => SandboxHostCall::RequestCapability {
            capability: action
                .get("capability")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("read:context")
                .to_string(),
            reason: action
                .get("reason")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("sdk requested capability")
                .to_string(),
        },
        "submit_action_batch" => SandboxHostCall::SubmitActionBatch {
            actions: action
                .get("actions")
                .and_then(serde_json::Value::as_array)
                .cloned()
                .unwrap_or_default(),
        },
        "read_artifact" => SandboxHostCall::ReadArtifact {
            artifact_id: action
                .get("artifact_id")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| AppError::bad_request("artifact_id is required"))?
                .to_string(),
        },
        other => {
            return Err(AppError::bad_request(format!(
                "unsupported sdk action kind: {other}"
            )))
        }
    };

    Ok(SandboxHostCallEnvelope {
        abi_version: "sandbox_abi/v1".to_string(),
        module_id: envelope.sender.actor_id.clone(),
        run_id: envelope.trace.run_id.clone(),
        trace_id: envelope.trace.trace_id.clone(),
        call,
    })
}

fn deserialize_granted_capabilities(
    raw: &str,
) -> Result<Vec<CapabilityDescriptor>, serde_json::Error> {
    serde_json::from_str(raw)
}
