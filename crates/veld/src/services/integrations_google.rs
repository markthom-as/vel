use reqwest::Url;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;
use vel_api_types::GoogleCalendarAuthStartData;
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

use crate::{
    errors::AppError,
    services::integrations::{GoogleCalendarSettings, StoredCalendar},
};

pub(crate) const GOOGLE_AUTH_BASE: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub(crate) const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
pub(crate) const GOOGLE_CALENDAR_BASE: &str = "https://www.googleapis.com/calendar/v3";
pub(crate) const GOOGLE_LOOKBACK_DAYS: i64 = 60;
pub(crate) const GOOGLE_LOOKAHEAD_DAYS: i64 = 180;

pub(crate) async fn start_google_auth(
    settings: &mut GoogleCalendarSettings,
    config: &AppConfig,
) -> Result<GoogleCalendarAuthStartData, AppError> {
    let client_id = settings
        .client_id
        .as_deref()
        .ok_or_else(|| AppError::bad_request("google calendar client_id is required"))?;

    if settings.client_secret.as_deref().unwrap_or("").is_empty() {
        return Err(AppError::bad_request(
            "google calendar client_secret is required",
        ));
    }

    let oauth_state = format!("gcal_{}", Uuid::new_v4().simple());
    settings.pending_oauth_state = Some(oauth_state.clone());

    let redirect_uri = google_redirect_uri(config)?;
    let auth_url = Url::parse_with_params(
        GOOGLE_AUTH_BASE,
        [
            ("client_id", client_id),
            ("redirect_uri", redirect_uri.as_str()),
            ("response_type", "code"),
            ("access_type", "offline"),
            ("prompt", "consent"),
            ("scope", "https://www.googleapis.com/auth/calendar.readonly"),
            ("state", oauth_state.as_str()),
        ],
    )
    .map_err(|error| AppError::internal(format!("google auth url: {}", error)))?;

    Ok(GoogleCalendarAuthStartData {
        auth_url: auth_url.to_string(),
    })
}

