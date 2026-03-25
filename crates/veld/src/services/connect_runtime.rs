use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
    sync::OnceLock,
};

use serde_json::{json, Value as JsonValue};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin},
    sync::Mutex,
};
use vel_core::{
    CapabilityDescriptor, ConnectInstance, ConnectInstanceCapabilityManifest,
    ConnectInstanceStatus, ConnectRuntimeCapability, RunId, RunKind, RunStatus, TraceId,
};
use vel_storage::{ConnectRunEventRecord, ConnectRunRecord};

use crate::{errors::AppError, state::AppState};

type RuntimeRegistry = Mutex<HashMap<String, ManagedRuntime>>;
const MAX_EVENT_CHUNK_BYTES: usize = 8 * 1024;
const MAX_STDIN_WRITE_BYTES: usize = 8 * 1024;

#[derive(Debug)]
struct ManagedRuntime {
    child: Child,
    stdin: Option<ChildStdin>,
}

#[derive(Debug, Clone)]
pub struct LaunchConnectRuntimeRequest {
    pub runtime_kind: String,
    pub actor_id: String,
    pub display_name: Option<String>,
    pub command: Vec<String>,
    pub working_dir: Option<String>,
    pub writable_roots: Vec<String>,
    pub capability_allowlist: Vec<CapabilityDescriptor>,
    pub lease_seconds: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ConnectHeartbeatAck {
    pub id: String,
    pub status: String,
    pub lease_expires_at: i64,
    pub trace_id: String,
}

#[derive(Debug, Clone)]
pub struct ConnectStdinWriteAck {
    pub run_id: String,
    pub accepted_bytes: u32,
    pub event_id: i64,
    pub trace_id: Option<String>,
}

fn runtime_registry() -> &'static RuntimeRegistry {
    static REGISTRY: OnceLock<RuntimeRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

pub async fn launch_connect_runtime(
    state: &AppState,
    request: LaunchConnectRuntimeRequest,
) -> Result<ConnectInstance, AppError> {
    reconcile_connect_runtime_state(state).await?;
    validate_launch_request(&request)?;

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let lease_seconds = request
        .lease_seconds
        .unwrap_or_else(crate::services::agent_protocol::default_connect_lease_seconds)
        .clamp(1, 3600);
    let run_id = RunId::new();
    let trace_id = TraceId::new();
    let input_json = json!({
        "runtime_kind": request.runtime_kind,
        "actor_id": request.actor_id,
        "display_name": request.display_name,
        "command": request.command,
        "working_dir": request.working_dir,
        "writable_roots": request.writable_roots,
        "capability_allowlist": request.capability_allowlist,
        "lease_seconds": lease_seconds,
        "trace_id": trace_id,
    });

    state
        .storage
        .create_run(&run_id, RunKind::Agent, &input_json)
        .await?;
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
        .insert_connect_run(
            run_id.as_ref(),
            &request.actor_id,
            state.config.node_id.as_deref().unwrap_or("authority"),
            &serde_json::to_string(&request.capability_allowlist)
                .map_err(|error| AppError::internal(error.to_string()))?,
            now + lease_seconds,
            now,
        )
        .await?;

    match request.runtime_kind.as_str() {
        "local_command" => {
            let mut child = match spawn_local_runtime(&request).await {
                Ok(child) => child,
                Err(error) => {
                    let error_json = json!({ "reason": error.to_string() });
                    state
                        .storage
                        .update_run_status(
                            run_id.as_ref(),
                            RunStatus::Failed,
                            Some(now),
                            Some(now),
                            None,
                            Some(&error_json),
                        )
                        .await?;
                    state
                        .storage
                        .terminate_connect_run(run_id.as_ref(), now, "launch_failed")
                        .await?;
                    return Err(AppError::internal(format!(
                        "launch connect runtime: {error}"
                    )));
                }
            };

            let stdin = child.stdin.take();
            if let Some(stdout) = child.stdout.take() {
                spawn_output_pump(state.storage.clone(), run_id.to_string(), "stdout", stdout);
            }
            if let Some(stderr) = child.stderr.take() {
                spawn_output_pump(state.storage.clone(), run_id.to_string(), "stderr", stderr);
            }

            runtime_registry()
                .lock()
                .await
                .insert(run_id.to_string(), ManagedRuntime { child, stdin });
        }
        "wasm_guest" => {
            let module_path = request
                .command
                .first()
                .cloned()
                .ok_or_else(|| AppError::bad_request("wasm_guest requires a module path"))?;
            let working_dir = request
                .working_dir
                .clone()
                .ok_or_else(|| AppError::bad_request("working_dir is required for wasm_guest"))?;

            let outcome = crate::services::wasm_guest_runtime::execute_guest_module(
                state,
                &run_id,
                &trace_id,
                &crate::services::wasm_guest_runtime::WasmGuestLaunchRequest {
                    module_path,
                    working_dir,
                    writable_roots: request.writable_roots.clone(),
                    capability_allowlist: request.capability_allowlist.clone(),
                },
            )
            .await;

            let terminated_at = time::OffsetDateTime::now_utc().unix_timestamp();
            match outcome {
                Ok(result) => {
                    let terminal_reason = match result.terminal_status {
                        vel_core::SandboxDecisionStatus::Approved => "guest_completed",
                        vel_core::SandboxDecisionStatus::Denied => "guest_denied",
                        vel_core::SandboxDecisionStatus::Failed => "guest_failed",
                    };
                    state
                        .storage
                        .terminate_connect_run(run_id.as_ref(), terminated_at, terminal_reason)
                        .await?;
                    if result.terminal_status != vel_core::SandboxDecisionStatus::Approved {
                        return Err(AppError::forbidden(format!(
                            "wasm guest {} denied: {}",
                            result.module_id, result.reason
                        )));
                    }
                }
                Err(error) => {
                    state
                        .storage
                        .update_run_status(
                            run_id.as_ref(),
                            RunStatus::Failed,
                            Some(now),
                            Some(terminated_at),
                            None,
                            Some(&json!({ "reason": error.to_string() })),
                        )
                        .await?;
                    state
                        .storage
                        .terminate_connect_run(
                            run_id.as_ref(),
                            terminated_at,
                            "guest_launch_failed",
                        )
                        .await?;
                    return Err(error);
                }
            }
        }
        _ => unreachable!("validate_launch_request enforces runtime kind"),
    }

    get_connect_instance(state, run_id.as_ref()).await
}

pub async fn list_connect_instances(state: &AppState) -> Result<Vec<ConnectInstance>, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let records = state.storage.list_connect_runs(None).await?;
    let mut instances = Vec::with_capacity(records.len());
    for record in records {
        instances.push(instance_from_record(state, record).await?);
    }
    Ok(instances)
}

