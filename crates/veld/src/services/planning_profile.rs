use serde_json::{json, Value};
use time::{Date, Time};
use vel_core::{
    AssistantProposalState, CurrentContextV1, DurableRoutineBlock, PlanningConstraint,
    PlanningConstraintKind, PlanningProfileContinuity, PlanningProfileEditProposal,
    PlanningProfileMutation, PlanningProfileRemoveTarget, PlanningProfileSurface, RoutineBlock,
    RoutineBlockSourceKind, RoutinePlanningProfile, ScheduleTimeWindow,
};
use vel_storage::{Storage, StorageError};

use crate::{
    errors::AppError,
    services::timezone::{self, ResolvedTimeZone},
};

#[derive(Debug, Clone)]
pub(crate) struct DayPlanningInputs {
    pub routine_blocks: Vec<RoutineBlock>,
    pub planning_constraints: Vec<PlanningConstraint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanningProfileProposalSummaryItem {
    pub thread_id: String,
    pub state: AssistantProposalState,
    pub title: String,
    pub summary: String,
    pub outcome_summary: Option<String>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlanningProfileProposalSummary {
    pub pending_count: u32,
    pub latest_pending: Option<PlanningProfileProposalSummaryItem>,
    pub latest_applied: Option<PlanningProfileProposalSummaryItem>,
    pub latest_failed: Option<PlanningProfileProposalSummaryItem>,
}

impl PlanningProfileProposalSummary {
    pub fn is_empty(&self) -> bool {
        self.pending_count == 0
            && self.latest_pending.is_none()
            && self.latest_applied.is_none()
            && self.latest_failed.is_none()
    }
}

pub async fn load_routine_planning_profile(
    storage: &Storage,
) -> Result<RoutinePlanningProfile, AppError> {
    Ok(storage.load_routine_planning_profile().await?)
}

pub async fn save_routine_planning_profile(
    storage: &Storage,
    profile: &RoutinePlanningProfile,
) -> Result<(), AppError> {
    storage.replace_routine_planning_profile(profile).await?;
    Ok(())
}

pub async fn load_planning_profile_proposal_summary(
    storage: &Storage,
) -> Result<PlanningProfileProposalSummary, AppError> {
    let rows = storage.list_threads(None, 100).await?;
    let mut summary = PlanningProfileProposalSummary::default();

    for (thread_id, thread_type, title, _, _metadata_json, _, updated_at) in rows {
        if thread_type != "planning_profile_edit" {
            continue;
        }

        let Some((_, _, _, _, metadata_json, _, _)) = storage.get_thread_by_id(&thread_id).await?
        else {
            continue;
        };
        let metadata = parse_proposal_metadata(&metadata_json)?;
        let proposal = proposal_from_thread_metadata(&thread_id, &metadata)?;
        let item = PlanningProfileProposalSummaryItem {
            thread_id: thread_id.clone(),
            state: proposal.state,
            title: title.clone(),
            summary: proposal.summary,
            outcome_summary: proposal.outcome_summary,
            updated_at,
        };

        match item.state {
            AssistantProposalState::Staged | AssistantProposalState::Approved => {
                summary.pending_count += 1;
                if summary.latest_pending.is_none() {
                    summary.latest_pending = Some(item);
                }
            }
            AssistantProposalState::Applied | AssistantProposalState::Reversed => {
                if summary.latest_applied.is_none() {
                    summary.latest_applied = Some(item);
                }
            }
            AssistantProposalState::Failed => {
                if summary.latest_failed.is_none() {
                    summary.latest_failed = Some(item);
                }
            }
        }
    }

    Ok(summary)
}

pub async fn apply_planning_profile_mutation(
    storage: &Storage,
    mutation: &PlanningProfileMutation,
) -> Result<RoutinePlanningProfile, AppError> {
    validate_mutation(mutation)?;
    storage
        .apply_routine_planning_profile_mutation(mutation)
        .await
        .map_err(map_storage_error)
}

pub async fn apply_staged_planning_profile_proposal(
    storage: &Storage,
    thread_id: &str,
) -> Result<(RoutinePlanningProfile, PlanningProfileEditProposal), AppError> {
    let Some((_, thread_type, _, _, metadata_json, _, _)) =
        storage.get_thread_by_id(thread_id).await?
    else {
        return Err(AppError::not_found(
            "planning profile proposal thread not found",
        ));
    };
    if thread_type != "planning_profile_edit" {
        return Err(AppError::bad_request(
            "thread is not a planning profile proposal",
        ));
    }

    let mut metadata = parse_proposal_metadata(&metadata_json)?;
    let mut proposal = proposal_from_thread_metadata(thread_id, &metadata)?;
    if proposal.state != AssistantProposalState::Staged
        && proposal.state != AssistantProposalState::Approved
    {
        return Err(AppError::bad_request(
            "planning profile proposal is not pending application",
        ));
    }

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let transition_via = "planning_profile_apply";
    update_proposal_metadata_transition(
        &mut metadata,
        AssistantProposalState::Approved,
        Some("Planning-profile proposal approved for canonical application.".to_string()),
        now,
        transition_via,
    );

    match apply_planning_profile_mutation(storage, &proposal.mutation).await {
        Ok(profile) => {
            let outcome =
                "Planning-profile proposal applied through canonical mutation seam.".to_string();
            update_proposal_metadata_transition(
                &mut metadata,
                AssistantProposalState::Applied,
                Some(outcome.clone()),
                now,
                transition_via,
            );
            storage
                .update_thread_metadata(thread_id, &metadata.to_string())
                .await?;
            storage.update_thread_status(thread_id, "resolved").await?;
            proposal.state = AssistantProposalState::Applied;
            proposal.outcome_summary = Some(outcome);
            proposal.thread_id = Some(thread_id.to_string());
            proposal.thread_type = Some("planning_profile_edit".to_string());
            Ok((profile, proposal))
        }
        Err(error) => {
            update_proposal_metadata_transition(
                &mut metadata,
                AssistantProposalState::Failed,
                Some(error.to_string()),
                now,
                transition_via,
            );
            storage
                .update_thread_metadata(thread_id, &metadata.to_string())
                .await?;
            storage.update_thread_status(thread_id, "open").await?;
            Err(error)
        }
    }
}

pub(crate) fn staged_edit_proposal_from_text(
    text: &str,
    source_surface: PlanningProfileSurface,
) -> Option<PlanningProfileEditProposal> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mutation =
        parse_routine_block_mutation(trimmed).or_else(|| parse_constraint_mutation(trimmed))?;
    let summary = planning_profile_mutation_summary(&mutation);

    Some(PlanningProfileEditProposal {
        source_surface,
        state: AssistantProposalState::Staged,
        mutation,
        summary,
        requires_confirmation: true,
        continuity: PlanningProfileContinuity::Thread,
        outcome_summary: None,
        thread_id: None,
        thread_type: None,
    })
}

pub(crate) async fn load_day_planning_inputs(
    storage: &Storage,
    context: &CurrentContextV1,
    now_ts: i64,
) -> Result<DayPlanningInputs, AppError> {
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let current_day = crate::services::timezone::current_day_window(
        &timezone,
        time::OffsetDateTime::from_unix_timestamp(now_ts)
            .unwrap_or(time::OffsetDateTime::UNIX_EPOCH),
    )?;
    let profile = load_routine_planning_profile(storage).await?;
    let routine_blocks = materialize_routine_blocks_for_day(&profile, &current_day.session_date)?;
    let planning_constraints = active_constraints(&profile);

    if !routine_blocks.is_empty() {
        return Ok(DayPlanningInputs {
            routine_blocks,
            planning_constraints,
        });
    }

    Ok(DayPlanningInputs {
        routine_blocks: inferred_routine_blocks(context, current_day.start_ts),
        planning_constraints,
    })
}

pub(crate) fn default_time_window(
    constraints: &[PlanningConstraint],
) -> Option<ScheduleTimeWindow> {
    constraints.iter().find_map(|constraint| {
        (constraint.active && constraint.kind == PlanningConstraintKind::DefaultTimeWindow)
            .then_some(constraint.time_window)
            .flatten()
    })
}

pub(crate) fn max_scheduled_items(constraints: &[PlanningConstraint]) -> Option<usize> {
    constraints.iter().find_map(|constraint| {
        (constraint.active && constraint.kind == PlanningConstraintKind::MaxScheduledItems)
            .then_some(constraint.max_items)
            .flatten()
            .map(|count| count as usize)
    })
}

pub(crate) fn reserve_buffer_before_calendar_minutes(constraints: &[PlanningConstraint]) -> i64 {
    buffer_minutes_for_kind(
        constraints,
        PlanningConstraintKind::ReserveBufferBeforeCalendar,
    )
}

pub(crate) fn reserve_buffer_after_calendar_minutes(constraints: &[PlanningConstraint]) -> i64 {
    buffer_minutes_for_kind(
        constraints,
        PlanningConstraintKind::ReserveBufferAfterCalendar,
    )
}

pub(crate) fn require_judgment_for_overflow(constraints: &[PlanningConstraint]) -> bool {
    constraints.iter().any(|constraint| {
        constraint.active && constraint.kind == PlanningConstraintKind::RequireJudgmentForOverflow
    })
}

fn buffer_minutes_for_kind(
    constraints: &[PlanningConstraint],
    kind: PlanningConstraintKind,
) -> i64 {
    constraints
        .iter()
        .find_map(|constraint| {
            (constraint.active && constraint.kind == kind)
                .then_some(constraint.minutes)
                .flatten()
        })
        .map(i64::from)
        .unwrap_or(0)
}

fn active_constraints(profile: &RoutinePlanningProfile) -> Vec<PlanningConstraint> {
    profile
        .planning_constraints
        .iter()
        .filter(|constraint| constraint.active)
        .cloned()
        .collect()
}

fn parse_proposal_metadata(metadata_json: &str) -> Result<Value, AppError> {
    let metadata = serde_json::from_str::<Value>(metadata_json)
        .map_err(|_| AppError::internal("planning profile proposal metadata is not valid JSON"))?;
    if !metadata.is_object() {
        return Err(AppError::internal(
            "planning profile proposal metadata must be an object",
        ));
    }
    Ok(metadata)
}

fn proposal_from_thread_metadata(
    thread_id: &str,
    metadata: &Value,
) -> Result<PlanningProfileEditProposal, AppError> {
    let object = metadata.as_object().ok_or_else(|| {
        AppError::internal("planning profile proposal metadata must be an object")
    })?;
    let mutation = serde_json::from_value::<PlanningProfileMutation>(
        object.get("mutation").cloned().ok_or_else(|| {
            AppError::bad_request("planning profile proposal is missing mutation")
        })?,
    )
    .map_err(|_| AppError::bad_request("planning profile proposal mutation is invalid"))?;
    let source_surface = object
        .get("lineage")
        .and_then(Value::as_object)
        .and_then(|lineage| lineage.get("source_surface"))
        .cloned()
        .map(serde_json::from_value::<PlanningProfileSurface>)
        .transpose()
        .map_err(|_| AppError::bad_request("planning profile proposal source surface is invalid"))?
        .unwrap_or(PlanningProfileSurface::Assistant);
    let continuity = object
        .get("continuity")
        .cloned()
        .map(serde_json::from_value::<PlanningProfileContinuity>)
        .transpose()
        .map_err(|_| AppError::bad_request("planning profile proposal continuity is invalid"))?
        .unwrap_or(PlanningProfileContinuity::Thread);
    let state = object
        .get("proposal_state")
        .and_then(Value::as_str)
        .map(parse_assistant_proposal_state)
        .transpose()?
        .unwrap_or(AssistantProposalState::Staged);

    Ok(PlanningProfileEditProposal {
        source_surface,
        state,
        mutation,
        summary: object
            .get("summary")
            .and_then(Value::as_str)
            .unwrap_or("Planning-profile proposal")
            .to_string(),
        requires_confirmation: object
            .get("requires_confirmation")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        continuity,
        outcome_summary: object
            .get("outcome_summary")
            .and_then(Value::as_str)
            .map(str::to_string),
        thread_id: Some(thread_id.to_string()),
        thread_type: Some("planning_profile_edit".to_string()),
    })
}

fn parse_assistant_proposal_state(value: &str) -> Result<AssistantProposalState, AppError> {
    match value {
        "staged" => Ok(AssistantProposalState::Staged),
        "approved" => Ok(AssistantProposalState::Approved),
        "applied" => Ok(AssistantProposalState::Applied),
        "failed" => Ok(AssistantProposalState::Failed),
        "reversed" => Ok(AssistantProposalState::Reversed),
        _ => Err(AppError::bad_request(format!(
            "unknown planning profile proposal state: {value}"
        ))),
    }
}

fn update_proposal_metadata_transition(
    metadata: &mut Value,
    state: AssistantProposalState,
    outcome_summary: Option<String>,
    now: i64,
    via: &str,
) {
    let object = match metadata.as_object_mut() {
        Some(object) => object,
        None => return,
    };
    let previous_state = object
        .get("proposal_state")
        .and_then(Value::as_str)
        .unwrap_or("staged")
        .to_string();

    object.insert(
        "proposal_state".to_string(),
        Value::String(state.to_string()),
    );
    object.insert(
        "outcome_summary".to_string(),
        outcome_summary
            .clone()
            .map(Value::String)
            .unwrap_or(Value::Null),
    );

    match state {
        AssistantProposalState::Approved => {
            object.insert("approved_at".to_string(), json!(now));
            object.insert("approved_via".to_string(), Value::String(via.to_string()));
        }
        AssistantProposalState::Applied => {
            object.insert("approved_at".to_string(), json!(now));
            object.insert("approved_via".to_string(), Value::String(via.to_string()));
            object.insert("applied_at".to_string(), json!(now));
            object.insert("applied_via".to_string(), Value::String(via.to_string()));
        }
        AssistantProposalState::Failed => {
            object.insert("failed_at".to_string(), json!(now));
            object.insert("failed_via".to_string(), Value::String(via.to_string()));
        }
        AssistantProposalState::Reversed => {
            object.insert("reversed_at".to_string(), json!(now));
            object.insert("reversed_via".to_string(), Value::String(via.to_string()));
        }
        AssistantProposalState::Staged => {}
    }

    object.insert(
        "follow_through".to_string(),
        json!({
            "kind": state.to_string(),
            "previous_state": previous_state,
            "changed_at": now,
            "changed_via": via,
            "summary": outcome_summary,
        }),
    );

    if let Some(lineage) = object
        .entry("lineage".to_string())
        .or_insert_with(|| json!({}))
        .as_object_mut()
    {
        lineage.insert(
            "last_transition".to_string(),
            Value::String(state.to_string()),
        );
    }
}

fn parse_routine_block_mutation(text: &str) -> Option<PlanningProfileMutation> {
    let lowered = text.to_ascii_lowercase();
    if !(lowered.contains(" block ") || lowered.ends_with(" block")) {
        return None;
    }
    if !(lowered.contains("add ") || lowered.contains("create ") || lowered.contains("set up ")) {
        return None;
    }

    let timezone = extract_timezone_token(text)?;
    let (start_local_time, end_local_time) = extract_time_range(text)?;
    let label = extract_block_label(text)?;
    let days_of_week = if lowered.contains("weekday") || lowered.contains("weekdays") {
        vec![1, 2, 3, 4, 5]
    } else {
        Vec::new()
    };
    let protected = lowered.contains("protected");
    let id = format!("routine_{}", slug_fragment(&label));

    Some(PlanningProfileMutation::UpsertRoutineBlock(
        DurableRoutineBlock {
            id,
            label,
            source: RoutineBlockSourceKind::OperatorDeclared,
            local_timezone: timezone,
            start_local_time,
            end_local_time,
            days_of_week,
            protected,
            active: true,
        },
    ))
}

fn parse_constraint_mutation(text: &str) -> Option<PlanningProfileMutation> {
    let lowered = text.to_ascii_lowercase();

    if let Some(window) = parse_named_time_window(&lowered) {
        if lowered.contains("default time window") || lowered.contains("default window") {
            return Some(PlanningProfileMutation::UpsertPlanningConstraint(
                PlanningConstraint {
                    id: format!("constraint_default_window_{}", window_token(window)),
                    label: format!("Default {}", window_label(window)),
                    kind: PlanningConstraintKind::DefaultTimeWindow,
                    detail: None,
                    time_window: Some(window),
                    minutes: None,
                    max_items: None,
                    active: true,
                },
            ));
        }
    }

    if lowered.contains("require judgment for overflow") {
        return Some(PlanningProfileMutation::UpsertPlanningConstraint(
            PlanningConstraint {
                id: "constraint_overflow_judgment".to_string(),
                label: "Require judgment for overflow".to_string(),
                kind: PlanningConstraintKind::RequireJudgmentForOverflow,
                detail: None,
                time_window: None,
                minutes: None,
                max_items: None,
                active: true,
            },
        ));
    }

    if let Some(max_items) = extract_u32_after_phrase(&lowered, "max scheduled items to ") {
        return Some(PlanningProfileMutation::UpsertPlanningConstraint(
            PlanningConstraint {
                id: "constraint_max_scheduled_items".to_string(),
                label: format!("Limit scheduled items to {max_items}"),
                kind: PlanningConstraintKind::MaxScheduledItems,
                detail: None,
                time_window: None,
                minutes: None,
                max_items: Some(max_items),
                active: true,
            },
        ));
    }

    if let Some(minutes) = extract_u32_before_word(&lowered, "minutes") {
        if lowered.contains("before calendar") {
            return Some(PlanningProfileMutation::UpsertPlanningConstraint(
                PlanningConstraint {
                    id: "constraint_buffer_before_calendar".to_string(),
                    label: format!("Reserve {minutes} minutes before calendar"),
                    kind: PlanningConstraintKind::ReserveBufferBeforeCalendar,
                    detail: None,
                    time_window: None,
                    minutes: Some(minutes),
                    max_items: None,
                    active: true,
                },
            ));
        }
        if lowered.contains("after calendar") {
            return Some(PlanningProfileMutation::UpsertPlanningConstraint(
                PlanningConstraint {
                    id: "constraint_buffer_after_calendar".to_string(),
                    label: format!("Reserve {minutes} minutes after calendar"),
                    kind: PlanningConstraintKind::ReserveBufferAfterCalendar,
                    detail: None,
                    time_window: None,
                    minutes: Some(minutes),
                    max_items: None,
                    active: true,
                },
            ));
        }
    }

    None
}

fn planning_profile_mutation_summary(mutation: &PlanningProfileMutation) -> String {
    match mutation {
        PlanningProfileMutation::UpsertRoutineBlock(block) => format!(
            "Stage routine block '{}' from {} to {} in {}.",
            block.label, block.start_local_time, block.end_local_time, block.local_timezone
        ),
        PlanningProfileMutation::RemoveRoutineBlock(target) => {
            format!("Stage removal of routine block '{}'.", target.id)
        }
        PlanningProfileMutation::UpsertPlanningConstraint(constraint) => {
            format!("Stage planning constraint '{}'.", constraint.label)
        }
        PlanningProfileMutation::RemovePlanningConstraint(target) => {
            format!("Stage removal of planning constraint '{}'.", target.id)
        }
    }
}

fn extract_timezone_token(text: &str) -> Option<String> {
    text.split_whitespace().find_map(|token| {
        let trimmed = token.trim_matches(|character: char| {
            !character.is_ascii_alphanumeric()
                && character != '/'
                && character != '_'
                && character != '-'
        });
        trimmed
            .contains('/')
            .then(|| trimmed.to_string())
            .filter(|value| value.split('/').count() >= 2)
    })
}

fn extract_time_range(text: &str) -> Option<(String, String)> {
    let times = text
        .split_whitespace()
        .filter_map(|token| normalize_hhmm_token(token))
        .collect::<Vec<_>>();
    (times.len() >= 2).then(|| (times[0].clone(), times[1].clone()))
}

fn normalize_hhmm_token(token: &str) -> Option<String> {
    let trimmed =
        token.trim_matches(|character: char| !character.is_ascii_digit() && character != ':');
    if trimmed.len() == 5 && trimmed.as_bytes().get(2) == Some(&b':') {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn extract_block_label(text: &str) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    let block_index = lowered.find(" block")?;
    let prefix = text[..block_index].trim();
    let prefix = prefix
        .trim_start_matches(|character: char| character.is_ascii_whitespace())
        .trim_start_matches("Add ")
        .trim_start_matches("add ")
        .trim_start_matches("Create ")
        .trim_start_matches("create ")
        .trim_start_matches("Set up ")
        .trim_start_matches("set up ")
        .trim();
    let prefix = prefix
        .trim_start_matches("a ")
        .trim_start_matches("an ")
        .trim_start_matches("weekday ")
        .trim_start_matches("weekdays ")
        .trim_start_matches("protected ")
        .trim();
    (!prefix.is_empty()).then(|| prefix.to_string())
}

fn parse_named_time_window(lowered: &str) -> Option<ScheduleTimeWindow> {
    if lowered.contains("prenoon") {
        Some(ScheduleTimeWindow::Prenoon)
    } else if lowered.contains("afternoon") {
        Some(ScheduleTimeWindow::Afternoon)
    } else if lowered.contains("evening") {
        Some(ScheduleTimeWindow::Evening)
    } else if lowered.contains("night") {
        Some(ScheduleTimeWindow::Night)
    } else if lowered.contains(" day") || lowered.contains("day ") {
        Some(ScheduleTimeWindow::Day)
    } else {
        None
    }
}

fn extract_u32_after_phrase(lowered: &str, phrase: &str) -> Option<u32> {
    let suffix = lowered.split_once(phrase)?.1;
    suffix
        .split_whitespace()
        .next()
        .and_then(|value| value.parse::<u32>().ok())
}

fn extract_u32_before_word(lowered: &str, word: &str) -> Option<u32> {
    let prefix = lowered.split_once(word)?.0;
    prefix
        .split_whitespace()
        .last()
        .and_then(|value| value.parse::<u32>().ok())
}

fn slug_fragment(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator {
            slug.push('_');
            previous_was_separator = true;
        }
    }
    slug.trim_matches('_').to_string()
}

fn window_token(window: ScheduleTimeWindow) -> &'static str {
    match window {
        ScheduleTimeWindow::Prenoon => "prenoon",
        ScheduleTimeWindow::Afternoon => "afternoon",
        ScheduleTimeWindow::Evening => "evening",
        ScheduleTimeWindow::Night => "night",
        ScheduleTimeWindow::Day => "day",
    }
}

