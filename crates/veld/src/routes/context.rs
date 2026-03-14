use axum::{extract::State, Json};
use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{ApiResponse, EndOfDayData, MorningData, TodayData};

use crate::{errors::AppError, state::AppState};

pub async fn today(State(state): State<AppState>) -> Result<Json<ApiResponse<TodayData>>, AppError> {
    let snapshot = state.storage.orientation_snapshot().await?;
    let reminders = extract_commitments(snapshot.recent_week.iter().map(|capture| capture.content_text.as_str()));
    let focus_candidates = extract_focus_candidates(snapshot.recent_week.iter().map(|capture| capture.content_text.as_str()));
    let request_id = format!("req_{}", Uuid::new_v4().simple());

    Ok(Json(ApiResponse::success(
        TodayData {
            date: OffsetDateTime::now_utc().date().to_string(),
            recent_captures: snapshot.recent_today,
            focus_candidates,
            reminders: reminders.into_iter().take(5).collect(),
        },
        request_id,
    )))
}

pub async fn morning(State(state): State<AppState>) -> Result<Json<ApiResponse<MorningData>>, AppError> {
    let snapshot = state.storage.orientation_snapshot().await?;
    let top_active_threads = extract_focus_candidates(snapshot.recent_week.iter().map(|capture| capture.content_text.as_str()));
    let pending_commitments = extract_commitments(snapshot.recent_week.iter().map(|capture| capture.content_text.as_str()));
    let suggested_focus = top_active_threads.first().cloned();
    let key_reminders = pending_commitments.iter().take(5).cloned().collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());

    Ok(Json(ApiResponse::success(
        MorningData {
            date: OffsetDateTime::now_utc().date().to_string(),
            top_active_threads,
            pending_commitments,
            suggested_focus,
            key_reminders,
        },
        request_id,
    )))
}

pub async fn end_of_day(State(state): State<AppState>) -> Result<Json<ApiResponse<EndOfDayData>>, AppError> {
    let snapshot = state.storage.orientation_snapshot().await?;
    let what_was_done = snapshot.recent_today;
    let what_remains_open = extract_commitments(snapshot.recent_week.iter().map(|capture| capture.content_text.as_str()));
    let what_may_matter_tomorrow = extract_focus_candidates(snapshot.recent_week.iter().map(|capture| capture.content_text.as_str()));
    let request_id = format!("req_{}", Uuid::new_v4().simple());

    Ok(Json(ApiResponse::success(
        EndOfDayData {
            date: OffsetDateTime::now_utc().date().to_string(),
            what_was_done,
            what_remains_open: what_remains_open.into_iter().take(10).collect(),
            what_may_matter_tomorrow: what_may_matter_tomorrow.into_iter().take(5).collect(),
        },
        request_id,
    )))
}

fn extract_focus_candidates<'a>(captures: impl Iterator<Item = &'a str>) -> Vec<String> {
    let stopwords = stopwords();
    let mut counts = HashMap::<String, usize>::new();

    for capture in captures {
        let mut seen = HashSet::new();
        for token in tokenize(capture) {
            if token.len() < 4 || stopwords.contains(token.as_str()) {
                continue;
            }
            if seen.insert(token.clone()) {
                *counts.entry(token).or_default() += 1;
            }
        }
    }

    let mut ranked = counts.into_iter().collect::<Vec<_>>();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.into_iter().take(5).map(|(term, _)| term).collect()
}

fn extract_commitments<'a>(captures: impl Iterator<Item = &'a str>) -> Vec<String> {
    let markers = [
        "remember",
        "follow up",
        "todo",
        "need to",
        "waiting on",
        "should",
        "must",
    ];
    let mut commitments = Vec::new();
    let mut seen = HashSet::new();

    for capture in captures {
        let lower = capture.to_lowercase();
        if markers.iter().any(|marker| lower.contains(marker)) && seen.insert(capture.to_string()) {
            commitments.push(capture.to_string());
        }
    }

    commitments.truncate(5);
    commitments
}

fn tokenize(input: &str) -> Vec<String> {
    input.split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_lowercase())
        .collect()
}

fn stopwords() -> HashSet<&'static str> {
    [
        "about", "after", "again", "also", "been", "budget", "capture", "chorus", "could", "from",
        "have", "idea", "into", "just", "like", "memo", "more", "note", "notes", "project", "quick",
        "remembering", "should", "some", "that", "this", "today", "what", "with", "work", "would",
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_focus_candidates_from_recent_captures() {
        let threads = extract_focus_candidates(
            ["remember lidar budget", "lidar estimate follow up", "budget draft"]
                .into_iter(),
        );

        assert!(threads.contains(&"lidar".to_string()));
    }

    #[test]
    fn extracts_commitments_from_marker_phrases() {
        let commitments = extract_commitments(
            ["remember lidar budget", "normal note", "follow up with Cornelius"].into_iter(),
        );

        assert_eq!(commitments.len(), 2);
    }
}
