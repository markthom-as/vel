use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{
    BranchSyncCapabilityData, BranchSyncRequestData, ClientActionBatchRequest,
    ClientActionBatchResultData, ClientActionData, ClientActionKind, ClientActionResultData,
    ClusterBootstrapData, ClusterNodeStateData, ClusterWorkerStateData, CommitmentCreateRequest,
    CommitmentData, CurrentContextData, NudgeData, QueuedWorkItemData, QueuedWorkRoutingData,
    QueuedWorkRoutingKindData, SyncBootstrapData, SyncClusterStateData, SyncHeartbeatRequestData,
    SyncHeartbeatResponseData, ValidationProfileData, ValidationRequestData,
    WorkAssignmentClaimNextRequestData, WorkAssignmentClaimNextResponseData,
    WorkAssignmentClaimRequestData, WorkAssignmentClaimedWorkData, WorkAssignmentReceiptData,
    WorkAssignmentStatusData, WorkAssignmentUpdateRequest,
};
use vel_core::{CommitmentStatus, PrivacyClass};
use vel_storage::{
    CaptureInsert, ClusterWorkerRecord, ClusterWorkerUpsert, CommitmentInsert, SignalInsert,
    SignalRecord, WorkAssignmentInsert, WorkAssignmentRecord, WorkAssignmentStatus,
    WorkAssignmentUpdate,
};

use crate::{errors::AppError, state::AppState};

const WORKER_HEARTBEAT_TTL_SECONDS: i64 = 90;
const WORK_ASSIGNMENT_STALE_SECONDS: i64 = 300;

pub async fn build_sync_bootstrap(state: &AppState) -> Result<SyncBootstrapData, AppError> {
    let current_context =
        state
            .storage
            .get_current_context()
            .await?
            .map(|(computed_at, context_str)| {
                let context =
                    serde_json::from_str(&context_str).unwrap_or_else(|_| serde_json::json!({}));
                CurrentContextData {
                    computed_at,
                    context,
                }
            });

    let active = state.storage.list_nudges(Some("active"), 50).await?;
    let pending = state.storage.list_nudges(Some("pending"), 50).await?;
    let snoozed = state.storage.list_nudges(Some("snoozed"), 50).await?;
    let mut nudges: Vec<NudgeData> = active.into_iter().map(nudge_record_to_data).collect();
    nudges.extend(pending.into_iter().map(nudge_record_to_data));
    nudges.extend(snoozed.into_iter().map(nudge_record_to_data));

    let commitments = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?
        .into_iter()
        .map(CommitmentData::from)
        .collect();

    Ok(SyncBootstrapData {
        cluster: cluster_bootstrap_data(state),
        current_context,
        nudges,
        commitments,
    })
}

