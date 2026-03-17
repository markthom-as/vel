# Local Sources

Vel currently ingests many sources from local files or snapshot exports.

This is the most important integration model to understand today because it is stable, inspectable, and local-first.

## Current shipped local source types

- calendar from `.ics`
- Todoist from snapshot JSON
- activity from snapshot JSON
- health from snapshot JSON
- git from snapshot JSON
- messaging from snapshot JSON
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
- `notes_path`
- `transcript_snapshot_path`

## How to sync a local source

Examples:

```bash
cargo run -p vel-cli -- sync calendar
cargo run -p vel-cli -- sync todoist
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync health
cargo run -p vel-cli -- sync git
cargo run -p vel-cli -- sync messaging
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

This makes it useful for local markdown or plaintext note stores.

## Messaging and health

`messaging` and `health` are currently snapshot-backed. If the export file is not updated, Vel has nothing new to ingest.

That is especially relevant on macOS where the exporter or host permissions determine whether those snapshots exist.
