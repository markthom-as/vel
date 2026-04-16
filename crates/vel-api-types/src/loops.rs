use serde::{Deserialize, Serialize};

use crate::UnixSeconds;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopData {
    pub kind: String,
    pub enabled: bool,
    pub interval_seconds: i64,
    pub last_started_at: Option<UnixSeconds>,
    pub last_finished_at: Option<UnixSeconds>,
    pub last_status: Option<String>,
    pub last_error: Option<String>,
    pub next_due_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopUpdateRequest {
    pub enabled: Option<bool>,
    pub interval_seconds: Option<i64>,
}
