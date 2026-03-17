pub mod ast;
pub mod completion;
pub mod explain;
pub mod infer;
pub mod parse;
pub mod preview;
pub mod registry;
pub mod tokenize;

use crate::client::ApiClient;
use crate::commands;
use anyhow::{bail, Context};
use serde_json::json;
use vel_api_types::CaptureCreateRequest;

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
                }))?
            );
        } else {
            println!("{}", preview::render(&resolution));
            println!();
            println!("{}", explain::render_explanation(&resolution));
        }
        return Ok(());
    }

    match (&resolution.parsed.family, &resolution.parsed.verb) {
        (PhraseFamily::Should, Verb::Capture) => {
            execute_capture(
                client,
                resolution
                    .parsed
                    .joined_target()
                    .context("capture command missing target text")?,
                "quick_note",
                json_output,
            )
            .await
        }
        (PhraseFamily::Should, Verb::Feature) => {
            execute_capture(
                client,
                resolution
                    .parsed
                    .joined_target()
                    .context("feature command missing target text")?,
                "feature_request",
                json_output,
            )
            .await
        }
        (PhraseFamily::Should, Verb::Review) => match resolution.parsed.primary_target() {
            Some("today") => commands::review::run_today(client, json_output).await,
            Some("week") => commands::review::run_week(client, json_output).await,
            Some(other) => bail!("unsupported review target: {other}"),
            None => bail!("review command requires a target like `today` or `week`"),
        },
        _ => bail!(
            "command language execution is not implemented yet for `{}`",
            resolution.parsed.verb
        ),
    }
}

async fn execute_capture(
    client: &ApiClient,
    content_text: String,
    capture_type: &str,
    json_output: bool,
) -> anyhow::Result<()> {
    let request = CaptureCreateRequest {
        content_text,
        capture_type: capture_type.to_string(),
        source_device: Some("vel-command".to_string()),
    };
    let response = client.capture(request).await?;
    let data = response.data.expect("capture response missing data");

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "capture_id": data.capture_id,
                "capture_type": capture_type,
                "source_device": "vel-command",
            }))?
        );
    } else {
        println!("capture_id: {}", data.capture_id);
    }

    Ok(())
}
