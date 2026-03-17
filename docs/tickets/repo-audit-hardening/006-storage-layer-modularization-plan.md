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

Current audit judgment:

- the next move is internal repository-family modularization, not a crate split
- the public `Storage` facade should remain stable through the first pass

The crate boundary is still correct. The internal module boundary is not.

## Progress snapshot

Implemented low-risk slices so far:

- infrastructure/bootstrap helpers were moved behind `infra.rs`
- run and ref row mappers were moved out of `db.rs` into `run_refs.rs`
- run, run-event, retry-ready-run, and ref repository methods now delegate through `runs.rs`
- conversation, message, intervention, and event-log persistence now delegate through `chat.rs`
- settings, work assignments, cluster workers, and runtime-loop config delegation now route through `runtime_cluster.rs`

That is the intended first pattern:

- keep `Storage` stable
- move focused helper families first
- avoid schema churn while shrinking `db.rs`

## Scope

- query-area inventory inside `vel-storage`
- stable submodule seams
- migration and test strategy for extraction

## Domain inventory

The current `Storage` impl is best grouped into these repository families:

1. infrastructure and bootstrap
2. capture, commitments, dependencies, signals, search, and ingest jobs
3. context, timeline, nudges, suggestions, risk, and uncertainty
4. integrations, integration events, threads, and links
5. runs, artifacts, run events, and refs
6. chat persistence and event log
7. settings, runtime loops, work assignments, and cluster workers
8. orientation projection and row mappers

## Target internal modules

Preserve one public `Storage` facade, but split implementation internally by domain:

- `infra.rs`
  - connect, migrate, healthcheck, schema_version, storage error, sqlite helpers
- `capture_commitments.rs`
  - captures, commitments, dependencies, signals, search, ingest jobs
- `context_suggestions.rs`
  - current context, inferred state, context timeline, nudges, risk snapshots, suggestions, uncertainty
- `integration_threads.rs`
  - integration connections, integration events, threads, links
- `runs.rs`
  - artifacts, runs, run events, refs
- `chat.rs`
  - conversations, messages, interventions, event log
- `runtime_cluster.rs`
  - settings, runtime loops, work assignments, cluster workers
- `orientation.rs`
  - orientation snapshot only if its ownership stays explicit
- per-domain `mapping/*`
  - row mappers and JSON/time helpers colocated with their repository families where possible

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
- only the minimum common row-mapping utilities needed to unblock domain extraction

This reduces clutter before touching higher-risk domains.

Status:

- `infra.rs` is already in place for bootstrap helpers
- `run_refs.rs` now carries run and ref row mappers
- `runs.rs` now owns the run/ref repository family while `Storage` stays a stable facade
- `chat.rs` now owns the chat persistence family while `Storage` stays a stable facade
- `runtime_cluster.rs` now owns the settings and cluster/runtime repository family while `Storage` stays a stable facade
- the next slice should keep following repository-family seams rather than creating a generic dumping-ground helpers module

### Rule 4. Extract low-fan-out domains before central ones

Suggested order:

1. `runs`
2. `chat`
3. `runtime_cluster`
4. `integration_threads`
5. `context_suggestions`
6. `capture_commitments`
7. per-domain row mappers

Rationale:

- start with the cleanest transactional and operational seams
- preserve repo behavior by keeping central capture/commitment persistence until the extraction pattern is proven
- avoid a fake low-risk start that only moves tiny helpers and leaves all real coupling untouched

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
- do not split runs from run-events or work assignments from queue/retry semantics
- do not centralize all row mappers into one new omnibus mapper module

## Acceptance criteria

- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs) is no longer the sole home for unrelated persistence domains
- `Storage` remains the stable public facade
- no schema changes are required for the first modularization pass
- existing runtime and route tests continue to pass without public API churn

## Recommended first implementation slice

Start with:

1. `infra.rs`
2. `runs.rs`
3. `chat.rs`
4. `runtime_cluster.rs`

Do not start by splitting commitments, signals, or current context. Those are too central for the first proof-of-pattern extraction, and `capture` currently carries ingest-job side effects that make it a worse first seam than it looks.
