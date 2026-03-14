mod app;
mod errors;
mod routes;
mod services;
mod state;
mod worker;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;
use vel_config::AppConfig;
use vel_storage::Storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = AppConfig::load().context("loading config")?;
    let storage = Storage::connect(&config.db_path).await.context("connecting db")?;
    storage.migrate().await.context("running migrations")?;

    let storage_for_worker = storage.clone();
    tokio::spawn(worker::run_ingestion_worker(storage_for_worker));

    let listener = TcpListener::bind(&config.bind_addr)
        .await
        .with_context(|| format!("binding {}", config.bind_addr))?;
    let app = app::build_app(storage, config);

    info!(bind_addr = %config.bind_addr, "veld starting");
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
