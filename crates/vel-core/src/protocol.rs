pub use vel_protocol::{
    CapabilityRequest, ProtocolEnvelope, ProtocolPayload, ProtocolSender, ProtocolTraceContext,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    fn repo_file(relative: &str) -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join(relative),
        )
        .expect("repo file should be readable")
    }

    #[test]
    fn protocol_envelope_example_parses_via_core_reexport() {
        let raw = repo_file("config/examples/swarm-protocol-envelope.example.json");
        let envelope: ProtocolEnvelope =
            serde_json::from_str(&raw).expect("protocol envelope should parse");
        assert_eq!(
            envelope.protocol_version,
            vel_protocol::CURRENT_PROTOCOL_VERSION
        );
    }
}