fn window_label(window: ScheduleTimeWindow) -> &'static str {
    window_token(window)
}

fn inferred_routine_blocks(context: &CurrentContextV1, day_start: i64) -> Vec<RoutineBlock> {
    let mut blocks = Vec::new();
    let has_morning_routine_signal = context.prep_window_active
        || matches!(
            context.morning_state.as_str(),
            "awake_unstarted" | "at_risk" | "engaged" | "underway"
        )
        || context.mode == "morning_mode";
    if has_morning_routine_signal {
        blocks.push(RoutineBlock {
            id: "routine_morning".to_string(),
            label: "Morning routine".to_string(),
            source: RoutineBlockSourceKind::Inferred,
            start_ts: day_start + (2 * 60 * 60),
            end_ts: day_start + (4 * 60 * 60),
            protected: true,
        });
    }
    if context.commute_window_active {
        blocks.push(RoutineBlock {
            id: "routine_commute".to_string(),
            label: "Commute".to_string(),
            source: RoutineBlockSourceKind::Inferred,
            start_ts: day_start + (4 * 60 * 60),
            end_ts: day_start + (5 * 60 * 60),
            protected: true,
        });
    }
    blocks
}

fn materialize_routine_blocks_for_day(
    profile: &RoutinePlanningProfile,
    session_date: &str,
) -> Result<Vec<RoutineBlock>, AppError> {
    let date_format = time::format_description::parse("[year]-[month]-[day]")
        .expect("hardcoded date format should parse");
    let session_date = Date::parse(session_date, &date_format)
        .map_err(|_| AppError::bad_request("invalid current-day session date"))?;
    let mut blocks = profile
        .routine_blocks
        .iter()
        .filter(|block| block.active)
        .filter_map(|block| materialize_routine_block(block, session_date).transpose())
        .collect::<Result<Vec<_>, _>>()?;
    blocks.sort_by_key(|block| (block.start_ts, block.end_ts));
    Ok(blocks)
}

