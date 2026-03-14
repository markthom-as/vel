//! `vel doctor` — verify config, DB, schema version, artifact directory, daemon health.

use crate::client::ApiClient;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = match client.doctor().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Could not reach veld: {}", e);
            eprintln!("Hint: Start the daemon with `cargo run -p veld` (or ensure VEL_BASE_URL is set)");
            std::process::exit(2);
        }
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("doctor response missing data");
    println!("daemon: {}", data.daemon);
    println!("db: {}", data.db);
    println!("schema_version: {}", data.schema_version);
    println!("artifact_dir: {}", data.artifact_dir);
    println!("version: {}", data.version);

    let all_ok = data.daemon == "ok"
        && data.db == "ok"
        && (data.artifact_dir == "ok" || data.artifact_dir.starts_with("ok "));
    if !all_ok {
        std::process::exit(1);
    }
    Ok(())
}
