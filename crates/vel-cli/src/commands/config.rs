use vel_config::AppConfig;

pub fn run(config: &AppConfig, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(config)?);
        return Ok(());
    }

    println!("base_url: {}", config.base_url);
    println!("bind_addr: {}", config.bind_addr);
    println!("db_path: {}", config.db_path);
    println!("artifact_root: {}", config.artifact_root);
    println!("log_level: {}", config.log_level);
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

