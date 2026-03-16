---
title: "Implement Conversation Repository"
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
Implement repository methods for conversations.

## Location

`crates/vel-store/src/conversations.rs`

## Methods

- `create`
- `list`
- `get`
- `rename`
- `pin`
- `archive`

## Acceptance Criteria

- integration tests pass
- repository methods map cleanly to domain types
- timestamps update correctly

## Notes for Agent

Keep repo API boring and crisp. This is not the place for cleverness.
