//! Capability broker service: mediates all agent capability requests.
//!
//! Agents never receive raw credentials. The broker enforces scope checks and
//! persists every grant/deny/execute decision with a stable trace ID.

use vel_core::{CapabilityDescriptor, CapabilityDenial, CapabilityGrant};
use vel_storage::Storage;

use crate::errors::AppError;

/// Mediates capability requests for agents.
///
/// Phase 2 scope: agents-only. Integration-level capability brokering deferred.
pub struct BrokerService<'a> {
    storage: &'a Storage,
}

impl<'a> BrokerService<'a> {
    pub fn new(storage: &'a Storage) -> Self {
        Self { storage }
    }

    /// Check if `requested` matches any entry in `allowlist`.
    ///
    /// On match: returns `Ok(CapabilityGrant)` and persists a grant broker_event.
    /// On no match: returns `Err(CapabilityDenial)` with reason "scope not in allowlist"
    ///   and persists a deny broker_event.
    pub async fn resolve_capability(
        &self,
        run_id: &str,
        requested: &CapabilityDescriptor,
        allowlist: &[CapabilityDescriptor],
    ) -> Result<CapabilityGrant, CapabilityDenial> {
        todo!("implement resolve_capability")
    }

    /// Convenience: always-deny path for fail-closed behavior.
    /// Persists a denial event with the given reason.
    pub async fn deny_with_trace(
        &self,
        run_id: &str,
        requested: CapabilityDescriptor,
        reason: &str,
    ) -> CapabilityDenial {
        todo!("implement deny_with_trace")
    }

    // Secret hygiene invariant: raw credentials MUST NOT appear in any return value.
    /// Execute a previously-granted capability.
    ///
    /// Returns an action result JSON, never a raw credential. Emits an execute event.
    ///
    /// Phase 2 scope: agents-only. Integration mediation deferred.
    pub async fn execute_brokered(
        &self,
        grant: &CapabilityGrant,
        action_payload: &serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        todo!("implement execute_brokered")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vel_storage::Storage;

    async fn test_storage() -> Storage {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
    }

    fn read_context_desc() -> CapabilityDescriptor {
        CapabilityDescriptor {
            scope: "read:context".to_string(),
            resource: None,
            action: "read".to_string(),
        }
    }

    fn write_captures_desc() -> CapabilityDescriptor {
        CapabilityDescriptor {
            scope: "write:captures".to_string(),
            resource: None,
            action: "write".to_string(),
        }
    }

    // ──────────────────────────────────────────────
    // CapabilityDescriptor.matches() unit tests
    // ──────────────────────────────────────────────

    #[test]
    fn descriptor_matches_same_scope_and_action() {
        let allowlist_entry = read_context_desc();
        let requested = read_context_desc();
        assert!(allowlist_entry.matches(&requested));
    }

    #[test]
    fn descriptor_does_not_match_action_mismatch() {
        // allowlist says "write:captures" / "write"; request says "read" action
        let allowlist_entry = write_captures_desc();
        let requested = CapabilityDescriptor {
            scope: "write:captures".to_string(),
            resource: None,
            action: "read".to_string(),
        };
        assert!(!allowlist_entry.matches(&requested));
    }

    // ──────────────────────────────────────────────
    // BrokerService integration tests
    // ──────────────────────────────────────────────

    #[tokio::test]
    async fn resolve_capability_grants_when_in_allowlist() {
        let storage = test_storage().await;
        let broker = BrokerService::new(&storage);
        let allowlist = vec![read_context_desc()];
        let result = broker
            .resolve_capability("run_test_1", &read_context_desc(), &allowlist)
            .await;
        assert!(result.is_ok(), "expected grant, got denial");
        let grant = result.unwrap();
        assert_eq!(grant.run_id, "run_test_1");
        assert_eq!(grant.descriptor.scope, "read:context");
    }

    #[tokio::test]
    async fn resolve_capability_denies_when_not_in_allowlist() {
        let storage = test_storage().await;
        let broker = BrokerService::new(&storage);
        let allowlist = vec![read_context_desc()];
        let result = broker
            .resolve_capability("run_test_2", &write_captures_desc(), &allowlist)
            .await;
        assert!(result.is_err(), "expected denial, got grant");
        let denial = result.unwrap_err();
        assert_eq!(denial.run_id, "run_test_2");
        assert_eq!(denial.reason, "scope not in allowlist");
    }

    #[tokio::test]
    async fn deny_with_trace_inserts_denial_into_broker_events() {
        let storage = test_storage().await;
        let broker = BrokerService::new(&storage);
        let denial = broker
            .deny_with_trace("run_trace_1", read_context_desc(), "test denial")
            .await;
        assert_eq!(denial.run_id, "run_trace_1");
        assert_eq!(denial.reason, "test denial");

        let events = storage
            .list_broker_events("run_trace_1")
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "deny");
    }

    #[tokio::test]
    async fn resolve_capability_grant_inserts_grant_into_broker_events() {
        let storage = test_storage().await;
        let broker = BrokerService::new(&storage);
        let allowlist = vec![read_context_desc()];
        let _grant = broker
            .resolve_capability("run_grant_persist", &read_context_desc(), &allowlist)
            .await
            .unwrap();
        let events = storage
            .list_broker_events("run_grant_persist")
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "grant");
    }

    #[tokio::test]
    async fn execute_brokered_returns_action_result_not_raw_credential() {
        let storage = test_storage().await;
        let broker = BrokerService::new(&storage);
        let allowlist = vec![read_context_desc()];
        let grant = broker
            .resolve_capability("run_exec_test", &read_context_desc(), &allowlist)
            .await
            .unwrap();
        let payload = serde_json::json!({"query": "current_context"});
        let result = broker.execute_brokered(&grant, &payload).await.unwrap();

        // Result must NOT contain any credential fields
        let result_str = result.to_string();
        assert!(!result_str.contains("api_key"));
        assert!(!result_str.contains("secret"));
        assert!(!result_str.contains("token"));
        // Result must contain executed confirmation
        assert_eq!(result["executed"], serde_json::json!(true));
    }
}
