---
created: 2026-04-16T00:00:00.000Z
title: Surface scheduled backup export failures in trust status
area: runtime
files:
  - crates/veld/src/services/backup.rs
  - crates/veld/src/routes/backup.rs
  - crates/vel-cli/src/commands/backup.rs
  - docs/api/runtime.md
---

## Problem

Manual export status reports the latest successful export run. Once scheduled export has real job/run semantics, scheduled failures need to degrade the export trust posture without polluting the separate backup-pack trust status.

## Scope

- Read scheduled export terminal state from the durable scheduled-job/run substrate.
- Mark export status degraded when the latest relevant scheduled attempt failed after the latest successful export.
- Include concise warnings with the failed target and operator-relevant error.
- Preserve manual export status behavior when no scheduled job has ever run.

## Dependency

The storage-only scheduled backup export job substrate is complete. This status slice should consume that job terminal state; it should still not enable scheduled export execution by itself.

## Progress

2026-04-16: completed scheduled-failure export trust status.

- `GET /v1/backup/export/status` and the backing service now read the latest finished scheduled cold-storage export job.
- If no successful export exists, status remains `missing` unless the latest scheduled terminal job failed, in which case it returns `degraded` with both the no-success warning and scheduled failure warning.
- If a successful manual export exists, status degrades only when the latest scheduled failure finished at or after that successful export.
- Queued/running jobs are not treated as degraded status, and scheduled execution remains disabled/fail-closed.

Verification:

- `cargo test -p vel-storage backup_job`
- `cargo test -p veld --test backup_flow backup_export_status`
