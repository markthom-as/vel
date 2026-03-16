---
title: "Implement Thread View"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 021-build-app-shell
  - 015-implement-message-api
labels:
  - vel
  - chat-interface
---
Build the main thread view.

## Components

- `ThreadView`
- `MessageRenderer`

## Acceptance Criteria

- messages display in correct order
- scrolling behavior is sane
- thread updates when switching conversations

## Notes for Agent

The renderer is the hinge point. Do not hardcode text-only assumptions into it.
