---
created: 2026-04-16T00:00:00.000Z
title: Implement backup export retention pruning
area: runtime
completed: 2026-04-16T00:00:00.000Z
files:
  - crates/veld/src/services/backup.rs
  - crates/vel-storage/src/repositories/backup_runs_repo.rs
  - docs/user/backup-and-restore.md
---

## Problem

`backup_export.retention_count` is currently contract-visible intent only. Export roots will accumulate old manifests and domain snapshots until pruning is implemented.

## Scope

- Prune old export outputs according to configured retention count after a successful export.
- Never prune the export that was just written.
- Keep pruning failures visible without corrupting the successful export manifest.
- Add focused tests for retention count behavior and failure-safe handling.

## Notes

Pruning should operate only inside the configured export target root and should fail closed on path-boundary ambiguity.

## Progress

- 2026-04-16: Added immutable per-run export directories at `runs/<export_id>/` with root `manifest.json` retained as a latest pointer.
- 2026-04-16: Added `export_root` to the backup export manifest contract so `target_root` remains the configured base while the manifest also identifies the immutable run directory.
- 2026-04-16: Implemented `retention_count` pruning for older successful export run directories under `target_root/runs/`.
- 2026-04-16: Added path-boundary safeguards so pruning refuses symlink or canonicalization escapes outside the `runs/` root and reports the failure as a warning while preserving the successful export.
- 2026-04-16: Updated user and architecture docs for run-addressable exports, retention pruning, and optional parquet derivative paths.

Remaining scope: none for the current retention slice. Scheduled export execution remains deferred.
