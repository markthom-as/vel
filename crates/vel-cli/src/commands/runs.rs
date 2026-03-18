//! `vel runs` and `vel run inspect <id>` — list and inspect runtime runs.

use crate::client::ApiClient;
use vel_api_types::RunUpdateRequest;

fn parse_retry_at(retry_at: Option<&str>) -> anyhow::Result<Option<time::OffsetDateTime>> {
    retry_at
        .map(|value| {
            time::OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
        })
        .transpose()
        .map_err(|e| anyhow::anyhow!("invalid --retry-at value (expected RFC3339): {}", e))
}

fn format_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub async fn run_list(
    client: &ApiClient,
    kind: Option<&str>,
    today: bool,
    limit: u32,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.list_runs(Some(limit), kind, today).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let runs = response.data.expect("list_runs response missing data");
    if runs.is_empty() {
        println!("No runs yet.");
        return Ok(());
    }
    println!(
        "{:<14} {:<22} {:<16} {:<26} NEXT ACTION",
        "RUN ID", "KIND", "STATUS", "CREATED AT"
    );
    for r in runs {
        let created = r.created_at.to_string();
        let next_action = if let Some(t) = r.retry_scheduled_at.as_ref() {
            if r.unsupported_retry_override {
                format!("retry @ {} (override)", t)
            } else {
                format!("retry @ {}", t)
            }
        } else if let Some(reason) = r.blocked_reason.as_deref() {
            format!("blocked: {}", reason)
        } else if !r.automatic_retry_supported {
            r.automatic_retry_reason
                .clone()
                .unwrap_or_else(|| "manual retry only".to_string())
        } else {
            r.finished_at
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "—".to_string())
        };
        println!(
            "{:<14} {:<22} {:<16} {:<26} {}",
            r.id, r.kind, r.status, created, next_action
        );
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let response = client.get_run(id).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let r = response.data.expect("get_run response missing data");
    println!("Run: {}", r.id);
    println!("Kind: {}", r.kind);
    println!("Status: {}", r.status);
    println!(
        "Automatic retry: {}",
        if r.automatic_retry_supported {
            "supported"
        } else {
            "unsupported"
        }
    );
    if let Some(reason) = &r.automatic_retry_reason {
        println!("Automatic retry policy: {}", reason);
    }
    if r.unsupported_retry_override {
        println!("Unsupported retry override: true");
        if let Some(reason) = &r.unsupported_retry_override_reason {
            println!("Unsupported retry override reason: {}", reason);
        }
    }
    if let Some(t) = &r.started_at {
        println!("Started: {}", t);
    }
    if let Some(t) = &r.finished_at {
        println!("Finished: {}", t);
    }
    if let Some(ms) = r.duration_ms {
        println!("Duration: {}ms", ms);
    }
    if let Some(t) = &r.retry_scheduled_at {
        println!("Retry scheduled at: {}", t);
    }
    if let Some(reason) = &r.retry_reason {
        println!("Retry reason: {}", reason);
    }
    if let Some(reason) = &r.blocked_reason {
        println!("Blocked reason: {}", reason);
    }
    println!(
        "\nInput:\n  {}",
        serde_json::to_string_pretty(&r.input).unwrap_or_else(|_| r.input.to_string())
    );
    if let Some(ref out) = r.output {
        println!(
            "\nOutput:\n  {}",
            serde_json::to_string_pretty(out).unwrap_or_else(|_| out.to_string())
        );
    }
    if let Some(ref err) = r.error {
        println!(
            "\nError:\n  {}",
            serde_json::to_string_pretty(err).unwrap_or_else(|_| err.to_string())
        );
    }
    println!("\nEvents:");
    for e in &r.events {
        let t = e.created_at.time();
        let time_str = format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second());
        println!("  {} {}", time_str, e.event_type);
    }
    if !r.artifacts.is_empty() {
        println!("\nArtifacts:");
        for a in &r.artifacts {
            let size_str = a
                .size_bytes
                .map(format_size)
                .unwrap_or_else(|| "—".to_string());
            println!("  {}  {}  {}", a.artifact_id, a.artifact_type, size_str);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn run_status(
    client: &ApiClient,
    id: &str,
    status: &str,
    retry_after_seconds: Option<u32>,
    retry_at: Option<&str>,
    reason: Option<&str>,
    allow_unsupported_retry: bool,
    blocked_reason: Option<&str>,
) -> anyhow::Result<()> {
    let retry_at = parse_retry_at(retry_at)?;
    let body = RunUpdateRequest {
        status: status.to_string(),
        retry_at,
        retry_after_seconds,
        reason: reason.map(ToString::to_string),
        allow_unsupported_retry,
        blocked_reason: blocked_reason.map(ToString::to_string),
    };
    let response = client.update_run(id, &body).await?;
    let r = response
        .data
        .expect("update_run_status response missing data");
    println!("Run {} status -> {}", r.id, r.status);
    if let Some(t) = r.retry_scheduled_at {
        println!("Retry scheduled at {}", t);
    }
    if r.unsupported_retry_override {
        println!("Unsupported retry override active");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_retry_at;

    #[test]
    fn parse_retry_at_accepts_valid_rfc3339() {
        let parsed = parse_retry_at(Some("2026-03-16T22:10:00Z"))
            .expect("valid RFC3339 should parse")
            .expect("value should be present");
        assert_eq!(
            parsed,
            time::OffsetDateTime::parse(
                "2026-03-16T22:10:00Z",
                &time::format_description::well_known::Rfc3339
            )
            .unwrap()
        );
    }

    #[test]
    fn parse_retry_at_rejects_invalid_timestamp() {
        let err = parse_retry_at(Some("not-a-timestamp")).expect_err("invalid timestamp must fail");
        let msg = err.to_string();
        assert!(msg.contains("invalid --retry-at value (expected RFC3339)"));
    }

    #[test]
    fn parse_retry_at_none_is_ok() {
        let parsed = parse_retry_at(None).expect("None should be accepted");
        assert!(parsed.is_none());
    }
}
