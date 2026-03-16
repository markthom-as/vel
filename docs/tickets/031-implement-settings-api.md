---
title: "Implement Settings API"
status: todo
owner: agent
type: implementation
priority: medium
created: 2026-03-15
depends_on:
  - 013-create-axum-server-skeleton
labels:
  - vel
  - chat-interface
---
Add settings persistence and API routes.

## Routes

- `GET /api/settings`
- `PATCH /api/settings`

## Settings

- `quiet_hours`
- `disable_proactive`
- `toggle_risks`
- `toggle_reminders`

## Acceptance Criteria

- settings persist
- API responses are typed and stable
- future policies can consult these values

## Notes for Agent

Build controls before Vel turns into an overclocked conscience parasite.
