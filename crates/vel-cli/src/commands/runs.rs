//! `vel runs` and `vel run inspect <id>` — list and inspect runtime runs.

use crate::client::ApiClient;

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.list_runs().await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let runs = response.data.expect("list_runs response missing data");
    if runs.is_empty() {
        println!("No runs yet.");
        return Ok(());
    }
    println!("{:<14} {:<22} {:<12} {:<26} {}", "RUN ID", "KIND", "STATUS", "CREATED AT", "FINISHED AT");
    for r in runs {
        let created = r.created_at.to_string();
        let finished = r.finished_at.as_ref().map(|t| t.to_string()).unwrap_or_else(|| "—".to_string());
        println!("{:<14} {:<22} {:<12} {:<26} {}", r.id, r.kind, r.status, created, finished);
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let response = client.get_run(id).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let r = response.data.expect("get_run response missing data");
    println!("Run: {}", r.id);
    println!("Kind: {}", r.kind);
    println!("Status: {}", r.status);
    println!("Created: {}", r.created_at);
    if let Some(t) = &r.started_at {
        println!("Started: {}", t);
    }
    if let Some(t) = &r.finished_at {
        println!("Finished: {}", t);
    }
    println!("\nInput:\n  {}", r.input.replace('\n', "\n  "));
    if let Some(ref out) = r.output {
        println!("\nOutput:\n  {}", out.replace('\n', "\n  "));
    }
    if let Some(ref err) = r.error {
        println!("\nError:\n  {}", err.replace('\n', "\n  "));
    }
    println!("\nEvents:");
    for e in &r.events {
        println!("  [{}] {} {}", e.seq, e.event_type, e.created_at);
    }
    Ok(())
}