fn materialize_routine_block(
    block: &DurableRoutineBlock,
    session_date: Date,
) -> Result<Option<RoutineBlock>, AppError> {
    let timezone = ResolvedTimeZone::parse(&block.local_timezone)?;
    let weekday = session_date.weekday().number_from_monday();
    if !block.days_of_week.is_empty() && !block.days_of_week.contains(&weekday) {
        return Ok(None);
    }

    let start_local_time = parse_local_hhmm(&block.start_local_time)?;
    let end_local_time = parse_local_hhmm(&block.end_local_time)?;
    let start_ts = timezone::local_datetime_timestamp(&timezone, session_date, start_local_time)?;
    let mut end_date = session_date;
    if end_local_time <= start_local_time {
        end_date = end_date
            .next_day()
            .ok_or_else(|| AppError::bad_request("unable to resolve routine block end date"))?;
    }
    let end_ts = timezone::local_datetime_timestamp(&timezone, end_date, end_local_time)?;

    Ok(Some(RoutineBlock {
        id: block.id.clone(),
        label: block.label.clone(),
        source: block.source,
        start_ts,
        end_ts,
        protected: block.protected,
    }))
}

fn parse_local_hhmm(value: &str) -> Result<Time, AppError> {
    let time_format = time::format_description::parse("[hour]:[minute]")
        .expect("hardcoded time format should parse");
    Time::parse(value, &time_format)
        .map_err(|_| AppError::bad_request(format!("invalid routine local time: {value}")))
}