pub(crate) async fn complete_google_auth(
    settings: &mut GoogleCalendarSettings,
    config: &AppConfig,
    state_param: &str,
    code: &str,
) -> Result<(), AppError> {
    let pending_state = settings
        .pending_oauth_state
        .clone()
        .ok_or_else(|| AppError::bad_request("no pending google oauth flow"))?;
    if pending_state != state_param {
        return Err(AppError::bad_request("google oauth state mismatch"));
    }

    let client_id = settings
        .client_id
        .clone()
        .ok_or_else(|| AppError::bad_request("google calendar client_id is required"))?;
    let client_secret = settings
        .client_secret
        .clone()
        .ok_or_else(|| AppError::bad_request("google calendar client_secret is required"))?;
    let redirect_uri = google_redirect_uri(config)?;

    let token: GoogleTokenResponse = reqwest::Client::new()
        .post(GOOGLE_TOKEN_URL)
        .form(&[
            ("code", code),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|error| AppError::internal(format!("google token exchange: {}", error)))?
        .error_for_status()
        .map_err(|error| AppError::internal(format!("google token exchange: {}", error)))?
        .json()
        .await
        .map_err(|error| AppError::internal(format!("google token decode: {}", error)))?;

    settings.access_token = Some(token.access_token);
    settings.refresh_token = token.refresh_token.or(settings.refresh_token.clone());
    settings.token_expires_at = Some(now_ts() + token.expires_in.unwrap_or(3600) - 60);
    settings.pending_oauth_state = None;
    settings.last_error = None;

    let calendars = list_google_calendars(settings).await?;
    settings.calendars = merge_calendar_selection(settings.calendars.clone(), calendars);
    Ok(())
}

pub(crate) async fn sync_google_calendar(
    storage: &Storage,
    settings: &mut GoogleCalendarSettings,
) -> Result<Option<u32>, AppError> {
    let Some(client_id) = settings.client_id.clone() else {
        return Ok(None);
    };
    let Some(client_secret) = settings.client_secret.clone() else {
        return Ok(None);
    };
    if settings.refresh_token.as_deref().unwrap_or("").is_empty() {
        return Ok(None);
    }

    let access_token = ensure_google_access_token(settings, &client_id, &client_secret).await?;
    let calendars = list_google_calendars_with_token(&access_token).await?;
    settings.calendars = merge_calendar_selection(settings.calendars.clone(), calendars);
    let selected = selected_calendars(settings);
    let time_min = OffsetDateTime::now_utc() - Duration::days(GOOGLE_LOOKBACK_DAYS);
    let time_max = OffsetDateTime::now_utc() + Duration::days(GOOGLE_LOOKAHEAD_DAYS);

    let mut inserted = 0u32;
    for calendar in &selected {
        let events =
            list_google_events_with_token(&access_token, &calendar.id, time_min, time_max).await?;
        for event in events {
            if event.status.as_deref() == Some("cancelled") {
                continue;
            }
            let Some(start_ts) = google_event_start_ts(&event) else {
                continue;
            };
            let source_ref = format!(
                "google_calendar:{}:{}:{}",
                calendar.id,
                event.id,
                event.updated.as_deref().unwrap_or("-"),
            );
            let payload = serde_json::json!({
                "event_id": event.id,
                "calendar_id": calendar.id,
                "calendar_name": calendar.summary,
                "title": event.summary.clone().unwrap_or_else(|| "(untitled event)".to_string()),
                "start": start_ts,
                "end": google_event_end_ts(&event),
                "location": event.location.unwrap_or_default(),
                "description": event.description.unwrap_or_default(),
                "status": event.status.unwrap_or_default(),
                "url": event.html_link.unwrap_or_default(),
                "attendees": event.attendees.unwrap_or_default().into_iter().filter_map(|item| item.email.or(item.display_name)).collect::<Vec<_>>(),
                "prep_minutes": 15,
                "travel_minutes": 0
            });
            let signal_id = storage
                .insert_signal(SignalInsert {
                    signal_type: "calendar_event".to_string(),
                    source: "google_calendar".to_string(),
                    source_ref: Some(source_ref),
                    timestamp: start_ts,
                    payload_json: Some(payload),
                })
                .await?;
            if signal_id.starts_with("sig_") {
                inserted += 1;
            }
        }
    }

    settings.last_sync_at = Some(now_ts());
    settings.last_sync_status = Some("ok".to_string());
    settings.last_error = None;
    settings.last_item_count = Some(inserted);
    Ok(Some(inserted))
}

async fn list_google_calendars(
    settings: &GoogleCalendarSettings,
) -> Result<Vec<StoredCalendar>, AppError> {
    let access_token = settings
        .access_token
        .as_deref()
        .ok_or_else(|| AppError::bad_request("google access token missing"))?;
    list_google_calendars_with_token(access_token).await
}

async fn list_google_calendars_with_token(
    access_token: &str,
) -> Result<Vec<StoredCalendar>, AppError> {
    let client = reqwest::Client::new();
    let mut calendars = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = Url::parse(&format!("{}/users/me/calendarList", GOOGLE_CALENDAR_BASE))
            .map_err(|error| AppError::internal(format!("google calendar list url: {}", error)))?;
        if let Some(token) = page_token.as_deref() {
            url.query_pairs_mut().append_pair("pageToken", token);
        }

        let response: GoogleCalendarListResponse = client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|error| AppError::internal(format!("google calendar list: {}", error)))?
            .error_for_status()
            .map_err(|error| AppError::internal(format!("google calendar list: {}", error)))?
            .json()
            .await
            .map_err(|error| AppError::internal(format!("google calendar decode: {}", error)))?;

        calendars.extend(response.items.into_iter().map(|item| StoredCalendar {
            id: item.id,
            summary: item.summary,
            primary: item.primary.unwrap_or(false),
            selected: true,
        }));

        match response.next_page_token {
            Some(token) if !token.is_empty() => page_token = Some(token),
            _ => break,
        }
    }

    Ok(calendars)
}

async fn list_google_events_with_token(
    access_token: &str,
    calendar_id: &str,
    time_min: OffsetDateTime,
    time_max: OffsetDateTime,
) -> Result<Vec<GoogleCalendarEvent>, AppError> {
    let client = reqwest::Client::new();
    let mut events = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = Url::parse(GOOGLE_CALENDAR_BASE)
            .map_err(|error| AppError::internal(format!("google events url: {}", error)))?;
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| AppError::internal("google events url path"))?;
            segments.push("calendars");
            segments.push(calendar_id);
            segments.push("events");
        }
        {
            let mut query = url.query_pairs_mut();
            query
                .append_pair("singleEvents", "true")
                .append_pair("orderBy", "startTime")
                .append_pair("timeMin", &format_rfc3339(time_min)?)
                .append_pair("timeMax", &format_rfc3339(time_max)?);
            if let Some(token) = page_token.as_deref() {
                query.append_pair("pageToken", token);
            }
        }

        let response: GoogleEventListResponse = client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|error| AppError::internal(format!("google event list: {}", error)))?
            .error_for_status()
            .map_err(|error| AppError::internal(format!("google event list: {}", error)))?
            .json()
            .await
            .map_err(|error| AppError::internal(format!("google event decode: {}", error)))?;
        events.extend(response.items);

        match response.next_page_token {
            Some(token) if !token.is_empty() => page_token = Some(token),
            _ => break,
        }
    }

    Ok(events)
}