pub async fn build_sync_cluster_state(state: &AppState) -> Result<SyncClusterStateData, AppError> {
    let cluster = cluster_bootstrap_data(state);
    let workers = cluster_workers_data(state).await?;
    let mut nodes = BTreeMap::new();

    for worker in &workers.workers {
        nodes
            .entry(worker.node_id.clone())
            .or_insert_with(|| ClusterNodeStateData {
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

    Ok(SyncClusterStateData {
        cluster_view_version: Some(workers.generated_at),
        authority_node_id: Some(cluster.active_authority_node_id.clone()),
        authority_epoch: Some(cluster.active_authority_epoch),
        sync_transport: Some(cluster.sync_transport.clone()),
        cluster: Some(cluster.clone()),
        nodes: nodes.into_values().collect(),
        workers: workers
            .workers
            .into_iter()
            .map(|worker| ClusterWorkerStateData {
                worker_id: worker.worker_id,
                node_id: Some(worker.node_id),
                node_display_name: Some(worker.node_display_name),
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
                last_heartbeat_at: Some(worker.last_heartbeat_at),
                started_at: Some(worker.started_at),
                available_concurrency: Some(worker.capacity.available_concurrency),
                capabilities: worker.capabilities,
            })
            .collect(),
    })
}

pub async fn queue_branch_sync_request(
    state: &AppState,
    request: BranchSyncRequestData,
    queued_via: &str,
    source_ref: Option<String>,
) -> Result<QueuedWorkRoutingData, AppError> {
    validate_branch_sync_request(&request)?;
    queue_work_request(
        state,
        QueuedWorkRoutingKindData::BranchSync,
        serde_json::json!(request),
        "client_branch_sync_requested",
        "repo_sync",
        "branch_sync",
        queued_via,
        source_ref,
    )
    .await
}

pub async fn queue_validation_request(
    state: &AppState,
    request: ValidationRequestData,
    queued_via: &str,
    source_ref: Option<String>,
) -> Result<QueuedWorkRoutingData, AppError> {
    validate_validation_request(&request)?;
    queue_work_request(
        state,
        QueuedWorkRoutingKindData::Validation,
        serde_json::json!(request),
        "client_validation_requested",
        "validation",
        "build_test_profiles",
        queued_via,
        source_ref,
    )
    .await
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

async fn queue_work_request(
    state: &AppState,
    request_type: QueuedWorkRoutingKindData,
    request_payload: serde_json::Value,
    signal_type: &str,
    target_worker_class: &str,
    requested_capability: &str,
    queued_via: &str,
    source_ref: Option<String>,
) -> Result<QueuedWorkRoutingData, AppError> {
    let bootstrap = cluster_bootstrap_data(state);
    let workers = cluster_workers_data(state).await?;
    let retry_policy = retry_policy_for_request_type(state, request_type);
    let target_worker = workers
        .workers
        .iter()
        .filter(|worker| worker_matches_request(worker, target_worker_class, requested_capability))
        .max_by_key(|worker| worker_placement_score(worker))
        .ok_or_else(|| {
            AppError::bad_request(format!(
                "no reachable worker advertises class={} capability={}",
                target_worker_class, requested_capability
            ))
        })?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let work_request_id = source_ref
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| format!("wrkreq_{}", Uuid::new_v4().simple()));
    let history = work_assignment_history_for_request(state, &work_request_id).await?;
    if let Some(existing) = history.first() {
        match existing.status {
            WorkAssignmentStatus::Completed => {
                return build_existing_routing_response(
                    state,
                    signal_type,
                    request_type,
                    &work_request_id,
                    "completed",
                    &bootstrap.active_authority_node_id,
                    bootstrap.active_authority_epoch,
                    request_payload,
                )
                .await;
            }
            WorkAssignmentStatus::Assigned | WorkAssignmentStatus::Started => {
                let status = if is_stale_assignment(existing, now) {
                    "stale_reclaim"
                } else {
                    "in_progress"
                };
                return build_existing_routing_response(
                    state,
                    signal_type,
                    request_type,
                    &work_request_id,
                    status,
                    &bootstrap.active_authority_node_id,
                    bootstrap.active_authority_epoch,
                    request_payload,
                )
                .await;
            }
            WorkAssignmentStatus::Failed => {
                let schedule = evaluate_queue_schedule(&history, now, retry_policy);
                return build_existing_routing_response(
                    state,
                    signal_type,
                    request_type,
                    &work_request_id,
                    schedule.claim_reason.as_deref().unwrap_or("retry_ready"),
                    &bootstrap.active_authority_node_id,
                    bootstrap.active_authority_epoch,
                    request_payload,
                )
                .await;
            }
            WorkAssignmentStatus::Cancelled => {
                return build_existing_routing_response(
                    state,
                    signal_type,
                    request_type,
                    &work_request_id,
                    "cancelled",
                    &bootstrap.active_authority_node_id,
                    bootstrap.active_authority_epoch,
                    request_payload,
                )
                .await;
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

    Ok(QueuedWorkRoutingData {
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

async fn build_existing_routing_response(
    state: &AppState,
    signal_type: &str,
    request_type: QueuedWorkRoutingKindData,
    work_request_id: &str,
    status: &str,
    authority_node_id: &str,
    authority_epoch: i64,
    request_payload: serde_json::Value,
) -> Result<QueuedWorkRoutingData, AppError> {
    let signal = find_latest_routing_signal(state, signal_type, work_request_id)
        .await?
        .ok_or_else(|| AppError::internal("routing signal missing for existing work request"))?;
    let routing = signal
        .payload_json
        .get("routing")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    Ok(QueuedWorkRoutingData {
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

pub async fn claim_work_assignment(
    state: &AppState,
    request: WorkAssignmentClaimRequestData,
) -> Result<WorkAssignmentReceiptData, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let existing = state
        .storage
        .list_work_assignments(Some(&request.work_request_id), None)
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
                                "reclaimed_by": request.worker_id,
                            })),
                        })
                        .await;
                } else if latest.worker_id == request.worker_id {
                    return Ok(work_assignment_to_data(latest.clone()));
                } else {
                    return Err(AppError::bad_request(format!(
                        "work request {} is already claimed by worker {}",
                        request.work_request_id, latest.worker_id
                    )));
                }
            }
            WorkAssignmentStatus::Completed => {
                return Ok(work_assignment_to_data(latest.clone()));
            }
            WorkAssignmentStatus::Failed | WorkAssignmentStatus::Cancelled => {}
        }
    }

    let receipt_id = state
        .storage
        .insert_work_assignment(WorkAssignmentInsert {
            receipt_id: None,
            work_request_id: request.work_request_id.clone(),
            worker_id: request.worker_id.clone(),
            worker_class: request.worker_class.clone(),
            capability: request.capability.clone(),
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
                "work_request_id": request.work_request_id,
                "worker_id": request.worker_id,
                "worker_class": request.worker_class,
                "capability": request.capability,
            })),
        })
        .await;

    let created = state
        .storage
        .list_work_assignments(Some(&request.work_request_id), Some(&request.worker_id))
        .await?
        .into_iter()
        .find(|assignment| assignment.status == WorkAssignmentStatus::Assigned)
        .ok_or_else(|| AppError::internal("claimed work assignment was not persisted"))?;
    Ok(work_assignment_to_data(created))
}

