---
title: "Implement Inbox API"
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
Implement the inbox endpoint.

## Route

- `GET /api/inbox`

## Behavior

Return active, unresolved interventions in a UI-friendly format.

## Acceptance Criteria

- inbox returns unresolved interventions
- snoozed/resolved items are excluded or clearly labeled
- payload is consumable without frontend archaeology

## Notes for Agent

Inbox is not a dump. It is the operational surface for proactive items.
