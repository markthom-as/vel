use serde_json::Value;

const REDACTED: &str = "***redacted***";
const SENSITIVE_KEY_HINTS: [&str; 8] = [
    "secret",
    "token",
    "password",
    "api_key",
    "apikey",
    "authorization",
    "cookie",
    "credential",
];

fn sensitive_key(key: &str) -> bool {
    let lowered = key.to_ascii_lowercase();
    SENSITIVE_KEY_HINTS
        .iter()
        .any(|hint| lowered.contains(hint))
}

fn secret_like_string(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    lowered.starts_with("sk-")
        || lowered.starts_with("ghp_")
        || lowered.starts_with("xox")
        || lowered.contains("bearer ")
}

pub fn redact_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    if sensitive_key(key) {
                        (key.clone(), Value::String(REDACTED.to_string()))
                    } else {
                        (key.clone(), redact_json(value))
                    }
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(redact_json).collect()),
        Value::String(value) if secret_like_string(value) => Value::String(REDACTED.to_string()),
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::redact_json;

    #[test]
    fn redacts_sensitive_keys_recursively() {
        let value = serde_json::json!({
            "api_key": "abc",
            "nested": {
                "token": "xyz",
                "ok": "value"
            }
        });
        let redacted = redact_json(&value);
        assert_eq!(redacted["api_key"], "***redacted***");
        assert_eq!(redacted["nested"]["token"], "***redacted***");
        assert_eq!(redacted["nested"]["ok"], "value");
    }

    #[test]
    fn redacts_secret_like_strings() {
        let value = serde_json::json!({ "note": "Bearer abc.def.ghi" });
        let redacted = redact_json(&value);
        assert_eq!(redacted["note"], "***redacted***");
    }
}
