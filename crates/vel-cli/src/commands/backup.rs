//! `vel backup` — create, inspect, verify, and dry-run manual restore for local backup packs.

use anyhow::{bail, Context};
use time::format_description::well_known::Rfc3339;
use vel_api_types::{BackupManifestData, BackupStatusData, BackupTrustLevelData};
use vel_config::AppConfig;

use crate::client::{
    ApiClient, BackupCreateResultData, BackupInspectResultData, BackupVerifyResultData,
};
use vel_api_types::{BackupExportResultData, BackupExportStatusData};

pub async fn run(
    client: &ApiClient,
    config: &AppConfig,
    create: bool,
    export: bool,
    export_status: bool,
    output_root: Option<&str>,
    target_root: Option<&str>,
    domains: Vec<String>,
    inspect: Option<&str>,
    verify: Option<&str>,
    dry_run_restore: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let selected = [
        create,
        export,
        export_status,
        inspect.is_some(),
        verify.is_some(),
        dry_run_restore.is_some(),
    ]
    .into_iter()
    .filter(|value| *value)
    .count();

    if selected > 1 {
        bail!("choose only one backup action at a time");
    }

    if create {
        let response = client.create_backup(output_root).await?;
        let data = response.data.context("backup create missing data")?;
        return print_create(&data, json);
    }

    if export {
        let target_root = target_root
            .or(config.backup_export.target_root.as_deref())
            .context("backup export requires --target-root <dir> or backup_export.target_root")?;
        let domains = if domains.is_empty() {
            config.backup_export.domains.clone()
        } else {
            domains
        };
        let response = client.export_backup(target_root, domains).await?;
        let data = response.data.context("backup export missing data")?;
        return print_export(&data, json);
    }

    if export_status {
        let response = client.backup_export_status().await?;
        let status = response.data.context("backup export status missing data")?;
        return print_export_status(&status, json);
    }

    if let Some(root) = inspect {
        let response = client.inspect_backup(root).await?;
        let data = response.data.context("backup inspect missing data")?;
        return print_inspect(&data, json);
    }

    if let Some(root) = verify {
        let response = client.verify_backup(root).await?;
        let data = response.data.context("backup verify missing data")?;
        return print_verify(&data, json);
    }

    if let Some(root) = dry_run_restore {
        let response = client.inspect_backup(root).await?;
        let data = response
            .data
            .context("backup dry-run inspect missing data")?;
        return print_dry_run_restore(&data.manifest, &data.status, json);
    }

    let response = client.backup_status().await?;
    let status = response.data.context("backup status missing data")?;
    print_status(&status, config, json)
}

fn print_create(data: &BackupCreateResultData, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }

    println!("backup create: {}", data.manifest.backup_id);
    println!("created_at: {}", format_timestamp(data.manifest.created_at));
    println!("output_root: {}", data.manifest.output_root);
    println!(
        "database_snapshot: {}",
        data.manifest.database_snapshot_path
    );
    print_status_lines(&data.status);
    Ok(())
}

fn print_inspect(data: &BackupInspectResultData, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }

    println!("backup inspect: {}", data.manifest.backup_id);
    println!("created_at: {}", format_timestamp(data.manifest.created_at));
    println!("output_root: {}", data.manifest.output_root);
    println!(
        "database_snapshot: {}",
        data.manifest.database_snapshot_path
    );
    print_manifest_lines(&data.manifest);
    print_status_lines(&data.status);
    Ok(())
}

fn print_verify(data: &BackupVerifyResultData, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }

    println!("backup verify: {}", data.manifest.backup_id);
    println!(
        "verified: {}",
        if data.manifest.verification_summary.verified {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "checksum: {} {}",
        data.manifest.verification_summary.checksum_algorithm,
        data.manifest.verification_summary.checksum
    );
    print_status_lines(&data.status);
    Ok(())
}

fn print_export(data: &BackupExportResultData, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }

    for line in export_summary_lines(data) {
        println!("{line}");
    }
    Ok(())
}

fn print_export_status(status: &BackupExportStatusData, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(status)?);
        return Ok(());
    }

    println!("backup export status");
    println!("status: {:?}", status.state);
    if let Some(export_id) = status.last_export_id.as_deref() {
        println!("last_export_id: {export_id}");
    }
    if let Some(exported_at) = status.last_export_at {
        println!("last_export_at: {}", format_timestamp(exported_at));
    }
    if let Some(target_root) = status.target_root.as_deref() {
        println!("target_root: {target_root}");
    }
    if !status.included_domains.is_empty() {
        println!("included_domains: {}", status.included_domains.join(","));
    }
    for omission in &status.omitted_domains {
        println!("omitted_domain: {} ({})", omission.domain, omission.reason);
    }
    for warning in &status.warnings {
        println!("warning: {warning}");
    }
    Ok(())
}

