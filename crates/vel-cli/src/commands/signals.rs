//! vel signals — list and create signals.

use crate::client::ApiClient;
use anyhow::Context;
use vel_api_types::SignalCreateRequest;

pub async fn run_list(
    client: &ApiClient,
    signal_type: Option<&str>,
    since_ts: Option<i64>,
    limit: u32,
    json: bool,
) -> anyhow::Result<()> {
    let resp = client
        .list_signals(signal_type, since_ts, limit)
        .await
        .context("list signals")?;
    let signals = resp.data.ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&signals)?);
        return Ok(());
    }
    if signals.is_empty() {
        println!("No signals.");
        return Ok(());
    }
    for s in &signals {
        println!(
            "{}  {}  src={}  ts={}",
            s.signal_id, s.signal_type, s.source, s.timestamp
        );
    }
    Ok(())
}

pub async fn run_create(
    client: &ApiClient,
    signal_type: &str,
    source: &str,
    source_ref: Option<&str>,
    payload: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let payload_value = payload
        .map(|s| serde_json::from_str(s).context("parse --payload JSON"))
        .transpose()?
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let req = SignalCreateRequest {
        signal_type: signal_type.to_string(),
        source: source.to_string(),
        source_ref: source_ref.map(str::to_string),
        timestamp: None,
        payload: payload_value,
    };
    let resp = client.create_signal(req).await.context("create signal")?;
    let s = resp.data.ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&s)?);
        return Ok(());
    }
    println!(
        "{}  {}  src={}  ts={}",
        s.signal_id, s.signal_type, s.source, s.timestamp
    );
    Ok(())
}
