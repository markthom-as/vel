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
        let title = s.title.as_deref().unwrap_or("-");
        let confidence = s.confidence.as_deref().unwrap_or("-");
        println!(
            "{}  {}  {}  p{}  conf={}  evidence={}  {}",
            s.id,
            s.suggestion_type,
            s.state,
            s.priority,
            confidence,
            s.evidence_count,
            title
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
    println!("title:           {}", s.title.as_deref().unwrap_or("-"));
    println!("summary:         {}", s.summary.as_deref().unwrap_or("-"));
    println!("priority:        {}", s.priority);
    println!("confidence:      {}", s.confidence.as_deref().unwrap_or("-"));
    println!("evidence_count:  {}", s.evidence_count);
    println!(
        "decision:        {}",
        s.decision_context_summary.as_deref().unwrap_or("-")
    );
    if let Some(context) = &s.decision_context {
        println!(
            "decision_json:   {}",
            serde_json::to_string_pretty(context)?
        );
    }
    println!("created_at:      {}", s.created_at);
    println!("resolved_at:     {:?}", s.resolved_at);
    println!(
        "payload:         {}",
        serde_json::to_string_pretty(&s.payload)?
    );
    if let Some(evidence) = &s.evidence {
        if evidence.is_empty() {
            println!("evidence:        []");
        } else {
            println!("evidence:");
            for item in evidence {
                println!(
                    "  - {}  {}  weight={:?}",
                    item.evidence_type, item.ref_id, item.weight
                );
                if let Some(details) = &item.evidence {
                    println!("    {}", serde_json::to_string_pretty(details)?);
                }
            }
        }
    }
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
