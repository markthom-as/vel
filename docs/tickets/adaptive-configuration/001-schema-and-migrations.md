---
id: vel-adaptive-config-001
title: Add adaptive configuration schema and migrations
status: proposed
priority: P0
owner: backend
---

## Summary
Create the initial database schema and Rust types for persisted settings, policies, profiles, effective snapshots, and config audit events.

## Scope
- add numbered migration for `user_settings`, `config_profiles`, `context_signals`, `config_policies`, `effective_config_snapshots`, `config_events`
- add indexes and uniqueness constraints
- add Rust models / repository interfaces
- add serde types for JSON payloads

## Acceptance Criteria
- migration applies cleanly on empty db
- migration is idempotent in repo conventions
- repository layer can insert/select/update settings and policies
- JSON validation stubs exist for future strict schema checks
- all schema choices documented in `docs/`

## Implementation Notes
- prefer `jsonb` for value payloads and evidence
- ensure append-only semantics for `config_events`
- index by `user_id`, `subject_kind`, `subject_id`, `created_at`
- add unique constraint on `(user_id, scope_type, scope_id, config_key)`

## Tests
- migration up/down if supported
- repo round-trip tests for settings and policies
