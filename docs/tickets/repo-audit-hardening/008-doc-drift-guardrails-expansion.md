---
title: Doc drift guardrails expansion
status: todo
owner: agent
type: tooling
priority: medium
created: 2026-03-17
depends_on:
  - 001-docs-truth-repair-and-entrypoint-alignment.md
  - 003-ticket-pack-schema-and-maturity-normalization.md
labels:
  - vel
  - docs
  - tooling
---

Expand repo-truth checks so high-signal canonical docs fail fast when they drift.

## Scope

- targeted checks for websocket path and key API entrypoints
- checks for active-work links from strict entrypoints
- light-weight semantic checks where possible without brittle overfitting
