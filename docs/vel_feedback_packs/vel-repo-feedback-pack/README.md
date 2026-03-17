---
title: Vel Repo Feedback Pack
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Vel Repo Feedback Pack

This pack turns high-level repo review into concrete, Codex-ready implementation work.

## Intent

Strengthen the codebase without changing Vel's product thesis:
- keep `vel-core` as domain truth
- keep `vel-storage` as persistence boundary
- keep `veld` as orchestration/runtime
- reduce drift between docs, routes, services, and schema
- harden the repo for the next phase: suggestion, uncertainty, and looping execution

## Pack contents

1. `00-spec-repo-hardening.md`
2. `tickets/001-docs-canonicalization.md`
3. `tickets/002-domain-ontology-and-naming.md`
4. `tickets/003-route-service-boundaries.md`
5. `tickets/004-current-context-contract.md`
6. `tickets/005-storage-schema-hardening.md`
7. `tickets/006-api-contract-alignment.md`
8. `tickets/007-test-matrix-and-fixtures.md`
9. `tickets/008-observability-and-debuggability.md`

## Guidance for the agent

Work top-down in this order:

1. docs and contract cleanup
2. domain naming and service boundaries
3. current-context contract hardening
4. storage/schema invariants
5. API cleanup
6. tests and operator debug surfaces

Do not introduce speculative subsystems while implementing this pack.
Prefer small explicit types over new generic abstractions.
