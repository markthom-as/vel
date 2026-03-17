---
title: Web Operator Runtime
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

# Web Operator Runtime

Owns `clients/web`.

This pack keeps the web client aligned to shared daemon contracts and shared UX semantics while decomposing the largest transport/state/surface hotspots.

## Tickets

- [WEB-001-split-web-transport-and-decoder-families.md](WEB-001-split-web-transport-and-decoder-families.md)
- [WEB-002-split-query-resource-and-realtime-state.md](WEB-002-split-query-resource-and-realtime-state.md)
- [WEB-003-decompose-settings-stats-and-main-surface-hotspots.md](WEB-003-decompose-settings-stats-and-main-surface-hotspots.md)

## Execution order

Run `WEB-001` first. `WEB-002` and `WEB-003` can proceed in parallel after that.

## Exit criteria

- web consumes shared contracts cleanly
- state and transport layers are explicit
- large UI hotspots are reduced without changing UX semantics
