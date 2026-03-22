# `0.5` Canonical Task Schema

**Captured:** 2026-03-22  
**Status:** Locked implementation contract for milestone `0.5`

## Purpose

This document defines the exact canonical `Task` shape that `0.5` implementers should reach for instead of reconstructing it from broader milestone prose.

It is a field contract, not a philosophy essay.

## Canonical Task Fields

| Field | Type | Required | Notes |
|---|---|---:|---|
| `id` | `TaskId` | yes | Stable Vel prefixed ID. |
| `title` | `string` | yes | Canonical task title/content. |
| `description` | `string` | no | Canonical long-form task description. |
| `status` | `TaskStatus` | yes | Vel-owned task lifecycle state. |
| `priority` | `TaskPriority` | yes | Semantic enum, not provider numeric truth. |
| `due` | `Due` | no | Canonical due shape preserving date vs floating vs zoned semantics. |
| `task_type` | `TaskType` | yes | First-class task species for `0.5`. |
| `project_ref` | `ProjectId` | no | One primary canonical project/container relation only in `0.5`. |
| `parent_task_ref` | `TaskId` | no | Single-parent subtask hierarchy only. |
| `tags` | `TagRef[]` or canonical tag refs | yes | Raw tags remain canonical and preserved. |
| `task_semantics` | `TaskSemantics` | no | Typed interpreted planning meaning. |
| `created_at` | timestamp | yes | Canonical creation timestamp. |
| `updated_at` | timestamp | yes | Canonical last-update timestamp. |
| `completed_at` | timestamp | no | Canonical completion timestamp when applicable. |
| `source_refs` | source references | yes | Canonical external-linkage references. |
| `provider_facets` | namespaced provider facets | yes | Provider-specific metadata without owning ontology. |

## Enums

### `TaskStatus`

- `inbox`
- `ready`
- `in_progress`
- `blocked`
- `done`
- `canceled`

### `TaskPriority`

- `critical`
- `high`
- `medium`
- `low`
- `lowest`

Operator/provider shorthand maps as:

- `p0 -> critical`
- `p1 -> high`
- `p2 -> medium`
- `p3 -> low`
- `p4 -> lowest`

### `TaskType`

- `generic`
- `maintain`
- `practice`
- `ritual`
- `chore`

## `Due` Shape

| Field | Type | Required | Notes |
|---|---|---:|---|
| `kind` | `date | floating_datetime | zoned_datetime` | yes | Preserve actual due semantics. |
| `value` | date/datetime value | yes | Canonical due value. |
| `timezone` | `string` | no | Used only for `zoned_datetime`. |
| `is_recurring_snapshot` | `bool` | no | When imported provider state requires it. |

## `TaskSemantics` Shape

`task_semantics` is optional, typed, and canonical.

Initial `0.5` fields:

- `estimated_duration_minutes`
- `time_of_day_hint`
- `routine_block_ref`
- `energy_level_hint`
- `context_tags`
- `schedule_flexibility`

Rules:

- raw tags remain preserved even when interpreted
- no separate stored `is_routine` boolean in `0.5`
- use `task_type` and `routine_block_ref` instead
- nothing syncs outward unless an explicit mapping rule exists

## Tag Interpretation Notes

`0.5` supports:

- freeform raw tags
- configurable interpretation rules
- recommended reserved tag namespaces such as:
  - `time:morning`
  - `duration:15m`
  - `energy:low`
  - `routine:admin`
  - `focus:deep`
  - `context:errands`

## Provider / Ownership Notes

- Todoist is a shaping adapter, not the canonical ontology.
- Local-only Vel tasks are fully editable without provider-mapping constraints.
- Provider constraints apply only when reconciling with or projecting to an external provider.
- Sections/lists remain provider-facet metadata in `0.5`, not canonical container types.

## Canonical References

- [0.5-CLARIFICATION-ADDENDUM.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md)
- [0.5-FIELD-OWNERSHIP-MATRIX.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md)
- [63-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/63-CONTEXT.md)

---

*Locked as canonical task field contract for milestone `0.5`*
