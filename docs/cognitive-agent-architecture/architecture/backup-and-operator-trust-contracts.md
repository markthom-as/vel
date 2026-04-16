---
title: Backup And Operator Trust Contracts
doc_type: spec
status: draft
owner: staff-eng
created: 2026-03-19
updated: 2026-04-16
keywords:
  - backup
  - restore
  - trust
  - phase-9
summary: Canonical Phase 09 contract vocabulary for typed backup packs, inspectable verification state, and manual-first restore posture.
---

# Purpose

Define the stable backup and operator-trust vocabulary before the runtime slices widen around backup export, status surfaces, or recovery flows.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Backup manifest schema and example | `config` | `config/schemas/backup-manifest.schema.json`, `config/examples/backup-manifest.example.json` |
| Backup export manifest schema and example | `config` | `config/schemas/backup-export-manifest.schema.json`, `config/examples/backup-export-manifest.example.json` |
| Backup export source record schema and example | `config` | `config/schemas/backup-export-source-record.schema.json`, `config/examples/backup-export-source-record.example.json` |
| Backup trust DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |
| Operator backup guidance | `docs/user` | `docs/user/backup-and-restore.md` |
| Manifest registration | `config` | `config/contracts-manifest.json` |

# Stable Vocabulary

## Backup pack contents

A backup pack should tell the operator exactly what it contains:

- `backup_id`
- `created_at`
- `output_root`
- `database_snapshot_path`
- `artifact_coverage`
- `config_coverage`
- `explicit_omissions`
- `secret_omission_flags`
- `verification_summary`

This is a trust contract, not an opaque archive format. The operator should be able to inspect the manifest and understand what was captured, what was omitted, and whether verification succeeded.

## Coverage semantics

Coverage data must stay explicit and bounded:

- `artifact_coverage` lists durable artifact roots included in the pack and what was intentionally left out.
- `config_coverage` lists the non-secret runtime config files that were captured and the config surfaces that remain external.
- Coverage notes may explain why a path was omitted, but they must not bury omission rules in prose alone.

## Secret omission semantics

Backup packs are not a secret export mechanism.

- secret settings stay out of the pack by default
- integration tokens stay out of the pack by default
- local private key material stays out of the pack by default
- any later secret-migration contract must be explicit, narrow, and separately documented

## Verification semantics

Verification should be visible and checksum-backed:

- the manifest should record whether the pack was verified
- the checksum algorithm should be explicit
- checksum summary and inspected paths should be part of the operator-visible record
- verification failure should be obvious from status, not implied by the absence of a happy-path message

## Manual-first restore posture

Restore remains secondary to backup confidence in Phase 09.

- restore automation is intentionally not the center of the product
- the operator should inspect the backup pack before trusting it
- the docs should explain how to reason about restore manually before any automated restore path exists
- destructive restore behavior belongs in a later slice only if the roadmap explicitly widens it

## NAS knowledge export job

The NAS export lane extends backup trust without replacing backup packs. It is for operator-inspectable source snapshots that Vel can ingest, explain, and rebuild from directly.

Default operator target, when configured:

- `/mnt/candnas/jove/knowledge/google/`

The target root is a configured path, not a hard-coded write permission. If the path is missing, read-only, or outside an allowed local export root, the job must fail closed with operator-readable verification state.

### Export layout

Each export run writes a durable manifest plus normalized domain snapshots:

```text
<nas_root>/
  manifest.json                    # latest export manifest pointer
  runs/
    <export_id>/
      manifest.json
      domains/
        calendar/events.ndjson
        tasks/tasks.ndjson
        messaging/threads.ndjson
        transcripts/messages.ndjson
        git/events.ndjson
        health/samples.ndjson
        reminders/items.ndjson
        notes/notes.ndjson
        activity/events.ndjson
        mail/messages.ndjson
        drive/files.ndjson
      cold-tier/
        calendar/events.parquet
```

