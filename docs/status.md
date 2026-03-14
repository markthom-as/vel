# Vel — Current Status

What is implemented, what is partial, and what is next.

## Implemented

- **Capture storage** — Insert captures, list by ID, lexical search (FTS5).
- **Artifacts API** — Create and fetch artifacts; storage layer with metadata.
- **Run/event schema and inspection** — `runs`, `run_events`, `events` tables; `vel runs`, `vel run inspect <id>`, `GET /v1/runs`, `GET /v1/runs/:id`.
- **Doctor diagnostics** — `vel doctor` and `GET /v1/doctor` with structured checks (DiagnosticCheck + DiagnosticStatus).
- **Context endpoints** — `GET /v1/context/today`, `GET /v1/context/morning`, `GET /v1/context/end-of-day` (computed from orientation snapshot; not yet run-backed).
- **CLI** — `vel health`, `vel capture`, `vel search`, `vel today`, `vel morning`, `vel end-of-day`, `vel doctor`, `vel inspect capture <id>`, `vel runs`, `vel run inspect <id>`.
- **Crate boundaries** — Domain types (ContextCapture, SearchResult, OrientationSnapshot) live in `vel-core`; storage returns them; API layer maps to DTOs. `vel-storage` does not depend on `vel-api-types`.
- **Run events uniqueness** — `(run_id, seq)` unique on `run_events`.
- **Typed run payloads** — Run and RunEvent use `serde_json::Value` in domain/API; DB remains TEXT; (de)serialization at storage boundary.
- **Run transitions** — Immutable transitions in `vel-core` (start/succeed/fail/cancel return new `Self`).
- **Service layer** — Context generation and doctor logic live in `veld` services; routes are thin.
- **Artifact storage kind** — `storage_kind` (managed | external) in schema, core, and API.

## Partial

- **Context generation** — Today/morning/end-of-day are computed by the context_generation service from `orientation_snapshot()`. They do not yet create runs, persist output artifacts, or write refs/run_events.
- **Artifact metadata** — `size_bytes`, `content_hash` exist in schema; population and provenance linking are not yet full.
- **Global events** — `emit_event` exists and is used for `CAPTURE_CREATED`; broader system-level observability is not yet defined.

## Planned next

- **Run-backed context generation** — Make today/morning/end-of-day create a run, transition status, persist output artifact, create refs, append run events. Spec: `docs/specs/context-runs.md`.

## Intentionally deferred

- Distributed sync, mobile clients, execution automation, synthesis jobs, agent flows.
