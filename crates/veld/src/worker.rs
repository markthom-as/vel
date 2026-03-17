//! Background workers for low-risk automation loops.
//!
//! Current responsibilities:
//! - claim and complete capture_ingest jobs
//! - promote retry-scheduled runs when their retry_at is due

use std::time::Duration;

#[cfg(not(test))]
use std::process::Command;
use tracing::{debug, warn};
use vel_core::{LoopKind, RunEventType, RunKind, RunStatus};
use vel_storage::{PendingJob, RetryReadyRun};

use crate::state::AppState;

const LOOP_INTERVAL: Duration = Duration::from_secs(5);
const JOB_TYPE_CAPTURE_INGEST: &str = "capture_ingest";
const RETRY_BATCH_LIMIT: u32 = 10;
#[cfg(test)]
const QUEUED_REPO_SYNC_WORKER_CLASS: &str = "repo_sync";
const QUEUED_REPO_SYNC_CAPABILITY: &str = "branch_sync";
#[cfg(test)]
const QUEUED_VALIDATION_WORKER_CLASS: &str = "validation";
const QUEUED_VALIDATION_CAPABILITY: &str = "build_test_profiles";

pub async fn run_background_workers(state: AppState) {
    for loop_definition in registered_loops_with_policy(&state.policy_config) {
        if !loop_definition.enabled {
            debug!(loop_kind = %loop_definition.kind, "background loop disabled");
            continue;
        }

        let state = state.clone();
        tokio::spawn(async move {
            run_registered_loop(state, loop_definition).await;
        });
    }

    std::future::pending::<()>().await;
}

#[derive(Debug, Clone, Copy)]
struct LoopDefinition {
    kind: LoopKind,
    interval: Duration,
    enabled: bool,
    runner: LoopRunner,
}

type LoopRunner = fn(&AppState) -> LoopFuture<'_>;
type LoopFuture<'a> = std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<(), crate::errors::AppError>> + Send + 'a>,
>;

fn registered_loops_with_policy(
    policy_config: &crate::policy_config::PolicyConfig,
) -> Vec<LoopDefinition> {
    let queue_work_scheduler_loop = policy_config
        .queue_work_scheduler_loop()
        .cloned()
        .unwrap_or_default();
    let evaluate_loop = policy_config
        .evaluate_current_state_loop()
        .cloned()
        .unwrap_or_default();
    let sync_calendar_loop = policy_config
        .sync_calendar_loop()
        .cloned()
        .unwrap_or_default();
    let sync_todoist_loop = policy_config
        .sync_todoist_loop()
        .cloned()
        .unwrap_or_default();
    let sync_activity_loop = policy_config
        .sync_activity_loop()
        .cloned()
        .unwrap_or_default();
    let sync_health_loop = policy_config
        .sync_health_loop()
        .cloned()
        .unwrap_or_default();
    let sync_git_loop = policy_config.sync_git_loop().cloned().unwrap_or_default();
    let sync_messaging_loop = policy_config
        .sync_messaging_loop()
        .cloned()
        .unwrap_or_default();
    let sync_notes_loop = policy_config.sync_notes_loop().cloned().unwrap_or_default();
    let sync_transcripts_loop = policy_config
        .sync_transcripts_loop()
        .cloned()
        .unwrap_or_default();
    let weekly_synthesis_loop = policy_config
        .weekly_synthesis_loop()
        .cloned()
        .unwrap_or_default();
    let stale_nudge_reconciliation_loop = policy_config
        .stale_nudge_reconciliation_loop()
        .cloned()
        .unwrap_or_default();

    vec![
        LoopDefinition {
            kind: LoopKind::CaptureIngest,
            interval: LOOP_INTERVAL,
            enabled: true,
            runner: run_capture_ingest_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::RetryDueRuns,
            interval: LOOP_INTERVAL,
            enabled: true,
            runner: run_retry_due_runs_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::QueueWorkScheduler,
            interval: Duration::from_secs(queue_work_scheduler_loop.interval_seconds),
            enabled: queue_work_scheduler_loop.enabled,
            runner: run_queue_work_scheduler_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::EvaluateCurrentState,
            interval: Duration::from_secs(evaluate_loop.interval_seconds),
            enabled: evaluate_loop.enabled,
            runner: run_evaluate_current_state_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncCalendar,
            interval: Duration::from_secs(sync_calendar_loop.interval_seconds),
            enabled: sync_calendar_loop.enabled,
            runner: run_sync_calendar_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncTodoist,
            interval: Duration::from_secs(sync_todoist_loop.interval_seconds),
            enabled: sync_todoist_loop.enabled,
            runner: run_sync_todoist_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncActivity,
            interval: Duration::from_secs(sync_activity_loop.interval_seconds),
            enabled: sync_activity_loop.enabled,
            runner: run_sync_activity_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncHealth,
            interval: Duration::from_secs(sync_health_loop.interval_seconds),
            enabled: sync_health_loop.enabled,
            runner: run_sync_health_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncGit,
            interval: Duration::from_secs(sync_git_loop.interval_seconds),
            enabled: sync_git_loop.enabled,
            runner: run_sync_git_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncMessaging,
            interval: Duration::from_secs(sync_messaging_loop.interval_seconds),
            enabled: sync_messaging_loop.enabled,
            runner: run_sync_messaging_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncNotes,
            interval: Duration::from_secs(sync_notes_loop.interval_seconds),
            enabled: sync_notes_loop.enabled,
            runner: run_sync_notes_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::SyncTranscripts,
            interval: Duration::from_secs(sync_transcripts_loop.interval_seconds),
            enabled: sync_transcripts_loop.enabled,
            runner: run_sync_transcripts_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::WeeklySynthesis,
            interval: Duration::from_secs(weekly_synthesis_loop.interval_seconds),
            enabled: weekly_synthesis_loop.enabled,
            runner: run_weekly_synthesis_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::StaleNudgeReconciliation,
            interval: Duration::from_secs(stale_nudge_reconciliation_loop.interval_seconds),
            enabled: stale_nudge_reconciliation_loop.enabled,
            runner: run_stale_nudge_reconciliation_loop_once,
        },
    ]
}

