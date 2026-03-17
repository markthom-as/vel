---
title: Ticket 401 - Add external item, project, and sync watermark tables
status: proposed
owner: codex
priority: critical
---

# Goal

Add the minimum relational structure needed so external awareness is durable, linkable, and incrementally ingestible.

# Files

## New migrations
- `migrations/0025_external_items.sql`
- `migrations/0026_projects.sql`
- `migrations/0027_sync_watermarks.sql`
- `migrations/0028_sync_operations.sql`

## Changed
- `crates/vel-storage/src/db.rs`
- `crates/vel-storage/src/lib.rs`

# Migration details

## `0025_external_items.sql`
Create:
- `external_items`
- `external_item_links`

Indexes:
- unique `(source_kind, external_id)`
- `idx_external_items_kind_state`
- `idx_external_items_project_slug`
- `idx_external_items_last_changed_at`
- `idx_external_item_links_commitment`
- `idx_external_item_links_signal`

## `0026_projects.sql`
Create:
- `projects`

Indexes:
- unique `slug`
- `idx_projects_status`
- `idx_projects_owner`

## `0027_sync_watermarks.sql`
Create:
- `sync_watermarks`

## `0028_sync_operations.sql`
Create:
- `sync_operations`

Suggested columns:
- `sync_operation_id TEXT PRIMARY KEY`
- `source_kind TEXT NOT NULL`
- `operation_kind TEXT NOT NULL`
- `target_external_id TEXT`
- `proposal_json TEXT NOT NULL`
- `state TEXT NOT NULL`
- `created_at INTEGER NOT NULL`
- `approved_at INTEGER`
- `applied_at INTEGER`
- `failed_at INTEGER`
- `error_text TEXT`

# Storage API additions

Add methods:
- `upsert_external_item`
- `list_external_items`
- `get_external_item_by_source`
- `link_external_item`
- `upsert_project`
- `list_projects`
- `upsert_sync_watermark`
- `get_sync_watermark`
- `insert_sync_operation`
- `update_sync_operation_state`

# Concrete SQL behavior

## Upsert rule
When `(source_kind, external_id)` already exists:
- update title/state/time/project/url/payload/fingerprint
- preserve `first_seen_at`
- update `last_seen_at`
- update `last_changed_at` only if fingerprint changed

## Link rule
External item links should be insert-only and idempotent per `(external_item_id, link_kind, commitment_id, signal_id, artifact_id)`.

# Acceptance criteria

- Vel can store normalized tasks, calendar events, and projects durably
- incremental sync state has a dedicated home
- writeback proposals are representable without mutating source-of-truth systems
