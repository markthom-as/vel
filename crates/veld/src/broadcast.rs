//! WebSocket broadcast envelope alias. Tickets 018–019.

use vel_api_types::WsEnvelope as ApiWsEnvelope;

/// Event envelope for WebSocket broadcast. Stable shape for all clients.
pub type WsEnvelope = ApiWsEnvelope;

#[cfg(test)]
mod tests {
    use super::WsEnvelope;
    use vel_api_types::WsEventType;

    #[test]
    fn websocket_envelope_serializes_type_and_payload() {
        let envelope = WsEnvelope::new(
            WsEventType::InterventionsNew,
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
        let timestamp = value["timestamp"]
            .as_str()
            .expect("timestamp should serialize as string");
        assert!(time::OffsetDateTime::parse(
            timestamp,
            &time::format_description::well_known::Rfc3339
        )
        .is_ok());
    }
}
