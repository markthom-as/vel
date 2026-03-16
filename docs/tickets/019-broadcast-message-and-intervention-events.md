---
title: "Broadcast Message and Intervention Events"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 018-implement-websocket-server
  - 015-implement-message-api
  - 017-implement-intervention-actions-api
labels:
  - vel
  - chat-interface
---
Broadcast typed realtime events on server mutations.

## Events

- `messages:new`
- `interventions:new`
- `interventions:updated`

## Acceptance Criteria

- websocket receives updates when mutations occur
- payloads align with REST DTOs where practical
- event emission is covered by tests or manual verification

## Notes for Agent

Realtime should reduce uncertainty, not invent a second schema.