async fn run_registered_loop(state: AppState, loop_definition: LoopDefinition) {
    let mut ticker = tokio::time::interval(loop_definition.interval);
    loop {
        ticker.tick().await;
        match run_claimed_loop_once(&state, loop_definition).await {
            Ok(true) | Ok(false) => {}
            Err(error) => {
                warn!(
                    error = %error,
                    loop_kind = %loop_definition.kind,
                    "background loop execution failed"
                );
            }
        }
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub async fn run_registered_loops_once(state: &AppState) -> Result<(), crate::errors::AppError> {
    for loop_definition in registered_loops_with_policy(&state.policy_config)
        .into_iter()
        .filter(|loop_definition| loop_definition.enabled)
    {
        let _ = run_claimed_loop_once(state, loop_definition).await?;
    }
    Ok(())
}

async fn run_claimed_loop_once(
    state: &AppState,
    loop_definition: LoopDefinition,
) -> Result<bool, crate::errors::AppError> {
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let interval_seconds = loop_definition.interval.as_secs() as i64;
    let loop_kind = loop_definition.kind.to_string();

    let claimed = state
        .storage
        .claim_due_loop(&loop_kind, interval_seconds, now_ts)
        .await?;
    if !claimed {
        debug!(loop_kind = %loop_kind, "background loop not due or already running");
        return Ok(false);
    }

    let next_due_at = now_ts + interval_seconds;
    match (loop_definition.runner)(state).await {
        Ok(()) => {
            state
                .storage
                .complete_loop(&loop_kind, "succeeded", None, next_due_at)
                .await?;
            Ok(true)
        }
        Err(error) => {
            let error_message = error.to_string();
            state
                .storage
                .complete_loop(&loop_kind, "failed", Some(&error_message), next_due_at)
                .await?;
            Err(error)
        }
    }
}

fn run_capture_ingest_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        if let Some(job) = state
            .storage
            .claim_next_pending_job(JOB_TYPE_CAPTURE_INGEST)
            .await?
        {
            process_capture_ingest_job(&state.storage, job).await?;
        }
        Ok(())
    })
}

fn run_retry_due_runs_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let ready_runs = state
            .storage
            .list_retry_ready_runs(now_ts, RETRY_BATCH_LIMIT)
            .await?;
        for retry in ready_runs {
            if let Err(error) = process_retry_ready_run(state, retry).await {
                warn!(error = %error, "run retry execution failed");
            }
        }
        Ok(())
    })
}

fn run_queue_work_scheduler_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let bootstrap = crate::services::client_sync::cluster_bootstrap_data(state);
        if !bootstrap.capabilities.iter().any(|capability| {
            capability == QUEUED_REPO_SYNC_CAPABILITY || capability == QUEUED_VALIDATION_CAPABILITY
        }) {
            return Ok(());
        }

        let claimed = crate::services::client_sync::claim_next_work_for_worker(
            state,
            vel_api_types::WorkAssignmentClaimNextRequestData {
                node_id: bootstrap.node_id.clone(),
                worker_id: bootstrap.node_id,
                worker_class: None,
                capability: None,
            },
        )
        .await?;

        if let Some(claim) = claimed.claim {
            process_claimed_work(state, claim).await?;
        }

        Ok(())
    })
}

