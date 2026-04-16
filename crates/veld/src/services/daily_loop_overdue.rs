use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use time::{format_description::well_known::Rfc3339, Date, Duration, Month, OffsetDateTime, Time};
use uuid::Uuid;
use vel_core::{CommitmentStatus, DailyLoopPhase, RunEventType, RunId, RunKind, RunStatus};
use vel_storage::Storage;

use crate::errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverdueAction {
    Close,
    Reschedule,
    BackToInbox,
    Tombstone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverdueGuessConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueVelGuess {
    pub suggested_due_at: String,
    pub confidence: OverdueGuessConfidence,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct OverdueMenuInput {
    pub today: String,
    pub include_vel_guess: bool,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub struct OverdueMenuItem {
    pub commitment_id: String,
    pub title: String,
    pub due_at: Option<String>,
    pub actions: Vec<OverdueAction>,
    pub vel_due_guess: Option<OverdueVelGuess>,
}

#[derive(Debug, Clone)]
pub struct OverdueMenu {
    pub session_id: String,
    pub items: Vec<OverdueMenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueReschedulePayload {
    pub due_at: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct OverdueConfirmInput {
    pub commitment_id: String,
    pub action: OverdueAction,
    pub payload: Option<OverdueReschedulePayload>,
    pub operator_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OverdueConfirm {
    pub proposal_id: String,
    pub confirmation_token: String,
    pub requires_confirmation: bool,
    pub write_scope: Vec<String>,
    pub idempotency_hint: String,
}

#[derive(Debug, Clone)]
pub struct OverdueApplyInput {
    pub proposal_id: String,
    pub idempotency_key: String,
    pub confirmation_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueStateSnapshot {
    pub due_at: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueApplyOutput {
    pub applied: bool,
    pub action_event_id: String,
    pub run_id: String,
    pub before: OverdueStateSnapshot,
    pub after: OverdueStateSnapshot,
    pub undo_supported: bool,
}

#[derive(Debug, Clone)]
pub struct OverdueUndoInput {
    pub action_event_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueUndoOutput {
    pub undone: bool,
    pub run_id: String,
    pub before: OverdueStateSnapshot,
    pub after: OverdueStateSnapshot,
}

pub async fn menu(
    storage: &Storage,
    session_id: &str,
    request: OverdueMenuInput,
) -> Result<OverdueMenu, AppError> {
    ensure_standup_session(storage, session_id).await?;
    let now = OffsetDateTime::now_utc();
    let overdue_cutoff = overdue_cutoff_for_today(request.today.trim())?;
    let overdue = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, request.limit)
        .await?
        .into_iter()
        .filter(|commitment| commitment.due_at.is_some_and(|due| due < overdue_cutoff))
        .collect::<Vec<_>>();
    let mut overdue = overdue;
    overdue.sort_by_key(|commitment| {
        commitment
            .due_at
            .map(|due| due.unix_timestamp())
            .unwrap_or(0)
    });

    let items = overdue
        .into_iter()
        .map(|commitment| {
            let guess = if request.include_vel_guess {
                Some(build_vel_guess(commitment.due_at, now))
            } else {
                None
            };
            OverdueMenuItem {
                commitment_id: commitment.id.to_string(),
                title: commitment.text,
                due_at: commitment.due_at.and_then(|due| due.format(&Rfc3339).ok()),
                actions: vec![
                    OverdueAction::Close,
                    OverdueAction::Reschedule,
                    OverdueAction::BackToInbox,
                    OverdueAction::Tombstone,
                ],
                vel_due_guess: guess,
            }
        })
        .collect::<Vec<_>>();

    Ok(OverdueMenu {
        session_id: session_id.to_string(),
        items,
    })
}

pub async fn confirm(
    storage: &Storage,
    session_id: &str,
    request: OverdueConfirmInput,
) -> Result<OverdueConfirm, AppError> {
    ensure_standup_session(storage, session_id).await?;
    let commitment = storage
        .get_commitment_by_id(request.commitment_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    if !matches!(commitment.status, CommitmentStatus::Open) {
        return Err(AppError::bad_request(
            "only open commitments can be changed through overdue workflow",
        ));
    }
    if matches!(request.action, OverdueAction::Reschedule) && request.payload.is_none() {
        return Err(AppError::bad_request(
            "reschedule action requires payload with due_at",
        ));
    }
    if let Some(payload) = request.payload.as_ref() {
        let _ = OffsetDateTime::parse(&payload.due_at, &Rfc3339)
            .map_err(|_| AppError::bad_request("payload.due_at must be RFC3339"))?;
    }

    let proposal_id = format!("ovdp_{}", Uuid::new_v4().simple());
    let confirmation_token = format!("confirm:{proposal_id}");
    let write_scope = write_scope_for_action(request.action, request.commitment_id.trim());
    let idempotency_hint = format!(
        "ovd:{}:{}:{}",
        session_id,
        request.commitment_id.trim(),
        action_label(request.action)
    );
    let metadata = OverdueProposalMetadata {
        session_id: session_id.to_string(),
        commitment_id: request.commitment_id.trim().to_string(),
        action: request.action,
        payload: request.payload.clone(),
        operator_reason: request.operator_reason.clone(),
        before: snapshot_from_commitment(&commitment),
        confirmation_token: confirmation_token.clone(),
        idempotency_hint: idempotency_hint.clone(),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
    };
    let metadata_json = serde_json::to_string(&metadata)
        .map_err(|error| AppError::internal(format!("serialize proposal metadata: {error}")))?;
    storage
        .insert_thread(
            &proposal_id,
            "daily_loop_overdue_proposal",
            &format!("Overdue {}", action_label(request.action)),
            "confirmed",
            &metadata_json,
        )
        .await?;

    Ok(OverdueConfirm {
        proposal_id,
        confirmation_token,
        requires_confirmation: true,
        write_scope,
        idempotency_hint,
    })
}

pub async fn apply(
    storage: &Storage,
    session_id: &str,
    request: OverdueApplyInput,
) -> Result<OverdueApplyOutput, AppError> {
    ensure_standup_session(storage, session_id).await?;
    let idempotency_thread_id = format!(
        "thr_ovd_apply_{}",
        normalize_idempotency_key(&request.idempotency_key)
    );
    if let Some(cached) = load_idempotent_apply_response(storage, &idempotency_thread_id).await? {
        return Ok(cached);
    }

    let proposal_record = load_overdue_proposal(storage, request.proposal_id.trim()).await?;
    if proposal_record.status == "applied" {
        return Err(AppError::conflict(
            "proposal has already been applied; reuse the original idempotency key",
        ));
    }
    if proposal_record.status != "confirmed" {
        return Err(AppError::conflict("proposal is not in a confirmable state"));
    }
    let proposal = proposal_record.metadata;
    if proposal.session_id != session_id {
        return Err(AppError::bad_request(
            "proposal does not belong to this session",
        ));
    }
    if proposal.confirmation_token != request.confirmation_token {
        return Err(AppError::conflict(
            "invalid confirmation token for proposal",
        ));
    }

    let commitment = storage
        .get_commitment_by_id(proposal.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let before = snapshot_from_commitment(&commitment);

    let (new_status, due_update) = apply_target_for_action(&proposal)?;
    storage
        .update_commitment(
            proposal.commitment_id.as_str(),
            None,
            Some(new_status),
            due_update,
            None,
            None,
            None,
        )
        .await?;
    let updated = storage
        .get_commitment_by_id(proposal.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found after apply"))?;
    let after = snapshot_from_commitment(&updated);

    let run_id = record_overdue_mutation_run(
        storage,
        "overdue_apply",
        &json!({
            "proposal_id": request.proposal_id,
            "action": action_label(proposal.action),
            "idempotency_key": request.idempotency_key,
            "operator_reason": proposal.operator_reason.clone(),
            "payload": proposal.payload.clone(),
            "before": before,
            "after": after
        }),
    )
    .await?;
    let action_event_id = format!("ovdevt_{}", Uuid::new_v4().simple());
    let action_metadata = OverdueActionEventMetadata {
        session_id: session_id.to_string(),
        commitment_id: proposal.commitment_id.clone(),
        action: proposal.action,
        before: before.clone(),
        after: after.clone(),
        run_id: run_id.clone(),
        applied_at: OffsetDateTime::now_utc().unix_timestamp(),
        undone: false,
    };
    storage
        .insert_thread(
            &action_event_id,
            "daily_loop_overdue_action",
            &format!("Overdue {}", action_label(proposal.action)),
            "applied",
            &serde_json::to_string(&action_metadata).map_err(|error| {
                AppError::internal(format!("serialize action metadata: {error}"))
            })?,
        )
        .await?;
    storage
        .update_thread_status(request.proposal_id.trim(), "applied")
        .await?;

    let response_data = OverdueApplyOutput {
        applied: true,
        action_event_id,
        run_id,
        before,
        after,
        undo_supported: true,
    };
    store_idempotent_apply_response(
        storage,
        &idempotency_thread_id,
        &request.idempotency_key,
        &response_data,
    )
    .await?;

    Ok(response_data)
}

pub async fn undo(
    storage: &Storage,
    session_id: &str,
    request: OverdueUndoInput,
) -> Result<OverdueUndoOutput, AppError> {
    ensure_standup_session(storage, session_id).await?;
    let idempotency_thread_id = format!(
        "thr_ovd_undo_{}",
        normalize_idempotency_key(&request.idempotency_key)
    );
    if let Some(cached) = load_idempotent_undo_response(storage, &idempotency_thread_id).await? {
        return Ok(cached);
    }
    let action_event = load_overdue_action_event(storage, request.action_event_id.trim()).await?;
    if action_event.session_id != session_id {
        return Err(AppError::bad_request(
            "action event does not belong to this session",
        ));
    }
    if action_event.undone {
        return Err(AppError::bad_request("action is already undone"));
    }

    let commitment = storage
        .get_commitment_by_id(action_event.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let before = snapshot_from_commitment(&commitment);

    let restored_status = CommitmentStatus::from_str(action_event.before.status.as_str())
        .map_err(|_| AppError::bad_request("stored undo status is invalid"))?;
    let restored_due = parse_optional_due_at(action_event.before.due_at.as_deref())?;
    storage
        .update_commitment(
            action_event.commitment_id.as_str(),
            None,
            Some(restored_status),
            Some(restored_due),
            None,
            None,
            None,
        )
        .await?;
    let updated = storage
        .get_commitment_by_id(action_event.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found after undo"))?;
    let after = snapshot_from_commitment(&updated);

    let run_id = record_overdue_mutation_run(
        storage,
        "overdue_undo",
        &json!({
            "action_event_id": request.action_event_id,
            "action": action_label(action_event.action),
            "idempotency_key": request.idempotency_key,
            "before": before,
            "after": after
        }),
    )
    .await?;

    let response_data = OverdueUndoOutput {
        undone: true,
        run_id,
        before,
        after,
    };
    let updated_action = OverdueActionEventMetadata {
        undone: true,
        ..action_event
    };
    storage
        .update_thread_status(request.action_event_id.trim(), "undone")
        .await?;
    storage
        .update_thread_metadata(
            request.action_event_id.trim(),
            &serde_json::to_string(&updated_action).map_err(|error| {
                AppError::internal(format!("serialize updated action metadata: {error}"))
            })?,
        )
        .await?;
    store_idempotent_undo_response(
        storage,
        &idempotency_thread_id,
        &request.idempotency_key,
        &response_data,
    )
    .await?;

    Ok(response_data)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverdueProposalMetadata {
    session_id: String,
    commitment_id: String,
    action: OverdueAction,
    payload: Option<OverdueReschedulePayload>,
    operator_reason: Option<String>,
    before: OverdueStateSnapshot,
    confirmation_token: String,
    idempotency_hint: String,
    created_at: i64,
}

#[derive(Debug, Clone)]
struct OverdueProposalRecord {
    status: String,
    metadata: OverdueProposalMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverdueActionEventMetadata {
    session_id: String,
    commitment_id: String,
    action: OverdueAction,
    before: OverdueStateSnapshot,
    after: OverdueStateSnapshot,
    run_id: String,
    applied_at: i64,
    undone: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredApplyIdempotency {
    idempotency_key: String,
    response: OverdueApplyOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredUndoIdempotency {
    idempotency_key: String,
    response: OverdueUndoOutput,
}

async fn ensure_standup_session(storage: &Storage, session_id: &str) -> Result<(), AppError> {
    let record = storage
        .get_daily_session(session_id)
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found"))?;
    if !matches!(record.session.phase, DailyLoopPhase::Standup) {
        return Err(AppError::bad_request(
            "overdue workflow is only supported for standup sessions",
        ));
    }
    Ok(())
}

fn build_vel_guess(due_at: Option<OffsetDateTime>, now: OffsetDateTime) -> OverdueVelGuess {
    let base = due_at.unwrap_or(now);
    let suggested_due_at = base + Duration::days(1);
    OverdueVelGuess {
        suggested_due_at: suggested_due_at
            .format(&Rfc3339)
            .unwrap_or_else(|_| suggested_due_at.to_string()),
        confidence: OverdueGuessConfidence::Medium,
        reason: "next free block + similar task duration".to_string(),
    }
}

fn overdue_cutoff_for_today(today: &str) -> Result<OffsetDateTime, AppError> {
    let mut parts = today.split('-');
    let year = parts
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or_else(|| AppError::bad_request("today must be YYYY-MM-DD"))?;
    let month = parts
        .next()
        .and_then(|value| value.parse::<u8>().ok())
        .and_then(|value| Month::try_from(value).ok())
        .ok_or_else(|| AppError::bad_request("today must be YYYY-MM-DD"))?;
    let day = parts
        .next()
        .and_then(|value| value.parse::<u8>().ok())
        .ok_or_else(|| AppError::bad_request("today must be YYYY-MM-DD"))?;
    if parts.next().is_some() {
        return Err(AppError::bad_request("today must be YYYY-MM-DD"));
    }
    let date = Date::from_calendar_date(year, month, day)
        .map_err(|_| AppError::bad_request("today must be YYYY-MM-DD"))?;
    Ok(date.with_time(Time::MIDNIGHT).assume_utc() + Duration::days(1))
}

fn snapshot_from_commitment(commitment: &vel_core::Commitment) -> OverdueStateSnapshot {
    OverdueStateSnapshot {
        due_at: commitment.due_at.and_then(|due| due.format(&Rfc3339).ok()),
        status: commitment.status.to_string(),
    }
}

fn write_scope_for_action(action: OverdueAction, commitment_id: &str) -> Vec<String> {
    match action {
        OverdueAction::Close | OverdueAction::Tombstone => {
            vec![format!("commitment:{commitment_id}:status")]
        }
        OverdueAction::Reschedule | OverdueAction::BackToInbox => {
            vec![format!("commitment:{commitment_id}:due_at")]
        }
    }
}

fn action_label(action: OverdueAction) -> &'static str {
    match action {
        OverdueAction::Close => "close",
        OverdueAction::Reschedule => "reschedule",
        OverdueAction::BackToInbox => "back_to_inbox",
        OverdueAction::Tombstone => "tombstone",
    }
}

fn normalize_idempotency_key(key: &str) -> String {
    let mut out = String::with_capacity(key.len().min(80));
    for ch in key.chars() {
        if out.len() >= 80 {
            break;
        }
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "default".to_string()
    } else {
        out
    }
}

fn parse_optional_due_at(value: Option<&str>) -> Result<Option<OffsetDateTime>, AppError> {
    let Some(value) = value else {
        return Ok(None);
    };
    OffsetDateTime::parse(value, &Rfc3339)
        .map(Some)
        .map_err(|_| AppError::bad_request("stored due_at is not valid RFC3339"))
}

async fn load_overdue_proposal(
    storage: &Storage,
    proposal_id: &str,
) -> Result<OverdueProposalRecord, AppError> {
    let record = storage
        .get_thread_by_id(proposal_id)
        .await?
        .ok_or_else(|| AppError::not_found("proposal not found"))?;
    if record.1 != "daily_loop_overdue_proposal" {
        return Err(AppError::bad_request(
            "proposal id is not an overdue proposal",
        ));
    }
    let metadata = serde_json::from_str::<OverdueProposalMetadata>(&record.4)
        .map_err(|error| AppError::internal(format!("invalid proposal metadata: {error}")))?;
    Ok(OverdueProposalRecord {
        status: record.3,
        metadata,
    })
}

async fn load_overdue_action_event(
    storage: &Storage,
    action_event_id: &str,
) -> Result<OverdueActionEventMetadata, AppError> {
    let record = storage
        .get_thread_by_id(action_event_id)
        .await?
        .ok_or_else(|| AppError::not_found("action event not found"))?;
    if record.1 != "daily_loop_overdue_action" {
        return Err(AppError::bad_request(
            "action event id is not an overdue action event",
        ));
    }
    serde_json::from_str::<OverdueActionEventMetadata>(&record.4)
        .map_err(|error| AppError::internal(format!("invalid action-event metadata: {error}")))
}

fn apply_target_for_action(
    proposal: &OverdueProposalMetadata,
) -> Result<(CommitmentStatus, Option<Option<OffsetDateTime>>), AppError> {
    match proposal.action {
        OverdueAction::Close => Ok((CommitmentStatus::Done, None)),
        OverdueAction::Tombstone => Ok((CommitmentStatus::Cancelled, None)),
        OverdueAction::BackToInbox => Ok((CommitmentStatus::Open, Some(None))),
        OverdueAction::Reschedule => {
            let payload = proposal
                .payload
                .as_ref()
                .ok_or_else(|| AppError::bad_request("reschedule proposal is missing payload"))?;
            let due = OffsetDateTime::parse(&payload.due_at, &Rfc3339)
                .map_err(|_| AppError::bad_request("reschedule due_at must be RFC3339"))?;
            Ok((CommitmentStatus::Open, Some(Some(due))))
        }
    }
}

async fn record_overdue_mutation_run(
    storage: &Storage,
    operation: &str,
    payload: &serde_json::Value,
) -> Result<String, AppError> {
    let run_id = RunId::new();
    storage
        .create_run(&run_id, RunKind::Agent, &json!({"operation": operation}))
        .await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Running,
            Some(now),
            None,
            None,
            None,
        )
        .await?;
    storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunStarted,
            &json!({ "operation": operation }),
        )
        .await?;
    storage
        .append_run_event_auto(run_id.as_ref(), RunEventType::MutationProposed, payload)
        .await?;
    storage
        .append_run_event_auto(run_id.as_ref(), RunEventType::MutationCommitted, payload)
        .await?;
    storage
        .append_run_event_auto(run_id.as_ref(), RunEventType::RunSucceeded, payload)
        .await?;
    storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Succeeded,
            None,
            Some(OffsetDateTime::now_utc().unix_timestamp()),
            Some(payload),
            None,
        )
        .await?;
    Ok(run_id.to_string())
}

async fn load_idempotent_apply_response(
    storage: &Storage,
    idempotency_thread_id: &str,
) -> Result<Option<OverdueApplyOutput>, AppError> {
    let Some(record) = storage.get_thread_by_id(idempotency_thread_id).await? else {
        return Ok(None);
    };
    let stored = serde_json::from_str::<StoredApplyIdempotency>(&record.4).map_err(|error| {
        AppError::internal(format!("invalid apply idempotency metadata: {error}"))
    })?;
    Ok(Some(stored.response))
}

async fn store_idempotent_apply_response(
    storage: &Storage,
    idempotency_thread_id: &str,
    idempotency_key: &str,
    response: &OverdueApplyOutput,
) -> Result<(), AppError> {
    let stored = StoredApplyIdempotency {
        idempotency_key: idempotency_key.to_string(),
        response: response.clone(),
    };
    storage
        .insert_thread(
            idempotency_thread_id,
            "daily_loop_overdue_apply_idempotency",
            "Overdue apply idempotency",
            "applied",
            &serde_json::to_string(&stored).map_err(|error| {
                AppError::internal(format!("serialize apply idempotency metadata: {error}"))
            })?,
        )
        .await?;
    Ok(())
}

async fn load_idempotent_undo_response(
    storage: &Storage,
    idempotency_thread_id: &str,
) -> Result<Option<OverdueUndoOutput>, AppError> {
    let Some(record) = storage.get_thread_by_id(idempotency_thread_id).await? else {
        return Ok(None);
    };
    let stored = serde_json::from_str::<StoredUndoIdempotency>(&record.4).map_err(|error| {
        AppError::internal(format!("invalid undo idempotency metadata: {error}"))
    })?;
    Ok(Some(stored.response))
}

async fn store_idempotent_undo_response(
    storage: &Storage,
    idempotency_thread_id: &str,
    idempotency_key: &str,
    response: &OverdueUndoOutput,
) -> Result<(), AppError> {
    let stored = StoredUndoIdempotency {
        idempotency_key: idempotency_key.to_string(),
        response: response.clone(),
    };
    storage
        .insert_thread(
            idempotency_thread_id,
            "daily_loop_overdue_undo_idempotency",
            "Overdue undo idempotency",
            "undone",
            &serde_json::to_string(&stored).map_err(|error| {
                AppError::internal(format!("serialize undo idempotency metadata: {error}"))
            })?,
        )
        .await?;
    Ok(())
}
