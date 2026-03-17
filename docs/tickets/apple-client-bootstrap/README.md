---
title: Apple Client Bootstrap
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

# Apple Client Bootstrap

Owns `clients/apple`.

This pack keeps Apple surfaces thin over shared daemon contracts and shared `VelAPI` code while allowing app-target-specific embodiment.

## Tickets

- [APL-001-split-velapi-shared-contracts-client-and-store.md](APL-001-split-velapi-shared-contracts-client-and-store.md)
- [APL-002-tighten-ios-and-watch-bootstrap-action-flows.md](APL-002-tighten-ios-and-watch-bootstrap-action-flows.md)
- [APL-003-tighten-macos-bootstrap-export-and-reachability.md](APL-003-tighten-macos-bootstrap-export-and-reachability.md)

## Execution order

Run `APL-001` first. `APL-002` and `APL-003` can proceed in parallel after that.

## Exit criteria

- Apple shared client code is the stable seam
- iOS/watch/macOS surfaces consume shared semantics without duplicated local logic
- bootstrap/offline/export behavior is easier to reason about
