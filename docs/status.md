# Vel — Current Status

What is implemented, what is partial, and what is next.

## Implemented

- **Capture storage** — Insert captures, list by ID, lexical search (FTS5).
- **Artifacts API** — Create and fetch artifacts; storage layer with metadata.
- **Run/event schema and inspection** — `runs`, `run_events`, `events` tables; `vel runs`, `vel run inspect <id>` (including linked artifacts); `GET /v1/runs`, `GET /v1/runs/:id`.
- **Doctor diagnostics** — `vel doctor` and `GET /v1/doctor` with structured checks (DiagnosticCheck + DiagnosticStatus).
- **Context endpoints (run-backed)** — `GET /v1/context/today`, `GET /v1/context/morning`, `GET /v1/context/end-of-day` create a run, compute from orientation snapshot, write a managed **context_brief** artifact (write temp → flush → fsync → rename; checksum, size_bytes, metadata_json with `generator`, `context_kind`, `snapshot_window`), link run → artifact and artifact → capture refs, append run events (`run_created`, `run_started`, `context_generated`, `artifact_written`, `refs_created`, `run_succeeded`), then transition to succeeded/failed.
- **Run timing** — API and CLI expose `duration_ms` (derived from started_at/finished_at) on run list and run detail.
- **Runtime invariants** — Documented in runtime-concepts: run_started precedes run_succeeded; artifact_written/refs_created before run_succeeded; run_failed does not produce artifact refs; run_succeeded implies artifact durability.
- **CLI** — `vel health`, `vel capture`, `vel search`, `vel today`, `vel morning`, `vel end-of-day`, `vel doctor`, `vel inspect capture <id>`, `vel runs`, `vel run inspect <id>` (duration, artifact size e.g. 3.4KB, event timestamps in CLI).
- **Crate boundaries** — Domain types (ContextCapture, SearchResult, OrientationSnapshot) live in `vel-core`; storage returns them; API layer maps to DTOs. `vel-storage` does not depend on `vel-api-types`.
- **Run events uniqueness** — `(run_id, seq)` unique on `run_events`.
- **Typed run payloads** — Run and RunEvent use `serde_json::Value` in domain/API; DB remains TEXT; (de)serialization at storage boundary.
- **Run transitions** — Immutable transitions in `vel-core` (start/succeed/fail/cancel return new `Self`).
- **Service layer** — Context generation and doctor logic live in `veld` services; routes are thin.
- **Artifact storage kind** — `storage_kind` (managed | external) in schema, core, and API.
- **Global events (optional)** — `DAEMON_STARTED` emitted on veld startup; `SCHEMA_MIGRATION_COMPLETE` emitted after migrations run (with `schema_version` in payload). Role remains system audit log.
- **Run statuses (future use)** — `RunStatus` includes `RetryScheduled` and `Blocked`; no transition logic yet (for future workflows).
- **Service-level result type** — Context generation returns `ContextRunOutput<T>` (run_id, artifact_id, context_kind, data); routes map `.data` to API response.
- **Canonical doc names** — Renamed `docs/vel-*.md` to `docs/*.md` (e.g. `runtime-concepts.md`, `data-model.md`); README and AGENTS.md updated.

## Partial

- **Global events** — Full set (e.g. `config_updated`) not implemented; role is narrow and documented.

## Planned next

- Run lifecycle: transition logic for RetryScheduled/Blocked once additional run-backed workflows exist.

## Intentionally deferred

- Distributed sync, mobile clients, execution automation, synthesis jobs, agent flows.
