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
        | (PhraseFamily::Should, Verb::Feature)
        | (PhraseFamily::Should, Verb::Review) => {
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
        println!("result_kind: {}", data.result_kind);
        if let Some(capture_id) = data
            .payload
            .get("capture_id")
            .and_then(serde_json::Value::as_str)
        {
            println!("capture_id: {}", capture_id);
        } else {
            println!("{}", serde_json::to_string_pretty(&data.payload)?);
        }
    }

    Ok(())
}
