use axum::extract::{Query, State};
use axum::Json;
use vel_api_types::{
    ApiResponse, BranchSyncRequestData, ClientActionBatchRequest, ClientActionBatchResultData,
    ClientActionData, ClientActionResultData, PlacementRecommendationData, QueuedWorkItemData,
    ProjectRecordData, QueuedWorkRoutingData, SyncBootstrapData, SyncClusterStateData,
    SyncHeartbeatRequestData, SyncHeartbeatResponseData, SyncResultData, ValidationRequestData,
    WorkAssignmentClaimNextRequestData, WorkAssignmentClaimNextResponseData,
    WorkAssignmentClaimRequestData, WorkAssignmentClaimedWorkData, WorkAssignmentReceiptData,
    WorkAssignmentUpdateRequest,
};
use vel_core::WorkAssignmentStatus;
use vel_storage::{NudgeRecord, WorkAssignmentRecord, WorkAssignmentUpdate};

use crate::{errors::AppError, routes::response, services, state::AppState};

fn nudge_record_to_api(data: NudgeRecord) -> vel_api_types::NudgeData {
    vel_api_types::NudgeData {
        nudge_id: data.nudge_id,
        nudge_type: data.nudge_type,
        level: data.level,
        state: data.state,
        related_commitment_id: data.related_commitment_id,
        message: data.message,
        created_at: data.created_at,
        snoozed_until: data.snoozed_until,
        resolved_at: data.resolved_at,
    }
}

fn work_assignment_status_to_api(
    status: WorkAssignmentStatus,
) -> vel_api_types::WorkAssignmentStatusData {
    match status {
        WorkAssignmentStatus::Assigned => vel_api_types::WorkAssignmentStatusData::Assigned,
        WorkAssignmentStatus::Started => vel_api_types::WorkAssignmentStatusData::Started,
        WorkAssignmentStatus::Completed => vel_api_types::WorkAssignmentStatusData::Completed,
        WorkAssignmentStatus::Failed => vel_api_types::WorkAssignmentStatusData::Failed,
        WorkAssignmentStatus::Cancelled => vel_api_types::WorkAssignmentStatusData::Cancelled,
    }
}

fn work_assignment_status_from_api(
    status: vel_api_types::WorkAssignmentStatusData,
) -> WorkAssignmentStatus {
    match status {
        vel_api_types::WorkAssignmentStatusData::Assigned => WorkAssignmentStatus::Assigned,
        vel_api_types::WorkAssignmentStatusData::Started => WorkAssignmentStatus::Started,
        vel_api_types::WorkAssignmentStatusData::Completed => WorkAssignmentStatus::Completed,
        vel_api_types::WorkAssignmentStatusData::Failed => WorkAssignmentStatus::Failed,
        vel_api_types::WorkAssignmentStatusData::Cancelled => WorkAssignmentStatus::Cancelled,
    }
}

fn work_assignment_to_api(data: WorkAssignmentRecord) -> WorkAssignmentReceiptData {
    WorkAssignmentReceiptData {
        receipt_id: data.receipt_id,
        work_request_id: data.work_request_id,
        worker_id: data.worker_id,
        worker_class: Some(data.worker_class),
        capability: Some(data.capability),
        status: work_assignment_status_to_api(data.status),
        assigned_at: data.assigned_at,
        started_at: data.started_at,
        completed_at: data.completed_at,
        result: data.result,
        error_message: data.error_message,
        last_updated: data.last_updated,
    }
}

pub(crate) fn queued_work_routing_to_api(
    data: crate::services::client_sync::QueuedWorkRouting,
) -> QueuedWorkRoutingData {
    QueuedWorkRoutingData {
        work_request_id: data.work_request_id,
        request_type: match data.request_type {
            crate::services::client_sync::QueuedWorkRoutingKind::BranchSync => {
                vel_api_types::QueuedWorkRoutingKindData::BranchSync
            }
            crate::services::client_sync::QueuedWorkRoutingKind::Validation => {
                vel_api_types::QueuedWorkRoutingKindData::Validation
            }
        },
        status: data.status,
        queued_signal_id: data.queued_signal_id,
        queued_signal_type: data.queued_signal_type,
        queued_at: data.queued_at,
        queued_via: data.queued_via,
        authority_node_id: data.authority_node_id,
        authority_epoch: data.authority_epoch,
        target_node_id: data.target_node_id,
        target_worker_class: data.target_worker_class,
        requested_capability: data.requested_capability,
        request_payload: data.request_payload,
    }
}

