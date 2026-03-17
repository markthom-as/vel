use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};
use vel_storage::{
    CommitmentInsert, CaptureInsert, ClusterWorkerUpsert, ClusterWorkerRecord, SignalInsert,
    SignalRecord, WorkAssignmentInsert, WorkAssignmentRecord, WorkAssignmentStatus,
    WorkAssignmentUpdate,
};
use vel_core::{Commitment, CommitmentStatus, PrivacyClass};

#[derive(Debug, Clone)]
pub struct SyncBootstrap {
    pub cluster: ClusterBootstrap,
    pub current_context: Option<CurrentContext>,
    pub nudges: Vec<vel_storage::NudgeRecord>,
    pub commitments: Vec<Commitment>,
}

#[derive(Debug, Clone)]
pub struct ClusterBootstrap {
    pub node_id: String,
    pub node_display_name: String,
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub capabilities: Vec<String>,
    pub branch_sync: Option<BranchSyncCapability>,
    pub validation_profiles: Vec<ValidationProfile>,
}

#[derive(Debug, Clone)]
pub struct BranchSyncCapability {
    pub repo_root: String,
    pub default_remote: String,
    pub supports_fetch: bool,
    pub supports_pull: bool,
    pub supports_push: bool,
}

#[derive(Debug, Clone)]
pub struct ValidationProfile {
    pub profile_id: String,
    pub label: String,
    pub command_hint: String,
    pub environment: String,
}

