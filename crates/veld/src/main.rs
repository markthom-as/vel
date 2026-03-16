mod adapters;
mod app;
mod errors;
mod policy_config;
mod routes;
mod services;
mod state;
mod worker;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;
use vel_config::AppConfig;
use vel_storage::Storage;

const DEFAULT_POLICIES_PATH: &str = "config/policies.yaml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = AppConfig::load().context("loading config")?;
    let policies_path = std::env::var("VEL_POLICIES_PATH").unwrap_or_else(|_| DEFAULT_POLICIES_PATH.to_string());
    let policy_config = policy_config::PolicyConfig::load(&policies_path)
        .with_context(|| format!("loading policy config from {}", policies_path))?;

    let storage = Storage::connect(&config.db_path).await.context("connecting db")?;
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

    let storage_for_worker = storage.clone();
    tokio::spawn(worker::run_ingestion_worker(storage_for_worker));

    let bind_addr = config.bind_addr.clone();
    let listener = TcpListener::bind(&config.bind_addr)
        .await
        .with_context(|| format!("binding {}", config.bind_addr))?;
    let app = app::build_app(storage, config, policy_config);

    info!(bind_addr = %bind_addr, "veld starting");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("serving http")?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
