---
title: "Implement Intervention Actions API"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 013-create-axum-server-skeleton
  - 011-implement-intervention-repository
labels:
  - vel
  - chat-interface
---
Implement intervention action routes.

## Routes

- `POST /api/interventions/:id/snooze`
- `POST /api/interventions/:id/resolve`
- `POST /api/interventions/:id/dismiss`

## Acceptance Criteria

- state transitions work correctly
- each mutation appends an event
- duplicate requests do not corrupt state

## Notes for Agent

Operational actions must behave like state transitions, not vibes.
