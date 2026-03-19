//! Bounded LAN peer discovery for nearby Vel daemons.
//!
//! This intentionally uses a small UDP broadcast/query protocol on the local subnet instead of
//! blind HTTP port scanning. Responses carry only safe cluster bootstrap metadata.

use std::net::SocketAddr;

use tokio::{
    net::UdpSocket,
    time::{timeout, Duration, Instant},
};
use uuid::Uuid;
use vel_api_types::ClusterBootstrapData;

use crate::{errors::AppError, state::AppState};

const DISCOVERY_PORT: u16 = 4131;
const DISCOVERY_QUERY_TIMEOUT_MS: u64 = 250;
const DISCOVERY_BUFFER_SIZE: usize = 16 * 1024;
const DISCOVERY_PROTOCOL_VERSION: &str = "vel_lan_discovery_v1";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DiscoveryQuery {
    protocol: String,
    request_id: String,
    sender_node_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DiscoveryResponse {
    protocol: String,
    request_id: String,
    cluster: ClusterBootstrapData,
}

#[derive(Debug, Clone)]
pub(crate) struct LanDiscoveredPeer {
    pub source_addr: SocketAddr,
    pub cluster: ClusterBootstrapData,
}

pub(crate) async fn run_responder(state: AppState) {
    let socket = match UdpSocket::bind(("0.0.0.0", DISCOVERY_PORT)).await {
        Ok(socket) => socket,
        Err(error) => {
            tracing::warn!(error = %error, port = DISCOVERY_PORT, "LAN discovery responder failed to bind");
            return;
        }
    };

    let mut buffer = vec![0u8; DISCOVERY_BUFFER_SIZE];
    loop {
        let Ok((size, peer_addr)) = socket.recv_from(&mut buffer).await else {
            continue;
        };
        let Ok(query) = serde_json::from_slice::<DiscoveryQuery>(&buffer[..size]) else {
            continue;
        };
        if query.protocol != DISCOVERY_PROTOCOL_VERSION {
            continue;
        }

        let Ok(cluster) =
            crate::services::client_sync::effective_cluster_bootstrap_data(&state).await
        else {
            continue;
        };
        let response = DiscoveryResponse {
            protocol: DISCOVERY_PROTOCOL_VERSION.to_string(),
            request_id: query.request_id,
            cluster: ClusterBootstrapData {
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
                branch_sync: cluster.branch_sync.map(|branch_sync| {
                    vel_api_types::BranchSyncCapabilityData {
                        repo_root: branch_sync.repo_root,
                        default_remote: branch_sync.default_remote,
                        supports_fetch: branch_sync.supports_fetch,
                        supports_pull: branch_sync.supports_pull,
                        supports_push: branch_sync.supports_push,
                    }
                }),
                validation_profiles: cluster
                    .validation_profiles
                    .into_iter()
                    .map(|profile| vel_api_types::ValidationProfileData {
                        profile_id: profile.profile_id,
                        label: profile.label,
                        command_hint: profile.command_hint,
                        environment: profile.environment,
                    })
                    .collect(),
                linked_nodes: vec![],
                projects: vec![],
                action_items: vec![],
                pending_writebacks: vec![],
                conflicts: vec![],
                people: vec![],
            },
        };
        let Ok(payload) = serde_json::to_vec(&response) else {
            continue;
        };
        let _ = socket.send_to(&payload, peer_addr).await;
    }
}

pub(crate) async fn discover_peers(
    _state: &AppState,
    local_node_id: &str,
) -> Result<Vec<LanDiscoveredPeer>, AppError> {
    let socket = UdpSocket::bind(("0.0.0.0", 0))
        .await
        .map_err(|error| AppError::internal(format!("bind LAN discovery probe socket: {error}")))?;
    socket
        .set_broadcast(true)
        .map_err(|error| AppError::internal(format!("enable LAN discovery broadcast: {error}")))?;

    let request_id = format!("lan-disc-{}", Uuid::new_v4().simple());
    let payload = serde_json::to_vec(&DiscoveryQuery {
        protocol: DISCOVERY_PROTOCOL_VERSION.to_string(),
        request_id: request_id.clone(),
        sender_node_id: local_node_id.to_string(),
    })
    .map_err(|error| AppError::internal(format!("serialize LAN discovery query: {error}")))?;

    let mut sent = false;
    let mut last_error = None;
    for target in crate::services::local_network::lan_broadcast_targets() {
        match socket.send_to(&payload, (target, DISCOVERY_PORT)).await {
            Ok(_) => sent = true,
            Err(error) => last_error = Some(error),
        }
    }
    if !sent {
        return Err(AppError::internal(format!(
            "send LAN discovery broadcast: {}",
            last_error
                .map(|error| error.to_string())
                .unwrap_or_else(|| "no broadcast targets were available".to_string())
        )));
    }

    collect_discovery_responses(&socket, &request_id, local_node_id).await
}

async fn collect_discovery_responses(
    socket: &UdpSocket,
    request_id: &str,
    local_node_id: &str,
) -> Result<Vec<LanDiscoveredPeer>, AppError> {
    let deadline = Instant::now() + Duration::from_millis(DISCOVERY_QUERY_TIMEOUT_MS);
    let mut buffer = vec![0u8; DISCOVERY_BUFFER_SIZE];
    let mut peers = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let recv = timeout(remaining, socket.recv_from(&mut buffer)).await;
        let Ok(Ok((size, source_addr))) = recv else {
            break;
        };
        let Ok(response) = serde_json::from_slice::<DiscoveryResponse>(&buffer[..size]) else {
            continue;
        };
        if response.protocol != DISCOVERY_PROTOCOL_VERSION || response.request_id != request_id {
            continue;
        }
        if response.cluster.node_id == local_node_id
            || !seen.insert(response.cluster.node_id.clone())
        {
            continue;
        }
        peers.push(LanDiscoveredPeer {
            source_addr,
            cluster: response.cluster,
        });
    }

    Ok(peers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_response_round_trips_cluster_payload() {
        let response = DiscoveryResponse {
            protocol: DISCOVERY_PROTOCOL_VERSION.to_string(),
            request_id: "req_1".to_string(),
            cluster: ClusterBootstrapData {
                node_id: "vel-lan".to_string(),
                node_display_name: "Vel LAN".to_string(),
                active_authority_node_id: "vel-lan".to_string(),
                active_authority_epoch: 1,
                sync_base_url: "http://192.168.1.20:4130".to_string(),
                sync_transport: "lan".to_string(),
                tailscale_base_url: None,
                lan_base_url: Some("http://192.168.1.20:4130".to_string()),
                localhost_base_url: Some("http://127.0.0.1:4130".to_string()),
                capabilities: vec!["read_context".to_string()],
                branch_sync: None,
                validation_profiles: vec![],
                linked_nodes: vec![],
                projects: vec![],
                action_items: vec![],
                pending_writebacks: vec![],
                conflicts: vec![],
                people: vec![],
            },
        };

        let encoded = serde_json::to_vec(&response).unwrap();
        let decoded = serde_json::from_slice::<DiscoveryResponse>(&encoded).unwrap();
        assert_eq!(decoded.cluster.node_id, "vel-lan");
        assert_eq!(
            decoded.cluster.lan_base_url.as_deref(),
            Some("http://192.168.1.20:4130")
        );
    }
}
