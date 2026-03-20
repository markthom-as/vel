use crate::{ScheduleRuleFacet, ScheduleRuleFacetKind};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleTimeWindow {
    Prenoon,
    Afternoon,
    Evening,
    Night,
    Day,
}

impl ScheduleTimeWindow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prenoon => "prenoon",
            Self::Afternoon => "afternoon",
            Self::Evening => "evening",
            Self::Night => "night",
            Self::Day => "day",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalScheduleRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i64>,
    #[serde(default)]
    pub calendar_free: bool,
    #[serde(default)]
    pub fixed_start: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<ScheduleTimeWindow>,
    #[serde(default)]
    pub local_urgency: bool,
    #[serde(default)]
    pub local_defer: bool,
}

impl CanonicalScheduleRules {
    pub fn normalize(text: &str, labels: &[String], due_ts: Option<i64>) -> Self {
        let normalized_labels = labels
            .iter()
            .map(|label| label.trim().to_ascii_lowercase())
            .filter(|label| !label.is_empty())
            .collect::<Vec<_>>();

        let block_target = normalized_labels
            .iter()
            .find_map(|label| label.strip_prefix("block:").map(str::to_string));
        let duration_minutes = normalized_labels
            .iter()
            .find_map(|label| parse_duration_token(label))
            .or_else(|| {
                text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '@'))
                    .find_map(parse_duration_token)
            });
        let calendar_free = normalized_labels
            .iter()
            .any(|label| label == "cal:free" || label == "@cal:free");
        let fixed_start = normalized_labels
            .iter()
            .any(|label| label == "fixed" || label == "fixed_start")
            || due_ts.filter(|timestamp| timestamp % 86_400 != 0).is_some();
        let time_window = normalized_labels.iter().find_map(|label| {
            if label.ends_with("prenoon") {
                Some(ScheduleTimeWindow::Prenoon)
            } else if label.ends_with("afternoon") {
                Some(ScheduleTimeWindow::Afternoon)
            } else if label.ends_with("evening") {
                Some(ScheduleTimeWindow::Evening)
            } else if label.ends_with("night") {
                Some(ScheduleTimeWindow::Night)
            } else if label.ends_with("day") {
                Some(ScheduleTimeWindow::Day)
            } else {
                None
            }
        });
        let local_urgency = normalized_labels
            .iter()
            .any(|label| label == "urgent" || label == "@urgent");
        let local_defer = normalized_labels
            .iter()
            .any(|label| label == "defer" || label == "@defer");

        Self {
            block_target,
            duration_minutes,
            calendar_free,
            fixed_start,
            time_window,
            local_urgency,
            local_defer,
        }
    }

    pub fn to_rule_facets(&self) -> Vec<ScheduleRuleFacet> {
        let mut facets = Vec::new();
        if let Some(block) = &self.block_target {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::BlockTarget,
                label: format!("block:{block}"),
                detail: Some("Task prefers a named block target.".to_string()),
            });
        }
        if let Some(duration_minutes) = self.duration_minutes {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::Duration,
                label: format!("{duration_minutes}m"),
                detail: Some("Duration came from canonical scheduler normalization.".to_string()),
            });
        }
        if self.calendar_free {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::CalendarFree,
                label: "cal:free".to_string(),
                detail: Some("Task prefers free calendar space.".to_string()),
            });
        }
        if self.fixed_start {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::FixedStart,
                label: "fixed_start".to_string(),
                detail: Some("Task has an anchored scheduled start.".to_string()),
            });
        }
        if let Some(window) = self.time_window {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::TimeWindow,
                label: format!("time:{}", window.as_str()),
                detail: Some("Task prefers a bounded part of the day.".to_string()),
            });
        }
        if self.local_urgency {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::LocalUrgency,
                label: "urgent".to_string(),
                detail: Some("Task carries local urgency.".to_string()),
            });
        }
        if self.local_defer {
            facets.push(ScheduleRuleFacet {
                kind: ScheduleRuleFacetKind::LocalDefer,
                label: "defer".to_string(),
                detail: Some("Task is marked for local defer logic.".to_string()),
            });
        }
        facets
    }

    pub fn from_metadata(metadata: &JsonValue) -> Option<Self> {
        metadata
            .get("scheduler_rules")
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
    }

    pub fn from_commitment_parts(
        text: &str,
        metadata: &JsonValue,
        due_at: Option<OffsetDateTime>,
    ) -> Self {
        Self::from_metadata(metadata).unwrap_or_else(|| {
            let labels = metadata
                .get("labels")
                .and_then(JsonValue::as_array)
                .into_iter()
                .flatten()
                .filter_map(JsonValue::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>();
            Self::normalize(text, &labels, due_at.map(|value| value.unix_timestamp()))
        })
    }

    pub fn write_into_metadata(&self, metadata: &mut JsonValue) {
        let object = ensure_object(metadata);
        object.insert("scheduler_rules".to_string(), json!(self));
    }
}

