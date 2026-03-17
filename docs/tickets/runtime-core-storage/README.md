---
title: Runtime Core And Storage
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

# Runtime Core And Storage

Owns `crates/vel-core`, `crates/vel-storage`, `crates/vel-api-types`, `crates/vel-config`, and `crates/vel-llm`.

This is the primary pack for shared semantics and shared transport contracts across every surface.

## Tickets

- [RCS-001-split-vel-storage-by-persistence-family.md](RCS-001-split-vel-storage-by-persistence-family.md)
- [RCS-002-split-domain-and-dto-contracts-by-family.md](RCS-002-split-domain-and-dto-contracts-by-family.md)
- [RCS-003-harden-config-and-provider-contracts.md](RCS-003-harden-config-and-provider-contracts.md)

## Execution order

Run `RCS-001` first. `RCS-002` and `RCS-003` can proceed in parallel after that.

## Exit criteria

- storage, domain, and transport contracts are modularized by family
- web and Apple can consume shared contracts with less local reinterpretation
- core shared-code seams are clearer than client-local seams