fn validate_mutation(mutation: &PlanningProfileMutation) -> Result<(), AppError> {
    match mutation {
        PlanningProfileMutation::UpsertRoutineBlock(block) => validate_routine_block(block),
        PlanningProfileMutation::RemoveRoutineBlock(target)
        | PlanningProfileMutation::RemovePlanningConstraint(target) => {
            validate_remove_target(target)
        }
        PlanningProfileMutation::UpsertPlanningConstraint(constraint) => {
            validate_planning_constraint(constraint)
        }
    }
}

fn validate_routine_block(block: &DurableRoutineBlock) -> Result<(), AppError> {
    if block.id.trim().is_empty() {
        return Err(AppError::bad_request("routine block id must not be empty"));
    }
    if block.label.trim().is_empty() {
        return Err(AppError::bad_request(
            "routine block label must not be empty",
        ));
    }
    let _timezone = ResolvedTimeZone::parse(&block.local_timezone)
        .map_err(|_| AppError::bad_request("timezone must be a valid IANA timezone"))?;
    let start = parse_local_hhmm(&block.start_local_time)?;
    let end = parse_local_hhmm(&block.end_local_time)?;
    if start == end {
        return Err(AppError::bad_request(
            "routine block start_local_time and end_local_time must not be equal",
        ));
    }
    if block.days_of_week.iter().any(|day| *day == 0 || *day > 7) {
        return Err(AppError::bad_request(
            "routine block days_of_week values must be between 1 and 7",
        ));
    }
    let unique_days: std::collections::BTreeSet<_> = block.days_of_week.iter().copied().collect();
    if unique_days.len() != block.days_of_week.len() {
        return Err(AppError::bad_request(
            "routine block days_of_week must not contain duplicates",
        ));
    }
    Ok(())
}