#[derive(Debug, Clone)]
pub struct CurrentContext {
    pub computed_at: i64,
    pub context: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ClusterWorkers {
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub generated_at: i64,
    pub workers: Vec<WorkerPresence>,
}

#[derive(Debug, Clone)]
pub struct WorkerPresence {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: String,
    pub client_kind: Option<String>,
    pub client_version: Option<String>,
    pub protocol_version: Option<String>,
    pub build_id: Option<String>,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub status: String,
    pub queue_depth: u32,
    pub reachability: String,
    pub latency_class: String,
    pub compute_class: String,
    pub power_class: String,
    pub recent_failure_rate: f64,
    pub tailscale_preferred: bool,
    pub last_heartbeat_at: i64,
    pub started_at: Option<i64>,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub ping_ms: Option<u32>,
    pub sync_status: String,
    pub last_upstream_sync_at: Option<i64>,
    pub last_downstream_sync_at: Option<i64>,
    pub last_sync_error: Option<String>,
    pub updated_at: i64,
    pub capacity: WorkerCapacity,
}

#[derive(Debug, Clone)]
pub struct WorkerCapacity {
    pub max_concurrency: u32,
    pub current_load: u32,
    pub available_concurrency: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueuedWorkRoutingKind {
    BranchSync,
    Validation,
}

#[derive(Debug, Clone)]
pub struct QueuedWorkRouting {
    pub work_request_id: String,
    pub request_type: QueuedWorkRoutingKind,
    pub status: String,
    pub queued_signal_id: String,
    pub queued_signal_type: String,
    pub queued_at: i64,
    pub queued_via: String,
    pub authority_node_id: String,
    pub authority_epoch: i64,
    pub target_node_id: String,
    pub target_worker_class: String,
    pub requested_capability: String,
    pub request_payload: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct QueuedWorkItem {
    pub work_request_id: String,
    pub request_type: QueuedWorkRoutingKind,
    pub queued_signal_id: String,
    pub queued_signal_type: String,
    pub queued_at: i64,
    pub target_node_id: String,
    pub target_worker_class: String,
    pub requested_capability: String,
    pub request_payload: serde_json::Value,
    pub latest_receipt: Option<WorkAssignmentRecord>,
    pub is_stale: bool,
    pub attempt_count: u32,
    pub claimable_now: bool,
    pub claim_reason: Option<String>,
    pub next_retry_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct WorkAssignmentClaimNextResponse {
    pub claim: Option<WorkAssignmentClaimedWork>,
}

#[derive(Debug, Clone)]
pub struct WorkAssignmentClaimedWork {
    pub queue_item: QueuedWorkItem,
    pub receipt: WorkAssignmentRecord,
}

#[derive(Debug, Clone)]
pub struct ClientActionBatchResult {
    pub applied: u32,
    pub results: Vec<ClientActionResult>,
}

#[derive(Debug, Clone)]
pub struct ClientActionResult {
    pub action_id: Option<String>,
    pub action_type: String,
    pub target_id: Option<String>,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct SyncHeartbeatResponse {
    pub accepted: bool,
    pub worker_id: String,
    pub expires_at: i64,
    pub cluster_view_version: i64,
    pub placement_hints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SyncClusterState {
    pub cluster_view_version: Option<i64>,
    pub authority_node_id: Option<String>,
    pub authority_epoch: Option<i64>,
    pub sync_transport: Option<String>,
    pub cluster: Option<ClusterBootstrap>,
    pub nodes: Vec<ClusterNodeState>,
    pub workers: Vec<WorkerPresence>,
}

#[derive(Debug, Clone)]
pub struct ClusterNodeState {
    pub node_id: String,
    pub node_display_name: Option<String>,
    pub node_class: Option<String>,
    pub sync_base_url: Option<String>,
    pub sync_transport: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub capabilities: Vec<String>,
    pub reachability: Option<String>,
    pub last_seen_at: Option<i64>,
}

const WORKER_HEARTBEAT_TTL_SECONDS: i64 = 90;
const WORK_ASSIGNMENT_STALE_SECONDS: i64 = 300;

pub async fn build_sync_bootstrap(state: &AppState) -> Result<SyncBootstrap, AppError> {
    let current_context =
        state
            .storage
            .get_current_context()
            .await?
            .map(|(computed_at, context)| CurrentContext {
                computed_at,
                context: context.into_json(),
            });

    let active = state.storage.list_nudges(Some("active"), 50).await?;
    let pending = state.storage.list_nudges(Some("pending"), 50).await?;
    let snoozed = state.storage.list_nudges(Some("snoozed"), 50).await?;
    let mut nudges = active;
    nudges.extend(pending);
    nudges.extend(snoozed);

    let commitments = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;

    Ok(SyncBootstrap {
        cluster: effective_cluster_bootstrap(state).await?,
        current_context,
        nudges,
        commitments,
    })
}

pub async fn build_sync_cluster_state(state: &AppState) -> Result<SyncClusterState, AppError> {
    let cluster = effective_cluster_bootstrap(state).await?;
    let workers = cluster_workers_data(state).await?;
    let mut nodes_map = BTreeMap::new();

    for worker in &workers.workers {
        nodes_map
            .entry(worker.node_id.clone())
            .or_insert_with(|| ClusterNodeState {
                node_id: worker.node_id.clone(),
                node_display_name: Some(worker.node_display_name.clone()),
                node_class: Some(if worker.node_id == cluster.active_authority_node_id {
                    "authority".to_string()
                } else {
                    "worker".to_string()
                }),
                sync_base_url: Some(worker.sync_base_url.clone()),
                sync_transport: Some(worker.sync_transport.clone()),
                tailscale_base_url: worker.tailscale_base_url.clone(),
                lan_base_url: worker.lan_base_url.clone(),
                localhost_base_url: worker.localhost_base_url.clone(),
                capabilities: worker.capabilities.clone(),
                reachability: Some(worker.reachability.clone()),
                last_seen_at: Some(worker.last_heartbeat_at),
            });
    }

    Ok(SyncClusterState {
        cluster_view_version: Some(workers.generated_at),
        authority_node_id: Some(cluster.active_authority_node_id.clone()),
        authority_epoch: Some(cluster.active_authority_epoch),
        sync_transport: Some(cluster.sync_transport.clone()),
        cluster: Some(cluster),
        nodes: nodes_map.into_values().collect(),
        workers: workers.workers,
    })
}

pub async fn effective_cluster_bootstrap_data(
    state: &AppState,
) -> Result<ClusterBootstrap, AppError> {
    effective_cluster_bootstrap(state).await
}

pub async fn cluster_bootstrap_data(state: &AppState) -> Result<ClusterBootstrap, AppError> {
    effective_cluster_bootstrap(state).await
}

pub async fn cluster_workers_data(state: &AppState) -> Result<ClusterWorkers, AppError> {
    refresh_local_worker_presence(state).await?;
    let bootstrap = effective_cluster_bootstrap(state).await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let _ = state
        .storage
        .expire_cluster_workers(now - WORKER_HEARTBEAT_TTL_SECONDS)
        .await?;
    let workers = state
        .storage
        .list_cluster_workers()
        .await?
        .into_iter()
        .map(worker_presence_from_record)
        .collect();

    Ok(ClusterWorkers {
        active_authority_node_id: bootstrap.active_authority_node_id.clone(),
        active_authority_epoch: bootstrap.active_authority_epoch,
        generated_at: now,
        workers,
    })
}

pub async fn effective_cluster_bootstrap(
    state: &AppState,
) -> Result<ClusterBootstrap, AppError> {
    let runtime_config =
        crate::services::operator_settings::runtime_sync_config(&state.storage, &state.config)
            .await?;
    Ok(cluster_bootstrap_from_config(&runtime_config))
}

pub fn cluster_bootstrap_from_app_config(config: &vel_config::AppConfig) -> ClusterBootstrap {
    cluster_bootstrap_from_config(config)
}

fn cluster_bootstrap_from_config(config: &vel_config::AppConfig) -> ClusterBootstrap {
    let node_id = config
        .node_id
        .clone()
        .unwrap_or_else(|| "vel-node".to_string());
    let node_display_name = config
        .node_display_name
        .clone()
        .unwrap_or_else(|| node_id.clone());
    let localhost_base_url = localhost_base_url(&config.bind_addr);
    let (sync_base_url, sync_transport) = preferred_sync_target(
        config.tailscale_base_url.as_deref(),
        config.base_url.as_str(),
        config.lan_base_url.as_deref(),
        localhost_base_url.as_deref(),
    );
    let repo_root = repo_root();

    ClusterBootstrap {
        node_id: node_id.clone(),
        node_display_name,
        active_authority_node_id: node_id,
        active_authority_epoch: 1,
        sync_base_url,
        sync_transport,
        tailscale_base_url: config.tailscale_base_url.clone(),
        lan_base_url: config.lan_base_url.clone(),
        localhost_base_url,
        capabilities: execution_capabilities(&repo_root),
        branch_sync: branch_sync_capability(&repo_root),
        validation_profiles: validation_profiles(&repo_root),
    }
}

pub async fn ingest_worker_heartbeat(
    state: &AppState,
    worker_id: String,
    node_id: String,
    node_display_name: Option<String>,
    client_kind: Option<String>,
    client_version: Option<String>,
    protocol_version: Option<String>,
    build_id: Option<String>,
    worker_classes: Vec<String>,
    capabilities: Vec<String>,
    status: Option<String>,
    max_concurrency: Option<u32>,
    current_load: Option<u32>,
    queue_depth: Option<u32>,
    reachability: Option<String>,
    latency_class: Option<String>,
    compute_class: Option<String>,
    power_class: Option<String>,
    recent_failure_rate: Option<f64>,
    tailscale_preferred: bool,
    sync_base_url: Option<String>,
    sync_transport: Option<String>,
    tailscale_base_url: Option<String>,
    preferred_tailnet_endpoint: Option<String>,
    tailscale_reachable: bool,
    lan_base_url: Option<String>,
    localhost_base_url: Option<String>,
    started_at: Option<i64>,
    last_heartbeat_at: Option<i64>,
) -> Result<SyncHeartbeatResponse, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let last_heartbeat_at = last_heartbeat_at.unwrap_or(now);

    state
        .storage
        .upsert_cluster_worker(ClusterWorkerUpsert {
            worker_id: worker_id.clone(),
            node_id,
            node_display_name,
            client_kind,
            client_version,
            protocol_version,
            build_id,
            worker_class: worker_classes.first().cloned(),
            worker_classes,
            capabilities,
            status,
            max_concurrency,
            current_load,
            queue_depth,
            reachability,
            latency_class,
            compute_class,
            power_class,
            recent_failure_rate,
            tailscale_preferred,
            sync_base_url,
            sync_transport: sync_transport.map(Some),
            tailscale_base_url,
            preferred_tailnet_endpoint,
            tailscale_reachable,
            lan_base_url,
            localhost_base_url,
            last_heartbeat_at,
            started_at,
        })
        .await?;

    Ok(SyncHeartbeatResponse {
        accepted: true,
        worker_id,
        expires_at: last_heartbeat_at + WORKER_HEARTBEAT_TTL_SECONDS,
        cluster_view_version: now,
        placement_hints: Vec::new(),
    })
}

pub(crate) async fn refresh_local_worker_presence(state: &AppState) -> Result<(), AppError> {
    let bootstrap = effective_cluster_bootstrap(state).await?;
    let runtime = state.worker_runtime.snapshot();
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let current_load = runtime.current_load.min(runtime.max_concurrency);
    let queue_depth = list_worker_queue_snapshot(state, &bootstrap.node_id, None, None)
        .await?
        .len() as u32;
    let tailscale_reachable = bootstrap
        .tailscale_base_url
        .as_ref()
        .map(|url: &String| !url.trim().is_empty())
        .unwrap_or(false);
    let worker_classes = worker_classes_for_capabilities(&bootstrap.capabilities);

    state
        .storage
        .upsert_cluster_worker(ClusterWorkerUpsert {
            worker_id: bootstrap.node_id.clone(),
            node_id: bootstrap.node_id.clone(),
            node_display_name: Some(bootstrap.node_display_name.clone()),
            client_kind: Some("veld".to_string()),
            client_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            protocol_version: Some("v1".to_string()),
            build_id: Some(format!("veld-{}", env!("CARGO_PKG_VERSION"))),
            worker_class: worker_classes.first().cloned(),
            worker_classes,
            capabilities: bootstrap.capabilities.clone(),
            status: Some("ready".to_string()),
            max_concurrency: Some(runtime.max_concurrency),
            current_load: Some(current_load),
            queue_depth: Some(queue_depth),
            reachability: Some("reachable".to_string()),
            latency_class: Some(latency_class_for_transport(&bootstrap.sync_transport)),
            compute_class: Some(compute_class_for_capacity(runtime.max_concurrency)),
            power_class: Some(infer_power_class(&bootstrap.node_id)),
            recent_failure_rate: Some(0.0),
            tailscale_preferred: tailscale_reachable,
            sync_base_url: Some(bootstrap.sync_base_url),
            sync_transport: Some(Some(bootstrap.sync_transport)),
            tailscale_base_url: bootstrap.tailscale_base_url.clone(),
            preferred_tailnet_endpoint: bootstrap.tailscale_base_url,
            tailscale_reachable,
            lan_base_url: bootstrap.lan_base_url,
            localhost_base_url: bootstrap.localhost_base_url,
            last_heartbeat_at: now,
            started_at: Some(runtime.started_at),
        })
        .await?;

    Ok(())
}

fn worker_presence_from_record(record: ClusterWorkerRecord) -> WorkerPresence {
    let max_concurrency = record.max_concurrency.unwrap_or(1);
    let current_load = record.current_load.unwrap_or(0).min(max_concurrency);

    WorkerPresence {
        worker_id: record.worker_id,
        node_id: record.node_id,
        node_display_name: record
            .node_display_name
            .unwrap_or_else(|| "Vel Node".to_string()),
        client_kind: record.client_kind,
        client_version: record.client_version,
        protocol_version: record.protocol_version,
        build_id: record.build_id,
        worker_classes: record.worker_classes,
        capabilities: record.capabilities,
        status: record.status,
        queue_depth: record.queue_depth.unwrap_or(0),
        reachability: record.reachability,
        latency_class: record.latency_class,
        compute_class: record.compute_class,
        power_class: record.power_class,
        recent_failure_rate: record.recent_failure_rate,
        tailscale_preferred: record.tailscale_preferred,
        last_heartbeat_at: record.last_heartbeat_at,
        started_at: Some(record.started_at),
        sync_base_url: record.sync_base_url.unwrap_or_default(),
        sync_transport: record.sync_transport,
        tailscale_base_url: record.tailscale_base_url,
        preferred_tailnet_endpoint: record.preferred_tailnet_endpoint,
        tailscale_reachable: record.tailscale_reachable,
        lan_base_url: record.lan_base_url,
        localhost_base_url: record.localhost_base_url,
        ping_ms: record.ping_ms,
        sync_status: record.sync_status,
        last_upstream_sync_at: record.last_upstream_sync_at,
        last_downstream_sync_at: record.last_downstream_sync_at,
        last_sync_error: record.last_sync_error,
        updated_at: record.updated_at,
        capacity: WorkerCapacity {
            max_concurrency,
            current_load,
            available_concurrency: max_concurrency.saturating_sub(current_load),
        },
    }
}

pub async fn claim_work_assignment(
    state: &AppState,
    work_request_id: String,
    worker_id: String,
    worker_class: Option<String>,
    capability: Option<String>,
) -> Result<WorkAssignmentRecord, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let existing = state
        .storage
        .list_work_assignments(Some(&work_request_id), None)
        .await?;
    if let Some(latest) = existing.first() {
        match latest.status {
            WorkAssignmentStatus::Assigned | WorkAssignmentStatus::Started => {
                if is_stale_assignment(latest, now) {
                    let _ = state
                        .storage
                        .insert_signal(SignalInsert {
                            signal_type: "work_assignment_reclaimed".to_string(),
                            source: "cluster_work_router".to_string(),
                            source_ref: Some(latest.receipt_id.clone()),
                            timestamp: now,
                            payload_json: Some(serde_json::json!({
                                "receipt_id": latest.receipt_id,
                                "work_request_id": latest.work_request_id,
                                "previous_worker_id": latest.worker_id,
                                "reclaimed_by": worker_id,
                            })),
                        })
                        .await;
                } else if latest.worker_id == worker_id {
                    return Ok(latest.clone());
                } else {
                    return Err(AppError::bad_request(format!(
                        "work request {} is already claimed by worker {}",
                        work_request_id, latest.worker_id
                    )));
                }
            }
            WorkAssignmentStatus::Completed => {
                return Ok(latest.clone());
            }
            WorkAssignmentStatus::Failed | WorkAssignmentStatus::Cancelled => {}
        }
    }

    let worker_class = worker_class.unwrap_or_else(|| "worker".to_string());
    let capability = capability.unwrap_or_else(|| "any".to_string());

    let receipt_id = state
        .storage
        .insert_work_assignment(WorkAssignmentInsert {
            receipt_id: None,
            work_request_id: work_request_id.clone(),
            worker_id: worker_id.clone(),
            worker_class: worker_class.clone(),
            capability: capability.clone(),
            status: WorkAssignmentStatus::Assigned,
            assigned_at: now,
        })
        .await?;

    let _ = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: "work_assignment_claimed".to_string(),
            source: "cluster_work_router".to_string(),
            source_ref: Some(receipt_id.clone()),
            timestamp: now,
            payload_json: Some(serde_json::json!({
                "receipt_id": receipt_id,
                "work_request_id": work_request_id,
                "worker_id": worker_id,
                "worker_class": worker_class,
                "capability": capability,
            })),
        })
        .await;

