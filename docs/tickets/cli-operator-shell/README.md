---
title: CLI Operator Shell
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

# CLI Operator Shell

Owns `crates/vel-cli`.

This pack keeps the operator shell modular and aligned to current runtime contracts rather than letting `main.rs` keep absorbing unrelated command work.

## Tickets

- [CLI-001-split-command-tree-and-registration.md](CLI-001-split-command-tree-and-registration.md)
- [CLI-002-align-runtime-and-sync-command-families.md](CLI-002-align-runtime-and-sync-command-families.md)
- [CLI-003-normalize-output-errors-and-operator-flow.md](CLI-003-normalize-output-errors-and-operator-flow.md)

## Execution order

Run `CLI-001` first. `CLI-002` and `CLI-003` can proceed in parallel after that.

## Exit criteria

- command ownership is family-oriented
- runtime/sync/operator surfaces are coherent
- CLI output and error behavior are consistent
