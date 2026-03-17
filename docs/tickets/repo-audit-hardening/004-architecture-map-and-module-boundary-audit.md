---
title: Architecture map and module-boundary audit
status: todo
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

## Scope

- top-level subsystem map
- route/service/storage/core/client ownership inventory
- oversized-file responsibility audit
- candidate extraction seams with rationale

## Acceptance criteria

- each oversized file has an explicit responsibility inventory
- extraction candidates are grouped by architectural seam, not line count
- follow-on decomposition tickets reference this map instead of guessing
