# Local Sources

Vel currently ingests many sources from local files or snapshot exports.

This is the most important integration model to understand today because it is stable, inspectable, and local-first.
It is a subset of the full connector model described in `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md`.

## Current shipped local source types

- calendar from `.ics`
- Todoist from snapshot JSON
- activity from snapshot JSON
- health from snapshot JSON
- git from snapshot JSON
- messaging from snapshot JSON
- reminders from snapshot JSON
- notes from a file or directory
- transcripts from snapshot JSON

## Why this model exists

Vel is intentionally local-first.

Using local files and snapshots means:

- you can inspect the raw input,
- you control the source of truth,
- ingestion is reproducible,
- the runtime does not need to become a giant vendor-specific connector layer too early.

## NAS knowledge export lane

Vel's backup contract now reserves a normalized NAS export lane for inspectable knowledge snapshots. This is separate from the normal backup pack: backup packs protect Vel's own database, artifacts, and non-secret config, while the NAS export lane preserves source-shaped data that Vel can ingest and explain from.

The manual runtime slice writes to an explicitly provided target root, commonly:

```text
/mnt/candnas/jove/knowledge/google/
```

The durable Vel-facing format is JSON or NDJSON per domain. Optional parquet files may be produced for analysis tools, but they are derivative cold-tier files and are not the source of truth.

Current layout:

```text
<nas_root>/
  manifest.json
  runs/<export_id>/manifest.json
  runs/<export_id>/domains/calendar/events.ndjson
  runs/<export_id>/domains/tasks/tasks.ndjson
  runs/<export_id>/domains/messaging/threads.ndjson
  runs/<export_id>/domains/transcripts/messages.ndjson
  runs/<export_id>/domains/git/events.ndjson
  runs/<export_id>/domains/health/samples.ndjson
  runs/<export_id>/domains/reminders/items.ndjson
  runs/<export_id>/domains/notes/notes.ndjson
  runs/<export_id>/domains/activity/events.ndjson
  runs/<export_id>/domains/activity/source.ndjson
```

The current normalizer slice writes typed NDJSON records for calendar events, Todoist tasks, messaging threads, transcript messages, git events, health samples, reminders, notes, and explicit activity snapshot files. Activity directory sources and generic/non-snapshot activity files still use the raw local source snapshot fallback so local-source discovery behavior stays unchanged.

Configure it in `vel.toml`:

```toml
[backup_export]
target_root = "/mnt/candnas/jove/knowledge/google"
domains = ["calendar", "tasks"]
schedule_mode = "manual_only"
retention_count = 7
include_parquet_derivatives = false
```

Run it with:

```bash
vel backup --export --target-root /mnt/candnas/jove/knowledge/google --domain calendar --domain tasks
```

With `backup_export.target_root` and `backup_export.domains` configured, `vel backup --export` uses those configured values.

Check the latest successful manual export with:

```bash
vel backup --export-status
```

The `backup_export` runtime loop is visible in loop policy/status surfaces but remains disabled and non-executing. Scheduled export job storage exists for future execution, and failed scheduled terminal jobs can degrade export-specific status without changing backup-pack trust.

If the NAS root is unavailable, stale, or not writable, Vel fails closed instead of silently falling back to an untracked location. Scheduled execution remains future work. Optional parquet derivatives can be enabled with `include_parquet` on the export API or `include_parquet_derivatives = true` in config. If `retention_count` is configured, pruning is limited to older immutable export directories under `runs/`.

If a normalized domain source is malformed, Vel omits that domain from the manifest with a reason and continues exporting other requested domains. This is currently active for calendar, tasks, messaging, transcripts, git, health, reminders, notes, and explicit activity snapshot files.

## How to verify current paths

Use:

```bash
cargo run -p vel-cli -- config show
```

Look for:

- `calendar_ics_path`
- `todoist_snapshot_path`
- `activity_snapshot_path`
- `health_snapshot_path`
- `git_snapshot_path`
- `messaging_snapshot_path`
- `reminders_snapshot_path`
- `notes_path`
- `transcript_snapshot_path`

The Settings page now surfaces two faster setup aids for local-source integrations:

- suggested paths from the host platform when Vel can infer likely locations
- a `Choose path` or `Choose vault` dialog from the integration card
- an operator-first path-selection card that points back to setup and troubleshooting docs when discovery is not enough

For Obsidian-backed notes, Vel also suggests locally configured vault roots when it can read the host Obsidian config.

When the web shell shows both operator paths and internal/default paths, treat them differently:

- operator paths are the ones you should select, save, and sync from
- internal/default paths are read-only diagnostics that explain how Vel is bootstrapping or where it would look automatically

## How to sync a local source

Examples:

```bash
cargo run -p vel-cli -- sync calendar
cargo run -p vel-cli -- sync todoist
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync health
cargo run -p vel-cli -- sync git
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- sync reminders
cargo run -p vel-cli -- sync notes
cargo run -p vel-cli -- sync transcripts
```

Then refresh derived state:

```bash
cargo run -p vel-cli -- evaluate
```

## What “working” looks like

A source is usually working when:

- its sync completes without error,
- recent state appears in current context or `Now`,
- the Settings page shows recent successful sync metadata,
- explain or source summary surfaces reflect the new data.

## Common failure modes

- wrong file path
- old snapshot
- empty export
- source updated but evaluate not rerun
- host permissions prevent the exporter from producing the file

## Notes

`notes` is path-backed rather than single-snapshot only. It can ingest a file or a directory.

This makes it useful for local markdown or plaintext note stores, especially an Obsidian vault that is already being replicated by Obsidian Sync.

Practical setup:

- keep Obsidian Sync responsible for multi-device file replication
- point Vel's `notes` / `Obsidian Vault` setting at the local vault root on the daemon host
- run `sync notes`, then `evaluate`

Scoped note writes now use the bounded operator routes under `/api/integrations/notes/*`.
Vel will only write inside the configured `notes_path` or a typed project's project notes roots.
That includes `project.primary_notes_root.path` and any `project.secondary_notes_roots[].path`.
If a requested note path escapes those roots, the runtime records a blocked write-back instead of applying it.

## Messaging and health

`messaging`, `reminders`, and `health` are currently snapshot-backed. If the export file is not updated, Vel has nothing new to ingest.

That is especially relevant on macOS where the exporter or host permissions determine whether those snapshots exist.

Reminder write-back is now intent-based rather than ambient file mutation.
The bounded routes under `/api/integrations/reminders/*` persist a reminder intent first, then either apply it through the configured local snapshot executor or leave an inspectable `executor_unavailable` conflict for a local executor or linked client to resolve later.

Transcript snapshots remain read-only, but transcript ingestion now folds under the same notes lane as a notes source subtype.
Use `sync transcripts` for the read path; write surfaces stay on notes and reminders only.

## Scope Clarification

This document covers local source modes (`local_file`, `local_directory`, `local_snapshot`).
Other integrations may use credential-backed API paths (for example `google_calendar` and `todoist`) and should be interpreted through the canonical connector contract and data source catalog.