fn queued_work_item_to_api(
    item: crate::services::client_sync::QueuedWorkItem,
) -> QueuedWorkItemData {
    QueuedWorkItemData {
        work_request_id: item.work_request_id,
        request_type: match item.request_type {
            crate::services::client_sync::QueuedWorkRoutingKind::BranchSync => {
                vel_api_types::QueuedWorkRoutingKindData::BranchSync
            }
            crate::services::client_sync::QueuedWorkRoutingKind::Validation => {
                vel_api_types::QueuedWorkRoutingKindData::Validation
            }
        },
        queued_signal_id: item.queued_signal_id,
        queued_signal_type: item.queued_signal_type,
        queued_at: item.queued_at,
        target_node_id: item.target_node_id,
        target_worker_class: item.target_worker_class,
        requested_capability: item.requested_capability,
        request_payload: item.request_payload,
        latest_receipt: item.latest_receipt.map(work_assignment_to_api),
        is_stale: item.is_stale,
        attempt_count: item.attempt_count,
        claimable_now: item.claimable_now,
        claim_reason: item.claim_reason,
        next_retry_at: item.next_retry_at,
    }
}

fn sync_heartbeat_to_api(
    data: crate::services::client_sync::SyncHeartbeatResponse,
) -> SyncHeartbeatResponseData {
    SyncHeartbeatResponseData {
        accepted: data.accepted,
        worker_id: data.worker_id,
        expires_at: data.expires_at,
        cluster_view_version: data.cluster_view_version,
        placement_hints: data
            .placement_hints
            .into_iter()
            .filter_map(|hint| serde_json::from_str::<PlacementRecommendationData>(&hint).ok())
            .collect(),
    }
}

fn sync_bootstrap_cluster_to_api(
    cluster: crate::services::client_sync::ClusterBootstrap,
) -> vel_api_types::ClusterBootstrapData {
    vel_api_types::ClusterBootstrapData {
        node_id: cluster.node_id,
        node_display_name: cluster.node_display_name,
        active_authority_node_id: cluster.active_authority_node_id,
        active_authority_epoch: cluster.active_authority_epoch,
        sync_base_url: cluster.sync_base_url,
        sync_transport: cluster.sync_transport,
        tailscale_base_url: cluster.tailscale_base_url,
        lan_base_url: cluster.lan_base_url,
        localhost_base_url: cluster.localhost_base_url,
        capabilities: cluster.capabilities,
        branch_sync: cluster
            .branch_sync
            .map(|b| vel_api_types::BranchSyncCapabilityData {
                repo_root: b.repo_root,
                default_remote: b.default_remote,
                supports_fetch: b.supports_fetch,
                supports_pull: b.supports_pull,
                supports_push: b.supports_push,
            }),
        validation_profiles: cluster
            .validation_profiles
            .into_iter()
            .map(|p| vel_api_types::ValidationProfileData {
                profile_id: p.profile_id,
                label: p.label,
                command_hint: p.command_hint,
                environment: p.environment,
            })
            .collect(),
        linked_nodes: cluster
            .linked_nodes
            .into_iter()
            .map(vel_api_types::LinkedNodeData::from)
            .collect(),
        projects: cluster
            .projects
            .into_iter()
            .map(ProjectRecordData::from)
            .collect(),
        action_items: cluster
            .action_items
            .into_iter()
            .map(vel_api_types::ActionItemData::from)
            .collect(),
    }
}

fn work_assignment_claimed_to_api(
    data: crate::services::client_sync::WorkAssignmentClaimedWork,
) -> WorkAssignmentClaimedWorkData {
    WorkAssignmentClaimedWorkData {
        queue_item: queued_work_item_to_api(data.queue_item),
        receipt: work_assignment_to_api(data.receipt),
    }
}

fn sync_bootstrap_to_api(data: crate::services::client_sync::SyncBootstrap) -> SyncBootstrapData {
    SyncBootstrapData {
        cluster: sync_bootstrap_cluster_to_api(data.cluster),
        current_context: data
            .current_context
            .map(|c| vel_api_types::CurrentContextData {
                computed_at: c.computed_at,
                context: c.context,
            }),
        nudges: data.nudges.into_iter().map(nudge_record_to_api).collect(),
        commitments: data
            .commitments
            .into_iter()
            .map(vel_api_types::CommitmentData::from)
            .collect(),
        linked_nodes: data
            .linked_nodes
            .into_iter()
            .map(vel_api_types::LinkedNodeData::from)
            .collect(),
        projects: data
            .projects
            .into_iter()
            .map(ProjectRecordData::from)
            .collect(),
        action_items: data
            .action_items
            .into_iter()
            .map(vel_api_types::ActionItemData::from)
            .collect(),
    }
}

