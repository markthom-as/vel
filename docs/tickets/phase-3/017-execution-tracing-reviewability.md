---
title: Execution Tracing, Handoff Telemetry & Reviewability
status: in-progress
owner: staff-eng
type: verification
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 006-connect-launch-protocol
  - 016-capability-broker-secret-mediation
  - 023-self-awareness-and-supervised-self-modification
  - 024-machine-readable-schema-and-manifest-publication
labels:
  - veld
  - tracing
  - observability
  - phase-3
---

# Context & Objectives

Vel already has durable run IDs, run events, and operator run inspection surfaces. Remaining work is to elevate this into full trace/handoff reviewability for multi-step and delegated workflows.

This ticket closes that gap by introducing trace linkage, handoff envelopes, and external-call attribution while preserving existing run-event coverage.

# Current Baseline (Already Present)

- run records and run events persist in storage
- run inspection exists in API and CLI (`/v1/runs`, `vel run inspect`)
- retry/requeue lifecycle events are already emitted and test-covered

# Remaining Work Focus

- add explicit `trace_id` and parent-child linkage semantics
- persist/inspect structured handoff envelopes
- attribute external calls and capability denials to trace/run boundaries
- provide operator-readable trace views (CLI/web/API)

# Impacted Files & Symbols

- **Crate**: `vel-core`
  - **Symbols**: trace/handoff envelope types and identifiers
- **Crate**: `vel-storage`
  - **Symbols**: trace/handoff persistence models and queries
- **Crate**: `veld`
  - **Symbols**: boundary instrumentation (connect, tools, external calls)
- **Crate**: `vel-cli` and `clients/web`
  - **Symbols**: trace inspection surfaces
- **Docs**: `docs/cognitive-agent-architecture/agents/handoffs.md`, `docs/api/runtime.md`
  - **Symbols**: handoff envelope and inspect-path contracts

# Technical Requirements

- **Stable Identifiers**: standardize run, trace, and parent/child step linkage.
- **Handoff Records**: capture objective, constraints, capability scope, and expected output contract.
- **Boundary Attribution**: emit structured records for external calls, denials, and terminal outcomes.
- **Inspection Surfaces**: operators can inspect trace-linked workflows without raw log spelunking.
- **Test Hooks**: deterministic tests can assert trace completeness and linkage.

# Cross-Cutting Trait Impact

- **Modularity**: required — tracing schema should be shared across runtime boundaries.
- **Accessibility**: affected — trace inspection must remain operator-readable.
- **Configurability**: affected — trace retention/detail knobs should be explicit.
- **Data Logging**: required — this is the primary observability ticket.
- **Rewind/Replay**: required — trace records must support replay and reconstruction.
- **Composability**: required — trace linkage should compose with eval and connect workflows.

# Implementation Steps (The How)

1. **Schema pass**: define trace/handoff envelope structures and IDs.
2. **Persistence pass**: add storage/query support for trace-linked records.
3. **Instrumentation pass**: emit structured boundary events in delegated flows.
4. **Inspection pass**: expose trace views in operator API/CLI/web and tests.

# Acceptance Criteria

1. [ ] Trace-linked identifiers are present for multi-step/delegated workflows.
2. [ ] Handoffs persist objective/scope/output-contract metadata.
3. [ ] External calls and denials are attributable to originating run/trace.
4. [ ] Operators can inspect workflow traces through supported surfaces.

# Verification & Regression

- **Unit Test**: trace ID/linkage and envelope serialization.
- **Integration Test**: multi-step workflow with trace completeness assertions.
- **Smoke Check**: inspect one traced workflow via API/CLI/web path.
- **Invariants**: no high-impact delegated workflow completes without boundary events.
