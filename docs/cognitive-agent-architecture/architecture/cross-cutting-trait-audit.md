---
title: Cross-Cutting Trait Audit Baseline
doc_type: audit
status: complete
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
| `crates/vel-core` | baseline | n/a | baseline | partial | baseline | baseline | Typed current-context and migration seams are explicit in code and stay versioned at the boundary. |
| `crates/vel-storage` | baseline | n/a | baseline | partial | baseline | baseline | Repositories compose under shared transactions and `Storage` now sits behind an explicit backend seam. |
| `crates/veld` runtime + routes | baseline | partial | baseline | partial | partial | baseline | Route/service layering and auth classification are explicit; remaining gaps are mostly operator-surface and observability depth. |
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

- operator-surface accessibility and effective-config clarity (`019-operator-accessibility-config-clarity.md`).
- machine-readable contract publication and consumer rollout (`024-machine-readable-schema-and-manifest-publication.md`).
- integration/runtime alignment for canonical connector vocabulary (`022-data-sources-and-connector-architecture.md`).

# Queue Coverage Check

Current ticket coverage for material gaps:

- `019-operator-accessibility-config-clarity.md`
- `022-data-sources-and-connector-architecture.md`
- `024-machine-readable-schema-and-manifest-publication.md`

If a future gap is found without ticket coverage, add coverage in `docs/tickets/README.md` and `docs/MASTER_PLAN.md` in the same change.

# Review Loop

When creating or updating an architecture doc or ticket:

1. classify each trait as `required`, `affected`, or `n/a` with owning seam references.
2. check this audit table for the affected subsystem and update coverage notes if the seam changed.
3. add or patch ticket coverage for any newly discovered material gap.
