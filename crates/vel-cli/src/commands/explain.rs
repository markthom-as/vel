//! vel explain — show why a nudge was generated, what shaped context, commitment risk, or drift.

use anyhow::Context;
use crate::client::ApiClient;

pub async fn run_context(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.get_explain_context().await.context("get explain context")?;
    let data = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("Computed at: {}", data.computed_at);
    if let Some(ref m) = data.mode {
        println!("Mode: {}", m);
    }
    if let Some(ref s) = data.morning_state {
        println!("Morning state: {}", s);
    }
    if !data.signals_used.is_empty() {
        println!("Signals used: {}", data.signals_used.join(", "));
    }
    if !data.commitments_used.is_empty() {
        println!("Commitments used: {}", data.commitments_used.join(", "));
    }
    if !data.risk_used.is_empty() {
        println!("Risk used: {}", data.risk_used.join(", "));
    }
    println!("\nReasons:");
    for r in &data.reasons {
        println!("  - {}", r);
    }
    Ok(())
}

pub async fn run_nudge(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.get_explain_nudge(id).await.context("explain nudge")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("nudge_id:    {}", d.nudge_id);
    println!("nudge_type: {}", d.nudge_type);
    println!("level:       {}", d.level);
    println!("state:       {}", d.state);
    println!("message:     {}", d.message);
    if let Some(ref inf) = d.inference_snapshot {
        println!("inference:   {}", serde_json::to_string_pretty(inf)?);
    }
    if let Some(ref sig) = d.signals_snapshot {
        println!("signals:     {}", serde_json::to_string_pretty(sig)?);
    }
    Ok(())
}

pub async fn run_commitment(client: &ApiClient, commitment_id: &str) -> anyhow::Result<()> {
    let resp = client.get_explain_commitment(commitment_id).await.context("explain commitment")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("commitment_id: {}", d.commitment_id);
    println!("commitment:    {}", serde_json::to_string_pretty(&d.commitment)?);
    if let Some(ref r) = d.risk {
        println!("risk:         {}", serde_json::to_string_pretty(r)?);
    } else {
        println!("risk:         (none — run `vel evaluate` to compute)");
    }
    println!("in_context_reasons:");
    for r in &d.in_context_reasons {
        println!("  - {}", r);
    }
    Ok(())
}

pub async fn run_drift(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.get_explain_drift().await.context("explain drift")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("attention_state:  {:?}", d.attention_state);
    println!("drift_type:       {:?}", d.drift_type);
    println!("drift_severity:   {:?}", d.drift_severity);
    println!("confidence:       {:?}", d.confidence);
    if !d.reasons.is_empty() {
        println!("reasons:");
        for r in &d.reasons {
            println!("  - {}", r);
        }
    }
    if !d.signals_used.is_empty() {
        println!("signals_used:     {}", d.signals_used.join(", "));
    }
    if !d.commitments_used.is_empty() {
        println!("commitments_used: {}", d.commitments_used.join(", "));
    }
    Ok(())
}
