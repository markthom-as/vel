---
title: "Implement Intervention Model"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 004-implement-core-id-types
labels:
  - vel
  - chat-interface
---
Add the intervention model in `vel-core`.

## Fields

- `id`
- `message_id`
- `kind`
- `state`
- `confidence`
- `surfaced_at`
- `resolved_at`
- `snoozed_until`

## States

- `active`
- `snoozed`
- `resolved`
- `dismissed`

## Acceptance Criteria

- state transition validation exists
- invalid transitions are tested
- model cleanly supports inbox surfacing

## Notes for Agent

Keep transition logic explicit. Hidden state mutation is how trust dies.
