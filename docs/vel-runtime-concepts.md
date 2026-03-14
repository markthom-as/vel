# Vel — Runtime Concepts

This document describes the runtime spine introduced by the full implementation spec.

## Runs

A **run** is a first-class execution record. Every meaningful operation (context generation, search, synthesis, etc.) can be represented as a run.

- **Run ID**: Stable identifier (`run_<uuid>`).
- **Kind**: `search`, `context_generation`, `artifact_extraction`, `synthesis`, or `agent`.
- **Status**: `queued` → `running` → `succeeded` | `failed` | `cancelled`.
- **Input/output/error**: JSON payloads for reproducibility and inspection.

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
- **Typical links**: run → artifact (`attached_to`), artifact → capture (`generated_from`).

This allows answering: what run produced this artifact? What sources were used for this context?

## Operator commands

- **`vel doctor`** — Config, DB, schema version, artifact directory, daemon reachability.
- **`vel runs`** — List recent runs (id, kind, status, timestamps).
- **`vel run inspect <id>`** — Full run detail: input, output, error, and event list.

## API

- `GET /v1/runs` — List runs (newest first).
- `GET /v1/runs/:id` — Run detail including events.

## Future: run-backed context

In the full spec, `today` and `morning` become run-backed: each request creates a run, transitions to `running`, produces an output artifact, writes provenance refs, then transitions to `succeeded` (or `failed` with structured `error_json`). That work is planned as Phase C; the run and event substrate is in place.
