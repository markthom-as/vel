---
title: Add project-specific events and websocket invalidation contract
status: ready
owner: agent
priority: P1
area: projects
---

# Goal
Make project surfaces realtime-aware without piggybacking on unrelated generic chat events.

## Scope
- define project-specific event names and websocket envelope payloads
- broadcast on project/task/session/outbox changes
- wire frontend invalidation keys cleanly

## Suggested event types
- `projects:updated`
- `project_tasks:updated`
- `project_sessions:updated`
- `project_outbox:updated`

## Requirements
- payload should include at least `project_slug`
- where possible include changed object ids and change kind
- keep envelopes compatible with existing websocket machinery
- do not force the frontend to refetch the entire app on every small change

## Tests
- task mutation broadcasts project task event
- session outbox mutation broadcasts outbox event
- frontend invalidation utilities can target project queries precisely

## Done when
- backend emits project-specific websocket events
- frontend query invalidation is precise and documented
