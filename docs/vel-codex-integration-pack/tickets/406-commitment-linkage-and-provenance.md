---
title: Ticket 406 - Link commitments and artifacts to canonical external items
status: proposed
owner: codex
priority: high
---

# Goal

Preserve provenance so Vel can explain why a commitment exists and what source object it came from.

# Current issue

`commitments.source_type` and `source_id` are useful but too coarse. They do not support:
- many-to-one linkage
- signal linkage
- artifact linkage
- future writeback proposals

# Files

## Changed
- `crates/veld/src/adapters/todoist.rs`
- `crates/veld/src/adapters/calendar.rs`
- `crates/veld/src/services/inference.rs`
- `crates/vel-storage/src/db.rs`

# Implementation

## Link table usage
When a commitment is created/updated from an external item:
- ensure `external_items` row exists
- ensure `external_item_links` row exists with `link_kind = "commitment_source"`

When a signal is emitted because of that item:
- add `link_kind = "signal_evidence"`

When a summary artifact includes that item:
- add `link_kind = "artifact_reference"`

## Provenance in API
Extend API response shapes so commitment detail can expose:
- canonical source kind
- external id
- source URL/path when available
- last-seen time

# Acceptance criteria

- operator can inspect a commitment and trace it back to Todoist / calendar / project registry
- provenance survives repeated syncs
