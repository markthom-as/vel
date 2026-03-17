---
title: TICKET-ID-short-name
status: planned | in-progress | completed
owner: agent | staff-eng
type: architecture | feature | bugfix | refactor
priority: high | medium | low
created: YYYY-MM-DD
updated: YYYY-MM-DD
depends_on:
  - TICKET-ID
labels:
  - subsystem
  - technical-debt
  - distributed
---

# Context & Objectives
*A technical, high-signal summary of the task. Explain the current state and the target state.*

# Impacted Files & Symbols
- **File**: `crates/veld/src/services/example.rs`
  - **Symbols**: `ExampleStruct`, `trait ExampleTrait`, `fn handle_event`
- **File**: `crates/vel-storage/src/db.rs`
  - **Symbols**: `impl Storage`, `fn get_example`

# Technical Requirements
- **Constraint 1**: No public API breaking changes.
- **Constraint 2**: Use `tokio::sync::mpsc` for inter-service communication.
- **Constraint 3**: All new logic must be unit-tested in isolation.
- **Constraint 4**: If contracts change, update schema/manifest docs and canonical templates or fixtures in the same change.

# Cross-Cutting Trait Impact
- **Modularity**: required | affected | n/a — explain the seam or boundary impact.
- **Accessibility**: required | affected | n/a — explain operator, UI, CLI, or machine-readability impact.
- **Configurability**: required | affected | n/a — explain config/default/effective-config impact.
- **Data Logging**: required | affected | n/a — explain logs, events, traces, or denial-record impact.
- **Rewind/Replay**: required | affected | n/a — explain replay, idempotency, snapshot, or reconstruction impact.
- **Composability**: required | affected | n/a — explain contract, manifest, reusable component, or service-composition impact.

# Implementation Steps (The "How")
1. **Research**: Locate symbols and verify current behavior with fast repo search and focused tests.
2. **Strategy**: Outline the code motion or new logic in a plan.
3. **Act**: Apply surgical changes with scoped patches.
4. **Clean**: Run `cargo fmt` and `cargo clippy`.

# Acceptance Criteria
1. [ ] Criterion A: Specific behavioral outcome.
2. [ ] Criterion B: Specific structural outcome (e.g., "File X is < 500 lines").
3. [ ] Criterion C: Performance target (e.g., "Inference latency < 50ms").

# Verification & Regression
- **Unit Test**: `cargo test -p veld services::example`
- **Integration Test**: `cargo test -p veld --test api_example`
- **Smoke Check**: `vel example run --debug`
- **Invariants**: Assert that `X` still holds true after the change.

# Agent Guardrails
- **Secret Protection**: Do not log or print anything from `var/data/` or `.env`.
- **Minimal Context**: Use `read_file` with line ranges for large files.
- **Parallelism**: Run independent `grep_search` calls in parallel.