    let created = state
        .storage
        .list_work_assignments(Some(&work_request_id), Some(&worker_id))
        .await?
        .into_iter()
        .find(|assignment| assignment.status == WorkAssignmentStatus::Assigned)
        .ok_or_else(|| AppError::internal("claimed work assignment was not persisted"))?;
    Ok(created)
}

pub async fn update_work_assignment_receipt(
    state: &AppState,
    update: WorkAssignmentUpdate,
) -> Result<WorkAssignmentRecord, AppError> {
    let updated = state
        .storage
        .update_work_assignment(update)
        .await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let signal_type = match updated.status {
        WorkAssignmentStatus::Assigned => "work_assignment_claimed",
        WorkAssignmentStatus::Started => "work_assignment_started",
        WorkAssignmentStatus::Completed => "work_assignment_completed",
        WorkAssignmentStatus::Failed => "work_assignment_failed",
        WorkAssignmentStatus::Cancelled => "work_assignment_cancelled",
    };
    let _ = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: signal_type.to_string(),
            source: "cluster_work_router".to_string(),
            source_ref: Some(updated.receipt_id.clone()),
            timestamp: now,
            payload_json: Some(serde_json::json!({
                "receipt_id": updated.receipt_id,
                "work_request_id": updated.work_request_id,
                "worker_id": updated.worker_id,
                "status": signal_type,
                "result": updated.result,
                "error_message": updated.error_message,
            })),
        })
        .await;

    Ok(updated)
}

pub async fn list_work_assignment_receipts(
    state: &AppState,
    work_request_id: Option<&str>,
    worker_id: Option<&str>,
) -> Result<Vec<WorkAssignmentRecord>, AppError> {
    Ok(state
        .storage
        .list_work_assignments(work_request_id, worker_id)
        .await?)
}

