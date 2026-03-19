---
title: Writeback And Conflict Contracts
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - writeback
  - conflicts
  - people
  - phase-6
summary: Canonical Phase 06 contract vocabulary for bounded write-back operations, typed conflict cases, and the practical people registry.
---

# Purpose

Publish the stable Phase 06 vocabulary before reconciliation, provider write slices, and operator surfaces widen around ad hoc provider payloads.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Write-back operations | `vel-core` | `crates/vel-core/src/writeback.rs` |
| Conflict cases | `vel-core` | `crates/vel-core/src/conflicts.rs` |
| People registry | `vel-core` | `crates/vel-core/src/people.rs` |
| Transport DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |

# Stable Vocabulary

## Write-back risks

Write-capable integrations must classify operations as exactly:

- `safe`
- `confirm_required`
- `blocked`

## Write-back statuses

Durable write-back records use exactly:

- `queued`
- `in_progress`
- `applied`
- `conflicted`
- `denied`
- `failed`
- `cancelled`

## Allowed write-back operation kinds

Provider slices may only expose these `WritebackOperationKind` values:

- `todoist_create_task`
- `todoist_update_task`
- `todoist_complete_task`
- `todoist_reopen_task`
- `notes_create_note`
- `notes_append_note`
- `reminders_create`
- `reminders_update`
- `reminders_complete`
- `github_create_issue`
- `github_add_comment`
- `github_close_issue`
- `github_reopen_issue`
- `email_create_draft_reply`
- `email_send_draft`

No provider slice may widen beyond this list without first extending `vel-core`, `vel-api-types`, the schema/example assets, and this owner doc in the same slice.

## Conflict kinds and statuses

Conflict cases use exactly these kinds:

- `upstream_vs_local`
- `cross_client`
- `stale_write`
- `executor_unavailable`

Conflict case lifecycle uses exactly these statuses:

- `open`
- `acknowledged`
- `resolved`
- `dismissed`
- `expired`

## People registry

The Phase 06 people registry is intentionally practical:

- `PersonRecord` holds display name, structured name parts, relationship context, birthday, last-contacted timestamp, aliases, and durable links.
- `PersonAlias` keeps `platform`, `handle`, `display`, and optional `source_ref`.
- `PersonLinkRef` keeps `kind`, `id`, and `label`.

# Contract Rules

- Upstream systems remain authoritative. A write-back record describes a bounded requested operation, not ambient provider power.
- Conflict cases must stay durable and inspectable; do not reduce them to log lines or client-only banners.
- Operator-facing payloads should expose `pending_writebacks`, `conflicts`, and `people` as typed arrays, not opaque JSON placeholders.
- Provider slices may map to the allowed operation kinds only. “Any provider action” is not an acceptable contract.
- People linkage must stay alias-driven and explainable from durable refs or source records.

# Published Artifacts

- `config/schemas/writeback-operation.schema.json`
- `config/examples/writeback-operation.example.json`
- `config/schemas/person-record.schema.json`
- `config/examples/person-record.example.json`

# Downstream Usage

- Storage and service slices should persist these records directly and reuse them across Todoist, notes, reminders, GitHub, email, sync, and operator-queue work.
- Clients may render status from these records, but risk policy, conflict policy, and durable write state remain backend-owned.
