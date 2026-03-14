use vel_config::AppConfig;
use vel_storage::Storage;

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(storage: Storage, config: AppConfig) -> Self {
        Self { storage, config }
    }
}
