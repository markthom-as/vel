use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, ClusterBootstrapData};

use crate::{errors::AppError, state::AppState};

pub async fn bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterBootstrapData>>, AppError> {
    state.storage.healthcheck().await?;

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

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
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
        },
        request_id,
    )))
}

fn preferred_sync_target(
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

    #[test]
    fn preferred_sync_target_prefers_tailscale() {
        let (url, transport) = preferred_sync_target(
            Some("http://vel.tailnet.ts.net:4130"),
            "http://127.0.0.1:4130",
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
        );
        assert_eq!(url, "http://vel.tailnet.ts.net:4130");
        assert_eq!(transport, "tailscale");
    }

    #[test]
    fn preferred_sync_target_prefers_localhost_when_no_tailscale() {
        let (url, transport) = preferred_sync_target(
            None,
            "http://127.0.0.1:4130",
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
        );
        assert_eq!(url, "http://127.0.0.1:4130");
        assert_eq!(transport, "localhost");
    }
}
