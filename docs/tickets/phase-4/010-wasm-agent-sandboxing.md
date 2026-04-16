---
title: Zero-Trust WASM Agent Sandboxing
status: planned
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
updated: 2026-04-16
depends_on:
  - 016-capability-broker-secret-mediation
  - 017-execution-tracing-reviewability
  - 023-self-awareness-and-supervised-self-modification
  - 024-machine-readable-schema-and-manifest-publication
labels:
  - veld
  - agentic
  - wasm
  - phase-4
---

# Context & Objectives

Vel does not yet ship an in-process zero-trust sandbox for third-party/community agents. This ticket introduces a WASM execution boundary with explicit host ABI, capability mediation, and deny-by-default policy.

# Impacted Files & Symbols

- **Crate**: `veld`
  - **Symbols**: wasm runtime host, policy gating, execution lifecycle
- **Crate**: `vel-core`
  - **Symbols**: host ABI contracts, capability request/denial records
- **Crate**: `vel-api-types`
  - **Symbols**: operator-facing sandbox run/diagnostic DTOs
- **Docs**: architecture and operator guidance for sandbox policy and limits

# Technical Requirements

- **Sandbox Runtime**: execute untrusted logic inside constrained WASM host.
- **Host ABI**: only explicit ABI calls are available; undefined calls fail closed.
- **Capability Mediation**: all external side effects go through brokered scope checks.
- **No Self-Escalation**: modules cannot widen permissions after launch.
- **Traceability**: ABI calls, denials, and terminal states emit trace-linked records.

## Runtime Decision

Use **wasmtime with the WebAssembly Component Model** as the default runtime and module format for this ticket.

Rationale:

- Component Model WIT gives typed imports and exports for the host ABI, which keeps capability boundaries explicit at the interface layer.
- deny-by-default is natural: guests only see host functions explicitly exported into their component world.
- async host support is required for non-blocking runtime integration.
- wasmtime has a production security track record and a mature resource-limiting surface.

Extism may be evaluated as a developer-experience wrapper over wasmtime, but it must not replace the explicit Component Model host ABI contract unless a later design record supersedes this decision.

# Cross-Cutting Trait Impact

- **Modularity**: required — isolate sandbox/runtime seam from core authority logic.
- **Accessibility**: affected — denial and failure reasons must be operator-readable.
- **Configurability**: required — policy scopes/timeouts/resource limits are explicit.
- **Data Logging**: required — sandbox calls and policy outcomes are inspectable.
- **Rewind/Replay**: affected — sandbox workflows should be reproducible for diagnostics.
- **Composability**: required — sandboxed agents must compose with connect/broker contracts.

# Implementation Steps (The How)

1. **Host ABI design**: finalize Component Model WIT callable surface and deny-by-default behavior.
2. **Runtime integration**: embed wasmtime with strict resource/policy limits.
3. **Capability wiring**: route side effects through broker mediation.
4. **Inspection tooling**: expose sandbox lifecycle/denial traces to operators.

# Acceptance Criteria

1. [ ] Sandboxed agents run without direct host filesystem/network authority.
2. [ ] Host ABI mediates all data/action requests with explicit policy checks.
3. [ ] Capability denials and outcomes are traceable and inspectable.
4. [ ] Secret material remains mediated and never leaked to sandbox payloads.

# Verification & Regression

- **Unit Test**: ABI allow/deny semantics and policy matching.
- **Integration Test**: sandboxed agent run with approved and denied operations.
- **Smoke Check**: run a sample sandbox module through operator flow.
- **Invariants**: no sandbox path bypasses capability broker checks.
