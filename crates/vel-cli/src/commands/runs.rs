//! `vel runs` and `vel run inspect <id>` — list and inspect runtime runs.

use crate::client::ApiClient;

fn format_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub async fn run_list(
    client: &ApiClient,
    kind: Option<&str>,
    today: bool,
    limit: u32,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.list_runs(Some(limit), kind, today).await?;
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
    if let Some(t) = &r.started_at {
        println!("Started: {}", t);
    }
    if let Some(t) = &r.finished_at {
        println!("Finished: {}", t);
    }
    if let Some(ms) = r.duration_ms {
        println!("Duration: {}ms", ms);
    }
    println!("\nInput:\n  {}", serde_json::to_string_pretty(&r.input).unwrap_or_else(|_| r.input.to_string()));
    if let Some(ref out) = r.output {
        println!("\nOutput:\n  {}", serde_json::to_string_pretty(out).unwrap_or_else(|_| out.to_string()));
    }
    if let Some(ref err) = r.error {
        println!("\nError:\n  {}", serde_json::to_string_pretty(err).unwrap_or_else(|_| err.to_string()));
    }
    println!("\nEvents:");
    for e in &r.events {
        let t = e.created_at.time();
        let time_str = format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second());
        println!("  {} {}", time_str, e.event_type);
    }
    if !r.artifacts.is_empty() {
        println!("\nArtifacts:");
        for a in &r.artifacts {
            let size_str = a
                .size_bytes
                .map(|b| format_size(b))
                .unwrap_or_else(|| "—".to_string());
            println!("  {}  {}  {}", a.artifact_id, a.artifact_type, size_str);
        }
    }
    Ok(())
}

pub async fn run_status(client: &ApiClient, id: &str, status: &str) -> anyhow::Result<()> {
    let response = client.update_run_status(id, status).await?;
    let r = response.data.expect("update_run_status response missing data");
    println!("Run {} status -> {}", r.id, r.status);
    Ok(())
}
