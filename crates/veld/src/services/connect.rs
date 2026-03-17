#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;
use time::OffsetDateTime;
use vel_api_types::{
    ConnectInstanceCapabilityManifestData, ConnectInstanceData, ConnectRuntimeCapabilityData,
};
use vel_core::{
    ConnectInstance, ConnectInstanceCapabilityManifest, ConnectInstanceStatus,
    ConnectRuntimeCapability,
};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Clone)]
struct ConnectInstanceAccumulator {
    id: String,
    node_id: String,
    display_name: String,
    status: ConnectInstanceStatus,
    reachability: String,
    sync_base_url: Option<String>,
    sync_transport: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    localhost_base_url: Option<String>,
    worker_ids: BTreeSet<String>,
    worker_classes: BTreeSet<String>,
    capabilities: BTreeSet<String>,
    last_seen_at: Option<i64>,
}

pub async fn list_connect_instances(
    state: &AppState,
) -> Result<Vec<ConnectInstanceData>, AppError> {
    let workers = crate::services::client_sync::cluster_workers_data(state).await?;
    let mut nodes: BTreeMap<String, ConnectInstanceAccumulator> = BTreeMap::new();

    for worker in workers.workers {
        let entry =
            nodes
                .entry(worker.node_id.clone())
                .or_insert_with(|| ConnectInstanceAccumulator {
                    id: worker.node_id.clone(),
                    node_id: worker.node_id.clone(),
                    display_name: worker.node_display_name.clone(),
                    status: connect_status_from_worker(
                        worker.status.as_str(),
                        worker.reachability.as_str(),
                    ),
                    reachability: worker.reachability.clone(),
                    sync_base_url: Some(worker.sync_base_url.clone()),
                    sync_transport: Some(worker.sync_transport.clone()),
                    tailscale_base_url: worker.tailscale_base_url.clone(),
                    lan_base_url: worker.lan_base_url.clone(),
                    localhost_base_url: worker.localhost_base_url.clone(),
                    worker_ids: BTreeSet::new(),
                    worker_classes: BTreeSet::new(),
                    capabilities: BTreeSet::new(),
                    last_seen_at: Some(worker.last_heartbeat_at),
                });

        entry.worker_ids.insert(worker.worker_id.clone());
        for worker_class in worker.worker_classes {
            entry.worker_classes.insert(worker_class);
        }
        for capability in worker.capabilities {
            entry.capabilities.insert(capability);
        }

        if status_rank(connect_status_from_worker(
            worker.status.as_str(),
            worker.reachability.as_str(),
        )) > status_rank(entry.status)
        {
            entry.status =
                connect_status_from_worker(worker.status.as_str(), worker.reachability.as_str());
        }
        if entry.display_name == "Vel Node" && worker.node_display_name != "Vel Node" {
            entry.display_name = worker.node_display_name;
        }
        entry.sync_base_url =
            pick_preferred_option(entry.sync_base_url.take(), Some(worker.sync_base_url));
        entry.sync_transport =
            pick_preferred_option(entry.sync_transport.take(), Some(worker.sync_transport));
        entry.tailscale_base_url =
            pick_preferred_option(entry.tailscale_base_url.take(), worker.tailscale_base_url);
        entry.lan_base_url = pick_preferred_option(entry.lan_base_url.take(), worker.lan_base_url);
        entry.localhost_base_url =
            pick_preferred_option(entry.localhost_base_url.take(), worker.localhost_base_url);
        entry.last_seen_at = match (entry.last_seen_at, Some(worker.last_heartbeat_at)) {
            (Some(current), Some(next)) => Some(current.max(next)),
            (None, next) => next,
            (current, None) => current,
        };
        if entry.reachability != "reachable" && worker.reachability == "reachable" {
            entry.reachability = worker.reachability;
        }
    }

    Ok(nodes
        .into_values()
        .map(connect_instance_from_accumulator)
        .map(connect_instance_to_data)
        .collect())
}

pub async fn get_connect_instance(
    state: &AppState,
    id: &str,
) -> Result<Option<ConnectInstanceData>, AppError> {
    let instances = list_connect_instances(state).await?;
    Ok(instances
        .into_iter()
        .find(|instance| instance.id == id.trim()))
}

fn connect_instance_from_accumulator(acc: ConnectInstanceAccumulator) -> ConnectInstance {
    let capabilities = acc.capabilities.into_iter().collect::<Vec<_>>();
    let worker_classes = acc.worker_classes.into_iter().collect::<Vec<_>>();
    let manifest = manifest_from_capabilities(&worker_classes, &capabilities);

    ConnectInstance {
        id: acc.id,
        node_id: acc.node_id.clone(),
        display_name: acc.display_name,
        connection_id: None,
        status: acc.status,
        reachability: acc.reachability,
        sync_base_url: acc.sync_base_url,
        sync_transport: acc.sync_transport,
        tailscale_base_url: acc.tailscale_base_url,
        lan_base_url: acc.lan_base_url,
        localhost_base_url: acc.localhost_base_url,
        worker_ids: acc.worker_ids.into_iter().collect(),
        worker_classes,
        last_seen_at: acc
            .last_seen_at
            .and_then(|timestamp| OffsetDateTime::from_unix_timestamp(timestamp).ok()),
        metadata_json: json!({
            "source": "cluster_worker_projection",
        }),
        manifest,
    }
}

