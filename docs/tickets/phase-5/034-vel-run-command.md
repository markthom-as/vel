---
title: vel run MVP Command Path
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 027-sqlite-run-store
  - 028-artifact-store
  - 030-capability-resolution-engine
  - 031-tool-runner-abstraction
  - 032-mutation-protocol-discipline
  - 033-llm-provider-interface
labels:
  - vel-cli
  - mvp
  - run
  - phase-5
---

# Context & Objectives

Ship `vel run` as the first end-to-end harness entrypoint: assemble context, resolve capabilities, select workflow/agent mode, execute gated steps, persist run, and emit artifacts.

# Impacted Files & Symbols

- **Crate**: `crates/vel-cli`
  - **Symbols**: `vel run` command parser and output renderer
- **Crate**: `crates/veld`
  - **Symbols**: run orchestrator service and persistence wiring
- **Crate**: `crates/vel-core`
  - **Symbols**: run plan and outcome contracts

# Technical Requirements

- Command accepts intent text, optional refs, optional workflow/skill hint.
- Runtime must persist `run_id`, decision trail, execution outcomes, artifacts, terminal status.
- Mutation-eligible steps must go through proposal/confirmation/commit path.
- Output must be operator-readable without leaking sensitive internals.

# Implementation Steps (The How)

1. Implement CLI argument shape and request DTO mapping.
2. Add orchestrator flow (context -> policy -> plan -> execute -> persist).
3. Wire artifact emission and terminal run reporting.
4. Add integration tests for happy-path and policy-denied path.

# Acceptance Criteria

1. [x] `vel run` persists complete run records with stable `run_id`.
2. [x] Command output includes terminal status and artifact references.
3. [x] Denied or failed steps are reflected in persisted event trail.
4. [x] No direct mutation path bypasses confirmation discipline.

# Verification & Regression

- **CLI Integration Test**: successful run with persisted artifacts.
- **CLI Integration Test**: policy-denied run with explicit reason.
- **Manual Smoke**: execute one local run and inspect run/event/artifact outputs.