pub async fn update_work_assignment_receipt(
    state: &AppState,
    request: WorkAssignmentUpdateRequest,
) -> Result<WorkAssignmentReceiptData, AppError> {
    validate_work_assignment_update(&request)?;
    let updated = state
        .storage
        .update_work_assignment(WorkAssignmentUpdate {
            receipt_id: request.receipt_id.clone(),
            status: work_assignment_status_from_data(request.status),
            started_at: request.started_at,
            completed_at: request.completed_at,
            result: request.result.clone(),
            error_message: request.error_message.clone(),
        })
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

    Ok(work_assignment_to_data(updated))
}

pub async fn list_work_assignment_receipts(
    state: &AppState,
    work_request_id: Option<&str>,
    worker_id: Option<&str>,
) -> Result<Vec<WorkAssignmentReceiptData>, AppError> {
    Ok(state
        .storage
        .list_work_assignments(work_request_id, worker_id)
        .await?
        .into_iter()
        .map(work_assignment_to_data)
        .collect())
}

pub async fn list_worker_queue(
    state: &AppState,
    node_id: &str,
    worker_class: Option<&str>,
    capability: Option<&str>,
) -> Result<Vec<QueuedWorkItemData>, AppError> {
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
        let history = work_assignment_history_for_request(state, &work_request_id).await?;
        let latest_receipt = history.first().cloned();
        let request_type = if signal.signal_type == "client_branch_sync_requested" {
            QueuedWorkRoutingKindData::BranchSync
        } else {
            QueuedWorkRoutingKindData::Validation
        };
        let schedule = evaluate_queue_schedule(
            &history,
            now,
            retry_policy_for_request_type(state, request_type),
        );
        if !schedule.include_in_queue {
            continue;
        }
        items.push(QueuedWorkItemData {
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
            latest_receipt: latest_receipt.map(work_assignment_to_data),
            is_stale: schedule.is_stale,
            attempt_count: schedule.attempt_count,
            claimable_now: schedule.claimable_now,
            claim_reason: schedule.claim_reason,
            next_retry_at: schedule.next_retry_at,
        });
    }
    items.sort_by_key(|item| (item.queued_at, item.work_request_id.clone()));
    Ok(items)
}

pub async fn claim_next_work_for_worker(
    state: &AppState,
    request: WorkAssignmentClaimNextRequestData,
) -> Result<WorkAssignmentClaimNextResponseData, AppError> {
    let mut items = list_worker_queue(
        state,
        &request.node_id,
        request.worker_class.as_deref(),
        request.capability.as_deref(),
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
        return Ok(WorkAssignmentClaimNextResponseData { claim: None });
    };

    let receipt = claim_work_assignment(
        state,
        WorkAssignmentClaimRequestData {
            work_request_id: queue_item.work_request_id.clone(),
            worker_id: request.worker_id,
            worker_class: request
                .worker_class
                .or_else(|| Some(queue_item.target_worker_class.clone())),
            capability: request
                .capability
                .or_else(|| Some(queue_item.requested_capability.clone())),
        },
    )
    .await?;

    Ok(WorkAssignmentClaimNextResponseData {
        claim: Some(WorkAssignmentClaimedWorkData {
            queue_item,
            receipt,
        }),
    })
}

