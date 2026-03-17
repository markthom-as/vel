---
title: Rework Settings and integrations into control-first IA
status: todo
owner: agent
priority: P1
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-001-shell-ia-and-route-ownership.md
  - WUI-002-transport-decoder-and-query-boundaries.md
labels:
  - web
  - settings
  - integrations
---

# Goal

Make Settings a coherent control plane instead of a mixture of control and passive observability.

## Scope

- settings page structure
- integration policy participation controls
- loop and runtime control organization

## Requirements

1. Distinguish configuration/control from passive diagnostics.
2. Surface integration participation semantics, not only credentials or connection existence.
3. Keep recovery guidance near the relevant control.
4. Move broad runtime inspection to `Stats`.

## Write scope

- settings page shell and extracted tab modules
- integration DTOs and settings read models
- loop control UI

## Acceptance criteria

- settings tabs have clear control-oriented responsibilities
- integrations explain how they participate in context and sync
- settings no longer acts as a backup diagnostics page for missing `Stats`