fn sync_claim_next_to_api(
    data: crate::services::client_sync::WorkAssignmentClaimNextResponse,
) -> WorkAssignmentClaimNextResponseData {
    WorkAssignmentClaimNextResponseData {
        claim: data.claim.map(work_assignment_claimed_to_api),
    }
}

fn map_client_action_kind(
    kind: vel_api_types::ClientActionKind,
) -> crate::services::client_sync::ClientActionKind {
    match kind {
        vel_api_types::ClientActionKind::NudgeDone => {
            crate::services::client_sync::ClientActionKind::NudgeDone
        }
        vel_api_types::ClientActionKind::NudgeSnooze => {
            crate::services::client_sync::ClientActionKind::NudgeSnooze
        }
        vel_api_types::ClientActionKind::CommitmentDone => {
            crate::services::client_sync::ClientActionKind::CommitmentDone
        }
        vel_api_types::ClientActionKind::CommitmentCreate => {
            crate::services::client_sync::ClientActionKind::CommitmentCreate
        }
        vel_api_types::ClientActionKind::CaptureCreate => {
            crate::services::client_sync::ClientActionKind::CaptureCreate
        }
        vel_api_types::ClientActionKind::BranchSyncRequest => {
            crate::services::client_sync::ClientActionKind::BranchSyncRequest
        }
        vel_api_types::ClientActionKind::ValidationRequest => {
            crate::services::client_sync::ClientActionKind::ValidationRequest
        }
    }
}

fn map_client_action_kind_from_str(kind: &str) -> vel_api_types::ClientActionKind {
    match kind {
        "nudge_done" => vel_api_types::ClientActionKind::NudgeDone,
        "nudge_snooze" => vel_api_types::ClientActionKind::NudgeSnooze,
        "commitment_done" => vel_api_types::ClientActionKind::CommitmentDone,
        "commitment_create" => vel_api_types::ClientActionKind::CommitmentCreate,
        "capture_create" => vel_api_types::ClientActionKind::CaptureCreate,
        "branch_sync_request" => vel_api_types::ClientActionKind::BranchSyncRequest,
        "validation_request" => vel_api_types::ClientActionKind::ValidationRequest,
        _ => vel_api_types::ClientActionKind::CommitmentCreate,
    }
}

fn map_client_action(action: ClientActionData) -> crate::services::client_sync::ClientAction {
    crate::services::client_sync::ClientAction {
        action_id: action.action_id,
        action_type: map_client_action_kind(action.action_type),
        target_id: action.target_id,
        text: action.text,
        minutes: action.minutes,
        payload: action.payload,
    }
}

fn api_client_action_result(
    data: crate::services::client_sync::ClientActionResult,
) -> ClientActionResultData {
    ClientActionResultData {
        action_id: data.action_id,
        action_type: map_client_action_kind_from_str(&data.action_type),
        target_id: data.target_id,
        status: data.status,
        message: data.message,
    }
}

