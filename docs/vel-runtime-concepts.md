# Vel — Runtime Concepts

This document describes the runtime spine and the contract between events, run events, and refs.

## Contract: events vs run_events vs refs

- **run_events** — Lifecycle of one run. Examples: `run_created`, `run_started`, `artifact_written`, `run_succeeded`. One row per step or transition within a single run. This is the **primary timeline** for runtime work; use it for run-backed flows.
- **events** — System-wide audit stream for a small set of occurrences: e.g. `capture_created`, `daemon_started`, schema migration. Use sparingly; do not duplicate run_events here. Right now the main consumer is capture creation.
- **refs** — Stable relationships between durable objects. Examples: run → artifact, artifact → capture. Use refs to answer “what is related to what”; use run_events to answer “what happened during this run.”

**Rule:** Events describe *what happened* (system-level). Refs describe *what is related to what*. Run events describe *what happened during one run*.

## Runs

A **run** is a first-class execution record. Every meaningful operation (context generation, search, synthesis, etc.) can be represented as a run.

- **Run ID**: Stable identifier (`run_<uuid>`).
- **Kind**: `search`, `context_generation`, `artifact_extraction`, `synthesis`, or `agent`.
- **Status**: `queued` → `running` → `succeeded` | `failed` | `cancelled`.
- **Input/output/error**: Structured JSON (e.g. `serde_json::Value`) for reproducibility and inspection; stored as TEXT in SQLite, (de)serialized at the storage boundary.

### Run lifecycle

1. Create run (status `queued`), append `run_created` event.
2. Transition to `running` (set `started_at`), append `run_started`.
3. Do work; append milestone events (e.g. `context_generated`, `artifact_written`).
4. Transition to terminal status (`succeeded` | `failed` | `cancelled`), set `finished_at`, append terminal event.

## Run events

Each run has an append-only **event log** (`run_events`):

- `run_created` — run was created
- `run_started` — run began executing
- `run_succeeded` / `run_failed` / `run_cancelled` — terminal state
- `artifact_written`, `search_executed`, `context_generated` — extension events

Events have a monotonic `seq` per run and a `payload_json` for details.

## Provenance (refs)

The **refs** table stores relations between objects:

- **Relation types**: `generated_from`, `derived_from`, `attached_to`.
- **Typical links**: run → artifact (`attached_to`), artifact → capture (`derived_from` for context sources).

This allows answering: what run produced this artifact? What sources were used for this context?

### Lineage (context run)

```text
Capture
   ↓
Snapshot
   ↓
Context Run
   ↓
Artifact (context_brief, managed)
   ↓
Inspection (run detail, artifact summaries, refs)
```

## Operator commands

- **`vel doctor`** — Config, DB, schema version, artifact directory, daemon reachability.
- **`vel runs`** — List recent runs (id, kind, status, timestamps).
- **`vel run inspect <id>`** — Full run detail: input, output, error, events, and linked artifacts.

## API

- `GET /v1/runs` — List runs (newest first).
- `GET /v1/runs/:id` — Run detail including events and linked artifact summaries.

## Context generation (run-backed)

Context requests (`today`, `morning`, `end_of_day`) are run-backed:

- Each request creates a run (kind `context_generation`), transitions to `running`, loads the orientation snapshot, computes the result, writes a **managed** artifact (`artifact_type: context_brief`, atomic write: temp file then rename), persists **checksum** (sha256) and **size_bytes**, **metadata_json** (`generator`, `context_kind`), creates run → artifact ref and **artifact → capture** refs (DerivedFrom) for snapshot sources, appends run events, then transitions to `succeeded` (or `failed` with `error_json`).
- **Canonical path**: relative `context/<kind>/<date>/<run_id>.json` under artifact root.
- Run detail and `vel run inspect <id>` include linked artifacts.

### Flow (success)

```text
context request
  → run created (queued)
  → run started
  → context computed
  → artifact written (context_brief, managed; atomic write; context/<kind>/<date>/<run_id>.json)
  → refs linked (run → artifact; artifact → capture for each source)
  → run succeeded
```

### Event sequence (success)

`run_created` → `run_started` → `context_generated` → `artifact_written` → `run_succeeded`.

On failure: `run_created` → `run_started` → `run_failed`.
