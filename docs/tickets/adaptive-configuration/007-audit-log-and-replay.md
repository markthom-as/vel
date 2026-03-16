---
id: vel-adaptive-config-007
title: Add config audit log and replay endpoints
status: proposed
priority: P1
owner: backend
---

## Summary
Create append-only config event logging plus replay capabilities for debugging and operator forensics.

## Scope
- create `audit/config_events.rs` and `audit/replay.rs`
- write events for setting mutations, policy matches, overrides, constraints, effective config changes, simulations
- add:
  - `GET /v1/audit/config-events`
  - `POST /v1/audit/config-replay`
- support filtering by key, subject, trace id, time window

## Acceptance Criteria
- event stream reconstructs effective config changes
- replay can explain a past config state for a given subject/time window
- sensitive evidence can be redacted in user-facing views

## Tests
- append-only event tests
- replay correctness tests
- redaction tests
