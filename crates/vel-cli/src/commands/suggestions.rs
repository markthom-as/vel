//! vel suggestions / vel suggestion — list and manage steering suggestions.

use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list(client: &ApiClient, state: Option<&str>, json: bool) -> anyhow::Result<()> {
    let resp = client
        .list_suggestions(state, Some(50))
        .await
        .context("list suggestions")?;
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }
    if data.is_empty() {
        println!("No suggestions.");
        return Ok(());
    }
    for s in data {
        println!(
            "{}  {}  {}  {}",
            s.id, s.suggestion_type, s.state, s.created_at
        );
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.get_suggestion(id).await.context("get suggestion")?;
    let s = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("id:              {}", s.id);
    println!("suggestion_type: {}", s.suggestion_type);
    println!("state:           {}", s.state);
    println!("created_at:      {}", s.created_at);
    println!("resolved_at:     {:?}", s.resolved_at);
    println!(
        "payload:         {}",
        serde_json::to_string_pretty(&s.payload)?
    );
    Ok(())
}

pub async fn run_accept(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client
        .update_suggestion(id, "accepted", None)
        .await
        .context("accept suggestion")?;
    let s = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("Accepted suggestion {} (state: {})", s.id, s.state);
    Ok(())
}

pub async fn run_reject(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client
        .update_suggestion(id, "rejected", None)
        .await
        .context("reject suggestion")?;
    let s = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("Rejected suggestion {} (state: {})", s.id, s.state);
    Ok(())
}

pub async fn run_modify(client: &ApiClient, id: &str, payload: Option<&str>) -> anyhow::Result<()> {
    let payload_value = payload
        .map(|s| serde_json::from_str(s).context("parse --payload JSON"))
        .transpose()?;
    let resp = client
        .update_suggestion(id, "modified", payload_value)
        .await
        .context("modify suggestion")?;
    let s = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("Modified suggestion {} (state: {})", s.id, s.state);
    Ok(())
}
