---
title: "Implement Event Log Repository"
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
Implement the append-only event log repository.

## Methods

- `append`
- `list_recent`
- `list_by_aggregate`

## Acceptance Criteria

- events are recorded for all mutations
- recent listing works
- aggregate filtering works

## Notes for Agent

If a state change occurs and no event is recorded, assume you built a bug nursery.
