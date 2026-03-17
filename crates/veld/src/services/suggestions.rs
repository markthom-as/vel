//! Deterministic suggestion evaluation pipeline.
//! Suggestions are steering recommendations derived from repeated runtime patterns.

use std::cmp::Reverse;

use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{ConfidenceBand, SuggestionConfidence, SuggestionType};
use vel_storage::{NudgeRecord, Storage};

const REPEAT_THRESHOLD: usize = 2;
const WINDOW_DAYS: i64 = 7;
const MAX_SUGGESTIONS_PER_PASS: usize = 4;

#[derive(Debug, Clone)]
struct SuggestionEvidenceCandidate {
    evidence_type: String,
    ref_id: String,
    evidence_json: JsonValue,
    weight: f64,
}

#[derive(Debug, Clone)]
struct SuggestionCandidate {
    suggestion_type: SuggestionType,
    title: String,
    summary: String,
    priority: i64,
    confidence: ConfidenceBand,
    dedupe_key: String,
    payload: JsonValue,
    decision_context: JsonValue,
    evidence: Vec<SuggestionEvidenceCandidate>,
}

pub async fn evaluate_after_nudges(storage: &Storage) -> Result<u32, crate::errors::AppError> {
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let candidates = collect_candidates(storage, now_ts).await?;
    let candidates = suppress_candidates(storage, candidates).await?;
    let candidates = rank_candidates(candidates, current_global_risk_score(storage).await?);
    persist_candidates(storage, candidates).await
}

async fn collect_candidates(
    storage: &Storage,
    now_ts: i64,
) -> Result<Vec<SuggestionCandidate>, crate::errors::AppError> {
    let window_start = now_ts - WINDOW_DAYS * 86_400;
    let nudges = storage.list_nudges(None, 500).await?;
    let mut candidates = Vec::new();

    let resolved_commute_danger = nudges_in_window(
        &nudges,
        window_start,
        "commute_leave_time",
        |nudge| nudge.state == "resolved" && nudge.level == "danger",
    );
    if resolved_commute_danger.len() >= REPEAT_THRESHOLD {
        candidates.push(build_candidate(
            SuggestionType::IncreaseCommuteBuffer,
            70,
            SuggestionConfidence::Medium,
            "Increase commute buffer",
            "Repeated commute danger nudges suggest the leave-time threshold is too tight.",
            serde_json::json!({
                "type": SuggestionType::IncreaseCommuteBuffer.to_string(),
                "current_minutes": 20,
                "suggested_minutes": 30
            }),
            serde_json::json!({
                "summary": format!(
                    "Resolved {} commute danger nudges in the last {} days.",
                    resolved_commute_danger.len(),
                    WINDOW_DAYS
                ),
                "trigger": "resolved_commute_danger",
                "window_days": WINDOW_DAYS,
                "count": resolved_commute_danger.len(),
                "threshold": REPEAT_THRESHOLD,
            }),
            resolved_commute_danger,
        ));
    }

    let resolved_prep = nudges_in_window(
        &nudges,
        window_start,
        "meeting_prep_window",
        |nudge| nudge.state == "resolved",
    );
    if resolved_prep.len() >= REPEAT_THRESHOLD {
        candidates.push(build_candidate(
            SuggestionType::IncreasePrepWindow,
            60,
            SuggestionConfidence::Medium,
            "Increase prep window",
            "Repeated prep nudges suggest the default meeting prep window is too small.",
            serde_json::json!({
                "type": SuggestionType::IncreasePrepWindow.to_string(),
                "current_minutes": 30,
                "suggested_minutes": 45
            }),
            serde_json::json!({
                "summary": format!(
                    "Resolved {} prep-window nudges in the last {} days.",
                    resolved_prep.len(),
                    WINDOW_DAYS
                ),
                "trigger": "resolved_prep_window",
                "window_days": WINDOW_DAYS,
                "count": resolved_prep.len(),
                "threshold": REPEAT_THRESHOLD,
            }),
            resolved_prep,
        ));
    }

    let morning_drift = nudges_in_window(&nudges, window_start, "morning_drift", |nudge| {
        matches!(nudge.state.as_str(), "active" | "resolved")
    });
    if morning_drift.len() >= REPEAT_THRESHOLD {
        candidates.push(build_candidate(
            SuggestionType::AddStartRoutine,
            35,
            SuggestionConfidence::Medium,
            "Add start routine",
            "Repeated morning drift suggests the day needs a stronger startup routine.",
            serde_json::json!({
                "type": SuggestionType::AddStartRoutine.to_string(),
                "suggested_block_minutes": 20,
                "reason": "repeated_morning_drift"
            }),
            serde_json::json!({
                "summary": format!(
                    "Observed {} morning drift nudges in the last {} days.",
                    morning_drift.len(),
                    WINDOW_DAYS
                ),
                "trigger": "morning_drift",
                "window_days": WINDOW_DAYS,
                "count": morning_drift.len(),
                "threshold": REPEAT_THRESHOLD,
            }),
            morning_drift,
        ));
    }

    let response_debt = nudges_in_window(&nudges, window_start, "response_debt", |nudge| {
        matches!(nudge.state.as_str(), "active" | "resolved")
    });
    if response_debt.len() >= REPEAT_THRESHOLD {
        candidates.push(build_candidate(
            SuggestionType::AddFollowupBlock,
            50,
            SuggestionConfidence::Medium,
            "Add follow-up block",
            "Repeated response-debt pressure suggests reserving dedicated follow-up time.",
            serde_json::json!({
                "type": SuggestionType::AddFollowupBlock.to_string(),
                "suggested_block_minutes": 30,
                "reason": "repeated_response_debt"
            }),
            serde_json::json!({
                "summary": format!(
                    "Observed {} response-debt nudges in the last {} days.",
                    response_debt.len(),
                    WINDOW_DAYS
                ),
                "trigger": "response_debt",
                "window_days": WINDOW_DAYS,
                "count": response_debt.len(),
                "threshold": REPEAT_THRESHOLD,
            }),
            response_debt,
        ));
    }

    Ok(candidates)
}