pub async fn get_connect_instance(state: &AppState, id: &str) -> Result<ConnectInstance, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let record = state
        .storage
        .get_connect_run(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    instance_from_record(state, record).await
}

pub async fn heartbeat_connect_instance(
    state: &AppState,
    id: &str,
    status: &str,
) -> Result<ConnectHeartbeatAck, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let id = id.trim();
    let record = state
        .storage
        .get_connect_run(id)
        .await?
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    if record.status != "running" {
        return Err(AppError::bad_request(format!(
            "connect instance {} is not running",
            id
        )));
    }

    let lease_seconds = run_input_json(state, id)
        .await?
        .get("lease_seconds")
        .and_then(JsonValue::as_i64)
        .unwrap_or_else(crate::services::agent_protocol::default_connect_lease_seconds);
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let lease_expires_at = now + lease_seconds;
    state
        .storage
        .update_connect_heartbeat(id, lease_expires_at)
        .await?;

    Ok(ConnectHeartbeatAck {
        id: id.to_string(),
        status: status.trim().to_string(),
        lease_expires_at,
        trace_id: run_input_json(state, id)
            .await?
            .get("trace_id")
            .and_then(JsonValue::as_str)
            .unwrap_or(id)
            .to_string(),
    })
}

pub async fn terminate_connect_instance(
    state: &AppState,
    id: &str,
    reason: &str,
) -> Result<ConnectInstance, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let id = id.trim();
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    if let Some(mut runtime) = runtime_registry().lock().await.remove(id) {
        let _ = runtime.child.kill().await;
        let _ = runtime.child.wait().await;
    }
    state.storage.terminate_connect_run(id, now, reason).await?;
    state
        .storage
        .update_run_status(
            id,
            RunStatus::Cancelled,
            None,
            Some(now),
            Some(&json!({ "terminal_reason": reason })),
            None,
        )
        .await?;
    get_connect_instance(state, id).await
}

