//! WebSocket broadcast envelope and helpers. Tickets 018–019.

use serde::Serialize;
use time::OffsetDateTime;

/// Event envelope for WebSocket broadcast. Stable shape for all clients.
#[derive(Debug, Clone, Serialize)]
pub struct WsEnvelope {
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

impl WsEnvelope {
    pub fn new(event_type: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            event_type: event_type.into(),
            timestamp: OffsetDateTime::now_utc().unix_timestamp().to_string(),
            payload,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
