---
title: "Implement Message API"
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
Implement message routes.

## Routes

- `GET /api/conversations/:id/messages`
- `POST /api/conversations/:id/messages`

## Acceptance Criteria

- messages persist and return correctly
- messages are ordered deterministically
- creating a message appends a `message.created` event

## Notes for Agent

Keep the API contract stable enough for the frontend to stop guessing.
