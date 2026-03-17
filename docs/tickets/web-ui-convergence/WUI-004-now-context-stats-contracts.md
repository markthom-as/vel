---
title: Converge Now, context inspection, and Stats around purpose-built contracts
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-001-shell-ia-and-route-ownership.md
  - WUI-002-transport-decoder-and-query-boundaries.md
labels:
  - web
  - now
  - context
  - stats
---

# Goal

Finish the separation between operational action surfaces and observability/debug surfaces.

## Scope

- `Now` read model and layout
- context inspection `State / Why / Debug`
- `Stats` read models and page

## Requirements

1. `Now` must answer what should happen next without carrying broad observability weight.
2. Context inspection must use explicit `State`, `Why`, and `Debug` modes.
3. `Stats` must become the canonical home for source health, context formation, and loop/runtime introspection.
4. Freshness and degraded states remain visible where relevant without duplicating whole diagnostics panes.

## Write scope

- `/v1/now` and related read models if still incomplete
- `Now` page
- context panel/drawer
- `Stats` page and DTOs

## Acceptance criteria

- `Now` no longer behaves like a debug dashboard
- context explanation and raw inspection are separated cleanly
- operators have one obvious place to inspect system health
