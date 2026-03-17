---
title: Architecture map and module-boundary audit
status: done
owner: agent
type: architecture
priority: high
created: 2026-03-17
depends_on:
  - 002-hermetic-local-integration-tests-and-loop-behavior.md
  - 003-ticket-pack-schema-and-maturity-normalization.md
labels:
  - vel
  - architecture
  - modularity
---

Produce the big-picture architecture map required before broad decomposition.

## Current audit artifact

Primary inventory:

- [docs/architecture-inventory.md](../../architecture-inventory.md)
- [docs/future-architecture-map.md](../../future-architecture-map.md)

This inventory currently covers:

- documentation authority classes
- doc drift and contradiction hotspots
- freshness and in-flight conflict signals
- runtime subsystem ownership
- oversized-file responsibility hotspots
- extraction seams grouped by architectural boundary
- systems to augment, simplify, split, or replace
- planned future subsystem boundaries and service subdivision direction

Use that inventory as the baseline map before opening decomposition PRs or follow-on extraction tickets.

Required audit method:

- [docs/specs/vel-architecture-audit-method.md](../../specs/vel-architecture-audit-method.md)

## Scope

- top-level subsystem map
- route/service/storage/core/client ownership inventory
- oversized-file responsibility audit
- candidate extraction seams with rationale

## Acceptance criteria

- each oversized file has an explicit responsibility inventory
- extraction candidates are grouped by architectural seam, not line count
- follow-on decomposition tickets reference this map instead of guessing

## Follow-on sequence

After the inventory is stable:

1. use ticket `005` for route/service extraction planning, starting with chat and explain surfaces
2. use ticket `006` for storage modularization planning around `vel-storage`
3. use ticket `007` for frontend decomposition planning around settings and client contract concentration

## Completed outputs

- current-state architecture inventory in [docs/architecture-inventory.md](../../architecture-inventory.md)
- future-state synthesis in [docs/future-architecture-map.md](../../future-architecture-map.md)
- explicit extraction seams for chat, storage, and frontend concentration hotspots
