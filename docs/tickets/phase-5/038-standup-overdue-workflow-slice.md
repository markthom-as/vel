---
title: Morning Standup Overdue-Task Workflow Slice
status: baseline shipped
owner: staff-eng
type: implementation
priority: high
created: 2026-03-25
updated: 2026-04-15
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

Baseline implementation note: the backend and CLI workflow is mounted. Apple Watch and iOS voice overdue quick reactions are mounted over the same confirmation contract. Service-layer extraction remains follow-up work.

# Impacted Files & Symbols

- **Crate**: `crates/veld`
  - **Symbols**: daily-loop standup workflow orchestration, overdue action proposal/commit handlers
- **Crate**: `crates/vel-core`
  - **Symbols**: overdue action proposal contracts, confirmation payloads, mutation outcome contracts
- **Crate**: `crates/vel-api-types`
  - **Symbols**: standup overdue menu/confirm/apply/undo transport DTOs
- **Crate**: `crates/vel-cli`
  - **Symbols**: standup overdue workflow command wrappers and operator rendering
- **Client**: `clients/apple`
  - **Symbols**: Apple Watch and iOS voice overdue quick reactions over the backend confirmation contract
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
6. Mount Apple Watch and iOS voice quick reactions over the same backend contract with typed fallback on uncertainty.

# Acceptance Criteria

1. [x] Overdue standup workflow exposes bounded action menu for overdue commitments.
2. [x] Mutations cannot apply without explicit confirmation state.
3. [x] Reschedule supports confirm/override of Vel due-date guess with explicit persisted reason.
4. [x] Undo path is available for supported actions and returns deterministic result.
5. [x] CLI/operator output is readable and includes run/action evidence references.
6. [x] Apple Watch and iOS voice surfaces use the same bounded overdue action vocabulary and fail closed to typed fallback when the target or payload is uncertain.

# Verification & Regression

- **Integration Test**: menu -> confirm -> apply for each action type.
- **Integration Test**: apply without confirm is rejected fail-closed.
- **Integration Test**: repeated apply with same idempotency key is deterministic/no duplicate side effects.
- **Integration Test**: undo restores before state and replays deterministically for the same idempotency key.
- **Integration Test**: overdue menu honors requested `today` boundary.
- **CLI Test**: overdue menu/confirm/apply/undo commands parse.
- **Manual Smoke**: run one morning standup overdue sequence and inspect run/event/artifact outputs.

Verification evidence captured on 2026-04-15:

- `cargo test -p veld --test daily_loop_standup`
- `cargo test -p vel-cli cli_parses_daily_loop_overdue_commands`
- `swift test --package-path clients/apple/VelAPI --filter DailyLoopTests`
- `xcodebuild -project clients/apple/Vel.xcodeproj -scheme VeliOS -destination 'generic/platform=iOS Simulator' CODE_SIGNING_ALLOWED=NO build`
