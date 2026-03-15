//! vel explain — show why a nudge was generated or what shaped current context.

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
