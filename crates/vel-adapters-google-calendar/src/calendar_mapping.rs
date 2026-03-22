use serde_json::{Value as JsonValue, json};
use vel_core::{Calendar, CalendarId, CalendarVisibility};

use crate::google_ids::{GOOGLE_CALENDAR_MODULE_ID, google_provider_object_ref};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleCalendarMappingPayload {
    pub remote_id: String,
    pub summary: String,
    pub timezone: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub access_role: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoogleMappedCalendar {
    pub calendar: Calendar,
    pub provider_facets: JsonValue,
}

pub fn map_google_calendar(
    calendar_id: CalendarId,
    payload: &GoogleCalendarMappingPayload,
) -> GoogleMappedCalendar {
    GoogleMappedCalendar {
        calendar: Calendar {
            id: calendar_id,
            display_name: payload.summary.clone(),
            timezone: payload.timezone.clone(),
            visibility: visibility(&payload.access_role),
            is_default: payload.is_primary,
            description: payload.description.clone(),
            color: payload.color.clone(),
        },
        provider_facets: json!({
            "google_calendar": {
                "calendar_id": payload.remote_id,
                "access_role": payload.access_role,
                "source_ref": google_provider_object_ref("calendar", &payload.remote_id),
                "module_id": GOOGLE_CALENDAR_MODULE_ID,
            }
        }),
    }
}

fn visibility(access_role: &str) -> CalendarVisibility {
    match access_role {
        "reader" | "freeBusyReader" => CalendarVisibility::Shared,
        "public" => CalendarVisibility::Public,
        _ => CalendarVisibility::Private,
    }
}

#[cfg(test)]
mod tests {
    use super::{GoogleCalendarMappingPayload, map_google_calendar};
    use vel_core::{CalendarId, CalendarVisibility};

    #[test]
    fn calendar_mapping_keeps_native_calendar_timezone_visibility_and_default_posture() {
        let mapped = map_google_calendar(
            CalendarId::from("calendar_primary".to_string()),
            &GoogleCalendarMappingPayload {
                remote_id: "primary".to_string(),
                summary: "Personal".to_string(),
                timezone: "America/Denver".to_string(),
                description: Some("Primary".to_string()),
                color: Some("#123456".to_string()),
                access_role: "owner".to_string(),
                is_primary: true,
            },
        );

        assert_eq!(mapped.calendar.display_name, "Personal");
        assert_eq!(mapped.calendar.timezone, "America/Denver");
        assert_eq!(mapped.calendar.visibility, CalendarVisibility::Private);
        assert!(mapped.calendar.is_default);
    }
}
