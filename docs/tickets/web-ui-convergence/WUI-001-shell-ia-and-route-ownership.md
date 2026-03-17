---
title: Canonicalize shell IA and top-level route ownership
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on: []
labels:
  - web
  - ui
  - shell
---

# Goal

Make the global web shell match the canonical surface map and stop route ownership drift.

## Scope

- top-level navigation and route inventory
- page ownership for `Now`, `Inbox`, `Threads`, `Suggestions`, `Projects`, `Stats`, `Settings`
- shared page shell structure and cross-surface handoff patterns

## Requirements

1. Define one canonical page/route owner for each top-level surface.
2. Add `Stats` as the observability home if it is not already first-class.
3. Ensure cross-surface actions use explicit handoffs instead of duplicate embedded mini-surfaces.
4. Document route ownership and page responsibilities in code-adjacent docs if needed.

## Write scope

- app shell navigation and routing
- top-level page containers
- docs that define page ownership

## Acceptance criteria

- every top-level page has one explicit role
- the shell no longer leaves `Projects` or `Stats` as side concepts
- page boundaries align with the canonical web operator surface spec
