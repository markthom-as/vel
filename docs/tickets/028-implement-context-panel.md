---
title: "Implement Context Panel"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 021-build-app-shell
labels:
  - vel
  - chat-interface
---
Implement the right-side context panel.

## Display

- current time block
- active commitments
- risk count
- routine state
- last update time

## Data Source

- `GET /api/context/current`

## Acceptance Criteria

- panel renders stable context data
- panel updates without breaking thread flow
- placeholders are clearly placeholders if backend data is partial

## Notes for Agent

Better an honest thin context model than a fake omniscience panel.
