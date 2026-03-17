use std::path::{Path, PathBuf};

use serde::Serialize;
use time::OffsetDateTime;
use vel_api_types::{
    BranchSyncCapabilityData, BranchSyncRequestData, ClientActionBatchRequest,
    ClientActionBatchResultData, ClientActionData, ClientActionKind, ClientActionResultData,
    ClusterBootstrapData, ClusterNodeStateData, ClusterWorkerStateData, CommitmentCreateRequest,
    CommitmentData, CurrentContextData, NudgeData, SyncBootstrapData, SyncClusterStateData,
    ValidationProfileData, ValidationRequestData,
};
use vel_core::{CommitmentStatus, PrivacyClass};
use vel_storage::{CaptureInsert, CommitmentInsert, SignalInsert};

use crate::{errors::AppError, state::AppState};

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

pub fn build_sync_cluster_state(state: &AppState) -> SyncClusterStateData {
    let cluster = cluster_bootstrap_data(state);
    let workers = cluster_workers_data(state);

    SyncClusterStateData {
        cluster_view_version: Some(workers.generated_at),
        authority_node_id: Some(cluster.active_authority_node_id.clone()),
        authority_epoch: Some(cluster.active_authority_epoch),
        sync_transport: Some(cluster.sync_transport.clone()),
        cluster: Some(cluster.clone()),
        nodes: vec![ClusterNodeStateData {
            node_id: cluster.node_id.clone(),
            node_display_name: Some(cluster.node_display_name.clone()),
            node_class: Some("authority".to_string()),
            sync_base_url: Some(cluster.sync_base_url.clone()),
            sync_transport: Some(cluster.sync_transport.clone()),
            tailscale_base_url: cluster.tailscale_base_url.clone(),
            lan_base_url: cluster.lan_base_url.clone(),
            localhost_base_url: cluster.localhost_base_url.clone(),
            capabilities: cluster.capabilities.clone(),
            reachability: Some("reachable".to_string()),
            last_seen_at: Some(workers.generated_at),
        }],
        workers: workers
            .workers
            .into_iter()
            .map(|worker| ClusterWorkerStateData {
                worker_id: worker.node_id.clone(),
                node_id: Some(worker.node_id),
                worker_class: worker.worker_classes.first().cloned(),
                status: Some("ready".to_string()),
                max_concurrency: Some(worker.capacity.max_concurrency),
                current_load: Some(worker.capacity.current_load),
                queue_depth: Some(0),
                reachability: Some(worker.reachability),
                latency_class: Some(worker.latency_class),
                compute_class: Some(worker.compute_class),
                power_class: Some(worker.power_class),
                recent_failure_rate: Some(0.0),
                tailscale_preferred: Some(worker.tailscale_reachable),
                last_heartbeat_at: Some(worker.last_heartbeat_at),
                capabilities: worker.capabilities,
            })
            .collect(),
    }
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
            validate_branch_sync_request(&request)?;
            let now = OffsetDateTime::now_utc().unix_timestamp();
            state
                .storage
                .insert_signal(SignalInsert {
                    signal_type: "client_branch_sync_requested".to_string(),
                    source: "client_sync".to_string(),
                    source_ref: action.action_id.clone(),
                    timestamp: now,
                    payload_json: Some(serde_json::json!({
                        "request": request,
                        "queued_via": "client_sync_actions",
                        "queued_at": now,
                    })),
                })
                .await?;
            Ok(applied(action, "branch sync request queued"))
        }
        ClientActionKind::ValidationRequest => {
            let request: ValidationRequestData = require_payload(action, "validation payload")?;
            validate_validation_request(&request)?;
            let now = OffsetDateTime::now_utc().unix_timestamp();
            state
                .storage
                .insert_signal(SignalInsert {
                    signal_type: "client_validation_requested".to_string(),
                    source: "client_sync".to_string(),
                    source_ref: action.action_id.clone(),
                    timestamp: now,
                    payload_json: Some(serde_json::json!({
                        "request": request,
                        "queued_via": "client_sync_actions",
                        "queued_at": now,
                    })),
                })
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

fn repo_root() -> PathBuf {
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

fn validation_profiles(repo_root: &Path) -> Vec<ValidationProfileData> {
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
    pub node_id: String,
    pub node_display_name: String,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub reachability: String,
    pub latency_class: String,
    pub compute_class: String,
    pub power_class: String,
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

pub fn cluster_workers_data(state: &AppState) -> ClusterWorkersData {
    let bootstrap = cluster_bootstrap_data(state);
    let runtime = state.worker_runtime.snapshot();
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let current_load = runtime.current_load.min(runtime.max_concurrency);
    let tailscale_reachable = bootstrap
        .tailscale_base_url
        .as_ref()
        .map(|url| !url.trim().is_empty())
        .unwrap_or(false);

    ClusterWorkersData {
        active_authority_node_id: bootstrap.active_authority_node_id.clone(),
        active_authority_epoch: bootstrap.active_authority_epoch,
        generated_at: now,
        workers: vec![WorkerPresenceData {
            node_id: bootstrap.node_id.clone(),
            node_display_name: bootstrap.node_display_name.clone(),
            worker_classes: worker_classes_for_capabilities(&bootstrap.capabilities),
            capabilities: bootstrap.capabilities.clone(),
            reachability: "reachable".to_string(),
            latency_class: latency_class_for_transport(&bootstrap.sync_transport),
            compute_class: compute_class_for_capacity(runtime.max_concurrency),
            power_class: infer_power_class(&bootstrap.node_id),
            last_heartbeat_at: now,
            started_at: runtime.started_at,
            sync_base_url: bootstrap.sync_base_url,
            sync_transport: bootstrap.sync_transport,
            preferred_tailnet_endpoint: bootstrap.tailscale_base_url.clone(),
            tailscale_base_url: bootstrap.tailscale_base_url,
            tailscale_reachable,
            lan_base_url: bootstrap.lan_base_url,
            localhost_base_url: bootstrap.localhost_base_url,
            capacity: WorkerCapacityData {
                max_concurrency: runtime.max_concurrency,
                current_load,
                available_concurrency: runtime.max_concurrency.saturating_sub(current_load),
            },
        }],
    }
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