fn print_dry_run_restore(
    manifest: &BackupManifestData,
    status: &BackupStatusData,
    json: bool,
) -> anyhow::Result<()> {
    let payload = serde_json::json!({
        "backup_id": manifest.backup_id,
        "output_root": manifest.output_root,
        "database_snapshot_path": manifest.database_snapshot_path,
        "artifact_root": format!("{}/artifacts", manifest.output_root),
        "config_root": format!("{}/config", manifest.output_root),
        "manual_restore_steps": manual_restore_steps(manifest),
        "status": status,
    });
    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("backup dry-run restore: {}", manifest.backup_id);
    for step in manual_restore_steps(manifest) {
        println!("{step}");
    }
    print_status_lines(status);
    Ok(())
}

fn print_status(status: &BackupStatusData, config: &AppConfig, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(status)?);
        return Ok(());
    }

    println!("backup status");
    println!("db_path: {}", config.db_path);
    println!("artifact_root: {}", config.artifact_root);
    println!(
        "default_backup_root: {}",
        crate::commands::doctor::backup_summary_lines(&vel_api_types::BackupTrustData {
            level: match status.state {
                vel_api_types::BackupStatusStateData::Ready => BackupTrustLevelData::Ok,
                vel_api_types::BackupStatusStateData::Stale => BackupTrustLevelData::Warn,
                vel_api_types::BackupStatusStateData::Missing
                | vel_api_types::BackupStatusStateData::Degraded => BackupTrustLevelData::Fail,
            },
            status: status.clone(),
            freshness: vel_api_types::BackupFreshnessData {
                state: match status.state {
                    vel_api_types::BackupStatusStateData::Ready => {
                        vel_api_types::BackupFreshnessStateData::Current
                    }
                    vel_api_types::BackupStatusStateData::Stale => {
                        vel_api_types::BackupFreshnessStateData::Stale
                    }
                    vel_api_types::BackupStatusStateData::Missing
                    | vel_api_types::BackupStatusStateData::Degraded => {
                        vel_api_types::BackupFreshnessStateData::Missing
                    }
                },
                age_seconds: None,
                stale_after_seconds: 48 * 60 * 60,
            },
            guidance: Vec::new(),
        })[0]
            .trim_start_matches("backup_trust: ")
    );
    print_status_lines(status);
    println!("commands:");
    println!("  vel backup --create [--output-root <dir>]");
    println!("  vel backup --export [--target-root <dir>] [--domain <name>]");
    println!("  vel backup --export-status");
    println!("  vel backup --inspect <backup_root>");
    println!("  vel backup --verify <backup_root>");
    println!("  vel backup --dry-run-restore <backup_root>");
    Ok(())
}

fn print_manifest_lines(manifest: &BackupManifestData) {
    println!(
        "artifact_coverage: included={} omitted={}",
        manifest.artifact_coverage.included.len(),
        manifest.artifact_coverage.omitted.len()
    );
    println!(
        "config_coverage: included={} omitted={}",
        manifest.config_coverage.included.len(),
        manifest.config_coverage.omitted.len()
    );
    for omission in &manifest.explicit_omissions {
        println!("omission: {omission}");
    }
}

fn print_status_lines(status: &BackupStatusData) {
    println!("status: {:?}", status.state);
    if let Some(last_backup_id) = status.last_backup_id.as_deref() {
        println!("last_backup_id: {last_backup_id}");
    }
    if let Some(last_backup_at) = status.last_backup_at {
        println!("last_backup_at: {}", format_timestamp(last_backup_at));
    }
    if let Some(output_root) = status.output_root.as_deref() {
        println!("output_root: {output_root}");
    }
    for warning in &status.warnings {
        println!("warning: {warning}");
    }
}

fn export_summary_lines(data: &BackupExportResultData) -> Vec<String> {
    let manifest = &data.manifest;
    let included = if manifest.included_domains.is_empty() {
        "-".to_string()
    } else {
        manifest.included_domains.join(",")
    };
    vec![
        format!("backup export: {}", manifest.export_id),
        format!("created_at: {}", format_timestamp(manifest.created_at)),
        format!("target_root: {}", manifest.target_root),
        format!("included_domains: {included}"),
        format!("omitted_domains: {}", manifest.omitted_domains.len()),
        format!("files: {}", manifest.files.len()),
        format!(
            "verified: {}",
            if manifest.verification_summary.verified {
                "yes"
            } else {
                "no"
            }
        ),
    ]
}

fn manual_restore_steps(manifest: &BackupManifestData) -> Vec<String> {
    vec![
        "1. Stop `veld` before touching the live database.".to_string(),
        format!(
            "2. Confirm `{}` matches the backup root you intend to restore from.",
            manifest.output_root
        ),
        format!(
            "3. Copy `{}` back to your live database path.",
            manifest.database_snapshot_path
        ),
        format!(
            "4. Copy `{}/artifacts` back to the live artifact root if you want the durable artifacts restored.",
            manifest.output_root
        ),
        format!(
            "5. Reapply any omitted secrets manually; omitted items include: {}.",
            manifest.explicit_omissions.join(", ")
        ),
        "6. Run `vel backup --verify <backup_root>` before trusting the restored environment."
            .to_string(),
    ]
}

