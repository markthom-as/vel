---
title: Morning Standup Overdue-Task Workflow Slice
status: planned
owner: staff-eng
type: implementation
priority: high
created: 2026-03-25
updated: 2026-03-25
depends_on:
  - 032-mutation-protocol-discipline
  - 034-vel-run-command
  - 036-explainability-history-commands
  - 037-security-observability-hardening
labels:
  - daily-loop
  - commitments
  - overdue
  - accessibility
  - phase-5
---

# Context & Objectives

Ship the first supervised workflow vertical for morning standup/planning: detect overdue commitments, present bounded action choices, require confirmation before mutation, and persist explainable run/event evidence.

# Impacted Files & Symbols

- **Crate**: `crates/veld`
  - **Symbols**: daily-loop standup workflow orchestration, overdue action proposal/commit handlers
- **Crate**: `crates/vel-core`
  - **Symbols**: overdue action proposal contracts, confirmation payloads, mutation outcome contracts
- **Crate**: `crates/vel-api-types`
  - **Symbols**: standup overdue menu/confirm/apply/undo transport DTOs
- **Crate**: `crates/vel-cli`
  - **Symbols**: standup overdue workflow command wrappers and operator rendering
- **Docs**: `docs/api`
  - **Symbols**: planned API and CLI contract for overdue-task workflow

# Technical Requirements

- Standup session can request an overdue-task action menu for each overdue commitment.
- Action set is bounded: `close`, `reschedule`, `back_to_inbox`, `tombstone`.
- Mutation path follows `proposal -> confirmation -> commit` and records idempotency metadata.
- Reschedule path can include a Vel due-date guess that must be explicitly confirmed or overridden.
- Voice/assistive surfaces must map to the same backend action contract and fail closed to typed fallback.
- Every action emits run/event evidence and operator-readable before/after state.

# Implementation Steps (The How)

1. Add overdue workflow DTOs and service-level proposal/confirmation contracts.
2. Implement daily-loop standup route handlers for menu/confirm/apply/undo path.
3. Add CLI command wrappers for operator flow (`menu`, `confirm`, `apply`, `undo`).
4. Persist run-event lineage and before/after commitment state evidence.
5. Add focused integration tests for happy path, deny path, and duplicate/idempotent apply.

# Acceptance Criteria

1. [ ] Overdue standup workflow exposes bounded action menu for overdue commitments.
2. [ ] Mutations cannot apply without explicit confirmation state.
3. [ ] Reschedule supports confirm/override of Vel due-date guess with explicit persisted reason.
4. [ ] Undo path is available for supported actions and returns deterministic result.
5. [ ] CLI/operator output is readable and includes run/action evidence references.

# Verification & Regression

- **Integration Test**: menu -> confirm -> apply for each action type.
- **Integration Test**: apply without confirm is rejected fail-closed.
- **Integration Test**: repeated apply with same idempotency key is deterministic/no duplicate side effects.
- **Manual Smoke**: run one morning standup overdue sequence and inspect run/event/artifact outputs.
