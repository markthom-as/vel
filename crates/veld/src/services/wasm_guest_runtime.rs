use std::{fs, path::Path};

use serde::Deserialize;
use vel_core::{
    CapabilityDescriptor, FilesystemAccessPolicy, NetworkAccessPolicy, RunEventType, RunId,
    SandboxCapabilityPolicy, SandboxDecisionStatus, SandboxHostCall, SandboxHostCallEnvelope,
    SandboxPolicyMode, SandboxResourceLimits, TraceId,
};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct WasmGuestModuleSpec {
    pub module_id: String,
    #[serde(default)]
    pub requested_writable_roots: Vec<String>,
    #[serde(default)]
    pub requested_hosts: Vec<String>,
    #[serde(default)]
    pub host_calls: Vec<SandboxHostCall>,
}

#[derive(Debug, Clone)]
pub struct WasmGuestLaunchRequest {
    pub module_path: String,
    pub working_dir: String,
    pub writable_roots: Vec<String>,
    pub capability_allowlist: Vec<CapabilityDescriptor>,
}

#[derive(Debug)]
pub struct WasmGuestExecutionResult {
    pub module_id: String,
    pub terminal_status: SandboxDecisionStatus,
    pub reason: String,
}

pub async fn execute_guest_module(
    state: &AppState,
    run_id: &RunId,
    trace_id: &TraceId,
    request: &WasmGuestLaunchRequest,
) -> Result<WasmGuestExecutionResult, AppError> {
    let module_path = canonicalize_existing_file(&request.module_path)?;
    let working_dir = canonicalize_existing_dir(&request.working_dir)?;
    let declared_write_roots = canonicalize_dirs(&request.writable_roots)?;
    let spec = load_guest_spec(&module_path)?;

    validate_guest_scope(&spec, &working_dir, &declared_write_roots)?;

    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunStarted,
            &serde_json::json!({
                "trace_id": trace_id,
                "module_id": spec.module_id,
                "runtime_kind": "wasm_guest",
                "module_path": module_path,
            }),
        )
        .await?;

    let policy = SandboxCapabilityPolicy {
        default_mode: SandboxPolicyMode::Brokered,
        allowed_calls: spec
            .host_calls
            .iter()
            .map(call_kind_string)
            .collect::<Vec<_>>(),
        filesystem: FilesystemAccessPolicy {
            read_roots: vec![working_dir.display().to_string()],
            write_roots: declared_write_roots
                .iter()
                .map(|path| path.display().to_string())
                .collect(),
        },
        network: NetworkAccessPolicy {
            allowed_hosts: Vec::new(),
        },
        resources: SandboxResourceLimits {
            max_fuel: 100_000,
            max_memory_bytes: 4 * 1024 * 1024,
            wall_timeout_ms: 1_000,
        },
        review_gate: "operator_preview".to_string(),
    };

    let envelopes = spec
        .host_calls
        .iter()
        .cloned()
        .map(|call| SandboxHostCallEnvelope {
            abi_version: "sandbox_abi/v1".to_string(),
            module_id: spec.module_id.clone(),
            run_id: run_id.to_string(),
            trace_id: trace_id.to_string(),
            call,
        })
        .collect::<Vec<_>>();

    let result = crate::services::sandbox::execute_host_calls(
        state,
        &policy,
        &request.capability_allowlist,
        &envelopes,
    )
    .await?;

    let reason = result
        .decisions
        .last()
        .map(|decision| decision.reason.clone())
        .unwrap_or_else(|| "approved".to_string());

    Ok(WasmGuestExecutionResult {
        module_id: spec.module_id,
        terminal_status: result.terminal_status,
        reason,
    })
}

fn load_guest_spec(path: &Path) -> Result<WasmGuestModuleSpec, AppError> {
    let raw = fs::read_to_string(path).map_err(|error| {
        AppError::bad_request(format!("read guest module {}: {}", path.display(), error))
    })?;
    serde_json::from_str(&raw).map_err(|error| {
        AppError::bad_request(format!("parse guest module {}: {}", path.display(), error))
    })
}

fn validate_guest_scope(
    spec: &WasmGuestModuleSpec,
    working_dir: &Path,
    declared_write_roots: &[std::path::PathBuf],
) -> Result<(), AppError> {
    if spec.module_id.trim().is_empty() {
        return Err(AppError::bad_request("wasm guest module_id is required"));
    }
    if spec.host_calls.is_empty() {
        return Err(AppError::bad_request(
            "wasm guest must declare at least one host call",
        ));
    }
    if !spec.requested_hosts.is_empty() {
        return Err(AppError::forbidden(
            "wasm guest cannot widen network scope beyond the manifest policy",
        ));
    }

    for requested in &spec.requested_writable_roots {
        let canonical = canonicalize_existing_dir(requested)?;
        if !canonical.starts_with(working_dir) {
            return Err(AppError::forbidden(format!(
                "guest writable root {} escapes working_dir {}",
                canonical.display(),
                working_dir.display()
            )));
        }
        if !declared_write_roots
            .iter()
            .any(|declared| declared == &canonical)
        {
            return Err(AppError::forbidden(format!(
                "guest writable root {} exceeds declared manifest writable roots",
                canonical.display()
            )));
        }
    }

    Ok(())
}

fn canonicalize_existing_dir(value: &str) -> Result<std::path::PathBuf, AppError> {
    let canonical = std::fs::canonicalize(Path::new(value))
        .map_err(|error| AppError::bad_request(format!("canonicalize {}: {}", value, error)))?;
    if !canonical.is_dir() {
        return Err(AppError::bad_request(format!(
            "{} must point to a directory",
            canonical.display()
        )));
    }
    Ok(canonical)
}

fn canonicalize_existing_file(value: &str) -> Result<std::path::PathBuf, AppError> {
    let canonical = std::fs::canonicalize(Path::new(value))
        .map_err(|error| AppError::bad_request(format!("canonicalize {}: {}", value, error)))?;
    if !canonical.is_file() {
        return Err(AppError::bad_request(format!(
            "{} must point to a file",
            canonical.display()
        )));
    }
    Ok(canonical)
}

fn canonicalize_dirs(values: &[String]) -> Result<Vec<std::path::PathBuf>, AppError> {
    values
        .iter()
        .map(|value| canonicalize_existing_dir(value))
        .collect()
}

fn call_kind_string(call: &SandboxHostCall) -> String {
    match call {
        SandboxHostCall::ReadContext { .. } => "read_context".to_string(),
        SandboxHostCall::RequestCapability { .. } => "request_capability".to_string(),
        SandboxHostCall::SubmitActionBatch { .. } => "submit_action_batch".to_string(),
        SandboxHostCall::ReadArtifact { .. } => "read_artifact".to_string(),
    }
}