fn sync_cluster_state_to_api(
    data: crate::services::client_sync::SyncClusterState,
) -> SyncClusterStateData {
    SyncClusterStateData {
        cluster_view_version: data.cluster_view_version,
        authority_node_id: data.authority_node_id,
        authority_epoch: data.authority_epoch,
        sync_transport: data.sync_transport,
        cluster: data.cluster.map(sync_bootstrap_cluster_to_api),
        nodes: data
            .nodes
            .into_iter()
            .map(|node| vel_api_types::ClusterNodeStateData {
                node_id: node.node_id,
                node_display_name: node.node_display_name,
                node_class: node.node_class,
                sync_base_url: node.sync_base_url,
                sync_transport: node.sync_transport,
                tailscale_base_url: node.tailscale_base_url,
                lan_base_url: node.lan_base_url,
                localhost_base_url: node.localhost_base_url,
                capabilities: node.capabilities,
                reachability: node.reachability,
                last_seen_at: node.last_seen_at,
            })
            .collect(),
        workers: data
            .workers
            .into_iter()
            .map(|worker| vel_api_types::ClusterWorkerStateData {
                worker_id: worker.worker_id,
                node_id: Some(worker.node_id),
                node_display_name: Some(worker.node_display_name),
                client_kind: worker.client_kind,
                client_version: worker.client_version,
                protocol_version: worker.protocol_version,
                build_id: worker.build_id,
                worker_class: worker.worker_classes.first().cloned(),
                worker_classes: worker.worker_classes,
                status: Some(worker.status),
                max_concurrency: Some(worker.capacity.max_concurrency),
                current_load: Some(worker.capacity.current_load),
                queue_depth: Some(worker.queue_depth),
                reachability: Some(worker.reachability),
                latency_class: Some(worker.latency_class),
                compute_class: Some(worker.compute_class),
                power_class: Some(worker.power_class),
                recent_failure_rate: Some(worker.recent_failure_rate),
                tailscale_preferred: Some(worker.tailscale_preferred),
                sync_base_url: Some(worker.sync_base_url),
                sync_transport: Some(worker.sync_transport),
                tailscale_base_url: worker.tailscale_base_url,
                preferred_tailnet_endpoint: worker.preferred_tailnet_endpoint,
                tailscale_reachable: Some(worker.tailscale_reachable),
                lan_base_url: worker.lan_base_url,
                localhost_base_url: worker.localhost_base_url,
                ping_ms: worker.ping_ms,
                heartbeat_age_seconds: None,
                sync_status: Some(worker.sync_status),
                last_upstream_sync_at: worker.last_upstream_sync_at,
                last_downstream_sync_at: worker.last_downstream_sync_at,
                last_sync_error: worker.last_sync_error,
                last_heartbeat_at: Some(worker.last_heartbeat_at),
                started_at: worker.started_at,
                available_concurrency: Some(worker.capacity.available_concurrency),
                active_work: Vec::new(),
                capabilities: Vec::new(),
            })
            .collect(),
        clients: Vec::new(),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct WorkAssignmentListQuery {
    pub work_request_id: Option<String>,
    pub worker_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct WorkerQueueQuery {
    pub node_id: String,
    pub worker_class: Option<String>,
    pub capability: Option<String>,
}

async fn evaluate_and_broadcast_context(state: &AppState) {
    if services::evaluate::run_and_broadcast(state).await.is_err() {
        tracing::warn!("evaluate after sync failed");
    }
}

pub async fn sync_calendar(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_calendar_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "calendar".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncBootstrapData>>, AppError> {
    let data = services::client_sync::build_sync_bootstrap(&state).await?;
    let api_data = sync_bootstrap_to_api(data);
    Ok(response::success(api_data))
}

pub async fn sync_cluster(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncClusterStateData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = services::client_sync::build_sync_cluster_state(&state).await?;
    Ok(response::success(sync_cluster_state_to_api(data)))
}

pub async fn sync_heartbeat(
    State(state): State<AppState>,
    Json(payload): Json<SyncHeartbeatRequestData>,
) -> Result<Json<ApiResponse<SyncHeartbeatResponseData>>, AppError> {
    let data = services::client_sync::ingest_worker_heartbeat(
        &state,
        payload.worker_id,
        payload.node_id,
        payload.node_display_name,
        payload.client_kind,
        payload.client_version,
        payload.protocol_version,
        payload.build_id,
        payload.worker_classes,
        payload.capabilities,
        payload.status,
        payload.max_concurrency,
        payload.current_load,
        payload.queue_depth,
        payload.reachability,
        payload.latency_class,
        payload.compute_class,
        payload.power_class,
        payload.recent_failure_rate,
        payload.tailscale_preferred.unwrap_or(false),
        payload.sync_base_url,
        payload.sync_transport,
        payload.tailscale_base_url,
        payload.preferred_tailnet_endpoint,
        payload.tailscale_reachable.unwrap_or(false),
        payload.lan_base_url,
        payload.localhost_base_url,
        payload.started_at,
        payload.last_heartbeat_at,
    )
    .await?;
    Ok(response::success(sync_heartbeat_to_api(data)))
}

pub async fn sync_branch_sync_request(
    State(state): State<AppState>,
    Json(payload): Json<BranchSyncRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    let request = crate::services::client_sync::BranchSyncRequest {
        repo_root: payload.repo_root,
        branch: payload.branch,
        remote: payload.remote,
        base_branch: payload.base_branch,
        mode: payload.mode,
        requested_by: payload.requested_by,
    };
    let data =
        services::client_sync::queue_branch_sync_request(&state, request, "sync_route", None)
            .await?;
    Ok(response::success(queued_work_routing_to_api(data)))
}

pub async fn sync_validation_request(
    State(state): State<AppState>,
    Json(payload): Json<ValidationRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    let request = crate::services::client_sync::ValidationRequest {
        repo_root: payload.repo_root,
        profile_id: payload.profile_id,
        branch: payload.branch,
        environment: payload.environment,
        requested_by: payload.requested_by,
    };
    let data = services::client_sync::queue_validation_request(&state, request, "sync_route", None)
        .await?;
    Ok(response::success(queued_work_routing_to_api(data)))
}

pub async fn claim_work_assignment(
    State(state): State<AppState>,
    Json(payload): Json<WorkAssignmentClaimRequestData>,
) -> Result<Json<ApiResponse<WorkAssignmentReceiptData>>, AppError> {
    let data = services::client_sync::claim_work_assignment(
        &state,
        payload.work_request_id,
        payload.worker_id,
        payload.worker_class,
        payload.capability,
    )
    .await?;
    Ok(response::success(work_assignment_to_api(data)))
}

pub async fn update_work_assignment(
    State(state): State<AppState>,
    Json(payload): Json<WorkAssignmentUpdateRequest>,
) -> Result<Json<ApiResponse<WorkAssignmentReceiptData>>, AppError> {
    let data = WorkAssignmentUpdate {
        receipt_id: payload.receipt_id,
        status: work_assignment_status_from_api(payload.status),
        started_at: payload.started_at,
        completed_at: payload.completed_at,
        result: payload.result,
        error_message: payload.error_message,
    };
    let data = services::client_sync::update_work_assignment_receipt(&state, data).await?;
    Ok(response::success(work_assignment_to_api(data)))
}

pub async fn list_work_assignments(
    State(state): State<AppState>,
    Query(query): Query<WorkAssignmentListQuery>,
) -> Result<Json<ApiResponse<Vec<WorkAssignmentReceiptData>>>, AppError> {
    let data = services::client_sync::list_work_assignment_receipts(
        &state,
        query.work_request_id.as_deref(),
        query.worker_id.as_deref(),
    )
    .await?;
    Ok(response::success(
        data.into_iter()
            .map(work_assignment_to_api)
            .collect::<Vec<_>>(),
    ))
}

pub async fn list_worker_queue(
    State(state): State<AppState>,
    Query(query): Query<WorkerQueueQuery>,
) -> Result<Json<ApiResponse<Vec<QueuedWorkItemData>>>, AppError> {
    let data = services::client_sync::list_worker_queue(
        &state,
        &query.node_id,
        query.worker_class.as_deref(),
        query.capability.as_deref(),
    )
    .await?;
    Ok(response::success(
        data.into_iter().map(queued_work_item_to_api).collect(),
    ))
}

pub async fn claim_next_worker_queue_item(
    State(state): State<AppState>,
    Json(payload): Json<WorkAssignmentClaimNextRequestData>,
) -> Result<Json<ApiResponse<WorkAssignmentClaimNextResponseData>>, AppError> {
    let data = services::client_sync::claim_next_work_for_worker(
        &state,
        payload.node_id,
        payload.worker_id,
        payload.worker_class,
        payload.capability,
    )
    .await?;
    Ok(response::success(sync_claim_next_to_api(data)))
}

pub async fn sync_actions(
    State(state): State<AppState>,
    Json(payload): Json<ClientActionBatchRequest>,
) -> Result<Json<ApiResponse<ClientActionBatchResultData>>, AppError> {
    if payload.actions.is_empty() {
        return Err(AppError::bad_request("actions must not be empty"));
    }
    if payload.actions.len() > 200 {
        return Err(AppError::bad_request("actions batch exceeds 200"));
    }

    let actions = payload
        .actions
        .into_iter()
        .map(map_client_action)
        .collect::<Vec<_>>();
    let data = services::client_sync::apply_client_actions(&state, actions).await?;
    if data.applied > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(ClientActionBatchResultData {
        applied: data.applied,
        results: data
            .results
            .into_iter()
            .map(api_client_action_result)
            .collect(),
    }))
}

pub async fn sync_todoist(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_todoist_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "todoist".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_activity(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_activity_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "activity".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_health(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_health_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "health".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_git(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_git_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "git".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_messaging(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_messaging_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "messaging".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_reminders(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_reminders_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "reminders".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_notes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_notes_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "notes".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_transcripts(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_transcripts_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "transcripts".to_string(),
        signals_ingested: count,
    }))
}
