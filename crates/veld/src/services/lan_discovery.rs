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
use vel_api_types::{ApiResponse, ClusterBootstrapData};

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
    state: &AppState,
    local_node_id: &str,
) -> Result<Vec<LanDiscoveredPeer>, AppError> {
    let mut peers = match UdpSocket::bind(("0.0.0.0", 0)).await {
        Ok(socket) => {
            if let Err(error) = socket.set_broadcast(true) {
                tracing::debug!(error = %error, "failed to enable LAN discovery broadcast");
                Vec::new()
            } else {
                let request_id = format!("lan-disc-{}", Uuid::new_v4().simple());
                let payload = serde_json::to_vec(&DiscoveryQuery {
                    protocol: DISCOVERY_PROTOCOL_VERSION.to_string(),
                    request_id: request_id.clone(),
                    sender_node_id: local_node_id.to_string(),
                })
                .map_err(|error| {
                    AppError::internal(format!("serialize LAN discovery query: {error}"))
                })?;

                let mut sent = false;
                let mut last_error = None;
                for target in crate::services::local_network::lan_broadcast_targets() {
                    match socket.send_to(&payload, (target, DISCOVERY_PORT)).await {
                        Ok(_) => sent = true,
                        Err(error) => last_error = Some(error),
                    }
                }

                if !sent {
                    tracing::debug!(
                        error = %last_error
                            .map(|error| error.to_string())
                            .unwrap_or_else(|| "no broadcast targets were available".to_string()),
                        "LAN discovery broadcast did not send; falling back to HTTP probe"
                    );
                    Vec::new()
                } else {
                    collect_discovery_responses(&socket, &request_id, local_node_id).await?
                }
            }
        }
        Err(error) => {
            tracing::debug!(error = %error, "failed to bind LAN discovery probe socket; falling back to HTTP probe");
            Vec::new()
        }
    };
    let fallback_peers = discover_peers_via_http_probe(state, local_node_id).await;
    merge_discovered_peers(&mut peers, fallback_peers);
    Ok(peers)
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

async fn discover_peers_via_http_probe(
    state: &AppState,
    local_node_id: &str,
) -> Vec<LanDiscoveredPeer> {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_millis(150))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            tracing::debug!(error = %error, "failed to build LAN probe client");
            return Vec::new();
        }
    };
    let port = reqwest::Url::parse(&state.config.base_url)
        .ok()
        .and_then(|url| url.port_or_known_default())
        .unwrap_or(4130);

    let futures = crate::services::local_network::lan_probe_targets()
        .into_iter()
        .map(|ip| fetch_http_probe_peer(&client, ip, port, local_node_id))
        .collect::<Vec<_>>();

    futures::future::join_all(futures)
        .await
        .into_iter()
        .flatten()
        .collect()
}

async fn fetch_http_probe_peer(
    client: &reqwest::Client,
    ip: std::net::Ipv4Addr,
    port: u16,
    local_node_id: &str,
) -> Option<LanDiscoveredPeer> {
    let source_addr = SocketAddr::from((ip, port));
    let url = format!("http://{ip}:{port}/v1/discovery/bootstrap");
    let response = client.get(url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    let body = response
        .json::<ApiResponse<ClusterBootstrapData>>()
        .await
        .ok()?;
    let cluster = body.data?;
    if cluster.node_id == local_node_id {
        return None;
    }
    Some(LanDiscoveredPeer {
        source_addr,
        cluster,
    })
}

fn merge_discovered_peers(
    current: &mut Vec<LanDiscoveredPeer>,
    additional: Vec<LanDiscoveredPeer>,
) {
    let mut seen = current
        .iter()
        .map(|peer| peer.cluster.node_id.clone())
        .collect::<std::collections::BTreeSet<_>>();

    for peer in additional {
        if seen.insert(peer.cluster.node_id.clone()) {
            current.push(peer);
        }
    }
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
