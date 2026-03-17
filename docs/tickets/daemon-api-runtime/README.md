---
title: Daemon API And Runtime
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

# Daemon API And Runtime

Owns `crates/veld`.

This is the primary pack for shared runtime orchestration, shared read-model assembly, and shared bootstrap/sync semantics used by multiple clients.

## Tickets

- [DAR-001-split-router-assembly-by-route-family.md](DAR-001-split-router-assembly-by-route-family.md)
- [DAR-002-normalize-shared-bootstrap-sync-and-cluster-contracts.md](DAR-002-normalize-shared-bootstrap-sync-and-cluster-contracts.md)
- [DAR-003-decompose-read-model-and-integration-service-hotspots.md](DAR-003-decompose-read-model-and-integration-service-hotspots.md)

## Execution order

Run `DAR-001` first. `DAR-002` and `DAR-003` can proceed in parallel after that.

## Exit criteria

- route and service ownership are legible by family
- shared client-facing runtime contracts live in one obvious place
- daemon hotspots are reduced without changing the core runtime model
