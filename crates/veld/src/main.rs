mod adapters;
mod app;
mod broadcast;
mod errors;
mod llm;
mod middleware;
mod policy_config;
mod routes;
mod services;
mod state;
mod worker;

use anyhow::Context;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tracing::info;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::state::AppState;

const DEFAULT_POLICIES_PATH: &str = "config/policies.yaml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = AppConfig::load().context("loading config")?;
    let policies_path =
        std::env::var("VEL_POLICIES_PATH").unwrap_or_else(|_| DEFAULT_POLICIES_PATH.to_string());
    let policy_config = policy_config::PolicyConfig::load(&policies_path)
        .with_context(|| format!("loading policy config from {}", policies_path))?;

    let storage = Storage::connect(&config.db_path)
        .await
        .context("connecting db")?;
    storage.migrate().await.context("running migrations")?;

    if let Err(e) = storage
        .emit_event(
            "DAEMON_STARTED",
            "daemon",
            None,
            &serde_json::json!({ "bind_addr": config.bind_addr }).to_string(),
        )
        .await
    {
        tracing::warn!(error = %e, "failed to emit DAEMON_STARTED event");
    }
    if let Err(e) = storage
        .emit_event(
            "CONFIG_LOADED",
            "daemon",
            None,
            &serde_json::json!({ "policies_path": policies_path }).to_string(),
        )
        .await
    {
        tracing::warn!(error = %e, "failed to emit CONFIG_LOADED event");
    }

    let (llm_router, chat_profile_id, chat_fallback_profile_id) =
        llm::build_chat_router(&storage).await;
    let llm_router = llm_router.map(std::sync::Arc::new);
    let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
    let state = AppState::new_with_fallback(
        storage,
        config.clone(),
        policy_config.clone(),
        broadcast_tx,
        llm_router,
        chat_profile_id,
        chat_fallback_profile_id,
    );
    let startup_state = state.clone();
    tokio::spawn(async move {
        match services::integrations::bootstrap_local_context_sources(
            &startup_state.storage,
            &startup_state.config,
        )
        .await
        {
            Ok(count) if count > 0 => {
                if let Err(error) = services::evaluate::run_and_broadcast(&startup_state).await {
                    tracing::warn!(error = %error, "evaluate after startup local source bootstrap failed");
                }
            }
            Ok(_) => {}
            Err(error) => {
                tracing::warn!(error = %error, "startup local source bootstrap failed");
            }
        }
    });
    tokio::spawn(worker::run_background_workers(state.clone()));
    tokio::spawn(services::lan_discovery::run_responder(state.clone()));

    let bind_addr = effective_bind_addr(&config).await;
    let listener = TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("binding {}", bind_addr))?;
    let app = app::build_app_with_state(state);

    info!(bind_addr = %bind_addr, "veld starting");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("serving http")?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

async fn effective_bind_addr(config: &AppConfig) -> String {
    if !is_loopback_bind_addr(&config.bind_addr) {
        return config.bind_addr.clone();
    }

    let has_remote_discovery_transport = config.lan_base_url.is_some()
        || crate::services::local_network::discover_lan_base_url(config).is_some()
        || config.tailscale_base_url.is_some()
        || crate::services::tailscale::discover_base_url(config)
            .await
            .is_some();

    if !has_remote_discovery_transport {
        return config.bind_addr.clone();
    }

    expanded_bind_addr(&config.bind_addr).unwrap_or_else(|| config.bind_addr.clone())
}

fn is_loopback_bind_addr(bind_addr: &str) -> bool {
    bind_addr
        .parse::<SocketAddr>()
        .map(|addr| addr.ip().is_loopback())
        .unwrap_or(
            matches!(bind_addr, s if s.starts_with("127.0.0.1:") || s.starts_with("localhost:")),
        )
}

fn expanded_bind_addr(bind_addr: &str) -> Option<String> {
    let addr = bind_addr.parse::<SocketAddr>().ok()?;
    Some(SocketAddr::from((Ipv4Addr::UNSPECIFIED, addr.port())).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expanded_bind_addr_rewrites_loopback_port_to_all_interfaces() {
        assert_eq!(
            expanded_bind_addr("127.0.0.1:4130").as_deref(),
            Some("0.0.0.0:4130")
        );
    }
}
