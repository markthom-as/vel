use serde_json::{json, Value as JsonValue};
use vel_core::{AvailabilityPolicyConfig, Calendar, Event, ParticipationResponseStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct GoogleAvailabilityBridgeInput {
    pub calendar: Calendar,
    pub event: Event,
    pub source_account_id: Option<String>,
    pub response_status: Option<ParticipationResponseStatus>,
    pub cancelled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleAvailabilityProjectionEnvelope {
    pub projection_id: String,
    pub rebuild_token: String,
    pub explain_metadata: JsonValue,
}

pub fn bridge_google_availability_input(
    calendar: Calendar,
    event: Event,
    source_account_id: Option<String>,
    response_status: Option<ParticipationResponseStatus>,
    cancelled: bool,
) -> GoogleAvailabilityBridgeInput {
    GoogleAvailabilityBridgeInput {
        calendar,
        event,
        source_account_id,
        response_status,
        cancelled,
    }
}

pub fn google_availability_projection_envelope(
    integration_account_id: &str,
    config: &AvailabilityPolicyConfig,
    window_start: &str,
    window_end: &str,
) -> GoogleAvailabilityProjectionEnvelope {
    GoogleAvailabilityProjectionEnvelope {
        projection_id: format!(
            "projection.availability.google-calendar:{integration_account_id}:{window_start}:{window_end}"
        ),
        rebuild_token: format!(
            "google-calendar:availability:{integration_account_id}:{window_start}:{window_end}"
        ),
        explain_metadata: json!({
            "rebuildable": true,
            "derived": true,
            "basis": "native_projection",
            "included_calendar_ids": config
                .included_calendar_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            "source_account_ids": config.source_account_ids,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{bridge_google_availability_input, google_availability_projection_envelope};
    use serde_json::Value as JsonValue;
    use vel_core::{
        AllDayHandlingRule, AvailabilityPolicyConfig, Calendar, CalendarId, CalendarVisibility,
        DeclinedResponsePolicy, Event, EventId, EventMoment, EventMomentKind, EventTransparency,
        ParticipationResponseStatus,
    };

    fn calendar() -> Calendar {
        Calendar {
            id: CalendarId::from("calendar_primary".to_string()),
            display_name: "Primary".to_string(),
            timezone: "UTC".to_string(),
            visibility: CalendarVisibility::Private,
            is_default: true,
            description: None,
            color: None,
        }
    }

    fn event() -> Event {
        Event {
            id: EventId::from("event_busy".to_string()),
            title: "Focus".to_string(),
            description: None,
            start: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-23T08:00:00Z".to_string(),
                timezone: Some("UTC".to_string()),
            },
            end: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-23T09:00:00Z".to_string(),
                timezone: Some("UTC".to_string()),
            },
            transparency: EventTransparency::Opaque,
            location: None,
        }
    }

    #[test]
    fn availability_bridge_preserves_native_inputs_and_rebuild_metadata() {
        let input = bridge_google_availability_input(
            calendar(),
            event(),
            Some("integration_account_google".to_string()),
            Some(ParticipationResponseStatus::Accepted),
            false,
        );
        assert_eq!(input.calendar.display_name, "Primary");
        assert_eq!(input.event.title, "Focus");

        let envelope = google_availability_projection_envelope(
            "integration_account_google",
            &AvailabilityPolicyConfig {
                included_calendar_ids: vec![CalendarId::from("calendar_primary".to_string())],
                source_account_ids: vec!["integration_account_google".to_string()],
                declined_response_policy: DeclinedResponsePolicy::IgnoreDeclined,
                exclude_cancelled_events: true,
                all_day_handling_rule: AllDayHandlingRule::RespectTransparency,
            },
            "2026-03-23T08:00:00Z",
            "2026-03-23T09:00:00Z",
        );

        assert!(envelope.projection_id.contains("projection.availability"));
        assert_eq!(envelope.explain_metadata["basis"], "native_projection");
        assert_eq!(
            envelope.explain_metadata["rebuildable"],
            JsonValue::Bool(true)
        );
    }
}
