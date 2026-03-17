---
title: Cross-Cutting Trait Audit Baseline
doc_type: audit
status: in-progress
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - cross-cutting traits
  - subsystem audit
  - phase-1
related_files:
  - docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md
  - docs/tickets/phase-1/018-cross-cutting-system-traits-baseline.md
  - docs/MASTER_PLAN.md
summary: Repository-wide baseline audit for modularity, accessibility, configurability, data logging, rewind/replay, and composability coverage by subsystem.
---

# Scope

This artifact operationalizes ticket `018-cross-cutting-system-traits-baseline.md` by listing major subsystems, current trait coverage, and explicit gap classification.

Coverage labels:

- `baseline`: clear owning seam exists and the trait is visible in current docs/code.
- `partial`: some seams exist, but visibility or consistency is incomplete.
- `gap`: material missing abstraction, contract, or verification coverage.
- `n/a`: explicitly not applicable for the subsystem boundary.

# Subsystem Audit Matrix

| Subsystem | Modularity | Accessibility | Configurability | Data Logging | Rewind/Replay | Composability | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `crates/vel-core` | baseline | n/a | partial | partial | partial | baseline | Domain seams are clear; typed-context migration remains in ticket `002`. |
| `crates/vel-storage` | partial | n/a | partial | partial | partial | partial | Repository modularization is in progress in ticket `001`. |
| `crates/veld` runtime + routes | partial | partial | baseline | partial | partial | partial | Service/DTO boundary and auth hardening still open in tickets `003` and `015`. |
| `crates/vel-config` | baseline | partial | baseline | partial | partial | baseline | Templates parse in tests; effective-config inspection is still planned. |
| `crates/vel-cli` | partial | baseline | partial | partial | n/a | partial | Docs catalog is machine-readable; broader command output consistency is pending. |
| `clients/web` | partial | partial | partial | partial | n/a | partial | Operator accessibility and effective-config clarity are tracked by ticket `019`. |
| `clients/apple` | partial | partial | partial | partial | n/a | partial | Shared documentation catalog is in place; broader parity remains planned. |
| `packages/*` shared web packages | partial | partial | partial | partial | n/a | partial | Package contracts exist but subsystem trait docs are still thin. |
| Integrations (`veld` services + docs pack) | partial | partial | baseline | partial | partial | baseline | Canonical vocabulary exists; runtime alignment is tracked by ticket `022`. |
| Docs/templates/tickets surfaces | baseline | baseline | baseline | partial | partial | baseline | Authority chain and templates exist; command-backed walkthrough density still needs growth. |

# Gap Classification

## Documentation-only gaps

- subsystem-level accessibility and logging expectations are not consistently documented for every client/package surface.
- replay/reconstruction expectations are not explicitly documented for several non-runtime subsystems.

## Architecture/implementation gaps

- typed context and schema-on-write enforcement (`002-typed-context-transition.md`).
- service/DTO boundary hardening (`003-service-dto-layering.md`).
- auth-by-default and deny-by-default route classification (`015-http-surface-auth-hardening.md`).
- storage decomposition and transaction seam completion (`001-storage-modularization.md`).
- operator-surface accessibility and effective-config clarity (`019-operator-accessibility-config-clarity.md`).

# Queue Coverage Check

Current ticket coverage for material gaps:

- `001-storage-modularization.md`
- `002-typed-context-transition.md`
- `003-service-dto-layering.md`
- `015-http-surface-auth-hardening.md`
- `019-operator-accessibility-config-clarity.md`
- `022-data-sources-and-connector-architecture.md`
- `025-config-and-contract-fixture-parity.md`

If a future gap is found without ticket coverage, add coverage in `docs/tickets/README.md` and `docs/MASTER_PLAN.md` in the same change.

# Review Loop

When creating or updating an architecture doc or ticket:

1. classify each trait as `required`, `affected`, or `n/a` with owning seam references.
2. check this audit table for the affected subsystem and update coverage notes if the seam changed.
3. add or patch ticket coverage for any newly discovered material gap.
