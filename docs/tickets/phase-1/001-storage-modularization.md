---
title: Modular Repository Pattern & Unit of Work (Storage Layer)
status: in-progress
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
labels:
  - vel-storage
  - modularity
  - transaction-safety
---

Re-architect `vel-storage` from a single `db.rs` file into a set of domain-specific repositories behind a `Storage` facade, implementing a `Unit of Work` pattern for transaction safety.

## Technical Details
- **Repository Split**: Move methods into `chat_repo`, `runs_repo`, `context_repo`, and `commitments_repo`.
- **Backend Trait**: Define `StorageBackend` trait for potential alternative backends.
- **Transaction Handling**: Repository methods must accept `&mut SqliteConnection` or similar to allow for shared transactions across repositories.
- **Unit of Work**: Create a wrapper that manages the transaction lifecycle (Commit/Rollback).
- **Replay-Friendly Records**: Preserve append-only event and run records behind stable repository seams instead of scattering sequence-sensitive writes.
- **Backend Configurability**: Keep backend selection and storage behavior behind explicit typed config and façade boundaries.
- **Composable Contracts**: Repository interfaces should compose cleanly under shared transactions without leaking implementation details upward.

## Acceptance Criteria
- `db.rs` contains only the facade and connectivity logic.
- Each domain has its own `.rs` implementation file in `src/repositories/`.
- Multi-repository writes can be wrapped in a single database transaction.
- Sequence-sensitive storage paths remain replayable and inspectable after extraction.
- All existing storage tests pass.
