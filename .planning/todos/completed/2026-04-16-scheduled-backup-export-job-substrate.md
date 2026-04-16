---
created: 2026-04-16T00:00:00.000Z
title: Add scheduled backup export job substrate
area: runtime
files:
  - docs/future/nas-backup-and-cold-storage-export-spec.md
  - crates/veld/src/worker.rs
  - crates/vel-storage/src/repositories/processing_jobs_repo.rs
  - crates/vel-storage/src/repositories/runtime_loops_repo.rs
---

## Problem

The manual NAS/local-source export path now exists, but recurring export is not safe to implement as a direct interval wrapper around manual export. The future spec requires target-level scheduling semantics that the current runtime loop table and `PendingJob` substrate do not yet provide.

## Scope

- Define a scheduled export job primitive with stable run IDs and per-target dedupe.
- Add skip-or-queue behavior for already-running target exports.
- Persist lifecycle events and terminal state for each scheduled export attempt.
- Add bounded retry semantics that are visible to operator status surfaces.
- Keep `backup_export` disabled by default until this substrate exists.

## Notes

Use the manual export service as the payload writer, but do not let the runtime loop call it directly without a claimable scheduled job/run record.

## Progress

2026-04-16: completed the storage-only scheduled job substrate slice.

- Added a focused `backup_jobs_repo` over `v0_backup_jobs`, `v0_backup_job_attempts`, and `v0_backup_job_events`.
- Added typed storage methods to queue scheduled cold-storage export jobs, dedupe queued/running jobs by storage target, claim the next due job, and persist queued/started/succeeded/failed lifecycle events.
- Added retry state for transient failures without enabling the worker or calling manual export from a loop.
- Left `backup_export` disabled/fail-closed; execution and trust degradation remain follow-up work.

Verification:

- `cargo test -p vel-storage backup_job`