pub async fn write_connect_instance_stdin(
    state: &AppState,
    id: &str,
    input: &str,
) -> Result<ConnectStdinWriteAck, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let id = id.trim();
    let record = state
        .storage
        .get_connect_run(id)
        .await?
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    if record.status != "running" {
        return Err(AppError::bad_request(format!(
            "connect instance {} is not running",
            id
        )));
    }

    let payload = input.as_bytes();
    if payload.is_empty() {
        return Err(AppError::bad_request("stdin input must not be empty"));
    }
    if payload.len() > MAX_STDIN_WRITE_BYTES {
        return Err(AppError::bad_request(format!(
            "stdin input exceeds {} bytes",
            MAX_STDIN_WRITE_BYTES
        )));
    }

    let mut registry = runtime_registry().lock().await;
    let runtime = registry
        .get_mut(id)
        .ok_or_else(|| AppError::bad_request("runtime process is not available"))?;
    let stdin = runtime
        .stdin
        .as_mut()
        .ok_or_else(|| AppError::bad_request("runtime does not accept stdin"))?;
    stdin
        .write_all(payload)
        .await
        .map_err(|error| AppError::internal(error.to_string()))?;
    stdin
        .flush()
        .await
        .map_err(|error| AppError::internal(error.to_string()))?;
    drop(registry);

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let chunk = truncate_event_chunk(input);
    let event_id = state
        .storage
        .insert_connect_run_event(id, "stdin", &chunk, now)
        .await?;
    let trace_id = run_input_json(state, id)
        .await?
        .get("trace_id")
        .and_then(JsonValue::as_str)
        .map(ToString::to_string);

    Ok(ConnectStdinWriteAck {
        run_id: id.to_string(),
        accepted_bytes: payload.len() as u32,
        event_id,
        trace_id,
    })
}

pub async fn list_connect_instance_events(
    state: &AppState,
    id: &str,
    after_id: Option<i64>,
    limit: u32,
) -> Result<Vec<ConnectRunEventRecord>, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let id = id.trim();
    state
        .storage
        .get_connect_run(id)
        .await?
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    let events = state
        .storage
        .list_connect_run_events(id, after_id, limit)
        .await?;
    Ok(events)
}

pub async fn latest_connect_instance_event_id(
    state: &AppState,
    id: &str,
) -> Result<Option<i64>, AppError> {
    reconcile_connect_runtime_state(state).await?;
    let id = id.trim();
    state
        .storage
        .get_connect_run(id)
        .await?
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    state
        .storage
        .latest_connect_run_event_id(id)
        .await
        .map_err(Into::into)
}

