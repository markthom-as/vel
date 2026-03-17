# Vel Integration Expansion Ticket Pack

This pack expands Vel from single-family adapters into a multi-provider, multi-connection integration architecture with first-class people identity.

Primary spec:

- [docs/specs/vel-multi-vendor-integration-and-person-identity-spec.md](../../specs/vel-multi-vendor-integration-and-person-identity-spec.md)

## Why this pack exists

Current Vel integration support proves out adapter ingestion, but the next expansion step needs a stronger substrate:

- multiple providers per family
- multiple connections per provider
- durable provider and connection provenance
- Vel-native person identity across vendors
- Apple-compatible bridge patterns
- Steam and workspace-style activity/document sources
- standards-aware import/export surfaces

## Execution order

1. foundation schema and provider registry
2. people and external identity model
3. connection-aware API and settings surfaces
4. family/provider-specific adapters
5. explainability, fixtures, and rollout

## Ticket list

- `INTG-001-foundation-family-provider-connection-model.md`
- `INTG-002-people-and-external-identity-graph.md`
- `INTG-003-provider-capability-registry-and-standards-contracts.md`
- `INTG-004-connection-aware-settings-api-and-operator-surface.md`
- `INTG-005-messaging-multi-vendor-ingestion.md`
- `INTG-006-notes-multi-vendor-ingestion.md`
- `INTG-007-transcript-session-and-speaker-ingestion.md`
- `INTG-008-apple-reminders-health-and-mindfulness-bridges.md`
- `INTG-009-steam-activity-adapter.md`
- `INTG-010-google-workspace-identity-and-document-integration.md`
- `INTG-011-explainability-and-debug-surfaces-for-provenance-and-people.md`
- `INTG-012-fixtures-tests-docs-and-rollout.md`
