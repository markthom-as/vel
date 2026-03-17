---
title: Storage-layer modularization plan
status: in_progress
owner: agent
type: architecture
priority: medium
created: 2026-03-17
depends_on:
  - 004-architecture-map-and-module-boundary-audit.md
labels:
  - vel
  - storage
  - modularity
---

Plan a responsible split of the large storage module without breaking domain/storage contracts.

## Current hotspot

Primary files:

- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs)
- [crates/vel-storage/src/lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs)

[crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs) currently combines:

- storage infrastructure and migration wiring
- domain repositories across many unrelated areas
- shared row-mapping and JSON/time helpers
- storage tests

The crate boundary is still correct. The internal module boundary is not.

## Scope

- query-area inventory inside `vel-storage`
- stable submodule seams
- migration and test strategy for extraction

## Domain inventory

The current `Storage` impl is logically split into these persistence domains:

1. infrastructure
2. captures and ingest jobs
3. commitments and dependencies
4. signals and transcripts
5. context and inference outputs
6. nudges, risk, suggestions, and uncertainty
7. integration connections and events
8. threads and links
9. runs, artifacts, and refs
10. chat persistence and settings
11. runtime loops
12. cluster workers and work assignments

## Target internal modules

Preserve one public `Storage` facade, but split implementation internally by domain:

- `infra.rs`
  - connect, migrate, healthcheck, schema_version, storage error, sqlite helpers
- `captures.rs`
  - capture CRUD, capture search, ingest jobs
- `commitments.rs`
  - commitments and commitment dependencies
- `signals.rs`
  - signals and assistant transcripts
- `context.rs`
  - inferred state, current context, context timeline, orientation snapshot
- `guidance.rs`
  - nudges, nudge events, risk snapshots, suggestions, uncertainty
- `integrations.rs`
  - integration connections, setting refs, integration events
- `threads.rs`
  - thread CRUD and links
- `runs.rs`
  - artifacts, runs, run events, refs
- `chat.rs`
  - conversations, messages, interventions, event log, settings
- `runtime_loops.rs`
  - loop claim, ensure, complete, list, get, update config
- `cluster.rs`
  - work assignments and cluster workers
- `mapping.rs`
  - row mappers and JSON/time helpers

## Extraction strategy

### Rule 1. Preserve the public facade

Keep:

- `pub struct Storage`
- current public method names
- current public insert/record type re-exports from [crates/vel-storage/src/lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs)

Do not turn this ticket into a public API rename pass.

### Rule 2. No schema changes in the first pass

This is code motion and internal modularization only.

Keep:

- `MIGRATOR`
- migration behavior
- SQL schema
- existing query semantics

### Rule 3. Extract helpers first

Move shared helpers and low-risk infra out first:

- sqlite connect options
- JSON parsing helpers
- timestamp conversion helpers
- common row-mapping utilities

This reduces clutter before touching higher-risk domains.

### Rule 4. Extract low-fan-out domains before central ones

Suggested order:

1. `threads`
2. `integrations`
3. `runtime_loops`
4. `cluster`
5. `chat`
6. `runs`
7. `guidance`
8. `captures`
9. `signals`
10. `context`
11. `commitments`

Rationale:

- start with domains that have clearer boundaries and less fan-out
- leave the most central runtime read/write surfaces until the extraction pattern is proven

### Rule 5. Keep one delegating `impl Storage` first

Recommended pattern:

- extracted modules define `pub(super)` helper functions that take `&SqlitePool`
- `impl Storage` remains a thin delegator initially

This minimizes churn in dependent crates and allows the split to happen without redesigning the storage API.

## Test guardrails

Protect these behaviors during extraction:

- existing storage unit tests in [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs)
- integration-style runtime loop tests in [crates/veld/tests/runtime_loops.rs](/home/jove/code/vel/crates/veld/tests/runtime_loops.rs)
- suggestion flow tests in [crates/veld/tests/suggestion_engine.rs](/home/jove/code/vel/crates/veld/tests/suggestion_engine.rs)
- broad app-level route tests that construct storage insert types directly

Add:

- one facade-level smoke test per extracted module family, using `Storage` methods rather than internal helpers
- gradual relocation of storage tests alongside extracted modules while preserving in-memory migration-first setup

## Coupling risks

Important current coupling:

- `veld` imports many `vel_storage` insert and record types directly
- [crates/vel-storage/src/lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs) is therefore a compatibility shell for the entire repo

Guardrail:

- keep re-exports stable throughout the modularization pass

## Acceptance criteria

- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs) is no longer the sole home for unrelated persistence domains
- `Storage` remains the stable public facade
- no schema changes are required for the first modularization pass
- existing runtime and route tests continue to pass without public API churn

## Recommended first implementation slice

Start with:

1. `infra.rs`
2. `mapping.rs`
3. one low-fan-out domain such as `threads.rs` or `runtime_loops.rs`

Do not start by splitting commitments, signals, or current context. Those are too central for the first proof-of-pattern extraction.
