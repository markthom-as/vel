---
title: "Implement Intervention Repository"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 008-implement-initial-database-schema
labels:
  - vel
  - chat-interface
---
Implement repository methods for interventions.

## Location

`crates/vel-store/src/interventions.rs`

## Methods

- `create`
- `list_active`
- `get_by_message`
- `snooze`
- `resolve`
- `dismiss`

## Acceptance Criteria

- state transitions update correctly
- tests cover snooze/resolve/dismiss
- active intervention queries are UI-friendly

## Notes for Agent

Idempotency matters. Duplicate clicks should not summon chaos.
