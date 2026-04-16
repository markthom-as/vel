---
created: 2026-04-16T00:00:00.000Z
title: Add richer backup export domain normalizers
area: runtime
completed: 2026-04-16T00:00:00.000Z
files:
  - crates/veld/src/services/backup.rs
  - crates/veld/src/services/integrations.rs
  - docs/user/integrations/local-sources.md
---

## Problem

The current manual export lane writes JSON/NDJSON snapshots from available local source files. Several domains still need richer normalization before the export lane is useful as a durable cognition substrate.

## Scope

- Normalize each supported domain into stable JSON/NDJSON records with source provenance.
- Preserve omitted-domain explanations when source data is unavailable or malformed.
- Keep the export manifest explainable from persisted inputs.
- Add focused tests for representative calendar, tasks, messaging, notes, and transcript exports.

## Notes

Prefer typed domain shapes and boundary serialization over deepening untyped JSON blobs in services.

## Progress

- 2026-04-16: Added normalized export slices for calendar events, Todoist tasks, messaging threads, transcript messages, git events, health samples, reminders, notes, and explicit activity snapshot files.
  - Calendar now writes `domains/calendar/events.ndjson` with `backup_export_calendar_events.v1`.
  - Tasks now write `domains/tasks/tasks.ndjson` with `backup_export_tasks.v1`.
  - Messaging now writes `domains/messaging/threads.ndjson` with `backup_export_messaging_threads.v1`.
  - Transcripts now write `domains/transcripts/messages.ndjson` with `backup_export_transcript_messages.v1`.
  - Git now writes `domains/git/events.ndjson` with `backup_export_git_events.v1`.
  - Health now writes `domains/health/samples.ndjson` with `backup_export_health_samples.v1`.
  - Reminders now write `domains/reminders/items.ndjson` with `backup_export_reminder_items.v1`.
  - Notes now write `domains/notes/notes.ndjson` with `backup_export_notes.v1`.
  - Explicit activity snapshot files now write `domains/activity/events.ndjson` with `backup_export_activity_events.v1`.
  - Activity directory sources and generic activity files continue to use the raw `local_source_snapshot.v1` fallback.
  - Malformed normalized sources become `omitted_domains` entries instead of failing the whole export when other domains can still be written.
  - Added the published source-record schema/example under `config/schemas/backup-export-source-record.schema.json` and `config/examples/backup-export-source-record.example.json`.

Remaining scope: none for the current manual export normalizer slice. Activity live/local-source fallback exports remain raw by design until a later spec defines a stable export contract for those sources.
