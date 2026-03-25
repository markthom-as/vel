---
title: SQLite Run Store and Migration Baseline
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 026-core-run-event-schema
labels:
  - vel-storage
  - sqlite
  - runs
  - migrations
  - phase-5
---

# Context & Objectives

Persist canonical runs/events in SQLite with a migration-backed schema and repository APIs that preserve event ordering and terminal-state truth.

# Impacted Files & Symbols

- **Crate**: `crates/vel-storage`
  - **Symbols**: run repository, event append, terminal status write
- **Crate**: `crates/veld`
  - **Symbols**: storage wiring for harness execution paths
- **Directory**: `migrations/`
  - **Symbols**: run/event tables and indices

# Technical Requirements

- Add migration(s) for runs/events and artifact reference linkage.
- Repository APIs must append events in order and persist terminal run status.
- Reads must support list-by-recency and fetch-by-run-id.
- No update-in-place mutation of historical event payloads.

# Implementation Steps (The How)

1. Introduce migration for run/event tables and index strategy.
2. Add repository methods for create run, append event, set terminal status.
3. Wire repository into service path with transaction-safe writes.
4. Add focused storage integration tests.

# Acceptance Criteria

1. [x] New migrations apply cleanly on empty and existing dev DBs.
2. [x] Run create/append/finalize paths persist deterministic ordering.
3. [x] Run lookup/list APIs return expected records for harness commands.
4. [x] No repository API allows mutating past events.

# Verification & Regression

- **Integration Test**: create run, append events, finalize, then reload state.
- **Integration Test**: migration up path on clean DB and existing DB fixture.
- **Regression Test**: duplicate append attempt handling for idempotency key cases.
