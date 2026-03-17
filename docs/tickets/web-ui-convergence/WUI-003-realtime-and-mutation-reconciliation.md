---
title: Harden websocket, optimistic mutations, and shared invalidation behavior
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-002-transport-decoder-and-query-boundaries.md
labels:
  - web
  - realtime
  - state
---

# Goal

Make realtime updates and optimistic actions predictable across the web shell.

## Scope

- websocket event ingestion
- optimistic send/action behavior
- targeted invalidation and cache reconciliation
- consistent mutation error recovery

## Requirements

1. Decode websocket events once in a shared layer.
2. Map events to targeted cache updates or invalidation rather than page-local logic.
3. Define optimistic versus confirmed behavior for user-critical actions.
4. Ensure failed optimistic actions restore truthful UI state.

## Write scope

- websocket client/event layer
- query invalidation helpers
- thread/inbox/intervention/message mutation paths

## Acceptance criteria

- realtime updates do not require full-page refetches by default
- optimistic UI behavior is consistent across chat and operator actions
- mutation failure states are explicit and recoverable