async fn process_claimed_work(
    state: &AppState,
    claim: vel_api_types::WorkAssignmentClaimedWorkData,
) -> Result<(), crate::errors::AppError> {
    match claim.queue_item.request_type {
        vel_api_types::QueuedWorkRoutingKindData::BranchSync => {
            process_claimed_branch_sync_work(state, claim).await
        }
        vel_api_types::QueuedWorkRoutingKindData::Validation => {
            process_claimed_validation_work(state, claim).await
        }
    }
}

fn run_evaluate_current_state_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        crate::services::evaluate::run_and_broadcast(state).await?;
        Ok(())
    })
}

fn run_sync_calendar_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_calendar_sync(&state.storage, &state.config).await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after calendar sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_todoist_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_todoist_sync(&state.storage, &state.config).await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after todoist sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_activity_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_activity_sync(&state.storage, &state.config).await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after activity sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_health_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_health_sync(&state.storage, &state.config).await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after health sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_git_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_git_sync(&state.storage, &state.config).await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after git sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_messaging_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_messaging_sync(&state.storage, &state.config)
                .await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after messaging sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_notes_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_notes_sync(&state.storage, &state.config).await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after notes sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_sync_transcripts_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let count =
            crate::services::integrations::run_transcripts_sync(&state.storage, &state.config)
                .await?;
        if count > 0 {
            if let Err(error) = crate::services::evaluate::run_and_broadcast(state).await {
                warn!(error = %error, "evaluate after transcripts sync loop failed");
            }
        }
        Ok(())
    })
}

fn run_weekly_synthesis_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let _ = crate::services::synthesis::run_week_synthesis_if_due(state).await?;
        Ok(())
    })
}

fn run_stale_nudge_reconciliation_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let _ = crate::services::nudge_engine::evaluate(&state.storage, &state.policy_config, 0)
            .await?;
        Ok(())
    })
}

