//! `vel agents specs|spawn|inspect` — agent runtime helpers.

use crate::client::ApiClient;
use anyhow::Context;
use serde_json::Value;

pub async fn run_specs(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client
        .list_agent_specs()
        .await
        .context("list agent specs")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let specs = response.data.ok_or_else(|| anyhow::anyhow!("no data"))?;
    if specs.is_empty() {
        println!("No agent specs available.");
        return Ok(());
    }

    println!(
        "{:<28} {:<16} {:<10} {:<12} {}",
        "SPEC ID", "MISSION", "KIND", "TTL", "STATUS"
    );
    for spec in specs {
        println!(
            "{:<28} {:<16} {:<10} {:<12} {}",
            spec.id,
            spec.mission.as_deref().unwrap_or("-"),
            spec.kind.as_deref().unwrap_or("-"),
            spec.ttl_seconds
                .map(|ttl| format!("{}s", ttl))
                .unwrap_or_else(|| "-".to_string()),
            spec.status.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

pub async fn run_spawn(
    client: &ApiClient,
    spec_id: &str,
    payload: Option<&str>,
) -> anyhow::Result<()> {
    let input: Value = match payload {
        Some(raw) => serde_json::from_str(raw).context("parse --payload as JSON")?,
        None => Value::Object(serde_json::Map::new()),
    };

    let response = client
        .spawn_agent(spec_id, input)
        .await
        .context("spawn agent run")?;
    let run = response.data.ok_or_else(|| anyhow::anyhow!("no data"))?;
    let run_id = run.id.as_deref().or(run.run_id.as_deref()).unwrap_or("-");
    let status = run.status.as_deref().unwrap_or("-");
    println!("Spawned agent run {} from spec {}", run_id, spec_id);
    println!("status: {}", status);
    if let Some(summary) = &run.summary {
        println!("summary: {}", summary);
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let response = client.get_agent_run(id).await.context("get agent run")?;
    let run = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;

    if json {
        println!("{}", serde_json::to_string_pretty(run)?);
        return Ok(());
    }

    let run_id = run.id.as_deref().or(run.run_id.as_deref()).unwrap_or(id);
    println!("run_id:      {}", run_id);
    println!("spec_id:     {}", run.spec_id.as_deref().unwrap_or("-"));
    println!("status:      {}", run.status.as_deref().unwrap_or("-"));
    println!("created_at:  {}", run.created_at.as_deref().unwrap_or("-"));
    println!("summary:     {}", run.summary.as_deref().unwrap_or("-"));
    println!(
        "confidence:  {}",
        run.confidence
            .map(|v| v.to_string())
            .unwrap_or("-".to_string())
    );
    if let Some(summary) = &run.summary_json {
        println!(
            "summary_json:\n{}",
            serde_json::to_string_pretty(summary).context("render summary_json")?
        );
    }
    if let Some(output) = &run.output {
        println!(
            "output:\n{}",
            serde_json::to_string_pretty(output).context("render output")?
        );
    }
    Ok(())
}
