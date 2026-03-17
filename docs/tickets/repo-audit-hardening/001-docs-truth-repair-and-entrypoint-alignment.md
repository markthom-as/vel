---
title: Docs truth repair and entrypoint alignment
status: in_progress
owner: agent
type: documentation
priority: high
created: 2026-03-17
depends_on: []
labels:
  - vel
  - docs
  - convergence
---

Repair the highest-signal mismatches between canonical docs and live code.

## Scope

- root README status summary
- canonical API entrypoints
- chat ticket/status accounting
- links from strict-status docs into the active work queue

## Acceptance criteria

- `README.md` does not claim already-shipped work is still merely "planned next"
- `docs/api/chat.md` matches the live websocket path and current event set
- chat status docs and ticket inventory agree on the 036/037 state
- strict entrypoints link to the active hardening pack
