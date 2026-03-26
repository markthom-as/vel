//! Portable transform helpers for `vel-embedded-bridge`.
//!
//! This module is the first extraction point for logic that should be reusable
//! across native Apple FFI and a future browser/WASM adapter.

pub fn normalize_domain_hint(value: String) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn normalize_payload(value: &str) -> String {
    value
        .trim()
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn trim_text(value: &str) -> String {
    value.trim().to_string()
}

pub fn normalized_optional_trimmed(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}
