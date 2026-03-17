use vel_config::AppConfig;

pub fn run(config: &AppConfig, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(config)?);
        return Ok(());
    }

    println!("base_url: {}", config.base_url);
    println!("node_id: {}", config.node_id.as_deref().unwrap_or("-"));
    println!(
        "node_display_name: {}",
        config.node_display_name.as_deref().unwrap_or("-")
    );
    println!(
        "tailscale_base_url: {}",
        config.tailscale_base_url.as_deref().unwrap_or("-")
    );
    println!(
        "lan_base_url: {}",
        config.lan_base_url.as_deref().unwrap_or("-")
    );
    println!("bind_addr: {}", config.bind_addr);
    println!("db_path: {}", config.db_path);
    println!("artifact_root: {}", config.artifact_root);
    println!("log_level: {}", config.log_level);
    println!("llm_model_path: {}", config.llm_model_path);
    println!("llm_fast_model_path: {}", config.llm_fast_model_path);
    println!(
        "calendar_ics_url: {}",
        config.calendar_ics_url.as_deref().unwrap_or("-")
    );
    println!(
        "calendar_ics_path: {}",
        config.calendar_ics_path.as_deref().unwrap_or("-")
    );
    println!(
        "todoist_snapshot_path: {}",
        config.todoist_snapshot_path.as_deref().unwrap_or("-")
    );
    println!(
        "activity_snapshot_path: {}",
        config.activity_snapshot_path.as_deref().unwrap_or("-")
    );
    println!(
        "git_snapshot_path: {}",
        config.git_snapshot_path.as_deref().unwrap_or("-")
    );
    println!(
        "notes_path: {}",
        config.notes_path.as_deref().unwrap_or("-")
    );
    println!(
        "transcript_snapshot_path: {}",
        config.transcript_snapshot_path.as_deref().unwrap_or("-")
    );
    println!(
        "messaging_snapshot_path: {}",
        config.messaging_snapshot_path.as_deref().unwrap_or("-")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_show_json_renders() {
        let config = AppConfig::default();
        let rendered = serde_json::to_string(&config).unwrap();
        assert!(rendered.contains("bind_addr"));
    }
}
