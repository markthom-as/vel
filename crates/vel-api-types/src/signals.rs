use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCreateRequest {
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalData {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload: JsonValue,
    pub created_at: i64,
}
