---
title: "Implement Core ID Types"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 003-create-rust-crates
labels:
  - vel
  - chat-interface
---
Introduce strongly typed IDs in `vel-core`.

## Location

`crates/vel-core/src/types.rs`

## Types

- `ConversationId`
- `MessageId`
- `InterventionId`
- `EventId`
- `Timestamp`

## Tasks

- Use UUID-backed IDs
- Support serde serialization/deserialization
- Add unit tests

## Acceptance Criteria

- IDs serialize via serde
- roundtrip tests pass
- API-facing types can reuse these IDs later

## Notes for Agent

Avoid stringly-typed entropy now; it compounds later.
