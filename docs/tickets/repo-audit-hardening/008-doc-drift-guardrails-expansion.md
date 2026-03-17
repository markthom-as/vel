---
title: Doc drift guardrails expansion
status: in_progress
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

## Recent guardrail additions

- require [docs/tickets/pack-schema.md](../pack-schema.md) to declare the required metadata, classification guidance, and enforcement rule as enforcement surfaces grow
- ensure [docs/tickets/repo-audit-hardening/004-architecture-map-and-module-boundary-audit.md](../004-architecture-map-and-module-boundary-audit.md) references [docs/specs/vel-architecture-audit-method.md](../../specs/vel-architecture-audit-method.md) so teams follow the mandated audit process before decomposing large files
