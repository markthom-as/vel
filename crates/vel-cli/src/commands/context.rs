//! vel context — show persistent current context (written by inference engine).
//! Read-only CLI surface: uses GET /v1/context/current and GET /v1/context/timeline only.
//! Recompute happens via `vel evaluate`, not here.

use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_current(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_current_context()
        .await
        .context("get current context")?;
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    match data {
        None => println!("No current context (run 'vel evaluate' first)."),
        Some(ctx) => {
            if json {
                println!("{}", serde_json::to_string_pretty(ctx)?);
            } else {
                let c = &ctx.context;
                println!(
                    "Mode: {}",
                    c.get("mode")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("—")
                );
                println!(
                    "Morning state: {}",
                    c.get("morning_state")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("—")
                );
                if let Some(id) = c
                    .get("next_commitment_id")
                    .and_then(serde_json::Value::as_str)
                    .filter(|s| !s.is_empty())
                {
                    println!("Next commitment: {}", id);
                }
                println!(
                    "Prep window: {}",
                    c.get("prep_window_active")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                );
                println!(
                    "Commute window: {}",
                    c.get("commute_window_active")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                );
                println!(
                    "Meds: {}",
                    c.get("meds_status")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("—")
                );
                println!(
                    "Global risk: {}",
                    c.get("global_risk_level")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("—")
                );
                if let Some(git) = c.get("git_activity_summary") {
                    let repo = git
                        .get("repo")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown");
                    let branch = git
                        .get("branch")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("—");
                    let operation = git
                        .get("operation")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("activity");
                    if let Some(message) = git
                        .get("message")
                        .and_then(serde_json::Value::as_str)
                        .filter(|value| !value.trim().is_empty())
                    {
                        println!("Git: {}  {}  {}  {}", repo, branch, operation, message);
                    } else {
                        println!("Git: {}  {}  {}", repo, branch, operation);
                    }
                }
                if let Some(arr) = c
                    .get("active_nudge_ids")
                    .and_then(serde_json::Value::as_array)
                {
                    println!("Active nudges: {}", arr.len());
                }
            }
        }
    }
    Ok(())
}

pub async fn run_timeline(client: &ApiClient, limit: u32, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_context_timeline(limit)
        .await
        .context("get context timeline")?;
    let entries = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if entries.is_empty() {
        println!("No context timeline entries (run 'vel evaluate' to recompute context).");
        return Ok(());
    }
    if json {
        println!("{}", serde_json::to_string_pretty(entries)?);
    } else {
        for e in entries {
            let ts = e.timestamp;
            let c = &e.context;
            let morning = c
                .get("morning_state")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("—");
            let mode = c
                .get("mode")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("—");
            let prep = c
                .get("prep_window_active")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let meds = c
                .get("meds_status")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("—");
            let risk = c
                .get("global_risk_level")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("—");
            println!("{} morning_state -> {}  mode -> {}  prep_window_active -> {}  meds_status -> {}  global_risk_level -> {}", ts, morning, mode, prep, meds, risk);
        }
    }
    Ok(())
}
