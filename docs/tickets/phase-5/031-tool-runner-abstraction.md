---
title: Tool Runner Abstraction with Structured Outcome Envelopes
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 030-capability-resolution-engine
labels:
  - tools
  - execution
  - reliability
  - phase-5
---

# Context & Objectives

Create a provider-agnostic tool runner interface for shell/tool execution with structured outcomes (success, timeout, refusal, failure) and run-event visibility.

# Impacted Files & Symbols

- **Crate**: `crates/vel-core`
  - **Symbols**: tool request/result envelope types
- **Crate**: `crates/veld`
  - **Symbols**: tool executor and timeout/retry handling
- **Crate**: `crates/vel-cli`
  - **Symbols**: tool outcome rendering in run output

# Technical Requirements

- Standardize tool invocation request and response envelopes.
- Capture timeout/refusal/retry semantics in typed result variants.
- Emit run events at invocation start and terminal outcome.
- Keep provider/tool-specific details at boundary mapping layers.

# Implementation Steps (The How)

1. Define tool abstraction traits and envelope types.
2. Implement shell-backed runner path with timeout support.
3. Add retry/refusal semantics for policy-compatible paths.
4. Wire run-event emission and CLI rendering.

# Acceptance Criteria

1. [x] Tool invocations return structured outcomes without ad-hoc strings.
2. [x] Timeout and refusal outcomes are distinguishable and persisted.
3. [x] Invocation lifecycle is visible in run events.
4. [x] Provider-specific shape does not leak into core tool contracts.

# Verification & Regression

- **Unit Test**: envelope mapping for success/timeout/refusal/failure.
- **Integration Test**: real shell command path with timeout case.
- **Regression Test**: retry behavior respects policy and idempotency constraints.