fn validate_planning_constraint(constraint: &PlanningConstraint) -> Result<(), AppError> {
    if constraint.id.trim().is_empty() {
        return Err(AppError::bad_request(
            "planning constraint id must not be empty",
        ));
    }
    if constraint.label.trim().is_empty() {
        return Err(AppError::bad_request(
            "planning constraint label must not be empty",
        ));
    }

    match constraint.kind {
        PlanningConstraintKind::MaxScheduledItems => {
            if constraint.max_items.unwrap_or(0) == 0 {
                return Err(AppError::bad_request(
                    "max_scheduled_items requires max_items greater than zero",
                ));
            }
            if constraint.minutes.is_some() || constraint.time_window.is_some() {
                return Err(AppError::bad_request(
                    "max_scheduled_items does not accept minutes or time_window",
                ));
            }
        }
        PlanningConstraintKind::ReserveBufferBeforeCalendar
        | PlanningConstraintKind::ReserveBufferAfterCalendar => {
            if constraint.minutes.unwrap_or(0) == 0 {
                return Err(AppError::bad_request(
                    "calendar buffer constraints require minutes greater than zero",
                ));
            }
            if constraint.max_items.is_some() || constraint.time_window.is_some() {
                return Err(AppError::bad_request(
                    "calendar buffer constraints do not accept max_items or time_window",
                ));
            }
        }
        PlanningConstraintKind::DefaultTimeWindow => {
            if constraint.time_window.is_none() {
                return Err(AppError::bad_request(
                    "default_time_window requires a time_window value",
                ));
            }
            if constraint.minutes.is_some() || constraint.max_items.is_some() {
                return Err(AppError::bad_request(
                    "default_time_window does not accept minutes or max_items",
                ));
            }
        }
        PlanningConstraintKind::RequireJudgmentForOverflow => {
            if constraint.minutes.is_some()
                || constraint.max_items.is_some()
                || constraint.time_window.is_some()
            {
                return Err(AppError::bad_request(
                    "require_judgment_for_overflow does not accept minutes, max_items, or time_window",
                ));
            }
        }
    }

    Ok(())
}

