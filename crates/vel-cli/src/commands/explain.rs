//! vel explain — show why a nudge was generated, what shaped context, commitment risk, or drift.
//! Read-only CLI surface: uses GET explain endpoints only and never recomputes persisted state.

use crate::client::ApiClient;
use crate::command_lang;
use anyhow::Context;

pub async fn run_context(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_explain_context()
        .await
        .context("get explain context")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
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

pub async fn run_nudge(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_explain_nudge(id)
        .await
        .context("explain nudge")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
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

pub async fn run_commitment(
    client: &ApiClient,
    commitment_id: &str,
    json: bool,
) -> anyhow::Result<()> {
    let resp = client
        .get_explain_commitment(commitment_id)
        .await
        .context("explain commitment")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("commitment_id: {}", d.commitment_id);
    println!(
        "commitment:    {}",
        serde_json::to_string_pretty(&d.commitment)?
    );
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

pub async fn run_drift(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.get_explain_drift().await.context("explain drift")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
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

pub async fn run_command(client: &ApiClient, input: Vec<String>, json: bool) -> anyhow::Result<()> {
    let resolution = command_lang::infer::parse_and_resolve(&input).context("parse command")?;
    let local_explanation = command_lang::explain::render_explanation(&resolution);
    let local_preview = command_lang::preview::render(&resolution);
    let completion_hints = command_lang::completion::next_tokens(&input);
    let intent_hints = command_lang::completion::intent_hints(&resolution);
    let daemon_plan = client
        .plan_command(&resolution.resolved)
        .await
        .ok()
        .and_then(|response| response.data);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "input": input,
                "parsed": resolution.parsed,
                "resolved_command": resolution.resolved,
                "local_preview": local_preview,
                "local_explanation": local_explanation,
                "completion_hints": completion_hints,
                "intent_hints": intent_hints,
                "daemon_plan": daemon_plan,
            }))?
        );
        return Ok(());
    }

    println!("Local explanation:");
    println!("{}", local_explanation);
    println!();
    println!("Local preview:");
    println!("{}", local_preview);
    if !completion_hints.is_empty() {
        println!();
        println!("Next tokens: {}", completion_hints.join(", "));
    }
    if let Some(hints) = intent_hints {
        println!("Intent hints:");
        println!("  target_kind: {}", hints.target_kind);
        println!("  mode: {}", hints.mode);
        println!("  suggestions: {}", hints.suggestions.join(", "));
    }

    println!();
    match daemon_plan {
        Some(plan) => {
            println!("Daemon plan:");
            println!("  mode: {:?}", plan.mode);
            println!("  summary: {}", plan.summary);
            if let Some(hints) = plan.intent_hints {
                println!("  intent hints:");
                println!("    target_kind: {}", hints.target_kind);
                println!("    mode: {}", hints.mode);
                println!("    suggestions: {}", hints.suggestions.join(", "));
            }
            if let Some(hints) = plan.delegation_hints {
                println!("  delegation hints:");
                println!("    worker_roles: {}", hints.worker_roles.join(", "));
                println!("    coordination: {}", hints.coordination);
                println!("    approval_required: {}", hints.approval_required);
                println!(
                    "    linked_record_strategy: {}",
                    hints.linked_record_strategy
                );
            }
            if !plan.planned_records.is_empty() {
                println!("  planned records:");
                for record in plan.planned_records {
                    println!("    - {}: {}", record.record_type, record.title);
                    for link in record.links {
                        println!(
                            "      link -> {} ({})",
                            link.entity_type, link.relation_type
                        );
                    }
                }
            }
            if !plan.steps.is_empty() {
                println!("  steps:");
                for step in plan.steps {
                    println!("    - {}: {}", step.title, step.detail);
                }
            }
            if !plan.validation.issues.is_empty() {
                println!("  validation issues:");
                for issue in plan.validation.issues {
                    println!("    - {:?}: {}", issue.code, issue.message);
                }
            }
        }
        None => {
            println!("Daemon plan: unavailable (using local explanation only)");
        }
    }

    Ok(())
}
