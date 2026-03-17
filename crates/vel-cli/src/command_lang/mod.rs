pub mod ast;
pub mod completion;
pub mod explain;
pub mod infer;
pub mod parse;
pub mod preview;
pub mod registry;
pub mod tokenize;

use crate::client::ApiClient;
use anyhow::bail;
use serde_json::json;
use vel_api_types::CommandExecutionPayloadData;

use ast::{PhraseFamily, Verb};
use infer::parse_and_resolve;

pub async fn run(
    client: &ApiClient,
    input: Vec<String>,
    dry_run: bool,
    json_output: bool,
) -> anyhow::Result<()> {
    let resolution = parse_and_resolve(&input)?;

    if dry_run {
        if json_output {
            let service_plan = client
                .plan_command(&resolution.resolved)
                .await
                .ok()
                .and_then(|response| response.data);
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "mode": "dry_run",
                    "parsed": resolution.parsed,
                    "resolved_command": resolution.resolved,
                    "preview": preview::render(&resolution),
                    "explanation": explain::render_explanation(&resolution),
                    "completion_hints": completion::next_tokens(&input),
                    "registry": registry::default_registry(),
                    "service_plan": service_plan,
                }))?
            );
        } else {
            println!("{}", preview::render(&resolution));
            println!();
            println!("{}", explain::render_explanation(&resolution));
            if let Ok(response) = client.plan_command(&resolution.resolved).await {
                if let Some(plan) = response.data {
                    println!();
                    println!("Service plan:");
                    println!("  Mode: {:?}", plan.mode);
                    println!("  Summary: {}", plan.summary);
                    for step in plan.steps {
                        println!("  - {}: {}", step.title, step.detail);
                    }
                    if !plan.validation.issues.is_empty() {
                        println!("  Validation issues:");
                        for issue in plan.validation.issues {
                            println!("    - {:?}: {}", issue.code, issue.message);
                        }
                    }
                }
            }
        }
        return Ok(());
    }

    match (&resolution.parsed.family, &resolution.parsed.verb) {
        (PhraseFamily::Should, Verb::Capture)
        | (PhraseFamily::Should, Verb::Commit)
        | (PhraseFamily::Should, Verb::Feature)
        | (PhraseFamily::Should, Verb::Plan)
        | (PhraseFamily::Should, Verb::Review) => {
            execute_via_service(client, &resolution.resolved, json_output).await
        }
        (PhraseFamily::Should, Verb::Spec) => {
            execute_via_service(client, &resolution.resolved, json_output).await
        }
        _ => bail!(
            "command language execution is not implemented yet for `{}`",
            resolution.parsed.verb
        ),
    }
}

async fn execute_via_service(
    client: &ApiClient,
    command: &vel_core::ResolvedCommand,
    json_output: bool,
) -> anyhow::Result<()> {
    let response = client.execute_command(command).await?;
    let data = response
        .data
        .expect("command execute response missing data");

    if json_output {
        println!("{}", serde_json::to_string_pretty(&data)?);
    } else {
        match &data.result {
            CommandExecutionPayloadData::CaptureCreated(result) => {
                println!("result_kind: capture_created");
                println!("capture_id: {}", result.capture_id);
            }
            CommandExecutionPayloadData::CommitmentCreated(result) => {
                println!("result_kind: commitment_created");
                println!("commitment_id: {}", result.id);
                println!("text: {}", result.text);
            }
            CommandExecutionPayloadData::ArtifactCreated(result) => {
                println!("result_kind: artifact_created");
                println!("artifact_id: {}", result.artifact_id);
                println!("artifact_type: {}", result.artifact_type);
            }
            CommandExecutionPayloadData::ReviewToday(result) => {
                println!("result_kind: review_today");
                println!("capture_count: {}", result.capture_count);
                println!("{}", serde_json::to_string_pretty(result)?);
            }
            CommandExecutionPayloadData::ReviewWeek(result) => {
                println!("result_kind: review_week");
                println!("capture_count: {}", result.capture_count);
                println!("{}", serde_json::to_string_pretty(result)?);
            }
        }
        if !data.warnings.is_empty() {
            println!("warnings:");
            for warning in &data.warnings {
                println!("- {}", warning);
            }
        }
    }

    Ok(())
}