fn manifest_from_capabilities(
    worker_classes: &[String],
    capabilities: &[String],
) -> ConnectInstanceCapabilityManifest {
    let mut runtimes = BTreeMap::<String, ConnectRuntimeCapability>::new();
    let supports_interactive_followup = capabilities
        .iter()
        .any(|capability| capability == "agent_interactive_followup");
    let supports_native_open = capabilities
        .iter()
        .any(|capability| capability == "agent_native_open");
    let supports_host_agent_control = capabilities
        .iter()
        .any(|capability| capability == "agent_host_control");

    for capability in capabilities {
        if let Some(runtime_id) = capability.strip_prefix("agent_runtime:") {
            let runtime_id = runtime_id.trim();
            if runtime_id.is_empty() {
                continue;
            }
            runtimes.insert(
                runtime_id.to_string(),
                ConnectRuntimeCapability {
                    runtime_id: runtime_id.to_string(),
                    display_name: runtime_display_name(runtime_id),
                    supports_launch: true,
                    supports_interactive_followup,
                    supports_native_open,
                    supports_host_agent_control,
                },
            );
        }
    }

    ConnectInstanceCapabilityManifest {
        worker_classes: worker_classes.to_vec(),
        capabilities: capabilities.to_vec(),
        supports_agent_launch: !runtimes.is_empty(),
        supports_interactive_followup,
        supports_native_open,
        supports_host_agent_control,
        launchable_runtimes: runtimes.into_values().collect(),
    }
}

fn runtime_display_name(runtime_id: &str) -> String {
    match runtime_id {
        "codex" => "Codex".to_string(),
        "copilot_agent" => "GitHub Copilot Agent".to_string(),
        "cursor_agent" => "Cursor Agent".to_string(),
        "claude_code" => "Claude Code".to_string(),
        "opencode" => "OpenCode".to_string(),
        "gemini_cli" => "Gemini CLI".to_string(),
        other => other.replace('_', " "),
    }
}

fn connect_status_from_worker(status: &str, reachability: &str) -> ConnectInstanceStatus {
    if reachability == "unreachable" {
        return ConnectInstanceStatus::Offline;
    }
    match status {
        "ready" => ConnectInstanceStatus::Ready,
        "degraded" | "busy" => ConnectInstanceStatus::Degraded,
        "" => ConnectInstanceStatus::Unknown,
        _ => ConnectInstanceStatus::Unknown,
    }
}

fn status_rank(status: ConnectInstanceStatus) -> u8 {
    match status {
        ConnectInstanceStatus::Ready => 3,
        ConnectInstanceStatus::Degraded => 2,
        ConnectInstanceStatus::Unknown => 1,
        ConnectInstanceStatus::Offline => 0,
    }
}

fn pick_preferred_option(current: Option<String>, next: Option<String>) -> Option<String> {
    current
        .filter(|value| !value.trim().is_empty())
        .or_else(|| next.filter(|value| !value.trim().is_empty()))
}

fn connect_instance_to_data(instance: ConnectInstance) -> ConnectInstanceData {
    ConnectInstanceData {
        id: instance.id,
        node_id: instance.node_id,
        display_name: instance.display_name,
        connection_id: instance.connection_id,
        status: instance.status.to_string(),
        reachability: instance.reachability,
        sync_base_url: instance.sync_base_url,
        sync_transport: instance.sync_transport,
        tailscale_base_url: instance.tailscale_base_url,
        lan_base_url: instance.lan_base_url,
        localhost_base_url: instance.localhost_base_url,
        worker_ids: instance.worker_ids,
        worker_classes: instance.worker_classes,
        last_seen_at: instance
            .last_seen_at
            .map(|timestamp| timestamp.unix_timestamp()),
        manifest: ConnectInstanceCapabilityManifestData {
            worker_classes: instance.manifest.worker_classes,
            capabilities: instance.manifest.capabilities,
            launchable_runtimes: instance
                .manifest
                .launchable_runtimes
                .into_iter()
                .map(|runtime| ConnectRuntimeCapabilityData {
                    runtime_id: runtime.runtime_id,
                    display_name: runtime.display_name,
                    supports_launch: runtime.supports_launch,
                    supports_interactive_followup: runtime.supports_interactive_followup,
                    supports_native_open: runtime.supports_native_open,
                    supports_host_agent_control: runtime.supports_host_agent_control,
                })
                .collect(),
            supports_agent_launch: instance.manifest.supports_agent_launch,
            supports_interactive_followup: instance.manifest.supports_interactive_followup,
            supports_native_open: instance.manifest.supports_native_open,
            supports_host_agent_control: instance.manifest.supports_host_agent_control,
        },
        metadata: instance.metadata_json,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_extracts_launchable_runtimes_from_capabilities() {
        let manifest = manifest_from_capabilities(
            &["authority".to_string()],
            &[
                "agent_runtime:codex".to_string(),
                "agent_runtime:claude_code".to_string(),
                "agent_interactive_followup".to_string(),
                "agent_host_control".to_string(),
            ],
        );

        assert!(manifest.supports_agent_launch);
        assert_eq!(manifest.launchable_runtimes.len(), 2);
        assert!(manifest
            .launchable_runtimes
            .iter()
            .any(|runtime| runtime.runtime_id == "codex"));
        assert!(manifest.supports_interactive_followup);
        assert!(manifest.supports_host_agent_control);
    }

    #[test]
    fn manifest_is_non_launchable_without_runtime_capabilities() {
        let manifest = manifest_from_capabilities(
            &["sync".to_string()],
            &["branch_sync".to_string(), "build_test_profiles".to_string()],
        );

        assert!(!manifest.supports_agent_launch);
        assert!(manifest.launchable_runtimes.is_empty());
    }
}
