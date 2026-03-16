---
title: "Harden web client state management and realtime sync"
status: todo
owner: agent
type: implementation
priority: medium
created: 2026-03-15
depends_on:
  - 005-normalize-api-types-time-fields-and-generated-client-contracts.md
labels:
  - vel
  - web
  - chat
  - realtime
---
The web client is good enough to prove shape, but it still feels like a prototype shell rather than a durable operator interface.

The biggest frontend smell is not aesthetics. It is **state orchestration**.

## Current concerns

- Thread/inbox/intervention fetching is component-local and chatty.
- `ThreadView` fans out intervention fetches per assistant message, which is fine for bootstrap but scales like a petty crime spree.
- API error handling is minimal and mostly inline.
- There is no obvious normalized client-side cache/query layer.
- WebSocket/realtime behavior exists in the backend, but the frontend architecture does not yet look centered around streaming updates.

## Goal

Move the web client from ad hoc fetch orchestration to a stable operator UI data layer.

## Tasks

- Introduce a query/cache layer such as TanStack Query or equivalent.
- Replace N+1 intervention lookups with:
  - a batch endpoint, or
  - message payload enrichment, or
  - thread-level intervention fetch
- Wire the frontend to the websocket/event stream for conversation, inbox, and intervention updates.
- Define optimistic vs confirmed UI behavior for:
  - sending messages
  - snoozing/resolving/dismissing interventions
- Centralize loading/error/empty states so each component is not reinventing them.

## Acceptance Criteria

- Selecting a thread does not trigger unnecessary fetch storms.
- Intervention actions update the UI predictably and reconcile with server truth.
- Websocket events can update thread/inbox state without full refetch.
- Core data loading is centralized enough that new screens do not repeat the same orchestration patterns.

## Notes for Agent

Right now the frontend is functional. The next step is to stop making every component run its own tiny foreign policy.
