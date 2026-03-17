---
title: Vel Flat Execution Packs
status: active
owner: agent
class: convergence
authority: execution
status_model:
  - todo
  - in_progress
  - done
  - deferred
source_of_truth: docs/status.md
created: 2026-03-17
updated: 2026-03-17
---

# Vel Flat Execution Packs

This index is the canonical near-term execution shape for Vel.

It organizes work around the code that actually exists today rather than around overlapping concept packets.

Primary inputs:

- [docs/reviews/vel-second-pass-architecture-audit-2026-03-17.md](../../reviews/vel-second-pass-architecture-audit-2026-03-17.md)
- [docs/specs/vel-modular-cross-platform-architecture-convergence-spec.md](../../specs/vel-modular-cross-platform-architecture-convergence-spec.md)
- [docs/status.md](../../status.md)

## Why this exists

The repo already has strong real seams:

- `vel-core` / `vel-storage` / `vel-api-types`
- `veld`
- `vel-cli`
- `clients/web`
- `clients/apple`
- `docs/`

The backlog should optimize for those seams so work can proceed in parallel with minimal overlap.

## Shared-first execution rules

1. Share semantics in `vel-core` before duplicating client logic.
2. Share bootstrap, sync, and read-model contracts before platform-local shaping.
3. Share UX semantics through vocabulary and state classes, not forced widget reuse.
4. Solve structural performance first by shrinking hotspots and repeated shaping work.

## Canonical packs

1. [runtime-core-storage/README.md](../runtime-core-storage/README.md)
2. [daemon-api-runtime/README.md](../daemon-api-runtime/README.md)
3. [cli-operator-shell/README.md](../cli-operator-shell/README.md)
4. [web-operator-runtime/README.md](../web-operator-runtime/README.md)
5. [apple-client-bootstrap/README.md](../apple-client-bootstrap/README.md)
6. [docs-truth-and-planning/README.md](../docs-truth-and-planning/README.md)

## Parallelization model

Each pack starts with one seam-normalization ticket. After that, branch tickets are intentionally disjoint by write scope.

## Exit criteria

- active work maps to current code ownership
- cross-platform sharing is contract-first
- large-file decomposition is planned by boundary, not by vibes
- the default backlog no longer requires cross-pack archaeology