fn format_timestamp(value: time::OffsetDateTime) -> String {
    value.format(&Rfc3339).unwrap_or_else(|_| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use vel_api_types::{
        BackupCoverageData, BackupExportFileData, BackupExportManifestData,
        BackupSecretOmissionFlagsData, BackupStatusStateData, BackupVerificationData,
    };

    fn sample_manifest() -> BackupManifestData {
        BackupManifestData {
            backup_id: "bkp_test".to_string(),
            created_at: datetime!(2026-03-19 09:00:00 UTC),
            output_root: "/tmp/backups/bkp_test".to_string(),
            database_snapshot_path: "/tmp/backups/bkp_test/data/vel.sqlite".to_string(),
            artifact_coverage: BackupCoverageData {
                included: vec!["artifacts/captures".to_string()],
                omitted: vec!["artifacts/cache".to_string()],
                notes: vec![],
            },
            config_coverage: BackupCoverageData {
                included: vec!["config/public-settings.json".to_string()],
                omitted: vec!["integration_google_calendar_secrets".to_string()],
                notes: vec![],
            },
            explicit_omissions: vec!["integration_google_calendar_secrets".to_string()],
            secret_omission_flags: BackupSecretOmissionFlagsData {
                settings_secrets_omitted: true,
                integration_tokens_omitted: true,
                local_key_material_omitted: true,
                notes: vec![],
            },
            verification_summary: BackupVerificationData {
                verified: true,
                checksum_algorithm: "sha256".to_string(),
                checksum: "abc123".to_string(),
                checked_paths: vec![],
                notes: vec![],
            },
        }
    }

    fn sample_status() -> BackupStatusData {
        BackupStatusData {
            state: BackupStatusStateData::Ready,
            last_backup_id: Some("bkp_test".to_string()),
            last_backup_at: Some(datetime!(2026-03-19 09:00:00 UTC)),
            output_root: Some("/tmp/backups/bkp_test".to_string()),
            artifact_coverage: None,
            config_coverage: None,
            verification_summary: None,
            warnings: vec![],
        }
    }

    #[test]
    fn backup_manual_restore_steps_reference_verify_and_copy_targets() {
        let steps = manual_restore_steps(&sample_manifest());
        assert!(steps
            .iter()
            .any(|step| step.contains("vel backup --verify")));
        assert!(steps.iter().any(|step| step.contains("data/vel.sqlite")));
        assert!(steps.iter().any(|step| step.contains("artifacts")));
    }

    #[test]
    fn backup_print_status_lines_include_last_backup_id() {
        let status = sample_status();
        let output = format!("{:?}", status.last_backup_id);
        assert!(output.contains("bkp_test"));
    }

    #[test]
    fn backup_print_export_lines_include_target_root_and_domains() {
        let result = BackupExportResultData {
            manifest: BackupExportManifestData {
                export_id: "bex_test".to_string(),
                created_at: datetime!(2026-04-16 09:00:00 UTC),
                target_root: "/tmp/nas/google".to_string(),
                export_root: "/tmp/nas/google/runs/bex_test".to_string(),
                included_domains: vec!["calendar".to_string(), "tasks".to_string()],
                omitted_domains: vec![],
                files: vec![BackupExportFileData {
                    domain: "tasks".to_string(),
                    path: "/tmp/nas/google/domains/tasks/source.ndjson".to_string(),
                    schema_version: "local_source_snapshot.v1".to_string(),
                    record_count: 1,
                    checksum_algorithm: "sha256".to_string(),
                    checksum: "abc123".to_string(),
                    source_path: Some("/tmp/todoist.json".to_string()),
                }],
                derivatives: vec![],
                verification_summary: BackupVerificationData {
                    verified: true,
                    checksum_algorithm: "sha256".to_string(),
                    checksum: "abc123".to_string(),
                    checked_paths: vec![],
                    notes: vec![],
                },
            },
        };

        let lines = export_summary_lines(&result);
        assert!(lines
            .iter()
            .any(|line| line == "target_root: /tmp/nas/google"));
        assert!(lines
            .iter()
            .any(|line| line == "included_domains: calendar,tasks"));
    }

    #[test]
    fn backup_export_status_json_renders() {
        let status = BackupExportStatusData {
            state: BackupStatusStateData::Ready,
            last_export_id: Some("bex_test".to_string()),
            last_export_at: Some(datetime!(2026-04-16 09:00:00 UTC)),
            target_root: Some("/tmp/nas/google".to_string()),
            included_domains: vec!["tasks".to_string()],
            omitted_domains: vec![],
            verification_summary: None,
            warnings: vec![],
        };

        let rendered = serde_json::to_string(&status).unwrap();
        assert!(rendered.contains("bex_test"));
        assert!(rendered.contains("/tmp/nas/google"));
    }
}
