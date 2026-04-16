---
created: 2026-03-26T03:24:07.120Z
title: Add NAS backup export job
area: docs
files:
  - docs/cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md
  - crates/veld/src/services/backup.rs
  - docs/user/integrations/local-sources.md
---

## Problem

Vel currently has a local backup pack flow, but it does not yet own a scheduled export job that writes operator-inspectable knowledge backups to the NAS layout under `/mnt/candnas/jove/knowledge/google/`. For local-first source data and future Google Takeout-style exports, the runtime should produce a durable normalized file lane that Vel can ingest and explain from directly, instead of treating parquet as the only persisted substrate. The current backup and local-source contracts already prefer inspectable snapshots, manifests, and explicit verification, so a NAS export job should extend that seam rather than introducing a parallel data lake path.

## Progress

2026-04-16: completed the docs-only contract slice.

- `docs/cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md` now reserves the NAS export lane as a separate JSON/NDJSON-first backup/export contract with manifest, verification, config, retention, and parquet-derivative rules.
- `docs/user/integrations/local-sources.md` now explains the user-facing NAS knowledge export lane and its distinction from backup packs.
- Runtime code was intentionally not widened in the same slice. The existing `crates/veld/src/services/backup.rs` path remains a backup-pack pipeline, not a scheduled normalized export job.

2026-04-16: completed a bounded manual runtime slice.

- `POST /v1/backup/export` and `vel backup --export --target-root <dir> [--domain <name>]` now validate an existing writable target root, write an export `manifest.json`, and emit JSON/NDJSON source snapshot files under `domains/<domain>/`.
- `config/schemas/backup-export-manifest.schema.json` and `config/examples/backup-export-manifest.example.json` now publish the export manifest contract through `config/contracts-manifest.json`.
- Focused route, CLI parser, CLI client, and output-summary tests now cover the manual export path.

2026-04-16: completed the manual export config fallback slice.

- `[backup_export]` in `vel.toml` now carries the explicit target root, domains, manual-only schedule mode, retention intent, and parquet-derivative flag.
- `POST /v1/backup/export` can now omit `target_root` and/or `domains` when `AppConfig.backup_export` supplies them.
- `vel backup --export` can now omit `--target-root` and repeated `--domain` flags when the config supplies those values.
- Retention and parquet fields are contract-visible intent only; pruning and derivative generation remain disabled.

2026-04-16: completed export run persistence.

- Successful manual export manifests now persist through the storage backup-manifest substrate under a distinct knowledge-export storage target.
- Backup pack status remains isolated from export runs so the existing backup trust card is not polluted by NAS export state.

2026-04-16: completed manual export status.

- `GET /v1/backup/export/status` and `vel backup --export-status` now expose the latest successful manual export run.
- Export status remains read-only and separate from backup-pack trust status.

2026-04-16: completed the runtime-loop contract repair.

- `backup_export` is now a known runtime loop kind and policy surface, disabled by default.
- If enabled before the real scheduled-job substrate exists, the worker fails closed with an explicit "scheduled backup export is not implemented" error instead of silently running the manual export path on an interval.
- The remaining broad scheduler work has been split into focused follow-up todos.

Outcome: the bounded manual export, config fallback, status, persistence, and disabled loop visibility slices are complete. Scheduled execution, scheduled failure trust degradation, retention pruning, richer per-domain normalizers, and optional parquet derivatives are intentionally deferred to separate todos.

## Solution

Define a backup/export job at the existing backup boundary that can target a configured NAS root, emit manifests and verification metadata, and write data in two layers:

- canonical normalized JSON/NDJSON snapshots per domain as the durable Vel-facing format
- optional parquet cold-tier derivatives for DuckDB/Polars experiments

The job should stay local-first, traceable, and restore/inspect friendly. It likely belongs with the Phase 09 backup/export surfaces, with config for target roots, schedule, included domains, retention, and whether parquet derivation is enabled.