fn nudges_in_window<'a, F>(
    nudges: &'a [NudgeRecord],
    window_start: i64,
    nudge_type: &str,
    predicate: F,
) -> Vec<&'a NudgeRecord>
where
    F: Fn(&NudgeRecord) -> bool,
{
    nudges
        .iter()
        .filter(|nudge| nudge.nudge_type == nudge_type && nudge.created_at >= window_start)
        .filter(|nudge| predicate(nudge))
        .collect()
}

fn build_candidate(
    suggestion_type: SuggestionType,
    priority: i64,
    confidence: ConfidenceBand,
    title: &str,
    summary: &str,
    payload: JsonValue,
    decision_context: JsonValue,
    nudges: Vec<&NudgeRecord>,
) -> SuggestionCandidate {
    let evidence = nudges
        .into_iter()
        .map(|nudge| SuggestionEvidenceCandidate {
            evidence_type: "nudge".to_string(),
            ref_id: nudge.nudge_id.clone(),
            evidence_json: serde_json::json!({
                "nudge_type": nudge.nudge_type,
                "level": nudge.level,
                "state": nudge.state,
                "created_at": nudge.created_at,
                "resolved_at": nudge.resolved_at,
            }),
            weight: 1.0,
        })
        .collect();
    SuggestionCandidate {
        suggestion_type,
        title: title.to_string(),
        summary: summary.to_string(),
        priority,
        confidence,
        dedupe_key: suggestion_type.to_string(),
        payload,
        decision_context,
        evidence,
    }
}

async fn suppress_candidates(
    storage: &Storage,
    candidates: Vec<SuggestionCandidate>,
) -> Result<Vec<SuggestionCandidate>, crate::errors::AppError> {
    let mut accepted = Vec::new();
    for candidate in candidates {
        let existing = storage
            .find_recent_suggestion_by_dedupe_key(&candidate.dedupe_key)
            .await?;
        if existing.is_some_and(|suggestion| suggestion.state == "pending") {
            continue;
        }
        accepted.push(candidate);
    }
    Ok(accepted)
}

fn rank_candidates(
    mut candidates: Vec<SuggestionCandidate>,
    global_risk_score: f64,
) -> Vec<SuggestionCandidate> {
    // Explicit priority formula:
    // final = base priority
    //       + 5 * evidence_count
    //       + min(15, recency_boost_from_newest_evidence)
    //       + round(global_risk_score * 10)
    candidates.sort_by_key(|candidate| {
        let evidence_count = candidate.evidence.len() as i64;
        let newest_created_at = candidate
            .evidence
            .iter()
            .filter_map(|evidence| evidence.evidence_json.get("created_at"))
            .filter_map(serde_json::Value::as_i64)
            .max()
            .unwrap_or(0);
        let recency_boost = if newest_created_at > 0 {
            let age_days = (OffsetDateTime::now_utc().unix_timestamp() - newest_created_at)
                .div_euclid(86_400)
                .max(0);
            (15 - age_days.min(15)) as i64
        } else {
            0
        };
        let risk_boost = (global_risk_score.clamp(0.0, 1.0) * 10.0).round() as i64;
        Reverse(candidate.priority + evidence_count * 5 + recency_boost + risk_boost)
    });
    candidates.truncate(MAX_SUGGESTIONS_PER_PASS);
    candidates
}

async fn persist_candidates(
    storage: &Storage,
    candidates: Vec<SuggestionCandidate>,
) -> Result<u32, crate::errors::AppError> {
    let mut created = 0u32;
    for candidate in candidates.into_iter().rev() {
        let suggestion_id = storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: candidate.suggestion_type.to_string(),
                state: "pending".to_string(),
                title: Some(candidate.title),
                summary: Some(candidate.summary),
                priority: candidate.priority,
                confidence: Some(candidate.confidence.to_string()),
                dedupe_key: Some(candidate.dedupe_key),
                payload_json: candidate.payload,
                decision_context_json: Some(candidate.decision_context),
            })
            .await?;
        for evidence in candidate.evidence {
            storage
                .insert_suggestion_evidence(vel_storage::SuggestionEvidenceInsert {
                    suggestion_id: suggestion_id.clone(),
                    evidence_type: evidence.evidence_type,
                    ref_id: evidence.ref_id,
                    evidence_json: Some(evidence.evidence_json),
                    weight: Some(evidence.weight),
                })
                .await?;
        }
        created += 1;
    }
    Ok(created)
}

async fn current_global_risk_score(storage: &Storage) -> Result<f64, crate::errors::AppError> {
    let Some((_, context_json)) = storage.get_current_context().await? else {
        return Ok(0.0);
    };
    let context: JsonValue =
        serde_json::from_str(&context_json).unwrap_or_else(|_| serde_json::json!({}));
    Ok(context
        .get("global_risk_score")
        .and_then(serde_json::Value::as_f64)
        .unwrap_or(0.0))
}
