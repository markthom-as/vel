use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::broadcast::WsEnvelope;
use crate::policy_config::PolicyConfig;

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub config: AppConfig,
    pub policy_config: PolicyConfig,
    /// Sender for WebSocket broadcast. Clone and send to notify all connected /ws clients.
    pub broadcast_tx: broadcast::Sender<WsEnvelope>,
}

impl AppState {
    pub fn new(
        storage: Storage,
        config: AppConfig,
        policy_config: PolicyConfig,
        broadcast_tx: broadcast::Sender<WsEnvelope>,
    ) -> Self {
        Self {
            storage,
            config,
            policy_config,
            broadcast_tx,
        }
    }
}