fn parse_duration_token(token: &str) -> Option<i64> {
    let normalized = token.trim().trim_start_matches('@').to_ascii_lowercase();
    if let Some(minutes) = normalized.strip_suffix('m') {
        return minutes.parse::<i64>().ok().filter(|value| *value > 0);
    }
    if let Some(hours) = normalized.strip_suffix('h') {
        return hours
            .parse::<i64>()
            .ok()
            .filter(|value| *value > 0)
            .map(|value| value * 60);
    }
    None
}

fn ensure_object(metadata: &mut JsonValue) -> &mut serde_json::Map<String, JsonValue> {
    if !metadata.is_object() {
        *metadata = json!({});
    }
    metadata
        .as_object_mut()
        .expect("scheduler metadata should normalize to an object")
}

#[cfg(test)]
mod tests {
    use super::{CanonicalScheduleRules, ScheduleTimeWindow};
    use serde_json::json;

    #[test]
    fn normalizes_codex_workspace_style_scheduler_rules() {
        let rules = CanonicalScheduleRules::normalize(
            "Deep work @30m",
            &[
                "block:focus".to_string(),
                "@cal:free".to_string(),
                "time:prenoon".to_string(),
                "@urgent".to_string(),
            ],
            None,
        );

        assert_eq!(rules.block_target.as_deref(), Some("focus"));
        assert_eq!(rules.duration_minutes, Some(30));
        assert!(rules.calendar_free);
        assert_eq!(rules.time_window, Some(ScheduleTimeWindow::Prenoon));
        assert!(rules.local_urgency);
        assert!(!rules.local_defer);
        assert!(!rules.fixed_start);
    }

    #[test]
    fn derives_fixed_start_from_due_datetime_and_defer_from_labels() {
        let rules = CanonicalScheduleRules::normalize(
            "Write proposal",
            &["defer".to_string()],
            Some(1_700_000_123),
        );

        assert!(rules.fixed_start);
        assert!(rules.local_defer);
        assert_eq!(rules.duration_minutes, None);
    }

    #[test]
    fn converts_canonical_rules_into_explainable_facets() {
        let rules = CanonicalScheduleRules::normalize(
            "Inbox cleanup @1h",
            &["block:admin".to_string(), "time:afternoon".to_string()],
            None,
        );

        let facets = rules.to_rule_facets();
        assert_eq!(facets.len(), 3);
        assert_eq!(facets[0].label, "block:admin");
        assert_eq!(facets[1].label, "60m");
        assert_eq!(facets[2].label, "time:afternoon");
    }

    #[test]
    fn round_trips_through_metadata() {
        let rules = CanonicalScheduleRules::normalize(
            "Deep work @30m",
            &["block:focus".to_string(), "@urgent".to_string()],
            None,
        );
        let mut metadata = json!({
            "labels": ["block:focus", "@urgent"]
        });

        rules.write_into_metadata(&mut metadata);
        let restored = CanonicalScheduleRules::from_metadata(&metadata).expect("stored rules");

        assert_eq!(restored, rules);
    }
}
