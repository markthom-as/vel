---
title: Sync Ordering & Conflict Resolution Baseline
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 003-service-dto-layering
  - 015-http-surface-auth-hardening
labels:
  - vel-core
  - distributed
  - sync
  - phase-2
---

# Context & Objectives

Current sync behavior is primarily queue-and-apply through authority-owned actions. This ticket defines the durable ordering/conflict baseline for multi-node sync without assuming a single mandatory algorithm too early.

This ticket intentionally replaces the previous HLC-only framing with a two-slice execution model so implementation can land incrementally.

# Execution Slices

- **Slice A (Ordering Primitive)**: add a stable, comparable ordering primitive for distributed updates (HLC or an approved equivalent).
- **Slice B (Conflict Policy Integration)**: apply deterministic merge policy in sync reconciliation paths and entity repositories that need conflict handling.

# Impacted Files & Symbols

- **Crate**: `vel-core`
  - **Symbols**: ordering timestamp/value object, comparator semantics
- **Crate**: `vel-storage`
  - **Symbols**: fields/indexes required for conflict ordering where needed
- **File**: `crates/veld/src/services/client_sync.rs`
  - **Symbols**: reconciliation and action-application conflict gates

# Technical Requirements

- **Deterministic Ordering**: choose one ordering primitive and document tie-break rules.
- **Scoped Adoption**: apply ordering to entities and flows that actually need multi-writer conflict resolution first.
- **Migration Safety**: schema or repository changes must include migration and backward-compatible reads.
- **Boundary Discipline**: keep transport DTO changes at route/client boundary; avoid bleeding storage metadata into unrelated APIs.

# Cross-Cutting Trait Impact

- **Modularity**: required — keep ordering primitive in core and integration at service/storage seams.
- **Accessibility**: affected — operator diagnostics should expose conflict outcomes.
- **Configurability**: affected — node identity/time assumptions must be explicit.
- **Data Logging**: required — conflict decisions should be traceable.
- **Rewind/Replay**: required — ordering and merge outcomes must replay deterministically.
- **Composability**: required — ordering primitive reusable across sync paths.

# Implementation Steps (The How)

1. **Define primitive**: introduce ordering value object + comparator semantics.
2. **Add storage support**: persist ordering metadata where needed.
3. **Wire reconciliation**: enforce deterministic merge decisions in sync service.
4. **Expose diagnostics**: surface conflict decisions in inspectable outputs/events.

# Acceptance Criteria

1. [ ] Sync ordering primitive is implemented in `vel-core` with deterministic comparator tests.
2. [ ] Targeted sync entities persist/consume ordering metadata safely.
3. [ ] Reconciliation applies deterministic conflict policy with explicit tie-breakers.
4. [ ] Conflict decisions are inspectable in logs/events and covered by tests.

# Verification & Regression

- **Unit Test**: comparator and tie-break tests.
- **Integration Test**: multi-node conflict reconciliation scenarios.
- **Smoke Check**: local two-node sync conflict replay.
- **Invariants**: same input history yields same resolved state on all nodes.