async fn ensure_google_access_token(
    settings: &mut GoogleCalendarSettings,
    client_id: &str,
    client_secret: &str,
) -> Result<String, AppError> {
    if let (Some(access_token), Some(expires_at)) =
        (settings.access_token.clone(), settings.token_expires_at)
    {
        if now_ts() < expires_at {
            return Ok(access_token);
        }
    }

    let refresh_token = settings
        .refresh_token
        .clone()
        .ok_or_else(|| AppError::bad_request("google refresh token missing"))?;

    let token: GoogleTokenResponse = reqwest::Client::new()
        .post(GOOGLE_TOKEN_URL)
        .form(&[
            ("refresh_token", refresh_token.as_str()),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .await
        .map_err(|error| AppError::internal(format!("google token refresh: {}", error)))?
        .error_for_status()
        .map_err(|error| AppError::internal(format!("google token refresh: {}", error)))?
        .json()
        .await
        .map_err(|error| AppError::internal(format!("google token refresh decode: {}", error)))?;

    settings.access_token = Some(token.access_token.clone());
    settings.token_expires_at = Some(now_ts() + token.expires_in.unwrap_or(3600) - 60);
    Ok(token.access_token)
}

fn merge_calendar_selection(
    existing: Vec<StoredCalendar>,
    latest: Vec<StoredCalendar>,
) -> Vec<StoredCalendar> {
    let selected_by_id = existing
        .into_iter()
        .map(|calendar| (calendar.id, calendar.selected))
        .collect::<std::collections::HashMap<_, _>>();
    latest
        .into_iter()
        .map(|mut calendar| {
            if let Some(selected) = selected_by_id.get(&calendar.id) {
                calendar.selected = *selected;
            }
            calendar
        })
        .collect()
}

fn selected_calendars(settings: &GoogleCalendarSettings) -> Vec<StoredCalendar> {
    if settings.all_calendars_selected {
        return settings.calendars.clone();
    }
    settings
        .calendars
        .iter()
        .filter(|calendar| calendar.selected)
        .cloned()
        .collect()
}

fn google_redirect_uri(config: &AppConfig) -> Result<String, AppError> {
    let base = config.base_url.trim_end_matches('/');
    if base.is_empty() {
        return Err(AppError::internal("base_url is required for google oauth"));
    }
    Ok(format!(
        "{}/api/integrations/google-calendar/oauth/callback",
        base
    ))
}

fn now_ts() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

fn format_rfc3339(value: OffsetDateTime) -> Result<String, AppError> {
    value
        .format(&Rfc3339)
        .map_err(|error| AppError::internal(format!("format rfc3339: {}", error)))
}

fn google_event_start_ts(event: &GoogleCalendarEvent) -> Option<i64> {
    google_event_time_to_ts(event.start.as_ref())
}

fn google_event_end_ts(event: &GoogleCalendarEvent) -> Option<i64> {
    google_event_time_to_ts(event.end.as_ref())
}

fn google_event_time_to_ts(value: Option<&GoogleEventDateTime>) -> Option<i64> {
    let value = value?;
    if let Some(date_time) = value.date_time.as_deref() {
        return parse_rfc3339(date_time);
    }
    value
        .date
        .as_deref()
        .and_then(|date| parse_iso_datetime(&format!("{}T00:00:00Z", date)))
}

fn parse_rfc3339(value: &str) -> Option<i64> {
    OffsetDateTime::parse(value, &Rfc3339)
        .ok()
        .map(|date_time| date_time.unix_timestamp())
}

fn parse_iso_datetime(value: &str) -> Option<i64> {
    parse_rfc3339(value).or_else(|| {
        let normalized = if value.ends_with('Z') {
            value.to_string()
        } else {
            format!("{}Z", value)
        };
        parse_rfc3339(&normalized)
    })
}

#[derive(Debug, serde::Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
struct GoogleCalendarListResponse {
    #[serde(default)]
    items: Vec<GoogleCalendarListItem>,
    #[serde(default, rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GoogleCalendarListItem {
    id: String,
    summary: String,
    #[serde(default)]
    primary: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
struct GoogleEventListResponse {
    #[serde(default)]
    items: Vec<GoogleCalendarEvent>,
    #[serde(default, rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GoogleCalendarEvent {
    id: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    location: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    updated: Option<String>,
    #[serde(default, rename = "htmlLink")]
    html_link: Option<String>,
    #[serde(default)]
    attendees: Option<Vec<GoogleEventAttendee>>,
    start: Option<GoogleEventDateTime>,
    end: Option<GoogleEventDateTime>,
}

#[derive(Debug, serde::Deserialize)]
struct GoogleEventDateTime {
    #[serde(default, rename = "dateTime")]
    date_time: Option<String>,
    #[serde(default)]
    date: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GoogleEventAttendee {
    #[serde(default)]
    email: Option<String>,
    #[serde(default, rename = "displayName")]
    display_name: Option<String>,
}
