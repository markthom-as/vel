//! `vel doctor` — verify config, DB, schema version, artifact directory, daemon health.

use crate::client::ApiClient;
use time::format_description::well_known::Rfc3339;
use vel_api_types::{BackupTrustData, BackupTrustLevelData, DiagnosticStatus};

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = match client.doctor().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Could not reach veld: {}", e);
            eprintln!(
                "Hint: Start the daemon with `cargo run -p veld` (or ensure VEL_BASE_URL is set)"
            );
            std::process::exit(2);
        }
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response.data.expect("doctor response missing data");
    for check in &data.checks {
        let status_str = match check.status {
            DiagnosticStatus::Ok => "ok",
            DiagnosticStatus::Warn => "warn",
            DiagnosticStatus::Fail => "fail",
        };
        println!("{}: {} — {}", check.name, status_str, check.message);
    }
    println!("schema_version: {}", data.schema_version);
    println!("version: {}", data.version);
    for line in backup_summary_lines(&data.backup) {
        println!("{line}");
    }

    let has_fail = data
        .checks
        .iter()
        .any(|c| matches!(c.status, DiagnosticStatus::Fail));
    if has_fail {
        std::process::exit(1);
    }
    Ok(())
}

pub(crate) fn backup_summary_lines(backup: &BackupTrustData) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "backup_trust: {}",
        match backup.level {
            BackupTrustLevelData::Ok => "ok",
            BackupTrustLevelData::Warn => "warn",
            BackupTrustLevelData::Fail => "fail",
        }
    ));

    if let Some(last_backup_at) = backup.status.last_backup_at {
        let rendered = last_backup_at
            .format(&Rfc3339)
            .unwrap_or_else(|_| last_backup_at.to_string());
        lines.push(format!("backup_last_success: {rendered}"));
    } else {
        lines.push("backup_last_success: none".to_string());
    }

    if let Some(output_root) = backup.status.output_root.as_deref() {
        lines.push(format!("backup_root: {output_root}"));
    }

    if let Some(coverage) = backup.status.artifact_coverage.as_ref() {
        lines.push(format!(
            "backup_artifacts: included={} omitted={}",
            coverage.included.len(),
            coverage.omitted.len()
        ));
    }

    if let Some(coverage) = backup.status.config_coverage.as_ref() {
        lines.push(format!(
            "backup_config: included={} omitted={}",
            coverage.included.len(),
            coverage.omitted.len()
        ));
    }

    for warning in &backup.status.warnings {
        lines.push(format!("backup_warning: {warning}"));
    }
    for guidance in &backup.guidance {
        lines.push(format!("backup_guidance: {guidance}"));
    }

    lines
}
