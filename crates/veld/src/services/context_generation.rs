//! Context generation: today, morning, end-of-day. Pure functions over orientation snapshot.

use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use vel_core::{
    Clock, ContextCapture, HybridRetrievalPolicy, OrientationSnapshot, RetrievalStrategy,
    SemanticQuery, SemanticQueryFilters, SemanticSourceKind, SystemClock,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodayContextData {
    pub date: String,
    pub recent_captures: Vec<ContextCapture>,
    pub focus_candidates: Vec<String>,
    pub reminders: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MorningContextData {
    pub date: String,
    pub top_active_threads: Vec<String>,
    pub pending_commitments: Vec<String>,
    pub suggested_focus: Option<String>,
    pub key_reminders: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EndOfDayContextData {
    pub date: String,
    pub what_was_done: Vec<ContextCapture>,
    pub what_remains_open: Vec<String>,
    pub what_may_matter_tomorrow: Vec<String>,
}

pub fn build_today(snapshot: &OrientationSnapshot) -> TodayContextData {
    build_today_at(snapshot, SystemClock.now())
}

pub fn build_today_at(snapshot: &OrientationSnapshot, now: OffsetDateTime) -> TodayContextData {
    let source_text = combined_source_text(snapshot);
    let reminders = extract_commitments(source_text.iter().map(String::as_str));
    let focus_candidates = extract_focus_candidates(source_text.iter().map(String::as_str));
    TodayContextData {
        date: now.date().to_string(),
        recent_captures: snapshot.recent_today.clone(),
        focus_candidates,
        reminders: reminders.into_iter().take(5).collect(),
    }
}

pub fn build_morning(snapshot: &OrientationSnapshot) -> MorningContextData {
    build_morning_at(snapshot, SystemClock.now())
}

pub fn build_morning_at(snapshot: &OrientationSnapshot, now: OffsetDateTime) -> MorningContextData {
    let source_text = combined_source_text(snapshot);
    let top_active_threads = extract_focus_candidates(source_text.iter().map(String::as_str));
    let pending_commitments = extract_commitments(source_text.iter().map(String::as_str));
    MorningContextData {
        date: now.date().to_string(),
        suggested_focus: top_active_threads.first().cloned(),
        key_reminders: pending_commitments.iter().take(5).cloned().collect(),
        top_active_threads,
        pending_commitments,
    }
}

pub fn build_end_of_day(snapshot: &OrientationSnapshot) -> EndOfDayContextData {
    build_end_of_day_at(snapshot, SystemClock.now())
}

pub fn build_end_of_day_at(
    snapshot: &OrientationSnapshot,
    now: OffsetDateTime,
) -> EndOfDayContextData {
    let source_text = combined_source_text(snapshot);
    let what_remains_open = extract_commitments(source_text.iter().map(String::as_str));
    let what_may_matter_tomorrow = extract_focus_candidates(source_text.iter().map(String::as_str));
    EndOfDayContextData {
        date: now.date().to_string(),
        what_was_done: snapshot.recent_today.clone(),
        what_remains_open: what_remains_open.into_iter().take(10).collect(),
        what_may_matter_tomorrow: what_may_matter_tomorrow.into_iter().take(5).collect(),
    }
}

pub fn semantic_query_for_snapshot(snapshot: &OrientationSnapshot) -> Option<SemanticQuery> {
    let source_text = combined_source_text(snapshot);
    let focus_candidates = extract_focus_candidates(source_text.iter().map(String::as_str));
    let reminders = extract_commitments(source_text.iter().map(String::as_str));

    let mut parts = Vec::new();
    parts.extend(focus_candidates.into_iter().take(3));
    parts.extend(
        reminders
            .into_iter()
            .take(2)
            .map(|value| truncate_query_phrase(&value, 48)),
    );

    let query_text = parts.join(" ").trim().to_string();
    if query_text.is_empty() {
        return None;
    }

    Some(SemanticQuery {
        query_text,
        top_k: 5,
        strategy: RetrievalStrategy::Hybrid,
        include_provenance: true,
        filters: SemanticQueryFilters {
            source_kinds: vec![SemanticSourceKind::Capture],
            ..Default::default()
        },
        policy: Some(HybridRetrievalPolicy {
            lexical_weight: 0.35,
            semantic_weight: 0.65,
            rerank_window: 12,
            min_combined_score: 0.05,
        }),
    })
}

fn combined_source_text(snapshot: &OrientationSnapshot) -> Vec<String> {
    snapshot
        .recent_week
        .iter()
        .map(|capture| capture.content_text.clone())
        .chain(snapshot.recent_signal_summaries.iter().cloned())
        .collect()
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
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_lowercase())
        .collect()
}

fn stopwords() -> HashSet<&'static str> {
    [
        "about",
        "after",
        "again",
        "also",
        "been",
        "budget",
        "capture",
        "chorus",
        "could",
        "from",
        "have",
        "idea",
        "into",
        "just",
        "like",
        "memo",
        "more",
        "note",
        "notes",
        "project",
        "quick",
        "remembering",
        "should",
        "some",
        "that",
        "this",
        "today",
        "what",
        "with",
        "work",
        "would",
    ]
    .into_iter()
    .collect()
}

fn truncate_query_phrase(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_focus_candidates_from_recent_captures() {
        let threads = extract_focus_candidates(
            [
                "remember lidar budget",
                "lidar estimate follow up",
                "budget draft",
            ]
            .into_iter(),
        );
        assert!(threads.contains(&"lidar".to_string()));
    }

    #[test]
    fn extracts_commitments_from_marker_phrases() {
        let commitments = extract_commitments(
            [
                "remember lidar budget",
                "normal note",
                "follow up with Cornelius",
            ]
            .into_iter(),
        );
        assert_eq!(commitments.len(), 2);
    }

    #[test]
    fn signal_summaries_contribute_to_brief_terms() {
        let snapshot = OrientationSnapshot {
            recent_today: Vec::new(),
            recent_week: Vec::new(),
            recent_signal_summaries: vec![
                "todo follow up with Dimitri on forecast".to_string(),
                "waiting on me forecast review".to_string(),
            ],
        };

        let today = build_today(&snapshot);
        assert!(today
            .reminders
            .iter()
            .any(|item| item.contains("follow up")));
        assert!(today
            .focus_candidates
            .iter()
            .any(|item| item.contains("forecast")));
    }

    #[test]
    fn build_today_uses_supplied_time_for_date() {
        let snapshot = OrientationSnapshot {
            recent_today: Vec::new(),
            recent_week: Vec::new(),
            recent_signal_summaries: Vec::new(),
        };

        let today = build_today_at(&snapshot, time::macros::datetime!(2026-03-17 23:59:00 UTC));

        assert_eq!(today.date, "2026-03-17");
    }

    #[test]
    fn semantic_query_is_derived_from_snapshot_terms() {
        let snapshot = OrientationSnapshot {
            recent_today: Vec::new(),
            recent_week: vec![ContextCapture {
                capture_id: "cap_tax".to_string().into(),
                capture_type: "quick_note".to_string(),
                content_text: "remember accountant tax estimate follow up".to_string(),
                occurred_at: time::OffsetDateTime::now_utc(),
                source_device: None,
            }],
            recent_signal_summaries: vec!["todo finish tax draft".to_string()],
        };

        let query = semantic_query_for_snapshot(&snapshot).expect("query should exist");

        assert_eq!(query.strategy, RetrievalStrategy::Hybrid);
        assert!(query.query_text.contains("accountant") || query.query_text.contains("tax"));
        assert_eq!(
            query.filters.source_kinds,
            vec![SemanticSourceKind::Capture]
        );
    }

    #[test]
    fn semantic_query_is_absent_for_empty_snapshot() {
        let snapshot = OrientationSnapshot {
            recent_today: Vec::new(),
            recent_week: Vec::new(),
            recent_signal_summaries: Vec::new(),
        };

        assert!(semantic_query_for_snapshot(&snapshot).is_none());
    }
}