pub async fn reconcile_connect_runtime_state(state: &AppState) -> Result<(), AppError> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let running_records = state.storage.list_connect_runs(Some("running")).await?;
    let expired_ids: Vec<String> = running_records
        .iter()
        .filter(|record| record.lease_expires_at < now)
        .map(|record| record.id.clone())
        .collect();

    if !expired_ids.is_empty() {
        state.storage.expire_stale_connect_runs(now).await?;
    }

    let mut registry = runtime_registry().lock().await;
    let mut expired_runtimes = Vec::new();
    for id in &expired_ids {
        if let Some(runtime) = registry.remove(id) {
            expired_runtimes.push((id.clone(), runtime));
        }
    }

    let mut exited = Vec::new();
    for (id, runtime) in registry.iter_mut() {
        if let Some(status) = runtime
            .child
            .try_wait()
            .map_err(|error| AppError::internal(error.to_string()))?
        {
            exited.push((id.clone(), status.code(), status.success()));
        }
    }
    for (id, _, _) in &exited {
        registry.remove(id);
    }
    drop(registry);

    for (id, mut runtime) in expired_runtimes {
        let _ = runtime.child.kill().await;
        let _ = runtime.child.wait().await;
        let lease_expired_error = json!({ "reason": "lease_expired" });
        let _ = state
            .storage
            .insert_connect_run_event(
                &id,
                "system",
                "runtime lease expired; process terminated",
                now,
            )
            .await;
        state
            .storage
            .update_run_status(
                &id,
                RunStatus::Expired,
                None,
                Some(now),
                None,
                Some(&lease_expired_error),
            )
            .await?;
    }

    for (id, code, success) in exited {
        let reason = if success {
            "process_exited"
        } else {
            "process_failed"
        };
        let _ = state
            .storage
            .insert_connect_run_event(
                &id,
                "system",
                &format!("runtime exited with status {}", code.unwrap_or(-1)),
                now,
            )
            .await;
        let output_json = json!({ "exit_code": code });
        let error_json = (!success).then(|| json!({ "reason": reason }));
        state
            .storage
            .terminate_connect_run(&id, now, reason)
            .await?;
        state
            .storage
            .update_run_status(
                &id,
                if success {
                    RunStatus::Succeeded
                } else {
                    RunStatus::Failed
                },
                None,
                Some(now),
                Some(&output_json),
                error_json.as_ref(),
            )
            .await?;
    }

    Ok(())
}

