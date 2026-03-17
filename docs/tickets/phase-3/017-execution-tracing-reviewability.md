---
title: Execution Tracing, Handoff Telemetry & Reviewability
status: planned
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

Vel already values inspectability, but the queue does not yet include a dedicated tracing and handoff reviewability ticket for agentic execution.

This ticket makes run IDs, handoff telemetry, external-call attribution, and operator inspection first-class so the system can explain what happened across multi-step workflows.

# Impacted Files & Symbols

- **Crate**: `veld`
  - **Symbols**: run events, connect lifecycle, external-call boundaries, handoff logging
- **Crate**: `vel-core`
  - **Symbols**: trace or handoff envelope types
- **Docs**: `docs/cognitive-agent-architecture/agents/handoffs.md`
  - **Symbols**: handoff envelope, trace linkage
- **Docs**: `docs/api/runtime.md`
  - **Symbols**: inspect surfaces for runs and traces

# Technical Requirements

- **Stable Identifiers**: Introduce or standardize `run_id`, `trace_id`, and child step/span identifiers for agentic workflows.
- **Handoff Telemetry**: Log handoffs with objective, constraints, output schema, and capability scope.
- **Boundary Events**: Emit structured records for external calls, capability denials, workflow transitions, and terminal outcomes.
- **Inspection Surface**: Provide an operator-readable inspection path through CLI, web, or both.
- **Verification Hooks**: Allow deterministic tests and eval harnesses to assert trace completeness.

# Implementation Steps (The "How")

1. **Envelope Design**: Define the trace and handoff record shapes.
2. **Instrumentation**: Add structured emission at major workflow boundaries.
3. **Inspection Surface**: Expose trace-linked run inspection to operators.
4. **Test Hooks**: Add assertions for trace presence and linkage in replay/eval flows.

# Acceptance Criteria

1. [ ] Agentic workflows produce stable run and trace identifiers.
2. [ ] Handoffs record objective, scope, and expected output shape.
3. [ ] External calls and capability denials are attributable to the initiating run or trace.
4. [ ] Operators can inspect a completed workflow without reading raw source code or logs only.

# Verification & Regression

- **Unit Test**: envelope serialization and linkage logic
- **Integration Test**: multi-step workflow with trace completeness assertions
- **Smoke Check**: CLI or web inspection of one traced workflow
- **Invariants**: no high-impact workflow completes without terminal state and boundary events

# Agent Guardrails

- **Trace Before Trust**: If an execution path cannot be inspected, do not treat it as production-ready.
- **No Secret Leakage**: Traces must carry identifiers and metadata, not decrypted credentials.
- **Structured Over Stringy**: Prefer typed envelopes over ad hoc log-string parsing.
