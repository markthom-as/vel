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

#[cfg(test)]
mod tests {
    use super::WsEnvelope;

    #[test]
    fn websocket_envelope_serializes_type_and_payload() {
        let envelope = WsEnvelope::new(
            "interventions:new",
            serde_json::json!({
                "id": "intv_1",
                "message_id": "msg_1",
                "kind": "reminder",
                "state": "active",
            }),
        );

        let json = envelope.to_json().unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["type"], "interventions:new");
        assert_eq!(value["payload"]["id"], "intv_1");
        assert_eq!(value["payload"]["message_id"], "msg_1");
        assert_eq!(value["payload"]["kind"], "reminder");
        assert_eq!(value["payload"]["state"], "active");
    }
}