async fn instance_from_record(
    state: &AppState,
    record: ConnectRunRecord,
) -> Result<ConnectInstance, AppError> {
    let input = run_input_json(state, &record.id).await?;
    let runtime_kind = input
        .get("runtime_kind")
        .and_then(JsonValue::as_str)
        .unwrap_or("local_command")
        .to_string();
    let display_name = input
        .get("display_name")
        .and_then(JsonValue::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(|| record.agent_id.clone());
    let writable_roots = input
        .get("writable_roots")
        .and_then(JsonValue::as_array)
        .cloned()
        .unwrap_or_default();

    let capability_allowlist: Vec<CapabilityDescriptor> =
        serde_json::from_str(&record.capabilities_json).unwrap_or_default();
    let manifest = ConnectInstanceCapabilityManifest {
        worker_classes: vec!["local_runtime".to_string()],
        capabilities: capability_allowlist
            .iter()
            .map(|capability| capability.scope.clone())
            .collect(),
        launchable_runtimes: vec![ConnectRuntimeCapability {
            runtime_id: runtime_kind.clone(),
            display_name: display_name.clone(),
            supports_launch: true,
            supports_interactive_followup: false,
            supports_native_open: false,
            supports_host_agent_control: true,
        }],
        supports_agent_launch: true,
        supports_interactive_followup: false,
        supports_native_open: false,
        supports_host_agent_control: true,
    };

    Ok(ConnectInstance {
        id: record.id.clone(),
        node_id: record.node_id,
        display_name,
        connection_id: Some(record.id.clone()),
        status: map_connect_status(&record.status),
        reachability: if record.status == "running" {
            "local".to_string()
        } else {
            "offline".to_string()
        },
        sync_base_url: None,
        sync_transport: Some("local_process".to_string()),
        tailscale_base_url: None,
        lan_base_url: None,
        localhost_base_url: Some("http://127.0.0.1:4130".to_string()),
        worker_ids: vec![record.agent_id.clone()],
        worker_classes: manifest.worker_classes.clone(),
        last_seen_at: time::OffsetDateTime::from_unix_timestamp(record.lease_expires_at).ok(),
        manifest,
        metadata_json: json!({
            "trace_id": input.get("trace_id").cloned().unwrap_or(JsonValue::Null),
            "runtime_kind": runtime_kind,
            "command": input.get("command").cloned().unwrap_or(JsonValue::Null),
            "working_dir": input.get("working_dir").cloned().unwrap_or(JsonValue::Null),
            "writable_roots": writable_roots,
            "lease_expires_at": record.lease_expires_at,
            "terminal_reason": record.terminal_reason,
        }),
    })
}

async fn run_input_json(state: &AppState, run_id: &str) -> Result<JsonValue, AppError> {
    let run = state
        .storage
        .get_run_by_id(run_id)
        .await?
        .ok_or_else(|| AppError::not_found("connect run backing run not found"))?;
    Ok(run.input_json)
}

fn validate_launch_request(request: &LaunchConnectRuntimeRequest) -> Result<(), AppError> {
    if request.actor_id.trim().is_empty() {
        return Err(AppError::bad_request("actor_id is required"));
    }
    if request.command.is_empty() || request.command[0].trim().is_empty() {
        return Err(AppError::bad_request("command must include an executable"));
    }
    if request.writable_roots.is_empty() {
        return Err(AppError::bad_request(
            "writable_roots must declare at least one bounded write root",
        ));
    }

    let working_dir = request.working_dir.as_deref().ok_or_else(|| {
        AppError::bad_request(format!(
            "working_dir is required for {}",
            request.runtime_kind
        ))
    })?;
    let working_dir = canonicalize_existing_dir(working_dir)?;
    for root in &request.writable_roots {
        let root = canonicalize_existing_dir(root)?;
        if !root.starts_with(&working_dir) {
            return Err(AppError::forbidden(format!(
                "writable root {} escapes working_dir {}",
                root.display(),
                working_dir.display()
            )));
        }
    }

    match request.runtime_kind.as_str() {
        "local_command" => {}
        "wasm_guest" => {
            if request.command.len() != 1 {
                return Err(AppError::bad_request(
                    "wasm_guest expects command[0] to be the guest module spec path",
                ));
            }
            let module_path = Path::new(&request.command[0]);
            if !module_path.exists() {
                return Err(AppError::bad_request(format!(
                    "guest module {} does not exist",
                    module_path.display()
                )));
            }
        }
        other => {
            return Err(AppError::bad_request(format!(
                "unsupported runtime kind: {}",
                other
            )));
        }
    }

    Ok(())
}

async fn spawn_local_runtime(
    request: &LaunchConnectRuntimeRequest,
) -> Result<Child, std::io::Error> {
    let mut command = tokio::process::Command::new(&request.command[0]);
    if request.command.len() > 1 {
        command.args(&request.command[1..]);
    }
    if let Some(working_dir) = &request.working_dir {
        command.current_dir(working_dir);
    }
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    command.spawn()
}

fn spawn_output_pump(
    storage: vel_storage::Storage,
    run_id: String,
    stream: &'static str,
    reader: impl tokio::io::AsyncRead + Unpin + Send + 'static,
) {
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let chunk = truncate_event_chunk(&line);
                    let _ = storage
                        .insert_connect_run_event(
                            &run_id,
                            stream,
                            &chunk,
                            time::OffsetDateTime::now_utc().unix_timestamp(),
                        )
                        .await;
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }
    });
}

fn truncate_event_chunk(value: &str) -> String {
    if value.len() <= MAX_EVENT_CHUNK_BYTES {
        return value.to_string();
    }
    let mut end = MAX_EVENT_CHUNK_BYTES;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_string()
}

fn canonicalize_existing_dir(value: &str) -> Result<PathBuf, AppError> {
    let path = Path::new(value);
    let canonical = std::fs::canonicalize(path)
        .map_err(|error| AppError::bad_request(format!("canonicalize {}: {}", value, error)))?;
    if !canonical.is_dir() {
        return Err(AppError::bad_request(format!(
            "{} must point to a directory",
            canonical.display()
        )));
    }
    Ok(canonical)
}

fn map_connect_status(status: &str) -> ConnectInstanceStatus {
    match status {
        "running" => ConnectInstanceStatus::Ready,
        "degraded" => ConnectInstanceStatus::Degraded,
        "terminated" | "expired" => ConnectInstanceStatus::Offline,
        _ => ConnectInstanceStatus::Unknown,
    }
}
