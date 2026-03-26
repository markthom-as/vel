use crate::client::ApiClient;
use anyhow::Context;
use serde_json::Value;

fn string_or_dash(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "-".to_string())
}

fn bool_or_false(value: Option<&Value>) -> bool {
    value.and_then(Value::as_bool).unwrap_or(false)
}

fn settings_lines(settings: &Value) -> Vec<String> {
    let mut lines = vec![
        format!(
            "timezone:                    {}",
            string_or_dash(settings.get("timezone"))
        ),
        format!(
            "node_display_name:           {}",
            string_or_dash(settings.get("node_display_name"))
        ),
        format!(
            "writeback_enabled:           {}",
            bool_or_false(settings.get("writeback_enabled"))
        ),
        format!(
            "tailscale_preferred:         {}",
            bool_or_false(settings.get("tailscale_preferred"))
        ),
        format!(
            "tailscale_base_url:          {}",
            string_or_dash(settings.get("tailscale_base_url"))
        ),
        format!(
            "tailscale_auto_discovered:   {}",
            bool_or_false(settings.get("tailscale_base_url_auto_discovered"))
        ),
        format!(
            "lan_base_url:                {}",
            string_or_dash(settings.get("lan_base_url"))
        ),
        format!(
            "lan_auto_discovered:         {}",
            bool_or_false(settings.get("lan_base_url_auto_discovered"))
        ),
    ];

    if let Some(llm) = settings.get("llm").and_then(Value::as_object) {
        lines.push(format!(
            "llm_models_dir:              {}",
            string_or_dash(llm.get("models_dir"))
        ));
        lines.push(format!(
            "llm_default_profile:         {}",
            string_or_dash(llm.get("default_chat_profile_id"))
        ));
        lines.push(format!(
            "llm_fallback_profile:        {}",
            string_or_dash(llm.get("fallback_chat_profile_id"))
        ));
        if let Some(profiles) = llm.get("profiles").and_then(Value::as_array) {
            lines.push(format!("llm_profiles:                {}", profiles.len()));
            for profile in profiles {
                let id = string_or_dash(profile.get("id"));
                let provider = string_or_dash(profile.get("provider"));
                let model = string_or_dash(profile.get("model"));
                let enabled = bool_or_false(profile.get("enabled"));
                lines.push(format!(
                    "  - {}  {}  {}  enabled={}",
                    id, provider, model, enabled
                ));
            }
        }
    }

    if let Some(core) = settings.get("core_settings").and_then(Value::as_object) {
        lines.push(format!(
            "core_user_display_name:      {}",
            string_or_dash(core.get("user_display_name"))
        ));
        lines.push(format!(
            "core_location:               {}",
            string_or_dash(core.get("client_location_label"))
        ));
        lines.push(format!(
            "core_developer_mode:         {}",
            bool_or_false(core.get("developer_mode"))
        ));
        lines.push(format!(
            "core_bypass_setup_gate:      {}",
            bool_or_false(core.get("bypass_setup_gate"))
        ));
    }

    if let Some(web) = settings.get("web_settings").and_then(Value::as_object) {
        lines.push(format!(
            "web_dense_rows:              {}",
            bool_or_false(web.get("dense_rows"))
        ));
        lines.push(format!(
            "web_tabular_numbers:         {}",
            bool_or_false(web.get("tabular_numbers"))
        ));
        lines.push(format!(
            "web_reduced_motion:          {}",
            bool_or_false(web.get("reduced_motion"))
        ));
        lines.push(format!(
            "web_strong_focus:            {}",
            bool_or_false(web.get("strong_focus"))
        ));
        lines.push(format!(
            "web_docked_action_bar:       {}",
            bool_or_false(web.get("docked_action_bar"))
        ));
    }

    lines
}

pub async fn run_show(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.get_settings().await.context("get settings")?;
    let settings = response
        .data
        .ok_or_else(|| anyhow::anyhow!("settings response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&settings)?);
        return Ok(());
    }
    for line in settings_lines(&settings) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::settings_lines;

    #[test]
    fn settings_lines_surface_sync_llm_and_web_fields() {
        let settings = serde_json::json!({
            "timezone": "America/Denver",
            "node_display_name": "desk",
            "writeback_enabled": true,
            "tailscale_preferred": true,
            "tailscale_base_url": "https://100.0.0.2:5173",
            "tailscale_base_url_auto_discovered": false,
            "lan_base_url": "http://192.168.1.20:4000",
            "lan_base_url_auto_discovered": true,
            "llm": {
                "models_dir": "/models",
                "default_chat_profile_id": "default",
                "fallback_chat_profile_id": "fallback",
                "profiles": [
                    {
                        "id": "default",
                        "provider": "openai",
                        "model": "gpt-5",
                        "enabled": true
                    }
                ]
            },
            "core_settings": {
                "user_display_name": "Jove",
                "client_location_label": "Office",
                "developer_mode": true,
                "bypass_setup_gate": false
            },
            "web_settings": {
                "dense_rows": true,
                "tabular_numbers": true,
                "reduced_motion": false,
                "strong_focus": true,
                "docked_action_bar": true
            }
        });

        let lines = settings_lines(&settings);

        assert!(lines.iter().any(|line| line.contains("timezone:")));
        assert!(lines
            .iter()
            .any(|line| line.contains("llm_profiles:                1")));
        assert!(lines
            .iter()
            .any(|line| line.contains("core_developer_mode:         true")));
        assert!(lines
            .iter()
            .any(|line| line.contains("web_docked_action_bar:       true")));
    }
}
