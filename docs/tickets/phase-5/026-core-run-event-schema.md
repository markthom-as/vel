---
title: Core Run and Event Schema
status: complete
owner: staff-eng
type: architecture
priority: high
created: 2026-03-24
updated: 2026-03-25
labels:
  - veld
  - vel-cli
  - schemas
  - runs
  - phase-5
---

# Context & Objectives

Define the canonical runtime truth objects for v0.1 harness work. `Run` and `RunEvent` become the append-only authority for planning, execution, policy decisions, and output provenance.

# Impacted Files & Symbols

- **Crate**: `crates/vel-core`
  - **Symbols**: `Run`, `RunId`, `RunStatus`, `RunEvent`, `ArtifactRef`, `PolicyDecision`
- **Docs**: `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`
  - **Symbols**: schema ownership and projection rules

# Technical Requirements

- Introduce typed Rust domain structs/enums for run lifecycle and event stream.
- Enforce append-only event semantics and explicit terminal statuses.
- Keep projection/read-model fields out of canonical event payload types.
- Include stable idempotency key shape for mutation-attempt events.

# Implementation Steps (The How)

1. Add core run/event domain types and status transition helpers.
2. Define serialization contracts with explicit version markers.
3. Add invariants for valid transitions (`planned -> running -> terminal`).
4. Document canonical/projection separation for v0.1 harness lane.

# Acceptance Criteria

1. [x] Core crate exposes typed canonical run/event contracts for v0.1 harness.
2. [x] Status transitions reject invalid edges.
3. [x] Event stream type supports append-only persistence without back-edit APIs.
4. [x] Schema ownership doc links include the new run/event contracts.

# Verification & Regression

- **Unit Test**: serialization round-trips for all enums and payload variants.
- **Unit Test**: status transition validator rejects invalid transitions.
- **Contract Test**: idempotency key field exists for mutation-attempt event variants.
