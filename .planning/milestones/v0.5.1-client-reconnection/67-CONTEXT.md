# Phase 67 Context

## Goal

Audit every stale client/backend seam before transport or surface rewrites begin.

## Boundary

This phase inventories and classifies seams; it does not yet implement the new transport layer.

## Required Outputs

- client contract inventory
- deprecated route kill list
- rewrite / quarantine / delete classification
- explicit note of any unavoidable temporary shims
- pre-frozen allow-list target for canonical actions in `Modules`, `Integrations`, `Accounts`, and `Scopes`
