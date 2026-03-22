use serde::{Deserialize, Serialize};

use crate::EventId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventMomentKind {
    AllDay,
    FloatingDateTime,
    ZonedDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventMoment {
    pub kind: EventMomentKind,
    pub value: String,
    pub timezone: Option<String>,
}

impl EventMoment {
    pub fn validate(&self, label: &str) -> Result<(), String> {
        if self.value.trim().is_empty() {
            return Err(format!("event {label} must not be empty"));
        }

        match self.kind {
            EventMomentKind::ZonedDateTime => {
                if self.timezone.as_deref().unwrap_or_default().trim().is_empty() {
                    return Err(format!("event {label} zoned_datetime requires timezone"));
                }
            }
            EventMomentKind::AllDay | EventMomentKind::FloatingDateTime => {
                if self.timezone.is_some() {
                    return Err(format!(
                        "event {label} {:?} must not carry timezone",
                        self.kind
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTransparency {
    Opaque,
    Transparent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventLocation {
    pub label: String,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub uri: Option<String>,
}

impl EventLocation {
    pub fn validate(&self) -> Result<(), String> {
        if self.label.trim().is_empty() {
            return Err("event location label must not be empty".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub title: String,
    pub description: Option<String>,
    pub start: EventMoment,
    pub end: EventMoment,
    pub transparency: EventTransparency,
    pub location: Option<EventLocation>,
}

impl Event {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("event title must not be empty".to_string());
        }

        self.start.validate("start")?;
        self.end.validate("end")?;

        if self.start.kind != self.end.kind {
            return Err("event start and end must use the same moment kind".to_string());
        }

        if let Some(location) = &self.location {
            location.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Event, EventLocation, EventMoment, EventMomentKind, EventTransparency,
    };
    use crate::EventId;

    #[test]
    fn event_requires_matching_time_kinds_and_location_payloads() {
        let mismatched = Event {
            id: EventId::new(),
            title: "Planning".to_string(),
            description: None,
            start: EventMoment {
                kind: EventMomentKind::FloatingDateTime,
                value: "2026-03-22T09:00:00".to_string(),
                timezone: None,
            },
            end: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-22T10:00:00-06:00".to_string(),
                timezone: Some("America/Denver".to_string()),
            },
            transparency: EventTransparency::Opaque,
            location: None,
        };

        assert!(mismatched.validate().unwrap_err().contains("same moment kind"));

        let valid = Event {
            id: EventId::new(),
            title: "Planning".to_string(),
            description: Some("Native event".to_string()),
            start: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-22T09:00:00-06:00".to_string(),
                timezone: Some("America/Denver".to_string()),
            },
            end: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-22T10:00:00-06:00".to_string(),
                timezone: Some("America/Denver".to_string()),
            },
            transparency: EventTransparency::Transparent,
            location: Some(EventLocation {
                label: "Home Office".to_string(),
                address: None,
                notes: Some("Desk".to_string()),
                uri: None,
            }),
        };

        valid.validate().unwrap();
    }
}
