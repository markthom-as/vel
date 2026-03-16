---
title: "Implement Card Renderer"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 023-implement-thread-view
  - 005-implement-message-domain-model
labels:
  - vel
  - chat-interface
---
Extend the renderer to support structured message kinds.

## Cards

- `ReminderCard`
- `RiskCard`
- `SuggestionCard`
- `SummaryCard`

## Acceptance Criteria

- renderer dispatches by `message.kind`
- all initial card types render distinctly
- cards share common layout primitives

## Notes for Agent

This is the exit from bubble monoculture. Treat it seriously.
