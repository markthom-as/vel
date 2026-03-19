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

For Obsidian-backed notes, Vel also suggests locally configured vault roots when it can read the host Obsidian config.

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
