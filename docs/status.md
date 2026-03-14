# Vel — Current Status

What is implemented, what is partial, and what is next.

## Implemented

- **Capture storage** — Insert captures, list by ID, lexical search (FTS5).
- **Artifacts API** — Create and fetch artifacts; storage layer with metadata.
- **Run/event schema and inspection** — `runs`, `run_events`, `events` tables; `vel runs`, `vel run inspect <id>`, `GET /v1/runs`, `GET /v1/runs/:id`.
- **Doctor diagnostics** — `vel doctor` and `GET /v1/doctor` (daemon, DB, schema version, artifact dir).
- **Context endpoints** — `GET /v1/context/today`, `GET /v1/context/morning`, `GET /v1/context/end-of-day` (computed from orientation snapshot; not yet run-backed).
- **CLI** — `vel health`, `vel capture`, `vel search`, `vel today`, `vel morning`, `vel end-of-day`, `vel doctor`, `vel inspect capture <id>`, `vel runs`, `vel run inspect <id>`.
- **Crate boundaries** — Domain types (ContextCapture, SearchResult, OrientationSnapshot) live in `vel-core`; storage returns them; API layer maps to DTOs. `vel-storage` does not depend on `vel-api-types`.
- **Run events uniqueness** — `(run_id, seq)` unique on `run_events`.

## Partial

- **Context generation** — Today/morning/end-of-day are computed in the route layer from `orientation_snapshot()`. They do not yet create runs, persist output artifacts, or write refs/run_events.
- **Artifact metadata** — `size_bytes`, `content_hash` exist in schema; population and provenance linking are not yet full.
- **Global events** — `emit_event` exists and is used for `CAPTURE_CREATED`; broader system-level observability is not yet defined.

## Planned next

- **Run-backed context generation** — Refactor today/morning/end-of-day into an application service that creates a run, transitions status, generates context, persists artifact, creates refs, emits run events, then returns result.
- **Typed payloads** — Replace raw JSON strings in run input/output/error and event payloads with `serde_json::Value` in domain/API; keep DB as TEXT.
- **Structured doctor** — DiagnosticCheck (name, status, message) instead of string-assembled fields.
- **Doc hierarchy** — Canonical (current-architecture, current-data-model, status) vs specs vs reviews/archive.

## Intentionally deferred

- Distributed sync, mobile clients, execution automation, synthesis jobs, agent flows.