async fn process_claimed_validation_work(
    state: &AppState,
    claim: vel_api_types::WorkAssignmentClaimedWorkData,
) -> Result<(), crate::errors::AppError> {
    let _load = state.worker_runtime.begin_work();
    let started_at = time::OffsetDateTime::now_utc().unix_timestamp();
    crate::services::client_sync::update_work_assignment_receipt(
        state,
        vel_api_types::WorkAssignmentUpdateRequest {
            receipt_id: claim.receipt.receipt_id.clone(),
            status: vel_api_types::WorkAssignmentStatusData::Started,
            started_at: Some(started_at),
            completed_at: None,
            result: None,
            error_message: None,
        },
    )
    .await?;

    let request: vel_api_types::ValidationRequestData =
        match serde_json::from_value(claim.queue_item.request_payload.clone()) {
            Ok(request) => request,
            Err(error) => {
                fail_claimed_work(
                    state,
                    &claim.receipt.receipt_id,
                    format!("invalid queued validation payload: {error}"),
                )
                .await?;
                return Ok(());
            }
        };
    let command_hint = match validation_command_hint(state, &request) {
        Some(command_hint) => command_hint,
        None => {
            fail_claimed_work(
                state,
                &claim.receipt.receipt_id,
                format!(
                    "validation profile {} is not available on this node",
                    request.profile_id
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let completed_at = time::OffsetDateTime::now_utc().unix_timestamp();
    match execute_validation_command(&request.repo_root, &command_hint).await {
        Ok(result) => {
            crate::services::client_sync::update_work_assignment_receipt(
                state,
                vel_api_types::WorkAssignmentUpdateRequest {
                    receipt_id: claim.receipt.receipt_id,
                    status: vel_api_types::WorkAssignmentStatusData::Completed,
                    started_at: None,
                    completed_at: Some(completed_at),
                    result: Some(result),
                    error_message: None,
                },
            )
            .await?;
        }
        Err(error_message) => {
            crate::services::client_sync::update_work_assignment_receipt(
                state,
                vel_api_types::WorkAssignmentUpdateRequest {
                    receipt_id: claim.receipt.receipt_id,
                    status: vel_api_types::WorkAssignmentStatusData::Failed,
                    started_at: None,
                    completed_at: Some(completed_at),
                    result: None,
                    error_message: Some(error_message),
                },
            )
            .await?;
        }
    }

    Ok(())
}

async fn process_claimed_branch_sync_work(
    state: &AppState,
    claim: vel_api_types::WorkAssignmentClaimedWorkData,
) -> Result<(), crate::errors::AppError> {
    let _load = state.worker_runtime.begin_work();
    let started_at = time::OffsetDateTime::now_utc().unix_timestamp();
    crate::services::client_sync::update_work_assignment_receipt(
        state,
        vel_api_types::WorkAssignmentUpdateRequest {
            receipt_id: claim.receipt.receipt_id.clone(),
            status: vel_api_types::WorkAssignmentStatusData::Started,
            started_at: Some(started_at),
            completed_at: None,
            result: None,
            error_message: None,
        },
    )
    .await?;

    let request: vel_api_types::BranchSyncRequestData =
        match serde_json::from_value(claim.queue_item.request_payload.clone()) {
            Ok(request) => request,
            Err(error) => {
                fail_claimed_work(
                    state,
                    &claim.receipt.receipt_id,
                    format!("invalid queued branch sync payload: {error}"),
                )
                .await?;
                return Ok(());
            }
        };

    let capability = match branch_sync_capability(state, &request) {
        Some(capability) => capability,
        None => {
            fail_claimed_work(
                state,
                &claim.receipt.receipt_id,
                "branch sync capability is not available on this node".to_string(),
            )
            .await?;
            return Ok(());
        }
    };

    let completed_at = time::OffsetDateTime::now_utc().unix_timestamp();
    match execute_branch_sync_command(&request, &capability.default_remote).await {
        Ok(result) => {
            crate::services::client_sync::update_work_assignment_receipt(
                state,
                vel_api_types::WorkAssignmentUpdateRequest {
                    receipt_id: claim.receipt.receipt_id,
                    status: vel_api_types::WorkAssignmentStatusData::Completed,
                    started_at: None,
                    completed_at: Some(completed_at),
                    result: Some(result),
                    error_message: None,
                },
            )
            .await?;
        }
        Err(error_message) => {
            crate::services::client_sync::update_work_assignment_receipt(
                state,
                vel_api_types::WorkAssignmentUpdateRequest {
                    receipt_id: claim.receipt.receipt_id,
                    status: vel_api_types::WorkAssignmentStatusData::Failed,
                    started_at: None,
                    completed_at: Some(completed_at),
                    result: None,
                    error_message: Some(error_message),
                },
            )
            .await?;
        }
    }

    Ok(())
}

async fn fail_claimed_work(
    state: &AppState,
    receipt_id: &str,
    error_message: String,
) -> Result<(), crate::errors::AppError> {
    crate::services::client_sync::update_work_assignment_receipt(
        state,
        vel_api_types::WorkAssignmentUpdateRequest {
            receipt_id: receipt_id.to_string(),
            status: vel_api_types::WorkAssignmentStatusData::Failed,
            started_at: None,
            completed_at: Some(time::OffsetDateTime::now_utc().unix_timestamp()),
            result: None,
            error_message: Some(error_message),
        },
    )
    .await?;
    Ok(())
}

fn validation_command_hint(
    state: &AppState,
    request: &vel_api_types::ValidationRequestData,
) -> Option<String> {
    crate::services::client_sync::cluster_bootstrap_data(state)
        .validation_profiles
        .into_iter()
        .find(|profile| profile.profile_id == request.profile_id)
        .map(|profile| profile.command_hint)
}

fn branch_sync_capability(
    state: &AppState,
    request: &vel_api_types::BranchSyncRequestData,
) -> Option<vel_api_types::BranchSyncCapabilityData> {
    crate::services::client_sync::cluster_bootstrap_data(state)
        .branch_sync
        .filter(|capability| capability.repo_root == request.repo_root)
}

#[cfg(not(test))]
async fn execute_branch_sync_command(
    request: &vel_api_types::BranchSyncRequestData,
    default_remote: &str,
) -> Result<String, String> {
    let repo_root = request.repo_root.clone();
    let remote = request
        .remote
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| default_remote.to_string());
    let branch = request.branch.clone();
    let mode = request.mode.as_deref().unwrap_or("pull").to_string();
    let command = match mode.as_str() {
        "fetch" => format!(
            "git fetch {} {}",
            shell_escape(&remote),
            shell_escape(&branch)
        ),
        "pull" => format!(
            "git pull {} {}",
            shell_escape(&remote),
            shell_escape(&branch)
        ),
        "push" => format!(
            "git push {} HEAD:{}",
            shell_escape(&remote),
            shell_escape(&branch)
        ),
        unsupported => {
            return Err(format!("unsupported branch sync mode: {unsupported}"));
        }
    };
    let command_for_exec = command.clone();
    let output = tokio::task::spawn_blocking(move || {
        Command::new("zsh")
            .arg("-lc")
            .arg(&command_for_exec)
            .current_dir(&repo_root)
            .output()
            .map_err(|error| format!("failed to run branch sync command: {error}"))
    })
    .await
    .map_err(|error| format!("failed to join branch sync command task: {error}"))??;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if output.status.success() {
        let summary = if stdout.is_empty() {
            format!("branch sync succeeded: {command}")
        } else {
            format!("branch sync succeeded: {command}; stdout={stdout}")
        };
        Ok(summary.chars().take(400).collect())
    } else {
        let code = output.status.code().unwrap_or(-1);
        let summary = if stderr.is_empty() {
            format!("branch sync failed ({code}): {command}")
        } else {
            format!("branch sync failed ({code}): {command}; stderr={stderr}")
        };
        Err(summary.chars().take(400).collect())
    }
}

#[cfg(not(test))]
fn shell_escape(value: &str) -> String {
    let escaped = value.replace('\'', "'\\''");
    format!("'{}'", escaped)
}

#[cfg(test)]
async fn execute_branch_sync_command(
    request: &vel_api_types::BranchSyncRequestData,
    default_remote: &str,
) -> Result<String, String> {
    let remote = request.remote.as_deref().unwrap_or(default_remote);
    let mode = request.mode.as_deref().unwrap_or("pull");
    if mode == "explode" {
        return Err(format!(
            "simulated branch sync failure: {} {} {}",
            mode, remote, request.branch
        ));
    }

    Ok(format!(
        "simulated branch sync success: {} {} {} @ {}",
        mode, remote, request.branch, request.repo_root
    ))
}

#[cfg(not(test))]
async fn execute_validation_command(repo_root: &str, command_hint: &str) -> Result<String, String> {
    let repo_root = repo_root.to_string();
    let command_hint = command_hint.to_string();
    let command_hint_for_exec = command_hint.clone();
    let output = tokio::task::spawn_blocking(move || {
        Command::new("zsh")
            .arg("-lc")
            .arg(&command_hint_for_exec)
            .current_dir(&repo_root)
            .output()
            .map_err(|error| format!("failed to run validation command: {error}"))
    })
    .await
    .map_err(|error| format!("failed to join validation command task: {error}"))??;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if output.status.success() {
        let summary = if stdout.is_empty() {
            format!("validation command succeeded: {command_hint}")
        } else {
            format!("validation command succeeded: {command_hint}; stdout={stdout}")
        };
        Ok(summary.chars().take(400).collect())
    } else {
        let code = output.status.code().unwrap_or(-1);
        let summary = if stderr.is_empty() {
            format!("validation command failed ({code}): {command_hint}")
        } else {
            format!("validation command failed ({code}): {command_hint}; stderr={stderr}")
        };
        Err(summary.chars().take(400).collect())
    }
}

#[cfg(test)]
async fn execute_validation_command(repo_root: &str, command_hint: &str) -> Result<String, String> {
    Ok(format!(
        "simulated validation success: {command_hint} @ {repo_root}"
    ))
}

#[cfg(test)]
async fn poll_once(state: &AppState) -> Result<(), crate::errors::AppError> {
    run_registered_loops_once(state).await
}

async fn process_capture_ingest_job(
    storage: &vel_storage::Storage,
    job: PendingJob,
) -> Result<(), vel_storage::StorageError> {
    debug!(job_id = %job.job_id, "processing capture_ingest job");
    storage.mark_job_succeeded(&job.job_id.to_string()).await?;
    Ok(())
}

async fn process_retry_ready_run(
    state: &AppState,
    retry: RetryReadyRun,
) -> Result<(), crate::errors::AppError> {
    let run_id = retry.run.id.clone();
    debug!(
        run_id = %run_id,
        retry_at = retry.retry_at,
        kind = %retry.run.kind,
        "processing retry-scheduled run"
    );

    state.storage.reset_run_for_retry(run_id.as_ref()).await?;
    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunRequeued,
            &serde_json::json!({
                "retry_at": retry.retry_at,
                "reason": retry.retry_reason,
                "source": "worker",
            }),
        )
        .await?;

    match retry.run.kind {
        RunKind::ContextGeneration => {
            crate::services::context_runs::retry_existing_run(
                state,
                &run_id,
                &retry.run.input_json,
            )
            .await?;
        }
        RunKind::Synthesis => {
            crate::services::synthesis::retry_existing_run(state, &run_id, &retry.run.input_json)
                .await?;
        }
        unsupported_kind => {
            let blocked_reason = unsupported_kind
                .retry_policy()
                .automatic_retry_reason
                .map(ToString::to_string)
                .unwrap_or_else(|| {
                    format!(
                        "automatic retry unsupported for run kind {}",
                        unsupported_kind
                    )
                });
            let error_json = serde_json::json!({
                "message": blocked_reason,
            });
            let output_json = serde_json::json!({
                "blocked_reason": blocked_reason,
            });
            state
                .storage
                .append_run_event_auto(
                    run_id.as_ref(),
                    RunEventType::RunRetryBlocked,
                    &serde_json::json!({
                        "reason": blocked_reason,
                        "kind": unsupported_kind.to_string(),
                    }),
                )
                .await?;
            state
                .storage
                .update_run_status(
                    run_id.as_ref(),
                    RunStatus::Blocked,
                    None,
                    Some(time::OffsetDateTime::now_utc().unix_timestamp()),
                    Some(&output_json),
                    Some(&error_json),
                )
                .await?;
        }
    }

    let _ = crate::routes::runs::broadcast_run_updated(state, run_id.as_ref()).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        poll_once, registered_loops_with_policy, QUEUED_REPO_SYNC_CAPABILITY,
        QUEUED_REPO_SYNC_WORKER_CLASS, QUEUED_VALIDATION_CAPABILITY,
        QUEUED_VALIDATION_WORKER_CLASS,
    };
    use crate::state::AppState;
    use vel_config::AppConfig;
    use vel_core::{LoopKind, RunEventType, RunId, RunKind, RunStatus};
    use vel_storage::{SignalInsert, Storage};

    async fn insert_queued_signal(
        storage: &Storage,
        target_node_id: &str,
        signal_type: &str,
        work_request_id: &str,
        target_worker_class: &str,
        requested_capability: &str,
        request_payload: serde_json::Value,
    ) {
        insert_queued_signal_at(
            storage,
            target_node_id,
            signal_type,
            work_request_id,
            target_worker_class,
            requested_capability,
            request_payload,
            time::OffsetDateTime::now_utc().unix_timestamp(),
        )
        .await;
    }

    async fn insert_queued_signal_at(
        storage: &Storage,
        target_node_id: &str,
        signal_type: &str,
        work_request_id: &str,
        target_worker_class: &str,
        requested_capability: &str,
        request_payload: serde_json::Value,
        timestamp: i64,
    ) {
        storage
            .insert_signal(SignalInsert {
                signal_type: signal_type.to_string(),
                source: "cluster_work_router".to_string(),
                source_ref: Some(work_request_id.to_string()),
                timestamp,
                payload_json: Some(serde_json::json!({
                    "request": request_payload,
                    "queued_via": "test",
                    "queued_at": timestamp,
                    "routing": {
                        "work_request_id": work_request_id,
                        "authority_node_id": target_node_id,
                        "authority_epoch": 1,
                        "target_node_id": target_node_id,
                        "target_worker_class": target_worker_class,
                        "requested_capability": requested_capability,
                    }
                })),
            })
            .await
            .unwrap();
    }

    #[test]
    fn registered_loops_are_explicit_and_enabled() {
        let loops = registered_loops_with_policy(&crate::policy_config::PolicyConfig::default());
        assert_eq!(loops.len(), 14);
        assert_eq!(loops[0].kind, LoopKind::CaptureIngest);
        assert_eq!(loops[1].kind, LoopKind::RetryDueRuns);
        assert_eq!(loops[2].kind, LoopKind::QueueWorkScheduler);
        assert_eq!(loops[3].kind, LoopKind::EvaluateCurrentState);
        assert_eq!(loops[4].kind, LoopKind::SyncCalendar);
        assert_eq!(loops[5].kind, LoopKind::SyncTodoist);
        assert_eq!(loops[6].kind, LoopKind::SyncActivity);
        assert_eq!(loops[7].kind, LoopKind::SyncHealth);
        assert_eq!(loops[8].kind, LoopKind::SyncGit);
        assert_eq!(loops[9].kind, LoopKind::SyncMessaging);
        assert_eq!(loops[10].kind, LoopKind::SyncNotes);
        assert_eq!(loops[11].kind, LoopKind::SyncTranscripts);
        assert_eq!(loops[12].kind, LoopKind::WeeklySynthesis);
        assert_eq!(loops[13].kind, LoopKind::StaleNudgeReconciliation);
        assert!(loops[0].enabled);
        assert!(loops[1].enabled);
        assert!(loops[2].enabled);
        assert!(loops[3].enabled);
        assert!(loops[4].enabled);
        assert!(loops[5].enabled);
        assert!(!loops[6].enabled);
        assert!(!loops[7].enabled);
        assert!(!loops[8].enabled);
        assert!(loops[9].enabled);
        assert!(!loops[10].enabled);
        assert!(!loops[11].enabled);
        assert!(loops[12].enabled);
        assert!(loops[13].enabled);
    }

    #[tokio::test]
    async fn poll_once_records_runtime_loop_status() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        poll_once(&state).await.unwrap();

        let loops = storage.list_runtime_loops().await.unwrap();
        assert_eq!(loops.len(), 9);
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.last_started_at.is_some()));
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.last_finished_at.is_some()));
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.last_status.as_deref() == Some("succeeded")));
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.next_due_at.is_some()));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "queue_work_scheduler"));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "evaluate_current_state"));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "sync_calendar"));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "sync_todoist"));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "sync_messaging"));
        assert!(!loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "sync_git"));
        assert!(!loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "sync_notes"));
        assert!(!loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "sync_transcripts"));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "weekly_synthesis"));
        assert!(loops
            .iter()
            .any(|loop_record| loop_record.loop_kind == "stale_nudge_reconciliation"));
    }

    #[tokio::test]
    async fn poll_once_processes_queued_validation_work() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let routed = crate::services::client_sync::queue_validation_request(
            &state,
            vel_api_types::ValidationRequestData {
                repo_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .to_string_lossy()
                    .to_string(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("worker-test".to_string()),
            },
            "test",
            Some("wrkreq-worker-loop".to_string()),
        )
        .await
        .unwrap();

        poll_once(&state).await.unwrap();

        let receipts = storage
            .list_work_assignments(Some(&routed.work_request_id), None)
            .await
            .unwrap();
        assert_eq!(receipts.len(), 1);
        assert_eq!(receipts[0].status.to_string(), "completed");
        assert!(receipts[0]
            .result
            .as_deref()
            .unwrap_or_default()
            .contains("simulated validation success"));
    }

    #[tokio::test]
    async fn poll_once_processes_queued_branch_sync_work() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let routed = crate::services::client_sync::queue_branch_sync_request(
            &state,
            vel_api_types::BranchSyncRequestData {
                repo_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .to_string_lossy()
                    .to_string(),
                branch: "main".to_string(),
                remote: Some("origin".to_string()),
                base_branch: Some("main".to_string()),
                mode: Some("pull".to_string()),
                requested_by: Some("worker-test".to_string()),
            },
            "test",
            Some("wrkreq-worker-branch-sync".to_string()),
        )
        .await
        .unwrap();

        poll_once(&state).await.unwrap();

        let receipts = storage
            .list_work_assignments(Some(&routed.work_request_id), None)
            .await
            .unwrap();
        assert_eq!(receipts.len(), 1);
        assert_eq!(receipts[0].status.to_string(), "completed");
        assert!(receipts[0]
            .result
            .as_deref()
            .unwrap_or_default()
            .contains("simulated branch sync success"));
    }

    #[tokio::test]
    async fn poll_once_marks_failed_branch_sync_receipt() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let routed = crate::services::client_sync::queue_branch_sync_request(
            &state,
            vel_api_types::BranchSyncRequestData {
                repo_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .to_string_lossy()
                    .to_string(),
                branch: "main".to_string(),
                remote: Some("origin".to_string()),
                base_branch: Some("main".to_string()),
                mode: Some("explode".to_string()),
                requested_by: Some("worker-test".to_string()),
            },
            "test",
            Some("wrkreq-worker-branch-sync-fail".to_string()),
        )
        .await
        .unwrap();

        poll_once(&state).await.unwrap();

        let receipts = storage
            .list_work_assignments(Some(&routed.work_request_id), None)
            .await
            .unwrap();
        assert_eq!(receipts.len(), 1);
        assert_eq!(receipts[0].status.to_string(), "failed");
        assert!(receipts[0]
            .error_message
            .as_deref()
            .unwrap_or_default()
            .contains("simulated branch sync failure"));
    }

    #[tokio::test]
    async fn poll_once_marks_invalid_validation_payload_failed() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );
        let bootstrap = crate::services::client_sync::cluster_bootstrap_data(&state);

        insert_queued_signal(
            &storage,
            &bootstrap.node_id,
            "client_validation_requested",
            "wrkreq-invalid-validation",
            QUEUED_VALIDATION_WORKER_CLASS,
            QUEUED_VALIDATION_CAPABILITY,
            serde_json::json!({
                "repo_root": std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .to_string_lossy()
                    .to_string(),
                "environment": "repo",
            }),
        )
        .await;

        poll_once(&state).await.unwrap();

        let receipts = storage
            .list_work_assignments(Some("wrkreq-invalid-validation"), None)
            .await
            .unwrap();
        assert_eq!(receipts.len(), 1);
        assert_eq!(receipts[0].status.to_string(), "failed");
        assert!(receipts[0]
            .error_message
            .as_deref()
            .unwrap_or_default()
            .contains("invalid queued validation payload"));
    }

    #[tokio::test]
    async fn poll_once_claims_oldest_queued_work_across_classes() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );
        let bootstrap = crate::services::client_sync::cluster_bootstrap_data(&state);
        let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .to_string_lossy()
            .to_string();

        insert_queued_signal_at(
            &storage,
            &bootstrap.node_id,
            "client_validation_requested",
            "wrkreq-oldest-validation",
            QUEUED_VALIDATION_WORKER_CLASS,
            QUEUED_VALIDATION_CAPABILITY,
            serde_json::json!({
                "repo_root": repo_root,
                "profile_id": "repo-verify",
                "environment": "repo",
            }),
            10,
        )
        .await;
        insert_queued_signal_at(
            &storage,
            &bootstrap.node_id,
            "client_branch_sync_requested",
            "wrkreq-newer-branch-sync",
            QUEUED_REPO_SYNC_WORKER_CLASS,
            QUEUED_REPO_SYNC_CAPABILITY,
            serde_json::json!({
                "repo_root": std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .to_string_lossy()
                    .to_string(),
                "branch": "main",
                "remote": "origin",
                "mode": "pull",
            }),
            20,
        )
        .await;

        poll_once(&state).await.unwrap();

        let validation_receipts = storage
            .list_work_assignments(Some("wrkreq-oldest-validation"), None)
            .await
            .unwrap();
        let branch_sync_receipts = storage
            .list_work_assignments(Some("wrkreq-newer-branch-sync"), None)
            .await
            .unwrap();
        assert_eq!(validation_receipts.len(), 1);
        assert_eq!(validation_receipts[0].status.to_string(), "completed");
        assert!(branch_sync_receipts.is_empty());
    }

    #[tokio::test]
    async fn poll_once_retries_due_context_run() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let mut config = AppConfig::default();
        let artifact_root = std::env::temp_dir().join(format!(
            "vel-worker-retry-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&artifact_root).unwrap();
        config.artifact_root = artifact_root.to_string_lossy().to_string();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            config,
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let run_id = RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::ContextGeneration,
                &serde_json::json!({ "context_kind": "today" }),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                RunStatus::Failed,
                Some(1),
                Some(2),
                None,
                Some(&serde_json::json!({ "message": "boom" })),
            )
            .await
            .unwrap();
        storage
            .append_run_event_auto(
                run_id.as_ref(),
                RunEventType::RunRetryScheduled,
                &serde_json::json!({
                    "retry_at": time::OffsetDateTime::now_utc().unix_timestamp() - 1,
                    "reason": "test",
                }),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                RunStatus::RetryScheduled,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert!(poll_once(&state).await.is_ok());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, RunStatus::Succeeded);

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRequeued));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunSucceeded));
    }

    #[tokio::test]
    async fn poll_once_retries_due_synthesis_run() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let mut config = AppConfig::default();
        let artifact_root = std::env::temp_dir().join(format!(
            "vel-worker-synthesis-retry-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&artifact_root).unwrap();
        config.artifact_root = artifact_root.to_string_lossy().to_string();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            config,
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let run_id = RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                RunStatus::Failed,
                Some(1),
                Some(2),
                None,
                Some(&serde_json::json!({ "message": "boom" })),
            )
            .await
            .unwrap();
        storage
            .append_run_event_auto(
                run_id.as_ref(),
                RunEventType::RunRetryScheduled,
                &serde_json::json!({
                    "retry_at": time::OffsetDateTime::now_utc().unix_timestamp() - 1,
                    "reason": "retry_weekly_synthesis",
                }),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                RunStatus::RetryScheduled,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert!(poll_once(&state).await.is_ok());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, RunStatus::Succeeded);
        assert!(run.output_json.is_some());

        let refs = storage
            .list_refs_from("run", run_id.as_ref())
            .await
            .unwrap();
        assert!(
            !refs.is_empty(),
            "retried synthesis run should relink an artifact"
        );

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRequeued));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::ArtifactWritten));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunSucceeded));
    }

    #[tokio::test]
    async fn poll_once_blocks_unsupported_retry_kind() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let run_id = RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                RunStatus::Failed,
                Some(1),
                Some(2),
                None,
                Some(&serde_json::json!({ "message": "boom" })),
            )
            .await
            .unwrap();
        storage
            .append_run_event_auto(
                run_id.as_ref(),
                RunEventType::RunRetryScheduled,
                &serde_json::json!({
                    "retry_at": time::OffsetDateTime::now_utc().unix_timestamp() - 1,
                    "reason": "retry_search",
                }),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                RunStatus::RetryScheduled,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert!(poll_once(&state).await.is_ok());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, RunStatus::Blocked);
        assert_eq!(
            run.error_json
                .as_ref()
                .and_then(|json| json.get("message"))
                .and_then(serde_json::Value::as_str),
            Some("search runs do not have an automatic retry executor")
        );
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|json| json.get("blocked_reason"))
                .and_then(serde_json::Value::as_str),
            Some("search runs do not have an automatic retry executor")
        );

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRequeued));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRetryBlocked));
    }
}
