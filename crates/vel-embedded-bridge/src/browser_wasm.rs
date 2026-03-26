//! Browser/WASM scaffold for `vel-embedded-bridge`.
//!
//! This file is intentionally non-executable today. It marks the future adapter
//! seam where portable packet-shaping logic from `portable_core` can be exposed
//! to browser code without the native Apple FFI loader path.

use crate::portable_core::{
    collect_remote_routes, normalize_domain_hint, normalize_pairing_token_input,
    normalized_optional_trimmed, normalize_positive_minutes,
    prepare_assistant_entry_fallback_payload, prepare_capture_metadata_payload,
    prepare_linking_request_packet, prepare_queued_action_packet,
    prepare_thread_draft_packet, prepare_voice_capture_payload,
    prepare_voice_quick_action_packet, trim_text,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserPacketResponse {
    pub kind: &'static str,
    pub payload_json: String,
}

pub struct BrowserWasmScaffold;

impl BrowserWasmScaffold {
    pub const STATUS: &'static str = "scaffold_only";

    pub fn intended_packet_families() -> &'static [&'static str] {
        &[
            "cached_now_hydration",
            "offline_request_packaging",
            "deterministic_domain_helpers",
            "thread_draft_packaging",
            "voice_capture_packaging",
            "voice_quick_action_packaging",
            "voice_continuity_packaging",
            "queued_action_packaging",
            "linking_settings_normalization",
            "assistant_entry_fallback_packaging",
            "linking_request_packaging",
            "capture_metadata_packaging",
            "voice_continuity_summary_packaging",
            "voice_offline_response_packaging",
            "voice_cached_query_packaging",
            "linking_feedback_packaging",
        ]
    }

    pub fn normalize_pairing_token_packet(input: &str) -> BrowserPacketResponse {
        BrowserPacketResponse {
            kind: "linking_settings_normalization",
            payload_json: format!(
                "{{\"tokenCode\":\"{}\"}}",
                normalize_pairing_token_input(input)
            ),
        }
    }

    pub fn normalize_domain_hint_packet(input: &str) -> BrowserPacketResponse {
        BrowserPacketResponse {
            kind: "deterministic_domain_helpers",
            payload_json: format!(
                "{{\"normalized\":\"{}\"}}",
                normalize_domain_hint(input.to_string())
            ),
        }
    }

    pub fn queued_action_packet(
        kind: String,
        target_id: Option<String>,
        text: Option<String>,
        minutes: Option<i64>,
    ) -> BrowserPacketResponse {
        let packet = prepare_queued_action_packet(
            trim_text(&kind),
            normalized_optional_trimmed(target_id),
            normalized_optional_trimmed(text),
            normalize_positive_minutes(minutes),
        );

        BrowserPacketResponse {
            kind: "queued_action_packaging",
            payload_json: format!(
                "{{\"queueKind\":\"{}\",\"targetId\":{},\"text\":{},\"minutes\":{},\"ready\":{}}}",
                packet.queue_kind,
                option_json(packet.target_id),
                option_json(packet.text),
                option_number_json(packet.minutes),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn voice_quick_action_packet(
        intent_storage_token: String,
        primary_text: String,
        target_id: Option<String>,
        minutes: Option<i64>,
    ) -> BrowserPacketResponse {
        let packet = prepare_voice_quick_action_packet(
            &trim_text(&intent_storage_token),
            &primary_text,
            normalized_optional_trimmed(target_id),
            normalize_positive_minutes(minutes),
        );

        BrowserPacketResponse {
            kind: "voice_quick_action_packaging",
            payload_json: format!(
                "{{\"queueKind\":\"{}\",\"targetId\":{},\"text\":{},\"minutes\":{},\"ready\":{}}}",
                packet.queue_kind,
                option_json(packet.target_id),
                option_json(packet.text),
                option_number_json(packet.minutes),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn assistant_entry_fallback_packet(
        text: String,
        requested_conversation_id: Option<String>,
    ) -> BrowserPacketResponse {
        let payload = prepare_assistant_entry_fallback_payload(
            &text,
            normalized_optional_trimmed(requested_conversation_id),
        );

        BrowserPacketResponse {
            kind: "assistant_entry_fallback_packaging",
            payload_json: format!("{{\"payload\":{}}}", string_json(&payload)),
        }
    }

    pub fn capture_metadata_packet(
        text: String,
        capture_type: String,
        source_device: String,
    ) -> BrowserPacketResponse {
        let payload = prepare_capture_metadata_payload(&text, &capture_type, &source_device);

        BrowserPacketResponse {
            kind: "capture_metadata_packaging",
            payload_json: format!("{{\"payload\":{}}}", string_json(&payload)),
        }
    }

    pub fn thread_draft_packet(
        text: String,
        requested_conversation_id: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_thread_draft_packet(&text, requested_conversation_id);

        BrowserPacketResponse {
            kind: "thread_draft_packaging",
            payload_json: format!(
                "{{\"payload\":{},\"requestedConversationId\":{}}}",
                string_json(&packet.payload),
                option_json(packet.requested_conversation_id)
            ),
        }
    }

    pub fn voice_capture_packet(
        transcript: String,
        intent_storage_token: String,
    ) -> BrowserPacketResponse {
        let payload = prepare_voice_capture_payload(&transcript, &intent_storage_token);

        BrowserPacketResponse {
            kind: "voice_capture_packaging",
            payload_json: format!("{{\"payload\":{}}}", string_json(&payload)),
        }
    }

    pub fn linking_request_packet(
        token_code: Option<String>,
        target_base_url: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_linking_request_packet(token_code, target_base_url);

        BrowserPacketResponse {
            kind: "linking_request_packaging",
            payload_json: format!(
                "{{\"tokenCode\":{},\"targetBaseUrl\":{}}}",
                option_json(packet.token_code),
                option_json(packet.target_base_url)
            ),
        }
    }

    pub fn remote_routes_packet(
        sync_base_url: Option<String>,
        tailscale_base_url: Option<String>,
        lan_base_url: Option<String>,
        public_base_url: Option<String>,
    ) -> BrowserPacketResponse {
        let routes = collect_remote_routes(
            sync_base_url,
            tailscale_base_url,
            lan_base_url,
            public_base_url,
        );

        let payload_json = format!(
            "[{}]",
            routes
                .into_iter()
                .map(|route| format!(
                    "{{\"label\":{},\"baseUrl\":{}}}",
                    string_json(&route.label),
                    string_json(&route.base_url)
                ))
                .collect::<Vec<_>>()
                .join(",")
        );

        BrowserPacketResponse {
            kind: "linking_settings_normalization",
            payload_json,
        }
    }
}

fn string_json(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('\"', "\\\"").replace('\n', "\\n"))
}

fn option_json(value: Option<String>) -> String {
    value.map(|value| string_json(&value)).unwrap_or_else(|| "null".to_string())
}

fn option_number_json(value: Option<i64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_else(|| "null".to_string())
}

fn bool_json(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
