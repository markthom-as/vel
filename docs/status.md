# Vel — Current Status

What is implemented, what is partial, and what is next.

## Implemented

- **Capture storage** — Insert captures, list by ID, lexical search (FTS5).
- **Artifacts API** — Create and fetch artifacts; storage layer with metadata.
- **Run/event schema and inspection** — `runs`, `run_events`, `events` tables; `vel runs`, `vel run inspect <id>` (including linked artifacts); `GET /v1/runs`, `GET /v1/runs/:id`.
- **Doctor diagnostics** — `vel doctor` and `GET /v1/doctor` with structured checks (DiagnosticCheck + DiagnosticStatus).
- **Context endpoints (run-backed)** — `GET /v1/context/today`, `GET /v1/context/morning`, `GET /v1/context/end-of-day` create a run, compute from orientation snapshot, write a managed JSON artifact, link run → artifact ref, and append run events.
- **CLI** — `vel health`, `vel capture`, `vel search`, `vel today`, `vel morning`, `vel end-of-day`, `vel doctor`, `vel inspect capture <id>`, `vel runs`, `vel run inspect <id>`.
- **Crate boundaries** — Domain types (ContextCapture, SearchResult, OrientationSnapshot) live in `vel-core`; storage returns them; API layer maps to DTOs. `vel-storage` does not depend on `vel-api-types`.
- **Run events uniqueness** — `(run_id, seq)` unique on `run_events`.
- **Typed run payloads** — Run and RunEvent use `serde_json::Value` in domain/API; DB remains TEXT; (de)serialization at storage boundary.
- **Run transitions** — Immutable transitions in `vel-core` (start/succeed/fail/cancel return new `Self`).
- **Service layer** — Context generation and doctor logic live in `veld` services; routes are thin.
- **Artifact storage kind** — `storage_kind` (managed | external) in schema, core, and API.

## Partial

- **Artifact metadata** — `size_bytes` is set for run-backed context artifacts; `content_hash` is optional and not yet computed on write.
- **Global events** — `events` table is used for `CAPTURE_CREATED`; other system-level events (e.g. daemon_started) are not yet emitted. Role is narrow and documented.

## Planned next

- Managed artifact write: checksum at write time; optional artifact → source capture refs.

## Intentionally deferred

- Distributed sync, mobile clients, execution automation, synthesis jobs, agent flows.