fn validate_remove_target(target: &PlanningProfileRemoveTarget) -> Result<(), AppError> {
    if target.id.trim().is_empty() {
        return Err(AppError::bad_request(
            "planning profile target id must not be empty",
        ));
    }
    Ok(())
}

fn map_storage_error(error: StorageError) -> AppError {
    match error {
        StorageError::Validation(message) => AppError::bad_request(message),
        StorageError::NotFound(message) => AppError::not_found(message),
        other => AppError::internal(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_planning_profile_mutation, apply_staged_planning_profile_proposal,
        default_time_window, load_day_planning_inputs, load_planning_profile_proposal_summary,
        load_routine_planning_profile, max_scheduled_items, require_judgment_for_overflow,
        reserve_buffer_before_calendar_minutes, save_routine_planning_profile,
    };
    use serde_json::json;
    use vel_core::{
        AssistantProposalState, CurrentContextV1, DurableRoutineBlock, PlanningConstraint,
        PlanningConstraintKind, PlanningProfileMutation, PlanningProfileRemoveTarget,
        PlanningProfileSurface, RoutineBlockSourceKind, RoutinePlanningProfile, ScheduleTimeWindow,
    };
    use vel_storage::Storage;

    async fn test_storage() -> Storage {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
    }

    #[tokio::test]
    async fn service_loads_and_saves_routine_planning_profile() {
        let storage = test_storage().await;
        let profile = RoutinePlanningProfile {
            routine_blocks: vec![DurableRoutineBlock {
                id: "routine_morning".to_string(),
                label: "Morning routine".to_string(),
                source: RoutineBlockSourceKind::OperatorDeclared,
                local_timezone: "America/Denver".to_string(),
                start_local_time: "06:30".to_string(),
                end_local_time: "08:00".to_string(),
                days_of_week: vec![1, 2, 3, 4, 5],
                protected: true,
                active: true,
            }],
            planning_constraints: vec![PlanningConstraint {
                id: "max_items".to_string(),
                label: "Keep the day bounded".to_string(),
                kind: PlanningConstraintKind::MaxScheduledItems,
                detail: Some("Cap scheduled work blocks".to_string()),
                time_window: None,
                minutes: None,
                max_items: Some(4),
                active: true,
            }],
        };

        save_routine_planning_profile(&storage, &profile)
            .await
            .unwrap();

        let stored = load_routine_planning_profile(&storage).await.unwrap();
        assert_eq!(stored, profile);
    }

    #[tokio::test]
    async fn service_applies_typed_profile_mutation() {
        let storage = test_storage().await;

        let profile = apply_planning_profile_mutation(
            &storage,
            &PlanningProfileMutation::UpsertRoutineBlock(DurableRoutineBlock {
                id: "routine_morning".to_string(),
                label: "Morning routine".to_string(),
                source: RoutineBlockSourceKind::OperatorDeclared,
                local_timezone: "America/Denver".to_string(),
                start_local_time: "06:30".to_string(),
                end_local_time: "08:00".to_string(),
                days_of_week: vec![1, 2, 3, 4, 5],
                protected: true,
                active: true,
            }),
        )
        .await
        .unwrap();

        assert_eq!(profile.routine_blocks.len(), 1);
    }

    #[tokio::test]
    async fn service_rejects_invalid_duplicate_weekdays() {
        let storage = test_storage().await;

        let error = apply_planning_profile_mutation(
            &storage,
            &PlanningProfileMutation::UpsertRoutineBlock(DurableRoutineBlock {
                id: "routine_bad".to_string(),
                label: "Bad routine".to_string(),
                source: RoutineBlockSourceKind::OperatorDeclared,
                local_timezone: "America/Denver".to_string(),
                start_local_time: "07:00".to_string(),
                end_local_time: "08:00".to_string(),
                days_of_week: vec![1, 1],
                protected: true,
                active: true,
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "routine block days_of_week must not contain duplicates"
        );
    }

    #[tokio::test]
    async fn service_rejects_invalid_constraint_shape() {
        let storage = test_storage().await;

        let error = apply_planning_profile_mutation(
            &storage,
            &PlanningProfileMutation::UpsertPlanningConstraint(PlanningConstraint {
                id: "window_bad".to_string(),
                label: "Bad window".to_string(),
                kind: PlanningConstraintKind::DefaultTimeWindow,
                detail: None,
                time_window: None,
                minutes: Some(15),
                max_items: None,
                active: true,
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "default_time_window requires a time_window value"
        );
    }

    #[tokio::test]
    async fn service_maps_missing_remove_target_to_not_found() {
        let storage = test_storage().await;

        let error = apply_planning_profile_mutation(
            &storage,
            &PlanningProfileMutation::RemovePlanningConstraint(PlanningProfileRemoveTarget {
                id: "missing".to_string(),
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(error.to_string(), "planning constraint missing not found");
    }

    #[tokio::test]
    async fn service_applies_staged_planning_profile_proposal_thread() {
        let storage = test_storage().await;
        storage
            .insert_thread(
                "thr_planning_profile_edit_msg_1",
                "planning_profile_edit",
                "Add shutdown block",
                "open",
                &json!({
                    "source": "planning_profile_proposal",
                    "proposal_state": "staged",
                    "summary": "Add shutdown block",
                    "requires_confirmation": true,
                    "continuity": "thread",
                    "mutation": {
                        "kind": "upsert_routine_block",
                        "data": {
                            "id": "routine_shutdown",
                            "label": "Shutdown",
                            "source": "operator_declared",
                            "local_timezone": "America/Denver",
                            "start_local_time": "17:00",
                            "end_local_time": "17:30",
                            "days_of_week": [1, 2, 3, 4, 5],
                            "protected": true,
                            "active": true
                        }
                    },
                    "lineage": {
                        "source_surface": "assistant"
                    }
                })
                .to_string(),
            )
            .await
            .unwrap();

        let (profile, proposal) =
            apply_staged_planning_profile_proposal(&storage, "thr_planning_profile_edit_msg_1")
                .await
                .unwrap();

        assert_eq!(profile.routine_blocks.len(), 1);
        assert_eq!(proposal.state, AssistantProposalState::Applied);
        assert_eq!(proposal.source_surface, PlanningProfileSurface::Assistant);
        assert_eq!(
            proposal.outcome_summary.as_deref(),
            Some("Planning-profile proposal applied through canonical mutation seam.")
        );

        let thread = storage
            .get_thread_by_id("thr_planning_profile_edit_msg_1")
            .await
            .unwrap()
            .unwrap();
        let metadata: serde_json::Value = serde_json::from_str(&thread.4).unwrap();
        assert_eq!(metadata["proposal_state"], "applied");
        assert_eq!(metadata["applied_via"], "planning_profile_apply");
        assert_eq!(thread.3, "resolved");
    }

    #[tokio::test]
    async fn service_marks_failed_planning_profile_proposal_when_apply_fails() {
        let storage = test_storage().await;
        storage
            .insert_thread(
                "thr_planning_profile_edit_msg_2",
                "planning_profile_edit",
                "Remove missing block",
                "open",
                &json!({
                    "source": "planning_profile_proposal",
                    "proposal_state": "staged",
                    "summary": "Remove missing block",
                    "requires_confirmation": true,
                    "continuity": "thread",
                    "mutation": {
                        "kind": "remove_routine_block",
                        "data": {
                            "id": "missing"
                        }
                    },
                    "lineage": {
                        "source_surface": "assistant"
                    }
                })
                .to_string(),
            )
            .await
            .unwrap();

        let error =
            apply_staged_planning_profile_proposal(&storage, "thr_planning_profile_edit_msg_2")
                .await
                .unwrap_err();

        assert_eq!(error.to_string(), "routine block missing not found");
        let thread = storage
            .get_thread_by_id("thr_planning_profile_edit_msg_2")
            .await
            .unwrap()
            .unwrap();
        let metadata: serde_json::Value = serde_json::from_str(&thread.4).unwrap();
        assert_eq!(metadata["proposal_state"], "failed");
        assert_eq!(metadata["failed_via"], "planning_profile_apply");
        assert_eq!(
            metadata["outcome_summary"],
            "routine block missing not found"
        );
        assert_eq!(thread.3, "open");
    }

    #[tokio::test]
    async fn proposal_summary_reports_pending_and_recent_outcomes() {
        let storage = test_storage().await;
        storage
            .insert_thread(
                "thr_planning_profile_edit_recent_pending",
                "planning_profile_edit",
                "Add shutdown block",
                "open",
                &json!({
                    "source": "planning_profile_proposal",
                    "proposal_state": "staged",
                    "summary": "Add a protected shutdown block.",
                    "requires_confirmation": true,
                    "continuity": "thread",
                    "mutation": {
                        "kind": "upsert_routine_block",
                        "data": {
                            "id": "routine_shutdown",
                            "label": "Shutdown",
                            "source": "operator_declared",
                            "local_timezone": "America/Denver",
                            "start_local_time": "17:00",
                            "end_local_time": "17:30",
                            "days_of_week": [1, 2, 3, 4, 5],
                            "protected": true,
                            "active": true
                        }
                    },
                    "lineage": {
                        "source_surface": "assistant"
                    }
                })
                .to_string(),
            )
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_planning_profile_edit_recent_applied",
                "planning_profile_edit",
                "Save morning focus window",
                "resolved",
                &json!({
                    "source": "planning_profile_proposal",
                    "proposal_state": "applied",
                    "summary": "Save a morning focus window.",
                    "requires_confirmation": true,
                    "continuity": "thread",
                    "outcome_summary": "Planning-profile proposal applied through canonical mutation seam.",
                    "mutation": {
                        "kind": "upsert_planning_constraint",
                        "data": {
                            "id": "constraint_default_window",
                            "label": "Morning default",
                            "kind": "default_time_window",
                            "detail": null,
                            "time_window": "prenoon",
                            "minutes": null,
                            "max_items": null,
                            "active": true
                        }
                    },
                    "lineage": {
                        "source_surface": "assistant"
                    }
                })
                .to_string(),
            )
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_planning_profile_edit_recent_failed",
                "planning_profile_edit",
                "Remove missing block",
                "open",
                &json!({
                    "source": "planning_profile_proposal",
                    "proposal_state": "failed",
                    "summary": "Remove missing block.",
                    "requires_confirmation": true,
                    "continuity": "thread",
                    "outcome_summary": "routine block missing not found",
                    "mutation": {
                        "kind": "remove_routine_block",
                        "data": {
                            "id": "missing"
                        }
                    },
                    "lineage": {
                        "source_surface": "assistant"
                    }
                })
                .to_string(),
            )
            .await
            .unwrap();

        let summary = load_planning_profile_proposal_summary(&storage)
            .await
            .unwrap();

        assert_eq!(summary.pending_count, 1);
        assert_eq!(
            summary
                .latest_pending
                .as_ref()
                .map(|item| item.thread_id.as_str()),
            Some("thr_planning_profile_edit_recent_pending")
        );
        assert_eq!(
            summary.latest_applied.as_ref().map(|item| item.state),
            Some(AssistantProposalState::Applied)
        );
        assert_eq!(
            summary
                .latest_failed
                .as_ref()
                .and_then(|item| item.outcome_summary.as_deref()),
            Some("routine block missing not found")
        );
    }

    #[tokio::test]
    async fn loads_operator_declared_routine_blocks_for_current_day() {
        let storage = test_storage().await;
        save_routine_planning_profile(
            &storage,
            &RoutinePlanningProfile {
                routine_blocks: vec![DurableRoutineBlock {
                    id: "routine_focus".to_string(),
                    label: "Focus block".to_string(),
                    source: RoutineBlockSourceKind::OperatorDeclared,
                    local_timezone: "America/Denver".to_string(),
                    start_local_time: "09:00".to_string(),
                    end_local_time: "11:00".to_string(),
                    days_of_week: vec![1],
                    protected: true,
                    active: true,
                }],
                planning_constraints: vec![],
            },
        )
        .await
        .unwrap();

        let inputs = load_day_planning_inputs(
            &storage,
            &CurrentContextV1 {
                prep_window_active: true,
                mode: "morning_mode".to_string(),
                ..CurrentContextV1::default()
            },
            1_710_788_400,
        )
        .await
        .unwrap();

        assert_eq!(inputs.routine_blocks.len(), 1);
        assert_eq!(
            inputs.routine_blocks[0].source,
            RoutineBlockSourceKind::OperatorDeclared
        );
    }

    #[tokio::test]
    async fn falls_back_to_inferred_routine_blocks_without_durable_profile() {
        let storage = test_storage().await;
        let inputs = load_day_planning_inputs(
            &storage,
            &CurrentContextV1 {
                prep_window_active: true,
                mode: "morning_mode".to_string(),
                ..CurrentContextV1::default()
            },
            1_710_788_400,
        )
        .await
        .unwrap();

        assert_eq!(inputs.routine_blocks.len(), 1);
        assert_eq!(
            inputs.routine_blocks[0].source,
            RoutineBlockSourceKind::Inferred
        );
    }

    #[test]
    fn extracts_constraint_helpers() {
        let constraints = vec![
            PlanningConstraint {
                id: "default_window".to_string(),
                label: "Prefer prenoon".to_string(),
                kind: PlanningConstraintKind::DefaultTimeWindow,
                detail: None,
                time_window: Some(ScheduleTimeWindow::Prenoon),
                minutes: None,
                max_items: None,
                active: true,
            },
            PlanningConstraint {
                id: "cap".to_string(),
                label: "Cap to three".to_string(),
                kind: PlanningConstraintKind::MaxScheduledItems,
                detail: None,
                time_window: None,
                minutes: None,
                max_items: Some(3),
                active: true,
            },
            PlanningConstraint {
                id: "buffer_before".to_string(),
                label: "Reserve 15".to_string(),
                kind: PlanningConstraintKind::ReserveBufferBeforeCalendar,
                detail: None,
                time_window: None,
                minutes: Some(15),
                max_items: None,
                active: true,
            },
            PlanningConstraint {
                id: "overflow".to_string(),
                label: "Judgment".to_string(),
                kind: PlanningConstraintKind::RequireJudgmentForOverflow,
                detail: None,
                time_window: None,
                minutes: None,
                max_items: None,
                active: true,
            },
        ];

        assert_eq!(
            default_time_window(&constraints),
            Some(ScheduleTimeWindow::Prenoon)
        );
        assert_eq!(max_scheduled_items(&constraints), Some(3));
        assert_eq!(reserve_buffer_before_calendar_minutes(&constraints), 15);
        assert!(require_judgment_for_overflow(&constraints));
    }
}
