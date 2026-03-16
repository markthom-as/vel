---
title: "Implement Message Repository"
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
Implement repository methods for messages.

## Location

`crates/vel-store/src/messages.rs`

## Methods

- `create`
- `list_by_conversation`
- `get`
- `update_status`

## Acceptance Criteria

- message CRUD tests pass
- ordering is stable
- JSON payloads are stored and retrieved without corruption

## Notes for Agent

Favor explicit ordering rules now. Ambiguous message ordering becomes UI hauntology later.
