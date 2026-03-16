//! `vel doctor` — verify config, DB, schema version, artifact directory, daemon health.

use crate::client::ApiClient;

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = match client.doctor().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Could not reach veld: {}", e);
            eprintln!(
                "Hint: Start the daemon with `cargo run -p veld` (or ensure VEL_BASE_URL is set)"
            );
            std::process::exit(2);
        }
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("doctor response missing data");
    for check in &data.checks {
        let status_str = match check.status {
            vel_api_types::DiagnosticStatus::Ok => "ok",
            vel_api_types::DiagnosticStatus::Warn => "warn",
            vel_api_types::DiagnosticStatus::Fail => "fail",
        };
        println!("{}: {} — {}", check.name, status_str, check.message);
    }
    println!("schema_version: {}", data.schema_version);
    println!("version: {}", data.version);

    let has_fail = data
        .checks
        .iter()
        .any(|c| matches!(c.status, vel_api_types::DiagnosticStatus::Fail));
    if has_fail {
        std::process::exit(1);
    }
    Ok(())
}
