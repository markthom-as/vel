# `0.5` TaskEvent Contract

**Captured:** 2026-03-22  
**Status:** Locked implementation contract for milestone `0.5`

## Purpose

This document defines the normalized task-side history record used by `0.5`.

It exists so provider history, sync diffs, and local Vel-originated task changes land in one stable event family instead of drifting into adapter-specific folklore.

## Canonical Type

The normalized task-side history record type is `TaskEvent`.

## Core Fields

| Field | Type | Required | Notes |
|---|---|---:|---|
| `id` | `TaskEventId` | yes | Stable event identifier. |
| `task_ref` | `TaskId` | yes | Canonical task being described. |
| `provider_account_ref` | `IntegrationAccountId` | no | Present when event is provider-scoped. |
| `provider_object_ref` | provider/source ref | no | Remote task/object identifier when relevant. |
| `event_type` | `TaskEventType` | yes | Normalized task event kind. |
| `occurred_at` | timestamp | yes | Best-known time the event occurred. |
| `ingested_at` | timestamp | yes | Time Vel ingested or materialized the event. |
| `provenance` | `TaskEventProvenance` | yes | How the event was observed or caused. |
| `field_changes` | `FieldChange[]` | no | Structured changed-field payload where meaningful. |
| `actor_ref` | actor/ref | no | User/provider actor when known. |
| `raw_payload_ref` | opaque payload reference | no | Optional forensic/raw provider payload reference. |
| `notes` | structured metadata | no | Extra bounded metadata without replacing normalized fields. |

## `TaskEventType`

Minimum `0.5` normalized event types:

- `created`
- `completed`
- `reopened`
- `deleted`
- `restored`
- `due_changed`
- `priority_changed`
- `title_changed`
- `description_changed`
- `tags_changed`
- `project_changed`
- `section_changed`
- `parent_changed`

These are the minimum useful shapes for Todoist-backed and local task behavior history in `0.5`.

## `TaskEventProvenance`

Recommended locked provenance enum:

- `provider_event`
- `sync_diff_inferred`
- `local_write_intent`
- `local_write_applied`
- `workflow_action`

## `FieldChange` Shape

| Field | Type | Required | Notes |
|---|---|---:|---|
| `field_name` | `string` | yes | Canonical field or facet path. |
| `old_value` | structured value | no | Prior value when known. |
| `new_value` | structured value | no | New value when known. |

Rules:

- normalized `field_changes` are the primary query surface
- do not rely on raw payload parsing for first-class analytics
- preserve structured old/new values where available

## Raw Payload Policy

Raw payload preservation is optional per event type for storage reasons, but the architecture must support it and prefer it where available.

Rules:

- normalized `TaskEvent` is the primary query and analytics surface
- `raw_payload_ref` is the forensic and provenance substrate
- raw payload does not replace normalized event fields

## Example Events

### Due changed

- `event_type = due_changed`
- `provenance = provider_event | sync_diff_inferred | local_write_applied`
- `field_changes` contains prior and new due values

### Title changed

- `event_type = title_changed`
- `field_changes` contains old/new title

### Completed

- `event_type = completed`
- may include `completed_at` in changed fields or notes

### Reopened

- `event_type = reopened`
- preserves recommitment behavior rather than hiding it

### Local write intent

- `event_type = due_changed` or another matching normalized type
- `provenance = local_write_intent`
- may exist before provider confirmation

## Analytics Notes

`0.5` should support deriving at minimum:

- `reschedule_count`
- `push_later_count`
- `rewrite_count_title`
- `rewrite_count_description`
- `rewrite_count_combined`

History is mandatory. Analytics are derived from it and must not replace it.

## Canonical References

- [0.5-CLARIFICATION-ADDENDUM.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md)
- [TASK-SCHEMA.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/TASK-SCHEMA.md)
- [63-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/63-CONTEXT.md)

---

*Locked as normalized task history contract for milestone `0.5`*
