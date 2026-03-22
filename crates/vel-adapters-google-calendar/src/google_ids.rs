use sha2::{Digest, Sha256};
use vel_core::{CalendarId, EventId, IntegrationAccountId, SyncLinkId};

pub const GOOGLE_CALENDAR_PROVIDER: &str = "google-calendar";
pub const GOOGLE_CALENDAR_MODULE_ID: &str = "module.integration.google-calendar";
pub const GOOGLE_CALENDAR_REMOTE_TYPE: &str = "calendar";
pub const GOOGLE_EVENT_REMOTE_TYPE: &str = "event";

pub fn google_calendar_integration_account_id(external_account_ref: &str) -> IntegrationAccountId {
    IntegrationAccountId::from(prefixed_hash_id(
        "integration_account",
        &format!("{GOOGLE_CALENDAR_PROVIDER}:account:{external_account_ref}"),
    ))
}

pub fn google_sync_link_id(
    integration_account_id: &str,
    remote_type: &str,
    remote_id: &str,
) -> SyncLinkId {
    SyncLinkId::from(prefixed_hash_id(
        "sync_link",
        &format!("{GOOGLE_CALENDAR_PROVIDER}:{integration_account_id}:{remote_type}:{remote_id}"),
    ))
}

pub fn google_calendar_id(integration_account_id: &str, remote_id: &str) -> CalendarId {
    CalendarId::from(prefixed_hash_id(
        "calendar",
        &format!("{GOOGLE_CALENDAR_PROVIDER}:{integration_account_id}:calendar:{remote_id}"),
    ))
}

pub fn google_event_id(integration_account_id: &str, remote_id: &str) -> EventId {
    EventId::from(prefixed_hash_id(
        "event",
        &format!("{GOOGLE_CALENDAR_PROVIDER}:{integration_account_id}:event:{remote_id}"),
    ))
}

pub fn google_provider_object_ref(remote_type: &str, remote_id: &str) -> String {
    format!("{GOOGLE_CALENDAR_PROVIDER}:{remote_type}:{remote_id}")
}

fn prefixed_hash_id(prefix: &str, source: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    let digest = hasher.finalize();
    format!("{prefix}_{}", hex::encode(&digest[..12]))
}

#[cfg(test)]
mod tests {
    use super::{
        google_calendar_id, google_calendar_integration_account_id, google_event_id,
        google_provider_object_ref, google_sync_link_id,
    };

    #[test]
    fn google_calendar_ids_are_stable_and_prefixed() {
        let account_id = google_calendar_integration_account_id("acct_primary");
        let calendar_id = google_calendar_id(account_id.as_ref(), "primary");
        let event_id = google_event_id(account_id.as_ref(), "event_123");
        let link_id = google_sync_link_id(account_id.as_ref(), "event", "event_123");

        assert!(account_id.as_ref().starts_with("integration_account_"));
        assert!(calendar_id.as_ref().starts_with("calendar_"));
        assert!(event_id.as_ref().starts_with("event_"));
        assert!(link_id.as_ref().starts_with("sync_link_"));
        assert_eq!(
            google_provider_object_ref("event", "event_123"),
            "google-calendar:event:event_123"
        );
    }
}
