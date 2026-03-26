use anyhow::Context;
use time::OffsetDateTime;
use vel_api_types::{
    DailyLoopCheckInSkipRequestData, DailyLoopOverdueActionData, DailyLoopOverdueApplyRequestData,
    DailyLoopOverdueConfirmRequestData, DailyLoopOverdueMenuRequestData,
    DailyLoopOverdueReschedulePayloadData, DailyLoopOverdueUndoRequestData, DailyLoopPhaseData,
    DailyLoopSessionData, DailyLoopSessionStateData, DailyLoopStartMetadataData,
    DailyLoopStartRequestData, DailyLoopStartSourceData, DailyLoopSurfaceData,
    DailyLoopTurnActionData,
};

use crate::client::ApiClient;

pub async fn run_start(
    client: &ApiClient,
    phase: DailyLoopPhaseData,
    json: bool,
) -> anyhow::Result<()> {
    let session_date = OffsetDateTime::now_utc().date().to_string();
    let active = client
        .active_daily_loop_session(&session_date, phase)
        .await
        .context("load active daily-loop session")?;
    let response = if let Some(session) = active.data.flatten() {
        client
            .submit_daily_loop_turn(&session.id, DailyLoopTurnActionData::Resume, None)
            .await
            .context("resume daily-loop session")?
    } else {
        client
            .start_daily_loop_session(&DailyLoopStartRequestData {
                phase,
                session_date,
                start: DailyLoopStartMetadataData {
                    source: DailyLoopStartSourceData::Manual,
                    surface: DailyLoopSurfaceData::Cli,
                },
            })
            .await
            .context("start daily-loop session")?
    };

    let session = response
        .data
        .ok_or_else(|| anyhow::anyhow!("daily-loop response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&session)?);
        return Ok(());
    }
    print_session(&session);
    Ok(())
}

pub async fn run_reply(
    client: &ApiClient,
    session_id: &str,
    text: String,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .submit_daily_loop_turn(session_id, DailyLoopTurnActionData::Submit, Some(text))
        .await
        .context("submit daily-loop reply")?;
    let session = response
        .data
        .ok_or_else(|| anyhow::anyhow!("daily-loop response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&session)?);
        return Ok(());
    }
    print_session(&session);
    Ok(())
}

pub async fn run_skip(client: &ApiClient, session_id: &str, json: bool) -> anyhow::Result<()> {
    let response = client
        .submit_daily_loop_turn(session_id, DailyLoopTurnActionData::Skip, None)
        .await
        .context("skip daily-loop prompt")?;
    let session = response
        .data
        .ok_or_else(|| anyhow::anyhow!("daily-loop response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&session)?);
        return Ok(());
    }
    print_session(&session);
    Ok(())
}

pub async fn run_skip_check_in(
    client: &ApiClient,
    check_in_event_id: &str,
    reason_code: Option<String>,
    reason_text: Option<String>,
    source: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .daily_loop_check_in_skip(
            check_in_event_id,
            &DailyLoopCheckInSkipRequestData {
                source,
                answered_at: None,
                reason_code,
                reason_text,
            },
        )
        .await
        .context("skip daily-loop check-in")?;
    let data = response
        .data
        .ok_or_else(|| anyhow::anyhow!("check-in skip response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("check_in_event_id: {}", data.check_in_event_id);
    println!("session_id: {}", data.session_id);
    println!("status: {}", data.status);
    println!("supersedes_event_id: {:?}", data.supersedes_event_id);
    Ok(())
}

pub async fn run_overdue_menu(client: &ApiClient, limit: u32, json: bool) -> anyhow::Result<()> {
    let session_id = resolve_active_standup_session_id(client).await?;
    let today = OffsetDateTime::now_utc().date().to_string();
    let response = client
        .daily_loop_overdue_menu(
            &session_id,
            &DailyLoopOverdueMenuRequestData {
                today,
                include_vel_guess: true,
                limit,
            },
        )
        .await
        .context("load overdue menu")?;
    let data = response
        .data
        .ok_or_else(|| anyhow::anyhow!("overdue menu response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("session: {}", data.session_id);
    if data.items.is_empty() {
        println!("overdue: none");
        return Ok(());
    }
    println!("overdue:");
    for item in data.items {
        let due = item.due_at.unwrap_or_else(|| "unscheduled".to_string());
        println!("  - {} ({})", item.title, item.commitment_id);
        println!("    due: {}", due);
        let actions = item
            .actions
            .iter()
            .map(|action| format!("{action:?}").to_ascii_lowercase())
            .collect::<Vec<_>>()
            .join(", ");
        println!("    actions: {}", actions);
        if let Some(guess) = item.vel_due_guess {
            println!(
                "    vel_guess: {} ({:?}) — {}",
                guess.suggested_due_at, guess.confidence, guess.reason
            );
        }
    }
    Ok(())
}

pub async fn run_overdue_confirm(
    client: &ApiClient,
    commitment_id: &str,
    action: DailyLoopOverdueActionData,
    due_at: Option<String>,
    reason: Option<String>,
    use_vel_guess: bool,
    json: bool,
) -> anyhow::Result<()> {
    let session_id = resolve_active_standup_session_id(client).await?;
    let payload = if matches!(action, DailyLoopOverdueActionData::Reschedule) {
        let due_at = due_at.ok_or_else(|| {
            anyhow::anyhow!(
                "--due-at is required for reschedule until vel guess application is implemented"
            )
        })?;
        Some(DailyLoopOverdueReschedulePayloadData {
            due_at,
            source: if use_vel_guess {
                "vel_guess".to_string()
            } else {
                "operator".to_string()
            },
        })
    } else {
        None
    };
    let response = client
        .daily_loop_overdue_confirm(
            &session_id,
            &DailyLoopOverdueConfirmRequestData {
                commitment_id: commitment_id.to_string(),
                action,
                payload,
                operator_reason: reason,
            },
        )
        .await
        .context("confirm overdue action")?;
    let data = response
        .data
        .ok_or_else(|| anyhow::anyhow!("overdue confirm response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("proposal: {}", data.proposal_id);
    println!("confirmation_token: {}", data.confirmation_token);
    println!("requires_confirmation: {}", data.requires_confirmation);
    if !data.write_scope.is_empty() {
        println!("write_scope: {}", data.write_scope.join(", "));
    }
    println!("idempotency_hint: {}", data.idempotency_hint);
    Ok(())
}

pub async fn run_overdue_apply(
    client: &ApiClient,
    proposal_id: &str,
    confirmation_token: &str,
    idempotency_key: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let session_id = resolve_active_standup_session_id(client).await?;
    let key = idempotency_key.unwrap_or_else(|| format!("ovd:apply:{proposal_id}"));
    let response = client
        .daily_loop_overdue_apply(
            &session_id,
            &DailyLoopOverdueApplyRequestData {
                proposal_id: proposal_id.to_string(),
                idempotency_key: key.clone(),
                confirmation_token: confirmation_token.to_string(),
            },
        )
        .await
        .context("apply overdue action")?;
    let data = response
        .data
        .ok_or_else(|| anyhow::anyhow!("overdue apply response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("applied: {}", data.applied);
    println!("run_id: {}", data.run_id);
    println!("action_event_id: {}", data.action_event_id);
    println!("idempotency_key: {}", key);
    println!("undo_supported: {}", data.undo_supported);
    Ok(())
}

pub async fn run_overdue_undo(
    client: &ApiClient,
    action_event_id: &str,
    idempotency_key: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let session_id = resolve_active_standup_session_id(client).await?;
    let key = idempotency_key.unwrap_or_else(|| format!("ovd:undo:{action_event_id}"));
    let response = client
        .daily_loop_overdue_undo(
            &session_id,
            &DailyLoopOverdueUndoRequestData {
                action_event_id: action_event_id.to_string(),
                idempotency_key: key.clone(),
            },
        )
        .await
        .context("undo overdue action")?;
    let data = response
        .data
        .ok_or_else(|| anyhow::anyhow!("overdue undo response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    println!("undone: {}", data.undone);
    println!("run_id: {}", data.run_id);
    println!("idempotency_key: {}", key);
    Ok(())
}

async fn resolve_active_standup_session_id(client: &ApiClient) -> anyhow::Result<String> {
    let session_date = OffsetDateTime::now_utc().date().to_string();
    let active = client
        .active_daily_loop_session(&session_date, DailyLoopPhaseData::Standup)
        .await
        .context("load active standup session")?;
    if let Some(session) = active.data.flatten() {
        return Ok(session.id);
    }
    let response = client
        .start_daily_loop_session(&DailyLoopStartRequestData {
            phase: DailyLoopPhaseData::Standup,
            session_date,
            start: DailyLoopStartMetadataData {
                source: DailyLoopStartSourceData::Manual,
                surface: DailyLoopSurfaceData::Cli,
            },
        })
        .await
        .context("start standup session")?;
    let session = response
        .data
        .ok_or_else(|| anyhow::anyhow!("start standup response missing data"))?;
    Ok(session.id)
}

fn print_session(session: &DailyLoopSessionData) {
    println!("session: {}", session.id);
    println!("phase: {:?}", session.phase);
    println!("status: {:?}", session.status);
    println!("continuity: {}", session.continuity_summary);
    if !session.allowed_actions.is_empty() {
        println!(
            "actions: {}",
            session
                .allowed_actions
                .iter()
                .map(|action| format!("{action:?}").to_ascii_lowercase())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    match &session.state {
        DailyLoopSessionStateData::MorningOverview(state) => {
            println!("{}", state.snapshot);
            for callout in &state.friction_callouts {
                println!("friction: {} — {}", callout.label, callout.detail);
            }
        }
        DailyLoopSessionStateData::Standup(state) => {
            if !state.commitments.is_empty() {
                println!("commitments:");
                for item in &state.commitments {
                    println!("  - {:?}: {}", item.bucket, item.title);
                }
            }
            for item in &state.deferred_tasks {
                println!("deferred: {} ({})", item.title, item.reason);
            }
            for item in &state.confirmed_calendar {
                println!("calendar: {}", item);
            }
        }
    }
    if let Some(prompt) = &session.current_prompt {
        println!("prompt {}: {}", prompt.ordinal, prompt.text);
    } else if let Some(outcome) = &session.outcome {
        println!(
            "outcome: {}",
            serde_json::to_string_pretty(outcome).unwrap_or_default()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vel_api_types::{
        DailyCommitmentDraftData, DailyLoopPromptData, DailyLoopPromptKindData,
        DailyLoopSessionOutcomeData, DailyLoopStartMetadataData, DailyLoopStatusData,
        DailyLoopTurnStateData, DailyStandupBucketData, DailyStandupOutcomeData,
    };

    #[test]
    fn renders_standup_session_without_panicking() {
        let session = DailyLoopSessionData {
            id: "dls_cli".to_string(),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhaseData::Standup,
            status: DailyLoopStatusData::WaitingForInput,
            start: DailyLoopStartMetadataData {
                source: DailyLoopStartSourceData::Manual,
                surface: DailyLoopSurfaceData::Cli,
            },
            turn_state: DailyLoopTurnStateData::WaitingForInput,
            current_prompt: Some(DailyLoopPromptData {
                prompt_id: "p1".to_string(),
                kind: DailyLoopPromptKindData::CommitmentReduction,
                text: "Name the one to three commitments.".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            continuity_summary:
                "Standup is waiting on question 1 with 1 commitment draft(s) and 0 deferred item(s)."
                    .to_string(),
            allowed_actions: vec![
                vel_api_types::DailyLoopCommitmentActionData::Accept,
                vel_api_types::DailyLoopCommitmentActionData::Defer,
                vel_api_types::DailyLoopCommitmentActionData::Choose,
                vel_api_types::DailyLoopCommitmentActionData::Close,
            ],
            state: DailyLoopSessionStateData::Standup(DailyStandupOutcomeData {
                commitments: vec![DailyCommitmentDraftData {
                    title: "Ship Phase 10".to_string(),
                    bucket: DailyStandupBucketData::Must,
                    source_ref: None,
                }],
                deferred_tasks: vec![],
                confirmed_calendar: vec![],
                focus_blocks: vec![],
                check_in_history: vec![],
            }),
            outcome: Some(DailyLoopSessionOutcomeData::Standup(
                DailyStandupOutcomeData {
                    commitments: vec![],
                    deferred_tasks: vec![],
                    confirmed_calendar: vec![],
                    focus_blocks: vec![],
                    check_in_history: vec![],
                },
            )),
        };

        print_session(&session);
    }
}
