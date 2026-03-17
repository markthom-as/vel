//! vel risk — list risk for all commitments or inspect one. See vel-risk-engine-spec.md.

use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.get_risk_list().await.context("get risk list")?;
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }
    if data.is_empty() {
        println!("No risk data (no open commitments or run 'vel evaluate' first).");
        return Ok(());
    }
    for r in data {
        println!(
            "{}  {}  {}  {}",
            r.commitment_id,
            r.risk_level,
            r.risk_score,
            r.factors.reasons.len()
        );
    }
    Ok(())
}

pub async fn run_commitment(
    client: &ApiClient,
    commitment_id: &str,
    json: bool,
) -> anyhow::Result<()> {
    let resp = client
        .get_risk_commitment(commitment_id)
        .await
        .context("get commitment risk")?;
    let r = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(r)?);
        return Ok(());
    }
    println!("commitment_id: {}", r.commitment_id);
    println!("risk_score:    {}", r.risk_score);
    println!("risk_level:   {}", r.risk_level);
    if !r.factors.reasons.is_empty() {
        println!("reasons:");
        for reason in &r.factors.reasons {
            println!("  - {}", reason);
        }
    }
    println!("factors: {}", serde_json::to_string_pretty(&r.factors)?);
    Ok(())
}
