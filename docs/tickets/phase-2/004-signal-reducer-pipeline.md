---
title: Pluggable Signal Reducer Pipeline
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 002-typed-context-transition
  - 003-service-dto-layering
labels:
  - veld
  - inference
  - modularity
  - phase-2
---

# Context & Objectives

`crates/veld/src/services/inference.rs` still contains a large mixed-responsibility inference path. This ticket extracts deterministic signal reduction into explicit reducer modules and keeps orchestration in a thin engine loop.

The target is modular reduction without changing transport DTO boundaries or storage responsibilities.

# Impacted Files & Symbols

- **File**: `crates/veld/src/services/inference.rs`
  - **Symbols**: `InferenceEngine`, signal application loop
- **Directory**: `crates/veld/src/services/inference/`
  - **Symbols**: reducer trait, registry, domain reducers
- **Crate**: `vel-core`
  - **Symbols**: typed context contracts consumed by reducers

# Technical Requirements

- **Reducer Contract**: Introduce a reducer trait that operates on current typed context structures (`CurrentContextV1`) and persisted signal records, not ad hoc JSON blobs.
- **Registry**: Register reducers explicitly at startup; no giant centralized `match` ladder.
- **Determinism**: Reducer order must be explicit and replay-safe.
- **Boundary Discipline**: Reducers stay in service/core boundaries; no HTTP DTO logic in reducers.
- **Testability**: Each reducer must support focused tests without booting full daemon routes.

# Cross-Cutting Trait Impact

- **Modularity**: required — extract domain logic into isolated reducers.
- **Accessibility**: n/a — no direct UI path change.
- **Configurability**: affected — reducer registration order must be inspectable.
- **Data Logging**: affected — keep run/reducer decisions visible in traces.
- **Rewind/Replay**: required — deterministic replay behavior is a core goal.
- **Composability**: required — new reducers should compose without touching core loop internals.

# Implementation Steps (The How)

1. **Carve the seam**: define reducer trait + registry module under `services/inference/`.
2. **Extract domains**: move existing domain branches into reducer modules.
3. **Stabilize ordering**: make execution order explicit and covered by tests.
4. **Harden traces**: log reducer application boundaries for replay diagnostics.

# Acceptance Criteria

1. [ ] `inference.rs` becomes a thin orchestration loop with explicit reducer registration.
2. [ ] Domain reducers are in isolated modules with focused tests.
3. [ ] Reducer order is explicit and replay-deterministic.
4. [ ] Existing inference behavior remains functionally equivalent for current tests.

# Verification & Regression

- **Unit Test**: reducer-level tests per module.
- **Integration Test**: targeted `veld` inference/context synthesis tests.
- **Smoke Check**: run context generation path via CLI (`vel today` / equivalent local command).
- **Invariants**: no service code returns HTTP DTO types from reducer/core logic.
