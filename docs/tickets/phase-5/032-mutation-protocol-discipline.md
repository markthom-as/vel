---
title: Mutation Proposal and Commit Discipline
status: complete
owner: staff-eng
type: architecture
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 026-core-run-event-schema
  - 030-capability-resolution-engine
labels:
  - mutations
  - safety
  - auditing
  - phase-5
---

# Context & Objectives

Enforce a mutation protocol where all state-changing actions follow `MutationProposal -> confirmation gate -> MutationCommit`, with idempotency keys and write scope checks.

# Impacted Files & Symbols

- **Crate**: `crates/vel-core`
  - **Symbols**: mutation proposal/commit types, idempotency key
- **Crate**: `crates/veld`
  - **Symbols**: confirmation gate and mutation executor
- **Crate**: `crates/vel-cli`
  - **Symbols**: proposal preview and confirmation UX

# Technical Requirements

- No mutation code path may execute without a proposal event.
- Confirmation-required policy classes must block commit until explicit operator approval.
- Commits must include idempotency key and write scope record.
- Repeated commit attempts with same key must be safe and deterministic.

# Implementation Steps (The How)

1. Add mutation proposal/commit event contracts.
2. Implement confirmation gate workflow and idempotency checks.
3. Enforce write scope matching before commit.
4. Add CLI confirmation and rejection handling path.

# Acceptance Criteria

1. [x] Mutation attempts without proposal are rejected.
2. [x] Confirmation-required operations pause until operator approves.
3. [x] Commit writes include idempotency key and scoped target metadata.
4. [x] Duplicate commit retries do not produce duplicate side effects.

# Verification & Regression

- **Integration Test**: no-confirmation mutation attempt is blocked.
- **Integration Test**: approved proposal commits exactly once.
- **Regression Test**: duplicate commit key returns already-applied result.