pub async fn list_worker_queue(
    state: &AppState,
    node_id: &str,
    worker_class: Option<&str>,
    capability: Option<&str>,
) -> Result<Vec<QueuedWorkItem>, AppError> {
    refresh_local_worker_presence(state).await?;
    list_worker_queue_snapshot(state, node_id, worker_class, capability).await
}

async fn list_worker_queue_snapshot(
    state: &AppState,
    node_id: &str,
    worker_class: Option<&str>,
    capability: Option<&str>,
) -> Result<Vec<QueuedWorkItem>, AppError> {
    let branch = state
        .storage
        .list_signals(Some("client_branch_sync_requested"), None, 200)
        .await?;
    let validation = state
        .storage
        .list_signals(Some("client_validation_requested"), None, 200)
        .await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();

    let mut items = Vec::new();
    for signal in branch.into_iter().chain(validation.into_iter()) {
        let routing = signal
            .payload_json
            .get("routing")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let target_node_id = routing
            .get("target_node_id")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        let target_worker_class = routing
            .get("target_worker_class")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        let requested_capability = routing
            .get("requested_capability")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        if target_node_id != node_id {
            continue;
        }
        if let Some(worker_class) = worker_class {
            if target_worker_class != worker_class {
                continue;
            }
        }
        if let Some(capability) = capability {
            if requested_capability != capability {
                continue;
            }
        }

        let work_request_id = routing
            .get("work_request_id")
            .and_then(|value| value.as_str())
            .or(signal.source_ref.as_deref())
            .unwrap_or_default()
            .to_string();
        let history = state.storage.list_work_assignments(Some(&work_request_id), None).await?;
        let latest_receipt = history.first().cloned();
        let request_type = if signal.signal_type == "client_branch_sync_requested" {
            QueuedWorkRoutingKind::BranchSync
        } else {
            QueuedWorkRoutingKind::Validation
        };
        let schedule = evaluate_queue_schedule(
            &history,
            now,
            retry_policy_for_request_type(state, request_type),
        );
        if !schedule.include_in_queue {
            continue;
        }
        items.push(QueuedWorkItem {
            work_request_id,
            request_type,
            queued_signal_id: signal.signal_id,
            queued_signal_type: signal.signal_type,
            queued_at: signal.timestamp,
            target_node_id: target_node_id.to_string(),
            target_worker_class: target_worker_class.to_string(),
            requested_capability: requested_capability.to_string(),
            request_payload: signal
                .payload_json
                .get("request")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({})),
            latest_receipt,
            is_stale: schedule.is_stale,
            attempt_count: schedule.attempt_count,
            claimable_now: schedule.claimable_now,
            claim_reason: schedule.claim_reason,
            next_retry_at: Some(schedule.next_retry_at.unwrap_or(0)),
        });
    }
    items.sort_by_key(|item| (item.queued_at, item.work_request_id.clone()));
    Ok(items)
}

pub async fn claim_next_work_for_worker(
    state: &AppState,
    node_id: String,
    worker_id: String,
    worker_class: Option<String>,
    capability: Option<String>,
) -> Result<WorkAssignmentClaimNextResponse, AppError> {
    let mut items = list_worker_queue(
        state,
        &node_id,
        worker_class.as_deref(),
        capability.as_deref(),
    )
    .await?;
    items.retain(|item| item.claimable_now);
    items.sort_by_key(|item| {
        (
            claim_priority(item.claim_reason.as_deref()),
            item.queued_at,
            item.work_request_id.clone(),
        )
    });

    let Some(queue_item) = items.into_iter().next() else {
        return Ok(WorkAssignmentClaimNextResponse { claim: None });
    };

    let receipt = claim_work_assignment(
        state,
        queue_item.work_request_id.clone(),
        worker_id,
        worker_class
            .or_else(|| Some(queue_item.target_worker_class.clone())),
        capability
            .or_else(|| Some(queue_item.requested_capability.clone())),
    )
    .await?;

    Ok(WorkAssignmentClaimNextResponse {
        claim: Some(WorkAssignmentClaimedWork {
            queue_item,
            receipt,
        }),
    })
}

pub async fn apply_client_actions(
    state: &AppState,
    actions: Vec<crate::services::client_sync::ClientAction>,
) -> Result<ClientActionBatchResult, AppError> {
    let mut results = Vec::with_capacity(actions.len());
    let mut applied_count = 0u32;

    for action in actions {
        match apply_single_action(state, &action).await {
            Ok(result) => {
                if result.status == "applied" {
                    applied_count += 1;
                }
                results.push(result);
            }
            Err(error) => results.push(ClientActionResult {
                action_id: action.action_id.clone(),
                action_type: action.action_type.to_string(),
                target_id: action.target_id.clone(),
                status: "rejected".to_string(),
                message: error.to_string(),
            }),
        }
    }

    Ok(ClientActionBatchResult { applied: applied_count, results })
}

#[derive(Debug, Clone)]
pub struct ClientAction {
    pub action_id: Option<String>,
    pub action_type: ClientActionKind,
    pub target_id: Option<String>,
    pub text: Option<String>,
    pub minutes: Option<u32>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientActionKind {
    NudgeDone,
    NudgeSnooze,
    CommitmentDone,
    CommitmentCreate,
    CaptureCreate,
    BranchSyncRequest,
    ValidationRequest,
}

impl std::fmt::Display for ClientActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NudgeDone => "nudge_done",
            Self::NudgeSnooze => "nudge_snooze",
            Self::CommitmentDone => "commitment_done",
            Self::CommitmentCreate => "commitment_create",
            Self::CaptureCreate => "capture_create",
            Self::BranchSyncRequest => "branch_sync_request",
            Self::ValidationRequest => "validation_request",
        };
        f.write_str(s)
    }
}

