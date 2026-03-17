use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};
use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_llm::Router;
use vel_storage::Storage;

use crate::broadcast::WsEnvelope;
use crate::policy_config::PolicyConfig;

#[derive(Debug, Clone, Copy)]
pub struct WorkerRuntimeSnapshot {
    pub started_at: i64,
    pub max_concurrency: u32,
    pub current_load: u32,
}

#[derive(Debug)]
pub struct WorkerRuntimeState {
    started_at: i64,
    max_concurrency: u32,
    current_load: AtomicU32,
}

pub struct WorkerLoadGuard<'a> {
    state: &'a WorkerRuntimeState,
}

impl WorkerRuntimeState {
    pub fn new() -> Self {
        let max_concurrency = std::thread::available_parallelism()
            .map(|parallelism| parallelism.get() as u32)
            .unwrap_or(1)
            .max(1);
        Self {
            started_at: time::OffsetDateTime::now_utc().unix_timestamp(),
            max_concurrency,
            current_load: AtomicU32::new(0),
        }
    }

    pub fn snapshot(&self) -> WorkerRuntimeSnapshot {
        WorkerRuntimeSnapshot {
            started_at: self.started_at,
            max_concurrency: self.max_concurrency,
            current_load: self.current_load.load(Ordering::Relaxed),
        }
    }

    pub fn begin_work(&self) -> WorkerLoadGuard<'_> {
        self.current_load.fetch_add(1, Ordering::Relaxed);
        WorkerLoadGuard { state: self }
    }
}

impl Drop for WorkerLoadGuard<'_> {
    fn drop(&mut self) {
        self.state.current_load.fetch_sub(1, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub config: AppConfig,
    pub policy_config: PolicyConfig,
    pub worker_runtime: Arc<WorkerRuntimeState>,
    /// Sender for WebSocket broadcast. Clone and send to notify all connected /ws clients.
    pub broadcast_tx: broadcast::Sender<WsEnvelope>,
    /// LLM router for chat assistant. None if no configs/models or no chat profile.
    pub llm_router: Option<Arc<Router>>,
    /// Profile ID for chat task (from routing.toml). Present when llm_router is Some.
    pub chat_profile_id: Option<String>,
    /// Optional fallback profile ID for chat generation (typically remote overflow).
    pub chat_fallback_profile_id: Option<String>,
}

impl AppState {
    pub fn new(
        storage: Storage,
        config: AppConfig,
        policy_config: PolicyConfig,
        broadcast_tx: broadcast::Sender<WsEnvelope>,
        llm_router: Option<Arc<Router>>,
        chat_profile_id: Option<String>,
    ) -> Self {
        Self::new_with_fallback(
            storage,
            config,
            policy_config,
            broadcast_tx,
            llm_router,
            chat_profile_id,
            None,
        )
    }

    pub fn new_with_fallback(
        storage: Storage,
        config: AppConfig,
        policy_config: PolicyConfig,
        broadcast_tx: broadcast::Sender<WsEnvelope>,
        llm_router: Option<Arc<Router>>,
        chat_profile_id: Option<String>,
        chat_fallback_profile_id: Option<String>,
    ) -> Self {
        Self {
            storage,
            config,
            policy_config,
            worker_runtime: Arc::new(WorkerRuntimeState::new()),
            broadcast_tx,
            llm_router,
            chat_profile_id,
            chat_fallback_profile_id,
        }
    }
}
