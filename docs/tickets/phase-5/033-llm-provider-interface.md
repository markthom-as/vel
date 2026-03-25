---
title: LLM Provider Interface and Structured Synthesis Contract
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 026-core-run-event-schema
labels:
  - llm
  - abstraction
  - synthesis
  - phase-5
---

# Context & Objectives

Define one provider-neutral LLM interface for v0.1 harness synthesis and planning outputs so model/provider switching does not alter core run contracts.

# Impacted Files & Symbols

- **Crate**: `crates/vel-core`
  - **Symbols**: synthesis request/response contracts
- **Crate**: `crates/veld`
  - **Symbols**: provider adapter implementation and error mapping
- **Config**: `config/`
  - **Symbols**: provider selection fields and defaults

# Technical Requirements

- Define structured synthesis response contract (plan steps, rationale, cautions).
- Map provider-specific failures into canonical error envelopes.
- Support deterministic mock adapter for tests.
- Keep tool-calling semantics optional and boundary-scoped.

# Implementation Steps (The How)

1. Add LLM abstraction trait and canonical payload types.
2. Implement one production adapter and one mock adapter.
3. Wire adapter selection from config.
4. Persist synthesis request/response summary events for explainability.

# Acceptance Criteria

1. [x] Core execution path depends on provider-neutral trait only.
2. [x] One provider adapter passes contract tests.
3. [x] Mock adapter supports deterministic CLI/integration tests.
4. [x] Canonical failure envelopes are surfaced in run output.

# Verification & Regression

- **Unit Test**: provider error mapping to canonical envelope.
- **Contract Test**: production and mock adapters satisfy same trait behavior.
- **Integration Test**: run path executes with provider switch via config.
