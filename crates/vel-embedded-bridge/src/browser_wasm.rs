//! Browser/WASM scaffold for `vel-embedded-bridge`.
//!
//! This file is intentionally non-executable today. It marks the future adapter
//! seam where portable packet-shaping logic from `portable_core` can be exposed
//! to browser code without the native Apple FFI loader path.

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
}
