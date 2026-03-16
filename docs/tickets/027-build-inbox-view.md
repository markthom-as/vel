---
title: "Build Inbox View"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 021-build-app-shell
  - 016-implement-inbox-api
labels:
  - vel
  - chat-interface
---
Build the inbox surface at `/`.

## Features

- active interventions
- severity grouping
- recency sorting

## Acceptance Criteria

- inbox loads from API
- selecting an item opens related content
- empty state is handled cleanly

## Notes for Agent

Inbox is where Vel becomes proactive in public. Do not make it feel like a junk drawer.