pub async fn apply_client_actions(
    state: &AppState,
    request: ClientActionBatchRequest,
) -> Result<ClientActionBatchResultData, AppError> {
    let mut results = Vec::with_capacity(request.actions.len());
    let mut applied = 0u32;

    for action in request.actions {
        match apply_single_action(state, &action).await {
            Ok(result) => {
                if result.status == "applied" {
                    applied += 1;
                }
                results.push(result);
            }
            Err(error) => results.push(ClientActionResultData {
                action_id: action.action_id,
                action_type: action.action_type,
                target_id: action.target_id,
                status: "rejected".to_string(),
                message: error.to_string(),
            }),
        }
    }

    Ok(ClientActionBatchResultData { applied, results })
}

async fn apply_single_action(
    state: &AppState,
    action: &ClientActionData,
) -> Result<ClientActionResultData, AppError> {
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
            let request = CommitmentCreateRequest {
                text,
                source_type: "apple".to_string(),
                source_id: None,
                due_at: None,
                project: None,
                commitment_kind: None,
                metadata: serde_json::json!({ "via": "sync_actions" }),
            };
            state
                .storage
                .insert_commitment(CommitmentInsert {
                    text: request.text,
                    source_type: request.source_type,
                    source_id: request.source_id,
                    status: CommitmentStatus::Open,
                    due_at: request.due_at,
                    project: request.project,
                    commitment_kind: request.commitment_kind,
                    metadata_json: Some(request.metadata),
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
            let request: BranchSyncRequestData = require_payload(action, "branch sync payload")?;
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
            let request: ValidationRequestData = require_payload(action, "validation payload")?;
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

fn require_target_id(action: &ClientActionData) -> Result<String, AppError> {
    action
        .target_id
        .clone()
        .filter(|id| !id.trim().is_empty())
        .ok_or_else(|| AppError::bad_request("action target_id is required"))
}

fn require_text(action: &ClientActionData) -> Result<String, AppError> {
    action
        .text
        .clone()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .ok_or_else(|| AppError::bad_request("action text is required"))
}

fn require_payload<T>(action: &ClientActionData, label: &str) -> Result<T, AppError>
where
    T: serde::de::DeserializeOwned,
{
    let payload = action
        .payload
        .clone()
        .ok_or_else(|| AppError::bad_request(format!("{label} is required")))?;
    serde_json::from_value(payload)
        .map_err(|error| AppError::bad_request(format!("invalid {label}: {error}")))
}

fn applied(action: &ClientActionData, message: &str) -> ClientActionResultData {
    ClientActionResultData {
        action_id: action.action_id.clone(),
        action_type: action.action_type.clone(),
        target_id: action.target_id.clone(),
        status: "applied".to_string(),
        message: message.to_string(),
    }
}

fn validate_work_assignment_update(request: &WorkAssignmentUpdateRequest) -> Result<(), AppError> {
    match request.status {
        WorkAssignmentStatusData::Assigned => Err(AppError::bad_request(
            "assigned receipts must be created via claim",
        )),
        WorkAssignmentStatusData::Started => {
            if request.started_at.is_none() {
                return Err(AppError::bad_request("started receipts require started_at"));
            }
            Ok(())
        }
        WorkAssignmentStatusData::Completed => {
            if request.completed_at.is_none() {
                return Err(AppError::bad_request(
                    "completed receipts require completed_at",
                ));
            }
            Ok(())
        }
        WorkAssignmentStatusData::Failed | WorkAssignmentStatusData::Cancelled => {
            if request.completed_at.is_none() {
                return Err(AppError::bad_request(
                    "failed/cancelled receipts require completed_at",
                ));
            }
            Ok(())
        }
    }
}

fn work_assignment_status_from_data(status: WorkAssignmentStatusData) -> WorkAssignmentStatus {
    match status {
        WorkAssignmentStatusData::Assigned => WorkAssignmentStatus::Assigned,
        WorkAssignmentStatusData::Started => WorkAssignmentStatus::Started,
        WorkAssignmentStatusData::Completed => WorkAssignmentStatus::Completed,
        WorkAssignmentStatusData::Failed => WorkAssignmentStatus::Failed,
        WorkAssignmentStatusData::Cancelled => WorkAssignmentStatus::Cancelled,
    }
}

fn work_assignment_status_to_data(status: WorkAssignmentStatus) -> WorkAssignmentStatusData {
    match status {
        WorkAssignmentStatus::Assigned => WorkAssignmentStatusData::Assigned,
        WorkAssignmentStatus::Started => WorkAssignmentStatusData::Started,
        WorkAssignmentStatus::Completed => WorkAssignmentStatusData::Completed,
        WorkAssignmentStatus::Failed => WorkAssignmentStatusData::Failed,
        WorkAssignmentStatus::Cancelled => WorkAssignmentStatusData::Cancelled,
    }
}

fn work_assignment_to_data(record: vel_storage::WorkAssignmentRecord) -> WorkAssignmentReceiptData {
    WorkAssignmentReceiptData {
        receipt_id: record.receipt_id,
        work_request_id: record.work_request_id,
        worker_id: record.worker_id,
        worker_class: record.worker_class,
        capability: record.capability,
        status: work_assignment_status_to_data(record.status),
        assigned_at: record.assigned_at,
        started_at: record.started_at,
        completed_at: record.completed_at,
        result: record.result,
        error_message: record.error_message,
        last_updated: record.last_updated,
    }
}

async fn work_assignment_history_for_request(
    state: &AppState,
    work_request_id: &str,
) -> Result<Vec<WorkAssignmentRecord>, AppError> {
    Ok(state
        .storage
        .list_work_assignments(Some(work_request_id), None)
        .await?)
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
    request_type: QueuedWorkRoutingKindData,
) -> &crate::policy_config::QueuedWorkRetryPolicy {
    match request_type {
        QueuedWorkRoutingKindData::Validation => {
            state.policy_config.queued_work_validation_policy()
        }
        QueuedWorkRoutingKindData::BranchSync => {
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
    worker: &WorkerPresenceData,
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

fn worker_placement_score(worker: &WorkerPresenceData) -> (u8, u8, u8, u32, u32, i64) {
    (
        u8::from(worker.tailscale_preferred || worker.tailscale_reachable),
        u8::from(worker.power_class != "battery"),
        u8::from(worker.status == "ready" && worker.reachability == "reachable"),
        worker.capacity.available_concurrency,
        u32::MAX.saturating_sub(worker.queue_depth),
        (1000.0 - (worker.recent_failure_rate * 1000.0)).max(0.0) as i64,
    )
}

fn nudge_record_to_data(r: vel_storage::NudgeRecord) -> NudgeData {
    NudgeData {
        nudge_id: r.nudge_id,
        nudge_type: r.nudge_type,
        level: r.level,
        state: r.state,
        related_commitment_id: r.related_commitment_id,
        message: r.message,
        created_at: r.created_at,
        snoozed_until: r.snoozed_until,
        resolved_at: r.resolved_at,
    }
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

fn branch_sync_capability(repo_root: &Path) -> Option<BranchSyncCapabilityData> {
    if !repo_root.join(".git").exists() {
        return None;
    }

    Some(BranchSyncCapabilityData {
        repo_root: repo_root.to_string_lossy().to_string(),
        default_remote: "origin".to_string(),
        supports_fetch: true,
        supports_pull: true,
        supports_push: true,
    })
}

pub(crate) fn validation_profiles(repo_root: &Path) -> Vec<ValidationProfileData> {
    let mut profiles = Vec::new();

    if repo_root.join("Cargo.toml").exists() {
        profiles.push(ValidationProfileData {
            profile_id: "api-build".to_string(),
            label: "Build Rust API".to_string(),
            command_hint: "cargo build -p veld".to_string(),
            environment: "api".to_string(),
        });
        profiles.push(ValidationProfileData {
            profile_id: "api-test".to_string(),
            label: "Test Rust workspace".to_string(),
            command_hint: "cargo test --workspace --all-features".to_string(),
            environment: "api".to_string(),
        });
    }

    if repo_root.join("clients/web/package.json").exists() {
        profiles.push(ValidationProfileData {
            profile_id: "web-build".to_string(),
            label: "Build web client".to_string(),
            command_hint: "cd clients/web && npm run build".to_string(),
            environment: "web".to_string(),
        });
        profiles.push(ValidationProfileData {
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
        profiles.push(ValidationProfileData {
            profile_id: "apple-swift-check".to_string(),
            label: "Check shared Apple package".to_string(),
            command_hint: "make check-apple-swift".to_string(),
            environment: "apple".to_string(),
        });
    }

    if repo_root.join("Makefile").exists() {
        profiles.push(ValidationProfileData {
            profile_id: "repo-verify".to_string(),
            label: "Verify repo truth and tests".to_string(),
            command_hint: "make verify".to_string(),
            environment: "repo".to_string(),
        });
        profiles.push(ValidationProfileData {
            profile_id: "smoke".to_string(),
            label: "Run smoke checks".to_string(),
            command_hint: "make smoke".to_string(),
            environment: "runtime".to_string(),
        });
    }

    profiles
}

fn validate_branch_sync_request(request: &BranchSyncRequestData) -> Result<(), AppError> {
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

fn validate_validation_request(request: &ValidationRequestData) -> Result<(), AppError> {
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

#[derive(Debug, Clone, Serialize)]
pub struct ClusterWorkersData {
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub generated_at: i64,
    pub workers: Vec<WorkerPresenceData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkerPresenceData {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: String,
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
    pub started_at: i64,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub capacity: WorkerCapacityData,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkerCapacityData {
    pub max_concurrency: u32,
    pub current_load: u32,
    pub available_concurrency: u32,
}

pub async fn cluster_workers_data(state: &AppState) -> Result<ClusterWorkersData, AppError> {
    refresh_local_worker_presence(state).await?;
    let bootstrap = cluster_bootstrap_data(state);
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

    Ok(ClusterWorkersData {
        active_authority_node_id: bootstrap.active_authority_node_id.clone(),
        active_authority_epoch: bootstrap.active_authority_epoch,
        generated_at: now,
        workers,
    })
}

pub fn cluster_bootstrap_data(state: &AppState) -> ClusterBootstrapData {
    let node_id = state
        .config
        .node_id
        .clone()
        .unwrap_or_else(|| "vel-node".to_string());
    let node_display_name = state
        .config
        .node_display_name
        .clone()
        .unwrap_or_else(|| node_id.clone());
    let localhost_base_url = localhost_base_url(&state.config.bind_addr);
    let (sync_base_url, sync_transport) = preferred_sync_target(
        state.config.tailscale_base_url.as_deref(),
        state.config.base_url.as_str(),
        state.config.lan_base_url.as_deref(),
        localhost_base_url.as_deref(),
    );
    let repo_root = repo_root();

    ClusterBootstrapData {
        node_id: node_id.clone(),
        node_display_name,
        active_authority_node_id: node_id,
        active_authority_epoch: 1,
        sync_base_url,
        sync_transport,
        tailscale_base_url: state.config.tailscale_base_url.clone(),
        lan_base_url: state.config.lan_base_url.clone(),
        localhost_base_url,
        capabilities: execution_capabilities(&repo_root),
        branch_sync: branch_sync_capability(&repo_root),
        validation_profiles: validation_profiles(&repo_root),
    }
}

pub async fn ingest_worker_heartbeat(
    state: &AppState,
    request: SyncHeartbeatRequestData,
) -> Result<SyncHeartbeatResponseData, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let last_heartbeat_at = request.last_heartbeat_at.unwrap_or(now);

    state
        .storage
        .upsert_cluster_worker(ClusterWorkerUpsert {
            worker_id: request.worker_id.clone(),
            node_id: request.node_id,
            node_display_name: request.node_display_name,
            worker_class: request.worker_classes.first().cloned(),
            worker_classes: request.worker_classes,
            capabilities: request.capabilities,
            status: request.status,
            max_concurrency: request.max_concurrency,
            current_load: request.current_load,
            queue_depth: request.queue_depth,
            reachability: request.reachability,
            latency_class: request.latency_class,
            compute_class: request.compute_class,
            power_class: request.power_class,
            recent_failure_rate: request.recent_failure_rate,
            tailscale_preferred: request.tailscale_preferred.unwrap_or(false),
            sync_base_url: request.sync_base_url,
            sync_transport: request.sync_transport,
            tailscale_base_url: request.tailscale_base_url,
            preferred_tailnet_endpoint: request.preferred_tailnet_endpoint,
            tailscale_reachable: request.tailscale_reachable.unwrap_or(false),
            lan_base_url: request.lan_base_url,
            localhost_base_url: request.localhost_base_url,
            last_heartbeat_at,
            started_at: request.started_at,
        })
        .await?;

    Ok(SyncHeartbeatResponseData {
        accepted: true,
        worker_id: request.worker_id,
        expires_at: last_heartbeat_at + WORKER_HEARTBEAT_TTL_SECONDS,
        cluster_view_version: now,
        placement_hints: Vec::new(),
    })
}

async fn refresh_local_worker_presence(state: &AppState) -> Result<(), AppError> {
    let bootstrap = cluster_bootstrap_data(state);
    let runtime = state.worker_runtime.snapshot();
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let current_load = runtime.current_load.min(runtime.max_concurrency);
    let tailscale_reachable = bootstrap
        .tailscale_base_url
        .as_ref()
        .map(|url| !url.trim().is_empty())
        .unwrap_or(false);
    let worker_classes = worker_classes_for_capabilities(&bootstrap.capabilities);

    state
        .storage
        .upsert_cluster_worker(ClusterWorkerUpsert {
            worker_id: bootstrap.node_id.clone(),
            node_id: bootstrap.node_id.clone(),
            node_display_name: Some(bootstrap.node_display_name.clone()),
            worker_class: worker_classes.first().cloned(),
            worker_classes,
            capabilities: bootstrap.capabilities.clone(),
            status: Some("ready".to_string()),
            max_concurrency: Some(runtime.max_concurrency),
            current_load: Some(current_load),
            queue_depth: Some(0),
            reachability: Some("reachable".to_string()),
            latency_class: Some(latency_class_for_transport(&bootstrap.sync_transport)),
            compute_class: Some(compute_class_for_capacity(runtime.max_concurrency)),
            power_class: Some(infer_power_class(&bootstrap.node_id)),
            recent_failure_rate: Some(0.0),
            tailscale_preferred: tailscale_reachable,
            sync_base_url: Some(bootstrap.sync_base_url),
            sync_transport: Some(bootstrap.sync_transport),
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

fn worker_presence_from_record(record: ClusterWorkerRecord) -> WorkerPresenceData {
    let max_concurrency = record.max_concurrency.unwrap_or(1);
    let current_load = record.current_load.unwrap_or(0).min(max_concurrency);

    WorkerPresenceData {
        worker_id: record.worker_id,
        node_id: record.node_id,
        node_display_name: record
            .node_display_name
            .unwrap_or_else(|| "Vel Node".to_string()),
        worker_classes: record.worker_classes,
        capabilities: record.capabilities,
        status: record.status.unwrap_or_else(|| "ready".to_string()),
        queue_depth: record.queue_depth.unwrap_or(0),
        reachability: record
            .reachability
            .unwrap_or_else(|| "reachable".to_string()),
        latency_class: record.latency_class.unwrap_or_else(|| "medium".to_string()),
        compute_class: record.compute_class.unwrap_or_else(|| "edge".to_string()),
        power_class: record
            .power_class
            .unwrap_or_else(|| "ac_or_unknown".to_string()),
        recent_failure_rate: record.recent_failure_rate.unwrap_or(0.0),
        tailscale_preferred: record.tailscale_preferred,
        last_heartbeat_at: record.last_heartbeat_at,
        started_at: record.started_at.unwrap_or(record.updated_at),
        sync_base_url: record.sync_base_url.unwrap_or_default(),
        sync_transport: record
            .sync_transport
            .unwrap_or_else(|| "configured".to_string()),
        tailscale_base_url: record.tailscale_base_url,
        preferred_tailnet_endpoint: record.preferred_tailnet_endpoint,
        tailscale_reachable: record.tailscale_reachable,
        lan_base_url: record.lan_base_url,
        localhost_base_url: record.localhost_base_url,
        capacity: WorkerCapacityData {
            max_concurrency,
            current_load,
            available_concurrency: max_concurrency.saturating_sub(current_load),
        },
    }
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
            ClientActionBatchRequest {
                actions: vec![ClientActionData {
                    action_id: Some("a1".to_string()),
                    action_type: ClientActionKind::CommitmentCreate,
                    target_id: None,
                    text: Some("queued commitment".to_string()),
                    minutes: None,
                    payload: None,
                }],
            },
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
            ClientActionBatchRequest {
                actions: vec![ClientActionData {
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
            },
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
            BranchSyncRequestData {
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

        assert_eq!(result.request_type, QueuedWorkRoutingKindData::BranchSync);
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
    async fn queue_validation_request_rejects_unknown_profile() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);

        let error = queue_validation_request(
            &state,
            ValidationRequestData {
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
}
