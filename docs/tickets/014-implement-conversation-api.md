---
title: "Implement Conversation API"
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
Implement conversation routes.

## Routes

- `GET /api/conversations`
- `POST /api/conversations`
- `GET /api/conversations/:id`
- `PATCH /api/conversations/:id`

## Acceptance Criteria

- CRUD works via curl or HTTP client
- DTOs are shaped for product use, not raw DB rows
- events are appended on mutations

## Notes for Agent

Do not leak storage row formats into the public API.
