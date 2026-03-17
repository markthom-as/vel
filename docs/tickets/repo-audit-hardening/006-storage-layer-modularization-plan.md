---
title: Storage-layer modularization plan
status: todo
owner: agent
type: architecture
priority: medium
created: 2026-03-17
depends_on:
  - 004-architecture-map-and-module-boundary-audit.md
labels:
  - vel
  - storage
  - modularity
---

Plan a responsible split of the large storage module without breaking domain/storage contracts.

## Scope

- query-area inventory inside `vel-storage`
- stable submodule seams
- migration and test strategy for extraction
