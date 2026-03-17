---
title: Implement session outbox, steering, feedback, and settings controls
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Give the operator durable control surfaces for project-linked agent sessions.

## Scope
- add durable outbox table/model
- add feedback table/model
- add session settings mutation path
- expose queue/feedback/settings endpoints

## Routes
- `POST /v1/projects/:slug/sessions/:id/queue-message`
- `POST /v1/projects/:slug/sessions/:id/feedback`
- `POST /v1/projects/:slug/sessions/:id/settings`
- optionally `GET /v1/projects/:slug/sessions/:id/outbox`

## Requirements
- queueing is explicit and durable
- support outbox states: `queued`, `sent`, `acked`, `failed`, `cancelled`
- allow read-only adapters: queued state still matters even when external delivery is not automated yet
- feedback supports quick reaction and structured note payloads
- steering can be represented as feedback/settings with explicit `feedback_type` or dedicated payload structure
- session settings are persisted and surfaced in workspace payload

## Auditability
Emit events for:
- outbox queued/sent/acked/failed/cancelled
- feedback recorded
- settings changed

## Tests
- queue message lifecycle state transitions
- feedback recording
- settings mutation reflected in subsequent workspace fetch

## Done when
- operator control plane is durable
- queue/feedback/settings show up in project workspace data
