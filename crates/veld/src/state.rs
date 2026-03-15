use vel_config::AppConfig;
use vel_storage::Storage;

use crate::policy_config::PolicyConfig;

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub config: AppConfig,
    pub policy_config: PolicyConfig,
}

impl AppState {
    pub fn new(storage: Storage, config: AppConfig, policy_config: PolicyConfig) -> Self {
        Self {
            storage,
            config,
            policy_config,
        }
    }
}
