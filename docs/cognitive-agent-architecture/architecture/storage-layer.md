# Storage Layer Architecture

Vel uses a **Modular Repository Pattern** to manage persistence. The storage layer is responsible for the durable state of the Authority Runtime.

## Overview

- **Crate**: `vel-storage`
- **Database**: SQLite (local-first)
- **Primary Interface**: `Storage` struct (Facade over an explicit storage backend)
- **Implementation**: Domain-specific repositories in `src/repositories/`

## Repository Pattern

To prevent the `db.rs` file from becoming an unmaintainable monolith, all domain logic is extracted into granular repositories.

### Repository Mandates

1.  **Granularity**: One repository per major domain (e.g., `chat_repo`, `runs_repo`, `commitments_repo`).
2.  **Statelessness**: Repository functions should generally be `pub(crate)` and take a `&SqlitePool` or `&mut Transaction`.
3.  **Domain Types**: Return types should be `vel-core` domain types or local `Record` structs.
4.  **No DTOs**: Repositories must never know about `vel-api-types`.
5.  **Backend Boundary**: `Storage` sits on top of an internal `StorageBackend` seam so the pool-backed implementation is explicit instead of implicit.

### Active Repositories

| Repository | Domain |
| :--- | :--- |
| `artifacts_repo` | File-backed metadata and storage pointers. |
| `assistant_transcripts_repo` | Raw LLM interaction logs. |
| `captures_repo` | User-initiated captures and search logic. |
| `chat_repo` | Conversations, messages, and interventions. |
| `cluster_workers_repo` | Swarm node/worker presence and heartbeats. |
| `commitment_risk_repo` | Risk snapshots for commitments. |
| `commitments_repo` | Actionable items and their dependencies. |
| `context_timeline_repo` | Historical context transitions. |
| `current_context_repo` | The "Current Truth" singleton. |
| `inferred_state_repo` | Result of cognitive inference loops. |
| `integration_connections_repo` | External service credentials and sync state. |
| `nudges_repo` | Proactive system alerts and snoozing. |
| `orientation_repo` | Bootstrapping data for context generation. |
| `processing_jobs_repo` | Async task queue for ingestion and LLM work. |
| `runs_repo` | Executable run lifecycles and event logs. |
| `runtime_loops_repo` | Scheduled maintenance and inference loops. |
| `settings_repo` | Key-value store for system configuration. |
| `signals_repo` | Raw telemetry events (the "input substrate"). |
| `suggestions_repo` | Steering recommendations and evidence. |
| `threads_repo` | Semantic grouping of related entities. |
| `uncertainty_records_repo` | Tracking and resolving low-confidence decisions. |
| `work_assignments_repo` | Specific task receipts within the swarm. |

## Transaction Management

The `Storage` facade provides high-level methods that manage their own transactions. For multi-repository atomic writes, the repository `*_in_tx` helpers can be called within a single `sqlx::Transaction`, and the crate has a focused test that exercises a cross-repository commit in one transaction.

## Schema Management

- Migrations are stored in `/migrations`.
- `infra::run_migrations` is called on startup via `Storage::migrate()`.
- The database file is typically located at `var/data/vel.sqlite`.