async fn apply_single_action(
    state: &AppState,
    action: &ClientAction,
) -> Result<ClientActionResult, AppError> {
    match action.action_type {
        ClientActionKind::NudgeDone => {
            let target_id = require_target_id(action)?;
            let now = OffsetDateTime::now_utc().unix_timestamp();
            state
                .storage
                .update_nudge_state(&target_id, "resolved", None, Some(now))
                .await?;
            let _ = state
                .storage
                .insert_nudge_event(&target_id, "nudge_resolved", "{}", now)
                .await;
            if let Some(nudge) = state.storage.get_nudge(&target_id).await? {
                if let Some(commitment_id) = nudge.related_commitment_id {
                    let _ = state
                        .storage
                        .update_commitment(
                            &commitment_id,
                            None,
                            Some(CommitmentStatus::Done),
                            None,
                            None,
                            None,
                            None,
                        )
                        .await;
                }
            }
            Ok(applied(action, "nudge resolved"))
        }
        ClientActionKind::NudgeSnooze => {
            let target_id = require_target_id(action)?;
            let minutes = action.minutes.unwrap_or(10);
            let now = OffsetDateTime::now_utc();
            let snoozed_until = (now + time::Duration::minutes(minutes as i64)).unix_timestamp();
            state
                .storage
                .update_nudge_state(&target_id, "snoozed", Some(snoozed_until), None)
                .await?;
            let _ = state
                .storage
                .insert_nudge_event(
                    &target_id,
                    "nudge_snoozed",
                    &serde_json::json!({ "snoozed_until": snoozed_until, "minutes": minutes })
                        .to_string(),
                    now.unix_timestamp(),
                )
                .await;
            Ok(applied(action, "nudge snoozed"))
        }
        ClientActionKind::CommitmentDone => {
            let target_id = require_target_id(action)?;
            state
                .storage
                .update_commitment(
                    &target_id,
                    None,
                    Some(CommitmentStatus::Done),
                    None,
                    None,
                    None,
                    None,
                )
                .await?;
            Ok(applied(action, "commitment resolved"))
        }
        ClientActionKind::CommitmentCreate => {
            let text = require_text(action)?;
            state
                .storage
                .insert_commitment(CommitmentInsert {
                    text,
                    source_type: "apple".to_string(),
                    source_id: "".to_string(),
                    status: CommitmentStatus::Open,
                    due_at: None,
                    project: None,
                    commitment_kind: None,
                    metadata_json: Some(serde_json::json!({ "via": "sync_actions" })),
                })
                .await?;
            Ok(applied(action, "commitment created"))
        }
        ClientActionKind::CaptureCreate => {
            let text = require_text(action)?;
            let capture_id = state
                .storage
                .insert_capture(CaptureInsert {
                    content_text: text.clone(),
                    capture_type: "note".to_string(),
                    source_device: Some("apple".to_string()),
                    privacy_class: PrivacyClass::Private,
                })
                .await?;
            let now = OffsetDateTime::now_utc().unix_timestamp();
            let signal_payload = serde_json::json!({
                "capture_id": capture_id.to_string(),
                "content": text,
                "tags": []
            });
            let _ = state
                .storage
                .insert_signal(SignalInsert {
                    signal_type: "capture_created".to_string(),
                    source: "vel".to_string(),
                    source_ref: Some(capture_id.to_string()),
                    timestamp: now,
                    payload_json: Some(signal_payload),
                })
                .await;
            Ok(applied(action, "capture created"))
        }
        ClientActionKind::BranchSyncRequest => {
            let payload = action.payload.as_ref().ok_or_else(|| AppError::bad_request("branch sync payload is required"))?;
            let request: BranchSyncRequest = serde_json::from_value(payload.clone()).map_err(|e| AppError::bad_request(format!("invalid branch sync payload: {e}")))?;
            queue_branch_sync_request(
                state,
                request,
                "client_sync_actions",
                action.action_id.clone(),
            )
            .await?;
            Ok(applied(action, "branch sync request queued"))
        }
        ClientActionKind::ValidationRequest => {
            let payload = action.payload.as_ref().ok_or_else(|| AppError::bad_request("validation payload is required"))?;
            let request: ValidationRequest = serde_json::from_value(payload.clone()).map_err(|e| AppError::bad_request(format!("invalid validation payload: {e}")))?;
            queue_validation_request(
                state,
                request,
                "client_sync_actions",
                action.action_id.clone(),
            )
            .await?;
            Ok(applied(action, "validation request queued"))
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BranchSyncRequest {
    pub repo_root: String,
    pub branch: String,
    pub remote: Option<String>,
    pub base_branch: Option<String>,
    pub mode: Option<String>,
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ValidationRequest {
    pub repo_root: String,
    pub profile_id: String,
    pub branch: Option<String>,
    pub environment: Option<String>,
    pub requested_by: Option<String>,
}

fn require_target_id(action: &ClientAction) -> Result<String, AppError> {
    action
        .target_id
        .clone()
        .filter(|id| !id.trim().is_empty())
        .ok_or_else(|| AppError::bad_request("action target_id is required"))
}

fn require_text(action: &ClientAction) -> Result<String, AppError> {
    action
        .text
        .clone()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .ok_or_else(|| AppError::bad_request("action text is required"))
}

fn applied(action: &ClientAction, message: &str) -> ClientActionResult {
    ClientActionResult {
        action_id: action.action_id.clone(),
        action_type: action.action_type.to_string(),
        target_id: action.target_id.clone(),
        status: "applied".to_string(),
        message: message.to_string(),
    }
}

pub async fn queue_branch_sync_request(
    state: &AppState,
    request: BranchSyncRequest,
    queued_via: &str,
    work_request_id: Option<String>,
) -> Result<QueuedWorkRouting, AppError> {
    validate_branch_sync_request(&request)?;
    let requested_by = request
        .requested_by
        .clone()
        .unwrap_or_else(|| "client".to_string());
    queue_work_request(
        state,
        "client_branch_sync_requested",
        QueuedWorkRoutingKind::BranchSync,
        &requested_by,
        "repo_sync",
        "branch_sync",
        serde_json::to_value(request).unwrap(),
        queued_via,
        work_request_id,
    )
    .await
}

pub async fn queue_validation_request(
    state: &AppState,
    request: ValidationRequest,
    queued_via: &str,
    work_request_id: Option<String>,
) -> Result<QueuedWorkRouting, AppError> {
    validate_validation_request(&request)?;
    let requested_by = request
        .requested_by
        .clone()
        .unwrap_or_else(|| "client".to_string());
    queue_work_request(
        state,
        "client_validation_requested",
        QueuedWorkRoutingKind::Validation,
        &requested_by,
        "validation",
        "build_test_profiles",
        serde_json::to_value(request).unwrap(),
        queued_via,
        work_request_id,
    )
    .await
}

async fn queue_work_request(
    state: &AppState,
    signal_type: &str,
    request_type: QueuedWorkRoutingKind,
    _requested_by: &str,
    target_worker_class: &str,
    requested_capability: &str,
    request_payload: serde_json::Value,
    queued_via: &str,
    work_request_id: Option<String>,
) -> Result<QueuedWorkRouting, AppError> {
    let work_request_id = work_request_id.unwrap_or_else(|| format!("wrkreq_{}", Uuid::new_v4().simple()));
    let bootstrap = effective_cluster_bootstrap(state).await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();

    let workers = cluster_workers_data(state).await?;
    let mut eligible: Vec<_> = workers
        .workers
        .iter()
        .filter(|w| worker_matches_request(w, target_worker_class, requested_capability))
        .collect();

    eligible.sort_by_key(|w| std::cmp::Reverse(worker_placement_score(w)));
    let target_worker = eligible
        .first()
        .ok_or_else(|| AppError::bad_request("no eligible workers available for request"))?;

    let existing = state.storage.list_work_assignments(Some(&work_request_id), None).await?;
    if let Some(latest) = existing.first() {
        let retry_policy = retry_policy_for_request_type(state, request_type);
        let schedule = evaluate_queue_schedule(&existing, now, retry_policy);

        if matches!(
            latest.status,
            WorkAssignmentStatus::Completed | WorkAssignmentStatus::Cancelled
        ) {
            return build_existing_routing_response(
                state,
                signal_type,
                request_type,
                &work_request_id,
                match latest.status {
                    WorkAssignmentStatus::Completed => "completed",
                    WorkAssignmentStatus::Cancelled => "cancelled",
                    _ => "completed",
                },
                &bootstrap.active_authority_node_id,
                bootstrap.active_authority_epoch,
                request_payload,
            )
            .await;
        }

        if let Some(claim_reason) = schedule.claim_reason.as_deref() {
            match claim_reason {
                "in_progress" | "stale_reclaim" | "retry_ready" | "retry_backoff" | "retry_exhausted" => {
                    return build_existing_routing_response(
                        state,
                        signal_type,
                        request_type,
                        &work_request_id,
                        match claim_reason {
                            "retry_ready" => "retry_ready",
                            "retry_backoff" => "retry_backoff",
                            "retry_exhausted" => "retry_exhausted",
                            "stale_reclaim" => "stale_reclaim",
                            _ => "in_progress",
                        },
                        &bootstrap.active_authority_node_id,
                        bootstrap.active_authority_epoch,
                        request_payload,
                    )
                    .await;
                }
                _ => {}
            }
        }
    }

    let signal_payload = serde_json::json!({
        "request": request_payload.clone(),
        "queued_via": queued_via,
        "queued_at": now,
        "routing": {
            "work_request_id": work_request_id,
            "authority_node_id": bootstrap.active_authority_node_id,
            "authority_epoch": bootstrap.active_authority_epoch,
            "target_node_id": target_worker.node_id,
            "target_worker_class": target_worker_class,
            "requested_capability": requested_capability,
        }
    });
    let signal_id = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: signal_type.to_string(),
            source: "cluster_work_router".to_string(),
            source_ref: Some(work_request_id.clone()),
            timestamp: now,
            payload_json: Some(signal_payload),
        })
        .await?;

    Ok(QueuedWorkRouting {
        work_request_id,
        request_type,
        status: "queued".to_string(),
        queued_signal_id: signal_id,
        queued_signal_type: signal_type.to_string(),
        queued_at: now,
        queued_via: queued_via.to_string(),
        authority_node_id: bootstrap.active_authority_node_id,
        authority_epoch: bootstrap.active_authority_epoch,
        target_node_id: target_worker.node_id.clone(),
        target_worker_class: target_worker_class.to_string(),
        requested_capability: requested_capability.to_string(),
        request_payload,
    })
}

#[derive(Debug, Clone)]
struct WorkQueueScheduleState {
    include_in_queue: bool,
    is_stale: bool,
    attempt_count: u32,
    claimable_now: bool,
    claim_reason: Option<String>,
    next_retry_at: Option<i64>,
}

async fn build_existing_routing_response(
    state: &AppState,
    signal_type: &str,
    request_type: QueuedWorkRoutingKind,
    work_request_id: &str,
    status: &str,
    authority_node_id: &str,
    authority_epoch: i64,
    request_payload: serde_json::Value,
) -> Result<QueuedWorkRouting, AppError> {
    let signal = find_latest_routing_signal(state, signal_type, work_request_id)
        .await?
        .ok_or_else(|| AppError::internal("routing_signal missing for existing work request"))?;
    let routing = signal
        .payload_json
        .get("routing")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    Ok(QueuedWorkRouting {
        work_request_id: work_request_id.to_string(),
        request_type,
        status: status.to_string(),
        queued_signal_id: signal.signal_id,
        queued_signal_type: signal.signal_type,
        queued_at: signal.timestamp,
        queued_via: signal
            .payload_json
            .get("queued_via")
            .and_then(|value| value.as_str())
            .unwrap_or("cluster_work_router")
            .to_string(),
        authority_node_id: authority_node_id.to_string(),
        authority_epoch,
        target_node_id: routing
            .get("target_node_id")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        target_worker_class: routing
            .get("target_worker_class")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        requested_capability: routing
            .get("requested_capability")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_string(),
        request_payload,
    })
}

pub fn preferred_sync_target(
    tailscale_base_url: Option<&str>,
    base_url: &str,
    lan_base_url: Option<&str>,
    localhost_base_url: Option<&str>,
) -> (String, String) {
    if let Some(url) = tailscale_base_url.filter(|value| !value.trim().is_empty()) {
        return (url.to_string(), "tailscale".to_string());
    }
    if is_localhost(base_url) {
        if let Some(url) = localhost_base_url {
            return (url.to_string(), "localhost".to_string());
        }
    }
    if let Some(url) = lan_base_url.filter(|value| !value.trim().is_empty()) {
        return (url.to_string(), "lan".to_string());
    }
    let transport = if is_localhost(base_url) {
        "localhost"
    } else {
        "configured"
    };
    (base_url.to_string(), transport.to_string())
}

fn worker_classes_for_capabilities(capabilities: &[String]) -> Vec<String> {
    let mut worker_classes = vec!["authority".to_string(), "sync".to_string()];

    if capabilities
        .iter()
        .any(|capability| capability == "branch_sync")
    {
        worker_classes.push("repo_sync".to_string());
    }
    if capabilities
        .iter()
        .any(|capability| capability == "build_test_profiles")
    {
        worker_classes.push("validation".to_string());
    }

    worker_classes
}

fn latency_class_for_transport(sync_transport: &str) -> String {
    match sync_transport {
        "localhost" => "ultra_low".to_string(),
        "tailscale" | "lan" => "low".to_string(),
        _ => "medium".to_string(),
    }
}

fn compute_class_for_capacity(max_concurrency: u32) -> String {
    match max_concurrency {
        0..=2 => "edge".to_string(),
        3..=8 => "standard".to_string(),
        _ => "high".to_string(),
    }
}

fn infer_power_class(node_id: &str) -> String {
    let node_id = node_id.to_ascii_lowercase();
    if node_id.contains("watch")
        || node_id.contains("iphone")
        || node_id.contains("ios")
        || node_id.contains("phone")
    {
        "battery".to_string()
    } else {
        "ac_or_unknown".to_string()
    }
}

fn localhost_base_url(bind_addr: &str) -> Option<String> {
    let port = bind_addr.rsplit(':').next()?.trim();
    if port.is_empty() {
        return None;
    }
    Some(format!("http://127.0.0.1:{port}"))
}

fn is_localhost(base_url: &str) -> bool {
    base_url.contains("://127.0.0.1") || base_url.contains("://localhost")
}

pub(crate) fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn execution_capabilities(repo_root: &Path) -> Vec<String> {
    let mut capabilities = vec![
        "sync_bootstrap".to_string(),
        "queued_low_risk_actions".to_string(),
        "queued_branch_sync_requests".to_string(),
        "queued_validation_requests".to_string(),
    ];

    if branch_sync_capability(repo_root).is_some() {
        capabilities.push("branch_sync".to_string());
    }
    if !validation_profiles(repo_root).is_empty() {
        capabilities.push("build_test_profiles".to_string());
    }

    capabilities
}

fn branch_sync_capability(repo_root: &Path) -> Option<BranchSyncCapability> {
    if !repo_root.join(".git").exists() {
        return None;
    }

    Some(BranchSyncCapability {
        repo_root: repo_root.to_string_lossy().to_string(),
        default_remote: "origin".to_string(),
        supports_fetch: true,
        supports_pull: true,
        supports_push: true,
    })
}

pub(crate) fn validation_profiles(repo_root: &Path) -> Vec<ValidationProfile> {
    let mut profiles = Vec::new();

    if repo_root.join("Cargo.toml").exists() {
        profiles.push(ValidationProfile {
            profile_id: "api-build".to_string(),
            label: "Build Rust API".to_string(),
            command_hint: "cargo build -p veld".to_string(),
            environment: "api".to_string(),
        });
        profiles.push(ValidationProfile {
            profile_id: "api-test".to_string(),
            label: "Test Rust workspace".to_string(),
            command_hint: "cargo test --workspace --all-features".to_string(),
            environment: "api".to_string(),
        });
    }

    if repo_root.join("clients/web/package.json").exists() {
        profiles.push(ValidationProfile {
            profile_id: "web-build".to_string(),
            label: "Build web client".to_string(),
            command_hint: "cd clients/web && npm run build".to_string(),
            environment: "web".to_string(),
        });
        profiles.push(ValidationProfile {
            profile_id: "web-test".to_string(),
            label: "Test web client".to_string(),
            command_hint: "cd clients/web && npm run test".to_string(),
            environment: "web".to_string(),
        });
    }

    if repo_root
        .join("clients/apple/VelAPI/Package.swift")
        .exists()
    {
        profiles.push(ValidationProfile {
            profile_id: "apple-swift-check".to_string(),
            label: "Check shared Apple package".to_string(),
            command_hint: "make check-apple-swift".to_string(),
            environment: "apple".to_string(),
        });
    }

    if repo_root.join("Makefile").exists() {
        profiles.push(ValidationProfile {
            profile_id: "repo-verify".to_string(),
            label: "Verify repo truth and tests".to_string(),
            command_hint: "make verify".to_string(),
            environment: "repo".to_string(),
        });
        profiles.push(ValidationProfile {
            profile_id: "smoke".to_string(),
            label: "Run smoke checks".to_string(),
            command_hint: "make smoke".to_string(),
            environment: "runtime".to_string(),
        });
    }

    profiles
}

fn validate_branch_sync_request(request: &BranchSyncRequest) -> Result<(), AppError> {
    if request.repo_root.trim().is_empty() {
        return Err(AppError::bad_request("branch sync repo_root is required"));
    }
    if request.branch.trim().is_empty() {
        return Err(AppError::bad_request("branch sync branch is required"));
    }

    let repo_root = repo_root();
    let Some(capability) = branch_sync_capability(&repo_root) else {
        return Err(AppError::bad_request(
            "branch sync is not available on this node",
        ));
    };
    if request.repo_root != capability.repo_root {
        return Err(AppError::bad_request(
            "branch sync repo_root does not match this node's repo",
        ));
    }

    Ok(())
}

fn validate_validation_request(request: &ValidationRequest) -> Result<(), AppError> {
    if request.repo_root.trim().is_empty() {
        return Err(AppError::bad_request("validation repo_root is required"));
    }
    if request.profile_id.trim().is_empty() {
        return Err(AppError::bad_request("validation profile_id is required"));
    }

    let repo_root = repo_root();
    if request.repo_root != repo_root.to_string_lossy() {
        return Err(AppError::bad_request(
            "validation repo_root does not match this node's repo",
        ));
    }
    if !validation_profiles(&repo_root)
        .iter()
        .any(|profile| profile.profile_id == request.profile_id)
    {
        return Err(AppError::bad_request(format!(
            "validation profile {} is not available on this node",
            request.profile_id
        )));
    }

    Ok(())
}

async fn find_latest_routing_signal(
    state: &AppState,
    signal_type: &str,
    work_request_id: &str,
) -> Result<Option<SignalRecord>, AppError> {
    Ok(state
        .storage
        .list_signals(Some(signal_type), None, 200)
        .await?
        .into_iter()
        .find(|signal| signal.source_ref.as_deref() == Some(work_request_id)))
}

fn is_stale_assignment(record: &WorkAssignmentRecord, now: i64) -> bool {
    matches!(
        record.status,
        WorkAssignmentStatus::Assigned | WorkAssignmentStatus::Started
    ) && record.last_updated + WORK_ASSIGNMENT_STALE_SECONDS < now
}

fn evaluate_queue_schedule(
    history: &[WorkAssignmentRecord],
    now: i64,
    retry_policy: &crate::policy_config::QueuedWorkRetryPolicy,
) -> WorkQueueScheduleState {
    let attempt_count = history.len() as u32;
    let Some(latest) = history.first() else {
        return WorkQueueScheduleState {
            include_in_queue: true,
            is_stale: false,
            attempt_count,
            claimable_now: true,
            claim_reason: Some("unclaimed".to_string()),
            next_retry_at: None,
        };
    };

    match latest.status {
        WorkAssignmentStatus::Assigned | WorkAssignmentStatus::Started => {
            let is_stale = is_stale_assignment(latest, now);
            WorkQueueScheduleState {
                include_in_queue: true,
                is_stale,
                attempt_count,
                claimable_now: is_stale,
                claim_reason: Some(if is_stale {
                    "stale_reclaim".to_string()
                } else {
                    "in_progress".to_string()
                }),
                next_retry_at: None,
            }
        }
        WorkAssignmentStatus::Failed => {
            let failures = history
                .iter()
                .filter(|record| record.status == WorkAssignmentStatus::Failed)
                .count();
            if failures >= retry_policy.max_failure_attempts {
                return WorkQueueScheduleState {
                    include_in_queue: true,
                    is_stale: false,
                    attempt_count,
                    claimable_now: false,
                    claim_reason: Some("retry_exhausted".to_string()),
                    next_retry_at: None,
                };
            }
            let retry_after = retry_backoff_seconds(failures, retry_policy);
            let next_retry_at = latest.completed_at.unwrap_or(latest.last_updated) + retry_after;
            WorkQueueScheduleState {
                include_in_queue: true,
                is_stale: false,
                attempt_count,
                claimable_now: next_retry_at <= now,
                claim_reason: Some(if next_retry_at <= now {
                    "retry_ready".to_string()
                } else {
                    "retry_backoff".to_string()
                }),
                next_retry_at: Some(next_retry_at),
            }
        }
        WorkAssignmentStatus::Completed | WorkAssignmentStatus::Cancelled => {
            WorkQueueScheduleState {
                include_in_queue: false,
                is_stale: false,
                attempt_count,
                claimable_now: false,
                claim_reason: None,
                next_retry_at: None,
            }
        }
    }
}

fn retry_backoff_seconds(
    failure_count: usize,
    retry_policy: &crate::policy_config::QueuedWorkRetryPolicy,
) -> i64 {
    let exponent = failure_count.saturating_sub(1).min(10) as u32;
    ((retry_policy.retry_base_seconds as i64).saturating_mul(1_i64 << exponent))
        .min(retry_policy.retry_max_seconds as i64)
}

fn retry_policy_for_request_type(
    state: &AppState,
    request_type: QueuedWorkRoutingKind,
) -> &crate::policy_config::QueuedWorkRetryPolicy {
    match request_type {
        QueuedWorkRoutingKind::Validation => {
            state.policy_config.queued_work_validation_policy()
        }
        QueuedWorkRoutingKind::BranchSync => {
            state.policy_config.queued_work_branch_sync_policy()
        }
    }
}

fn claim_priority(reason: Option<&str>) -> u8 {
    match reason {
        Some("unclaimed") => 0,
        Some("stale_reclaim") => 1,
        Some("retry_ready") => 2,
        _ => 9,
    }
}

fn worker_matches_request(
    worker: &WorkerPresence,
    target_worker_class: &str,
    requested_capability: &str,
) -> bool {
    worker
        .worker_classes
        .iter()
        .any(|class| class == target_worker_class)
        && worker
            .capabilities
            .iter()
            .any(|capability| capability == requested_capability)
        && worker.status == "ready"
        && worker.reachability == "reachable"
}

fn worker_placement_score(worker: &WorkerPresence) -> (u8, u8, u8, u8, u32, u32, i64) {
    (
        u8::from(worker.capacity.available_concurrency > 0),
        u8::from(worker.tailscale_preferred || worker.tailscale_reachable),
        u8::from(worker.power_class != "battery"),
        u8::from(worker.status == "ready" && worker.reachability == "reachable"),
        worker.capacity.available_concurrency,
        u32::MAX.saturating_sub(worker.queue_depth),
        (1000.0 - (worker.recent_failure_rate * 1000.0)).max(0.0) as i64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_storage::Storage;

    fn test_state(storage: Storage) -> AppState {
        let (tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            tx,
            None,
            None,
        )
    }

    #[tokio::test]
    async fn client_action_batch_can_create_commitment() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage.clone());
        let result = apply_client_actions(
            &state,
            vec![ClientAction {
                action_id: Some("a1".to_string()),
                action_type: ClientActionKind::CommitmentCreate,
                target_id: None,
                text: Some("queued commitment".to_string()),
                minutes: None,
                payload: None,
            }],
        )
        .await
        .unwrap();
        assert_eq!(result.applied, 1);
        let commitments = storage
            .list_commitments(Some(CommitmentStatus::Open), None, None, 10)
            .await
            .unwrap();
        assert_eq!(commitments.len(), 1);
        assert_eq!(commitments[0].text, "queued commitment");
    }

    #[tokio::test]
    async fn validation_request_action_is_persisted_as_signal() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage.clone());
        let result = apply_client_actions(
            &state,
            vec![ClientAction {
                action_id: Some("val-1".to_string()),
                action_type: ClientActionKind::ValidationRequest,
                target_id: None,
                text: None,
                minutes: None,
                payload: Some(serde_json::json!({
                    "repo_root": repo_root().to_string_lossy(),
                    "profile_id": "repo-verify",
                    "environment": "repo",
                })),
            }],
        )
        .await
        .unwrap();

        assert_eq!(result.applied, 1);
        let signals = storage
            .list_signals(Some("client_validation_requested"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source_ref.as_deref(), Some("val-1"));
    }

    #[tokio::test]
    async fn queue_branch_sync_request_returns_capability_routing_receipt() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage.clone());

        let result = queue_branch_sync_request(
            &state,
            BranchSyncRequest {
                repo_root: repo_root().to_string_lossy().to_string(),
                branch: "main".to_string(),
                remote: Some("origin".to_string()),
                base_branch: Some("main".to_string()),
                mode: Some("pull".to_string()),
                requested_by: Some("test".to_string()),
            },
            "sync_route",
            Some("wrkreq_test".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(result.request_type, QueuedWorkRoutingKind::BranchSync);
        assert_eq!(result.status, "queued");
        assert_eq!(result.work_request_id, "wrkreq_test");
        assert_eq!(result.target_worker_class, "repo_sync");
        assert_eq!(result.requested_capability, "branch_sync");
        assert_eq!(result.queued_signal_type, "client_branch_sync_requested");
        assert_eq!(result.queued_via, "sync_route");

        let signals = storage
            .list_signals(Some("client_branch_sync_requested"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source, "cluster_work_router");
        assert_eq!(signals[0].source_ref.as_deref(), Some("wrkreq_test"));
    }

    #[tokio::test]
    async fn cluster_workers_data_reports_local_queue_depth_without_recursing() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage.clone());

        queue_validation_request(
            &state,
            ValidationRequest {
                repo_root: repo_root().to_string_lossy().to_string(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "sync_route",
            Some("wrkreq_validation".to_string()),
        )
        .await
        .unwrap();

        let workers = cluster_workers_data(&state).await.unwrap();
        let local_worker = workers
            .workers
            .iter()
            .find(|worker| worker.worker_id == workers.active_authority_node_id)
            .expect("local worker should be present");

        assert_eq!(local_worker.queue_depth, 1);
        assert_eq!(local_worker.capacity.current_load, 0);
    }

    #[tokio::test]
    async fn queue_validation_request_rejects_unknown_profile() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);

        let error = queue_validation_request(
            &state,
            ValidationRequest {
                repo_root: repo_root().to_string_lossy().to_string(),
                profile_id: "unknown-profile".to_string(),
                branch: None,
                environment: None,
                requested_by: None,
            },
            "sync_route",
            None,
        )
        .await
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("validation profile unknown-profile is not available on this node"));
    }

    #[test]
    fn bootstrap_capabilities_include_validation_profiles() {
        let repo_root = repo_root();
        let capabilities = execution_capabilities(&repo_root);
        let profiles = validation_profiles(&repo_root);

        assert!(capabilities.iter().any(|cap| cap == "build_test_profiles"));
        assert!(profiles
            .iter()
            .any(|profile| profile.profile_id == "repo-verify"));
        assert!(profiles
            .iter()
            .any(|profile| profile.profile_id == "api-test"));
    }

    #[test]
    fn preferred_sync_target_prioritizes_tailscale_when_configured() {
        let (url, transport) = preferred_sync_target(
            Some("https://vel.tailnet.ts.net"),
            "https://vel.example.com",
            Some("http://192.168.1.12:4130"),
            Some("http://127.0.0.1:4130"),
        );

        assert_eq!(url, "https://vel.tailnet.ts.net");
        assert_eq!(transport, "tailscale");
    }

    #[test]
    fn preferred_sync_target_falls_back_when_tailscale_missing() {
        let (url, transport) = preferred_sync_target(
            Some("   "),
            "http://127.0.0.1:4130",
            Some("http://192.168.1.12:4130"),
            Some("http://127.0.0.1:4130"),
        );

        assert_eq!(url, "http://127.0.0.1:4130");
        assert_eq!(transport, "localhost");
    }
}
