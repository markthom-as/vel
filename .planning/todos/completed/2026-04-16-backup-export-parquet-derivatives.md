---
created: 2026-04-16T00:00:00.000Z
title: Add optional backup export parquet derivatives
area: runtime
completed: 2026-04-16T00:00:00.000Z
files:
  - crates/veld/src/services/backup.rs
  - crates/vel-api-types/src/lib.rs
  - config/schemas/backup-export-manifest.schema.json
  - docs/cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md
---

## Problem

`include_parquet_derivatives` is currently rejected even though the export contract reserves parquet as an optional cold-tier derivative. The durable source of truth should remain JSON/NDJSON, with parquet generated only as a derivative.

## Scope

- Generate optional parquet derivatives from normalized JSON/NDJSON export files.
- Record derivative paths and checksums in the export manifest.
- Keep JSON/NDJSON as the authoritative Vel-facing export lane.
- Add verification that checked-in examples and schema stay in sync.

## Notes

Do not require parquet tooling for the core manual export path unless the derivative flag is enabled.

## Progress

- 2026-04-16: Added real Apache Parquet derivatives for normalized export files when `include_parquet` is requested or `[backup_export].include_parquet_derivatives = true`.
  - Derivatives are written under `cold-tier/<domain>/<source-stem>.parquet`.
  - Raw `local_source_snapshot.v1` fallback files are skipped; the durable source of truth remains JSON/NDJSON.
  - Derivative manifest entries now include `format`, `record_count`, `source_path`, `checksum_algorithm`, and `checksum`.
  - The checked manifest paths include derivative files so export verification covers generated cold-tier artifacts.
  - The published manifest schema/example and backup/export docs now describe parquet derivatives as implemented optional artifacts.

Remaining scope: none for the current derivative slice. Retention pruning remains pending until the export layout becomes run-addressable.
