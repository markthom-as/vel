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

pub fn prepare_quick_capture_text(value: &str) -> String {
    value
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
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

pub fn normalize_pairing_token_input(value: &str) -> String {
    let normalized: String = value
        .to_uppercase()
        .chars()
        .filter(|character| character.is_ascii() && character.is_ascii_alphanumeric())
        .take(6)
        .collect();

    if normalized.len() <= 3 {
        return normalized;
    }

    format!("{}-{}", &normalized[..3], &normalized[3..])
}
