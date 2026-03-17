---
title: Decompose web transport decoders and resource/query boundaries by domain
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-001-shell-ia-and-route-ownership.md
labels:
  - web
  - transport
  - state
---

# Goal

Replace transport and query monoliths with domain-aligned modules that match surface ownership.

## Scope

- decoder extraction from monolithic transport modules
- shared query/resource modules by domain
- stable query keys and domain-level invalidation helpers

## Requirements

1. Split decoder logic by transport domain instead of one cross-surface choke point.
2. Align resource loaders with those domain decoders.
3. Preserve one shared query/cache path rather than per-page ad hoc fetch logic.
4. Avoid page-local transport duplication.

## Write scope

- `clients/web/src/types.ts` or its replacements
- `clients/web/src/data/resources.ts` and related query modules
- tests for decoders and resource loaders

## Acceptance criteria

- transport decoding is organized by domain
- query/resource ownership maps cleanly to the surface model
- adding a new top-level surface no longer requires touching one global decoder monolith
