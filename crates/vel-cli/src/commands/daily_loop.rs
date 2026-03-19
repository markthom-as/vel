use anyhow::Context;
use time::OffsetDateTime;
use vel_api_types::{
    DailyLoopPhaseData, DailyLoopSessionData, DailyLoopSessionStateData,
    DailyLoopStartMetadataData, DailyLoopStartRequestData, DailyLoopStartSourceData,
    DailyLoopSurfaceData, DailyLoopTurnActionData,
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

fn print_session(session: &DailyLoopSessionData) {
    println!("session: {}", session.id);
    println!("phase: {:?}", session.phase);
    println!("status: {:?}", session.status);
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
            state: DailyLoopSessionStateData::Standup(DailyStandupOutcomeData {
                commitments: vec![DailyCommitmentDraftData {
                    title: "Ship Phase 10".to_string(),
                    bucket: DailyStandupBucketData::Must,
                    source_ref: None,
                }],
                deferred_tasks: vec![],
                confirmed_calendar: vec![],
                focus_blocks: vec![],
            }),
            outcome: Some(DailyLoopSessionOutcomeData::Standup(
                DailyStandupOutcomeData {
                    commitments: vec![],
                    deferred_tasks: vec![],
                    confirmed_calendar: vec![],
                    focus_blocks: vec![],
                },
            )),
        };

        print_session(&session);
    }
}
