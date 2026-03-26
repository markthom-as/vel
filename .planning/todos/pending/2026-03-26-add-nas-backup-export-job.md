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

## Solution

Define a backup/export job at the existing backup boundary that can target a configured NAS root, emit manifests and verification metadata, and write data in two layers:

- canonical normalized JSON/NDJSON snapshots per domain as the durable Vel-facing format
- optional parquet cold-tier derivatives for DuckDB/Polars experiments

The job should stay local-first, traceable, and restore/inspect friendly. It likely belongs with the Phase 09 backup/export surfaces, with config for target roots, schedule, included domains, retention, and whether parquet derivation is enabled.
