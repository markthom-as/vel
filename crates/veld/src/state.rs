use std::sync::Arc;
use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_storage::Storage;
use vel_llm::Router;

use crate::broadcast::WsEnvelope;
use crate::policy_config::PolicyConfig;

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub config: AppConfig,
    pub policy_config: PolicyConfig,
    /// Sender for WebSocket broadcast. Clone and send to notify all connected /ws clients.
    pub broadcast_tx: broadcast::Sender<WsEnvelope>,
    /// LLM router for chat assistant. None if no configs/models or no chat profile.
    pub llm_router: Option<Arc<Router>>,
    /// Profile ID for chat task (from routing.toml). Present when llm_router is Some.
    pub chat_profile_id: Option<String>,
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
        Self {
            storage,
            config,
            policy_config,
            broadcast_tx,
            llm_router,
            chat_profile_id,
        }
    }
}
