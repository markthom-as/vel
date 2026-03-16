---
title: "Implement Conversation List"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 021-build-app-shell
  - 014-implement-conversation-api
labels:
  - vel
  - chat-interface
---
Render the conversation list in the left rail.

## Display

- conversation title
- updated timestamp
- pinned status

## Acceptance Criteria

- list loads from API
- clicking a conversation loads its thread
- pinned state is visually distinct

## Notes for Agent

Favor clear information density over ornamental chrome.
