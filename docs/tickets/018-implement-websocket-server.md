---
title: "Implement WebSocket Server"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 013-create-axum-server-skeleton
labels:
  - vel
  - chat-interface
---
Add the realtime WebSocket endpoint.

## Route

- `/ws`

## Envelope

```json
{
  "type": "messages:new",
  "timestamp": "...",
  "payload": {}
}
```

## Acceptance Criteria

- a client can connect
- a client receives broadcast events
- event envelope shape is consistent

## Notes for Agent

Keep event names stable. Future clients should not need divination.
