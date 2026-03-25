---
title: Capability Resolution Engine and Policy Decisions
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 026-core-run-event-schema
  - 029-policy-config-loader
labels:
  - capabilities
  - policy
  - runs
  - phase-5
---

# Context & Objectives

Implement a deterministic resolver that evaluates requested actions against policy config and emits explicit `PolicyDecision` run events for allow/deny/escalate outcomes.

# Impacted Files & Symbols

- **Crate**: `crates/vel-core`
  - **Symbols**: capability request/decision models
- **Crate**: `crates/veld`
  - **Symbols**: resolver service and run-event emission
- **Crate**: `crates/vel-cli`
  - **Symbols**: surfaced decision summaries for execute/dry-run

# Technical Requirements

- Every attempted tool call or mutation path must pass through resolver.
- Resolver outputs typed reason codes and human-readable decision notes.
- Escalation class must be available for confirmation-required operations.
- Decision results must be persisted as run events before execution/mutation.

# Implementation Steps (The How)

1. Add resolver trait + default implementation.
2. Map policy config semantics to decision outcomes.
3. Integrate resolver into run planning and execution gates.
4. Persist decision trail for explainability and audit.

# Acceptance Criteria

1. [x] Allow/deny/escalate decisions are deterministic for same inputs.
2. [x] Decision reason codes are persisted in run event stream.
3. [x] Mutation and tool paths cannot bypass resolver.
4. [x] CLI can surface policy decisions from persisted run data.

# Verification & Regression

- **Unit Test**: table-driven capability resolution cases.
- **Integration Test**: denied operation emits persisted decision and halts.
- **Integration Test**: escalated operation waits for confirmation gate.
