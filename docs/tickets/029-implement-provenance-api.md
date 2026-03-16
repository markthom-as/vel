---
title: "Implement Provenance API"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 015-implement-message-api
  - 012-implement-event-log-repository
labels:
  - vel
  - chat-interface
---
Implement provenance retrieval.

## Route

- `GET /api/messages/:id/provenance`

## Returns

- signals
- policy decisions
- linked objects
- timestamps

## Acceptance Criteria

- endpoint returns structured provenance
- output is suitable for UI display
- provenance is not canned filler text

## Notes for Agent

“Why this?” must be grounded, or it is theater.
