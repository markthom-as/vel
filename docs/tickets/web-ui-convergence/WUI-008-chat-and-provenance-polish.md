---
title: Complete chat and provenance surface polish under the shared operator model
status: todo
owner: agent
priority: P2
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-003-realtime-and-mutation-reconciliation.md
  - WUI-007-inbox-threads-suggestions-role-cleanup.md
labels:
  - web
  - chat
  - provenance
---

# Goal

Finish the remaining chat/operator polish without breaking the shared shell architecture.

## Scope

- thread-level loading/error polish
- provenance presentation
- remaining chat rendering quality work that fits the operator model

## Requirements

1. Tighten shared loading/error behavior in thread and provenance flows.
2. Improve provenance presentation without turning it into a raw payload dump.
3. Keep structured cards and inline actions aligned with the shared shell patterns.
4. Leave room for richer markdown/code rendering where it fits the operator model.

## Write scope

- chat thread UI
- provenance drawer/panel
- message rendering polish

## Acceptance criteria

- chat feels like part of the operator console, not a separate prototype app
- provenance remains useful and legible
- remaining chat polish does not reintroduce custom state orchestration per component
