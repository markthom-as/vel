---
title: Ticket 504 - Add loop claims, idempotency, and source watermarks
status: proposed
owner: codex
priority: high
---

# Goal

Prevent duplicate loop execution and make loops safe to retry.

# Files

## New migration
- `migrations/0031_loop_claims.sql`

## Changed
- `crates/veld/src/services/loop_scheduler.rs`
- `crates/veld/src/services/external_sync.rs`
- `crates/veld/src/worker.rs`

# Implementation

## Claiming
Before a loop starts:
- acquire `loop_claims` lease
- reject if active unexpired claim exists
- release or expire on completion/failure

## Watermarks
Use `sync_watermarks` from the integration pack for:
- Todoist last snapshot fingerprint
- calendar last event cursor / range fingerprint
- project registry last parsed checksum

## Idempotency
Each loop should decide:
- no-op when watermark unchanged
- rerun when forced
- backfill when schema version changed

# Acceptance criteria

- double-starting the same loop kind does not cause duplicate writes
- loops can resume safely after crash or restart
