//! Capability domain types for the broker-mediated security boundary.
//!
//! Agents request capabilities through the broker. The broker resolves
//! grants or denials — raw credentials are never passed to agents.

use serde::{Deserialize, Serialize};

/// Describes a capability being requested or granted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    /// Scoped resource access string, e.g. "read:context", "execute:todoist"
    pub scope: String,
    /// Optional specific resource path or entity. None = wildcard match.
    pub resource: Option<String>,
    /// The action being requested, e.g. "read", "write", "execute"
    pub action: String,
}

impl CapabilityDescriptor {
    /// Returns true if this descriptor (as an allowlist entry) matches the requested descriptor.
    ///
    /// Matching rules:
    /// - scope must be equal
    /// - action must be equal
    /// - resource: None in this (allowlist) descriptor acts as wildcard — matches any resource
    pub fn matches(&self, requested: &CapabilityDescriptor) -> bool {
        todo!("implement matches")
    }
}

/// A granted capability for a specific run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityGrant {
    /// Stable unique ID for this grant.
    pub grant_id: String,
    /// Run that requested the capability.
    pub run_id: String,
    /// The capability that was granted.
    pub descriptor: CapabilityDescriptor,
    /// Unix timestamp when granted.
    pub granted_at: i64,
    /// Optional expiry timestamp. None = does not expire.
    pub expires_at: Option<i64>,
}

/// A denied capability request, with stable trace ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDenial {
    /// Stable unique ID for this denial — traceable in broker_events.
    pub denial_id: String,
    /// Run that made the request.
    pub run_id: String,
    /// The capability that was requested.
    pub requested: CapabilityDescriptor,
    /// Human-readable denial reason.
    pub reason: String,
    /// Unix timestamp when denied.
    pub denied_at: i64,
}
