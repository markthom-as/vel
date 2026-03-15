//! `vel synthesize` — run-backed synthesis (week, project).

use anyhow::Context;
use crate::client::ApiClient;

pub async fn run_week(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.synthesis_week().await.context("synthesis week")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(d)?);
    } else {
        println!("run_id: {}  artifact_id: {}", d.run_id, d.artifact_id);
    }
    Ok(())
}

pub async fn run_project(client: &ApiClient, name: &str, json: bool) -> anyhow::Result<()> {
    let resp = client.synthesis_project(name).await.context("synthesis project")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(d)?);
    } else {
        println!("run_id: {}  artifact_id: {}", d.run_id, d.artifact_id);
    }
    Ok(())
}