The JSON/NDJSON layer is the Vel-facing source of truth. Parquet is an optional cold-tier derivative for DuckDB/Polars experiments and must never be the only durable representation.

Implemented normalizers currently cover calendar events, Todoist tasks, messaging threads, transcript messages, git events, health samples, reminders, notes, and explicit activity snapshot files:

- `domains/calendar/events.ndjson` uses `backup_export_calendar_events.v1`
- `domains/tasks/tasks.ndjson` uses `backup_export_tasks.v1`
- `domains/messaging/threads.ndjson` uses `backup_export_messaging_threads.v1`
- `domains/transcripts/messages.ndjson` uses `backup_export_transcript_messages.v1`
- `domains/git/events.ndjson` uses `backup_export_git_events.v1`
- `domains/health/samples.ndjson` uses `backup_export_health_samples.v1`
- `domains/reminders/items.ndjson` uses `backup_export_reminder_items.v1`
- `domains/notes/notes.ndjson` uses `backup_export_notes.v1`
- `domains/activity/events.ndjson` uses `backup_export_activity_events.v1` for explicit activity snapshot files

Activity directory sources and generic activity files still use the `local_source_snapshot.v1` fallback at `domains/activity/source.ndjson`. Malformed normalized sources are represented as manifest omissions with reasons so one bad source does not hide the status of the rest of the requested export.

### Export manifest

The export manifest must record:

- `export_id`
- `created_at`
- `target_root`, the configured export base
- `export_root`, the immutable per-run directory under `target_root/runs/<export_id>`
- included domains
- omitted domains with reasons
- schema version for every domain file
- record counts
- checksum algorithm and per-file checksums
- optional derivative files and the source file they were derived from
- verification status and notes

### Job configuration

The runtime slice that implements this job should use explicit config for:

- target root
- included domains
- schedule or manual-only mode
- retention count or age
- whether parquet derivatives are enabled

The current bounded runtime config lives in `vel.toml` under `[backup_export]`. It supports `target_root`, `domains`, `schedule_mode = "manual_only"`, `retention_count`, and `include_parquet_derivatives`. Runtime behavior remains manual-only. The `backup_export` loop kind is registered for visibility and policy consistency, disabled by default, and fail-closed. A storage-only scheduled-job substrate now exists over `v0_backup_jobs`, `v0_backup_job_attempts`, and `v0_backup_job_events`, and failed scheduled terminal jobs can degrade export-specific status, but no worker executes those jobs yet. Retention pruning operates only on immutable sibling directories under `target_root/runs/` and never deletes the export that was just written. Parquet derivatives are implemented as optional cold-tier files generated from normalized JSON/NDJSON exports.

The job belongs at the backup/export boundary. It should not become a general data-lake subsystem, and it should not bypass local-source ingestion contracts.

# Contract Rules

- Keep the backup contract inspectable, typed, and explainable from persisted inputs.
- Do not turn backup/control into a generic configuration editor.
- Do not expose secret-bearing values in the manifest, example, or status surfaces.
- Use status surfaces to summarize trust posture, not to replace the backup manifest itself.
- Keep normalized NAS exports JSON/NDJSON-first; parquet remains optional and derivative.
- Scheduled export failures must be visible in the same trust posture surfaces as backup degradation.

# Published Artifacts

- `config/schemas/backup-manifest.schema.json`
- `config/examples/backup-manifest.example.json`
- `config/schemas/backup-export-manifest.schema.json`
- `config/examples/backup-export-manifest.example.json`
- `config/schemas/backup-export-source-record.schema.json`
- `config/examples/backup-export-source-record.example.json`
- `config/contracts-manifest.json`
- `docs/user/backup-and-restore.md`

# Downstream Usage

- CLI and web surfaces should render backup confidence from typed manifest/status data, not ad hoc JSON blobs.
- Later runtime slices may add snapshot creation and inspection endpoints, but they should keep this contract stable.
- Manual restore guidance should continue to prefer inspect-before-trust semantics even when more automation arrives.
