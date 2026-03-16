---
title: "Create Axum Server Skeleton"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 009-implement-conversation-repository
  - 010-implement-message-repository
  - 011-implement-intervention-repository
  - 012-implement-event-log-repository
labels:
  - vel
  - chat-interface
---
Create the baseline Axum server and wire shared app state.

## Location

`crates/vel-server/src/main.rs`

## Tasks

- initialize router
- initialize SQLite pool
- initialize `AppState`
- add health route if useful

## Acceptance Criteria

- server runs locally
- migrations run on boot
- app state wiring is clean and testable

## Notes for Agent

Thin handlers, thick domain. Resist god-server syndrome.
