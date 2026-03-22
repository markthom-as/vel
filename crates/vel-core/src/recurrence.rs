use serde::{Deserialize, Serialize};

use crate::{EventId, EventMoment};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecurrenceWeekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeriesRule {
    pub frequency: RecurrenceFrequency,
    pub interval: u16,
    #[serde(default)]
    pub by_weekdays: Vec<RecurrenceWeekday>,
    pub count: Option<u32>,
    pub until: Option<String>,
    pub raw_rrule: Option<String>,
}

impl SeriesRule {
    pub fn validate(&self) -> Result<(), String> {
        if self.interval == 0 {
            return Err("RRULE interval must be greater than zero".to_string());
        }
        if matches!(self.frequency, RecurrenceFrequency::Weekly) && self.by_weekdays.is_empty() {
            return Err("weekly RRULE requires at least one weekday".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExceptionStatus {
    Modified,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exception {
    pub occurrence_key: String,
    pub status: ExceptionStatus,
    pub replacement_start: Option<EventMoment>,
    pub replacement_end: Option<EventMoment>,
}

impl Exception {
    pub fn validate(&self) -> Result<(), String> {
        if self.occurrence_key.trim().is_empty() {
            return Err("recurrence exception occurrence_key must not be empty".to_string());
        }
        if matches!(self.status, ExceptionStatus::Modified)
            && (self.replacement_start.is_none() || self.replacement_end.is_none())
        {
            return Err("modified recurrence exception requires replacement start and end".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Series {
    pub series_id: String,
    pub anchor_event_id: EventId,
    pub timezone: Option<String>,
    pub rule: SeriesRule,
    #[serde(default)]
    pub exceptions: Vec<Exception>,
}

impl Series {
    pub fn validate(&self) -> Result<(), String> {
        if self.series_id.trim().is_empty() {
            return Err("series_id must not be empty".to_string());
        }
        self.rule.validate()?;
        for exception in &self.exceptions {
            exception.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Occurrence {
    pub occurrence_key: String,
    pub series_id: String,
    pub anchor_event_id: EventId,
    pub start: EventMoment,
    pub end: EventMoment,
    pub materialized: bool,
    pub exception_status: Option<ExceptionStatus>,
}

#[cfg(test)]
mod tests {
    use super::{
        Exception, ExceptionStatus, RecurrenceFrequency, RecurrenceWeekday, Series, SeriesRule,
    };
    use crate::{EventId, EventMoment, EventMomentKind};

    #[test]
    fn weekly_series_requires_weekday_and_modified_exception_requires_replacements() {
        let invalid_rule = SeriesRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            by_weekdays: vec![],
            count: None,
            until: None,
            raw_rrule: Some("FREQ=WEEKLY".to_string()),
        };
        assert!(invalid_rule.validate().unwrap_err().contains("weekday"));

        let invalid_exception = Exception {
            occurrence_key: "event_01:2026-03-23T08:00:00-06:00".to_string(),
            status: ExceptionStatus::Modified,
            replacement_start: None,
            replacement_end: None,
        };
        assert!(invalid_exception.validate().unwrap_err().contains("replacement"));

        let valid = Series {
            series_id: "series_01".to_string(),
            anchor_event_id: EventId::new(),
            timezone: Some("America/Denver".to_string()),
            rule: SeriesRule {
                frequency: RecurrenceFrequency::Weekly,
                interval: 1,
                by_weekdays: vec![RecurrenceWeekday::Monday],
                count: Some(4),
                until: None,
                raw_rrule: Some("FREQ=WEEKLY;BYDAY=MO;COUNT=4".to_string()),
            },
            exceptions: vec![Exception {
                occurrence_key: "event_01:2026-03-24T08:00:00-06:00".to_string(),
                status: ExceptionStatus::Cancelled,
                replacement_start: None,
                replacement_end: None,
            }],
        };

        valid.validate().unwrap();

        let replacement = EventMoment {
            kind: EventMomentKind::ZonedDateTime,
            value: "2026-03-24T09:00:00-06:00".to_string(),
            timezone: Some("America/Denver".to_string()),
        };
        Exception {
            occurrence_key: "event_01:2026-03-24T08:00:00-06:00".to_string(),
            status: ExceptionStatus::Modified,
            replacement_start: Some(replacement.clone()),
            replacement_end: Some(replacement),
        }
        .validate()
        .unwrap();
    }
}
