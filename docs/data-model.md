# Vel — Data Model

## Purpose

This document defines the **initial data model** for Vel.

It is intentionally designed to be:

- **modular**
- **evolvable**
- **local-first**
- **friendly to iteration**
- **usable before full productization**

Vel is expected to change substantially as it is used. The data model should therefore prioritize:

1. **clear primary objects**
2. **stable identities**
3. **append-friendly event history**
4. **soft schemas where useful**
5. **easy migration later**

This is **not** a final enterprise schema.  
It is a practical, flexible model for building Vel without painting it into a corner.

---

## Canonical entities (current)

| Entity      | Purpose                          | Durable | Event-backed      | Ref-backed   |
|------------|-----------------------------------|:-------:|-------------------|--------------|
| run        | Execution record                  | yes     | run_events        | optional     |
| run_event  | Per-run timeline                  | yes     | —                 | —            |
| event      | Global system event / audit      | yes     | —                 | —            |
| ref        | Durable relationship (provenance) | yes     | —                 | —            |
| artifact   | Durable output or reference      | yes     | optional          | optional     |
| capture    | User/system capture              | yes     | CAPTURE_CREATED   | optional     |
| commitment | Actionable / reviewable item     | yes     | —                 | —            |

- **run_events** describe what happened during one run; **events** describe system-wide occurrences; **refs** describe what is related to what.

### Lineage (provenance)

```text
Capture
   ↓
Run
   ↓
Artifact
   ↓
Refs (run → artifact; artifact → capture)
```

### Commitments (implemented)

A **commitment** is an actionable, reviewable, time-relevant object — distinct from a raw capture. Captures are input; commitments are what matters, what is unresolved, and what is next.

- **Schema** — `commitments` table: id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json. Indexes on status, due_at, project, source, created_at.
- **Statuses** — open, done, cancelled (v1).
- **Creation** — (1) `vel commitment add ...`, (2) capture promotion when `capture_type == "todo"`, (3) external task import (Phase B).
- **API** — POST/GET /v1/commitments, GET/PATCH /v1/commitments/:id.
- **CLI** — `vel commitments`, `vel commitment add/done/cancel/inspect`.

See [specs/commitments.md](specs/commitments.md).

### Extended schema (migrations 0012–0022)

Additional tables support persistent context, dependencies, risk, nudge history, suggestions, threads, self-model, and transcripts:

- **current_context** — Singleton (id=1) holding latest computed context; written by inference engine; `vel context`.
- **context_timeline** — Append-only context transition snapshots (debugging, temporal synthesis).
- **commitment_dependencies** — Parent/child dependency graph (e.g. meeting → prep, commute).
- **commitment_risk** — Risk score snapshots per commitment (consequence, proximity, factors).
- **nudge_events** — Append-only log (nudge_created, nudge_sent, nudge_snoozed, nudge_resolved).
- **suggestions** — Steerable proposals (accept/reject/modify); suggestion_type, state, payload.
- **threads** — First-class threads (project, person, conversation, theme); status, metadata.
- **thread_links** — Relate threads to commitments, captures, signals, artifacts.
- **vel_self_metrics** — Self-model metrics (nudge effectiveness, feedback_score, etc.).
- **assistant_transcripts** — Ingested chat/assistant transcripts for capture extraction, project tagging.

Signals table extended with **source_ref** for deduplication. Canonical spec: [specs/vel-migrations-and-schema-spec.md](specs/vel-migrations-and-schema-spec.md).

---

## Design Principles

### 1. Stable IDs Everywhere
All first-class entities should have stable IDs.

This includes:

- containers
- projects
- goals
- milestones
- tasks
- commitments
- people
- artifacts
- captures
- conversations
- suggestions
- reflections
- timeline events
- behavior configs

Stable IDs make it easier to:

- build graph relationships
- sync across devices
- migrate storage backends
- support ML / analytics later
- attach provenance and revision history

---

### 2. Hybrid Structured + Flexible Schema
Vel should use a **hybrid model**:

- structured fields for core identity and relationships
- flexible metadata blobs for evolving fields

Example pattern:

- required columns in SQLite
- optional `metadata_json` for experimental or context-specific fields

This allows fast iteration without schema thrash.

---

### 3. Event-Sourced Enough, Not Religiously Event-Sourced
Vel should preserve **important events over time**, but should not force everything into pure event sourcing.

Recommended approach:

- current-state tables for primary objects
- timeline/event tables for history
- revision/change tracking for important entities

This gives:

- usability
- auditability
- reversibility
- easier analytics

without becoming a cathedral of append-only suffering.

---

### 4. Time Is First-Class
Time is not just a timestamp field.

Vel should explicitly model:

- creation time
- update time
- occurrence time
- due time
- dormant duration
- planning horizon
- cadence / recurrence
- temporal relationships between objects

Vel is partly a time-management and alignment system, so temporal reasoning must be central.

---

### 5. Privacy Is First-Class
Privacy is not an afterthought.

Every recordable object should be able to carry a privacy class and storage/sync policy.

Minimum privacy classes:

- `private`
- `work`
- `sensitive`
- `do_not_record`

Future classes may be added later.

---

### 6. Artifact/Data Separation
Large artifacts should not live in the main relational DB.

Use:

- relational DB for metadata and relationships
- filesystem / object store for blobs

Examples of blobs:

- audio
- transcript files
- large summaries
- PDFs
- images
- exported reports

---

### 7. Offline-First Core
The data model should support partial operation offline.

That means some objects should be cached and marked by sync tier.

Example tiers:

- `hot` — must be available on phone/device
- `warm` — available on local network / NAS
- `cold` — archive / object storage only

---

## Storage Model

## Structured Metadata Store
Initial recommendation:

- **SQLite** as primary metadata store
- migration path open to:
  - Postgres
  - distributed SQLite
  - read replicas / sync systems later

SQLite is the right starting point because:

- simple
- local-first
- fast enough
- easy to backup
- easy to inspect
- easy to ship

---

## Artifact Store
Artifacts should live in filesystem/object storage.

Suggested layout:

```text
vel-data/
  db/
    vel.sqlite
  artifacts/
    audio/
    transcripts/
    notes/
    summaries/
    exports/
  cache/
  indexes/
  logs/
```

Suggested storage tiers:

- local machine
- NAS
- optional S3-compatible object store
- selective mobile cache

Artifacts should be referenced by:

- stable artifact ID
- content hash
- storage URI(s)
- mime type
- sync class

---

## Core Object Model

The following are Vel's primary objects.

---

## 1. Container

### Purpose
A top-level life/work domain that groups projects and commitments.

Examples:

- Mimesis Institute
- Opertus Systems
- Personal Art Practice
- Writing Projects

### Required fields

- `container_id`
- `name`
- `kind`
- `status`
- `created_at`
- `updated_at`

### Optional fields

- `description`
- `metadata_json`
- `privacy_class`
- `priority`
- `tags`

### Relationships

- contains many projects
- contains many goals
- contains many tasks / commitments
- linked to people and artifacts

---

## 2. Project

### Purpose
A bounded body of work within a container.

Examples:

- Data Chorus
- memoir draft
- grant application
- Vel core runtime

### Required fields

- `project_id`
- `container_id`
- `name`
- `status`
- `created_at`
- `updated_at`

### Optional fields

- `description`
- `start_date`
- `target_date`
- `priority`
- `metadata_json`
- `privacy_class`
- `energy_profile`
- `tags`

### Relationships

- belongs to one container
- linked to goals, milestones, tasks, artifacts, conversations, people
- linked to timeline events
- may be related to other projects

---

## 3. Goal

### Purpose
A desired long-term outcome.

Examples:

- write a memoir
- launch Mimesis Institute
- build Vel
- improve health stability

### Required fields

- `goal_id`
- `name`
- `status`
- `created_at`
- `updated_at`

### Optional fields

- `description`
- `target_date`
- `priority`
- `goal_horizon` (`daily`, `weekly`, `monthly`, `quarterly`, `yearly`, `life`)
- `container_id`
- `project_id`
- `success_definition`
- `metadata_json`

### Relationships

- may belong to a container
- may be linked to one or more projects
- has milestones
- informs suggestion/alignment engine

---

## 4. Milestone

### Purpose
A significant checkpoint toward a goal or project.

### Required fields

- `milestone_id`
- `name`
- `status`
- `created_at`
- `updated_at`

### Optional fields

- `goal_id`
- `project_id`
- `target_date`
- `description`
- `priority`
- `metadata_json`

### Relationships

- belongs to a goal and/or project
- has tasks
- linked to timeline events

---

## 5. Task

### Purpose
An actionable work item.

### Required fields

- `task_id`
- `name`
- `status`
- `created_at`
- `updated_at`

### Optional fields

- `project_id`
- `goal_id`
- `milestone_id`
- `due_at`
- `scheduled_at`
- `estimated_effort`
- `actual_effort`
- `energy_requirement`
- `priority`
- `source_system`
- `external_ref`
- `metadata_json`

### Relationships

- linked to projects/goals/milestones
- may originate from Todoist or other systems
- may generate commitments or timeline events

---

## 6. Commitment

### Purpose
A promise, obligation, or expectation tied to a person, meeting, project, or goal.

Examples:

- send proposal draft
- follow up with collaborator
- deliver materials by a date

### Required fields

- `commitment_id`
- `name`
- `status`
- `created_at`
- `updated_at`

### Optional fields

- `due_at`
- `person_id`
- `project_id`
- `conversation_id`
- `source_artifact_id`
- `importance`
- `nag_level`
- `metadata_json`

### Relationships

- linked to people, projects, conversations, artifacts
- drives accountability and reminders

---

## 7. Person

### Purpose
A collaborator, contact, friend, or relevant human entity.

### Required fields

- `person_id`
- `display_name`
- `created_at`
- `updated_at`

### Optional fields

- `emails`
- `phones`
- `roles`
- `notes`
- `privacy_class`
- `metadata_json`

### Relationships

- linked to conversations, projects, commitments, artifacts
- may belong to multiple containers/projects

---

## 8. Conversation

### Purpose
A meeting, call, discussion, critique, or recorded interaction.

### Required fields

- `conversation_id`
- `title`
- `occurred_at`
- `created_at`
- `updated_at`

### Optional fields

- `conversation_type`
- `participants`
- `location`
- `project_id`
- `container_id`
- `summary_artifact_id`
- `transcript_artifact_id`
- `audio_artifact_id`
- `privacy_class`
- `metadata_json`

### Relationships

- linked to people
- linked to artifacts
- can generate commitments, notes, tasks, suggestions, timeline events

---

## 9. Capture

### Purpose
A raw intake event created by the user or a device.

Examples:

- voice memo
- quick note
- imported transcript
- watch capture
- manual task dump

### Required fields

- `capture_id`
- `capture_type`
- `occurred_at`
- `created_at`

### Optional fields

- `source_device`
- `raw_artifact_id`
- `derived_artifact_id`
- `privacy_class`
- `ingest_status`
- `metadata_json`

### Relationships

- may create artifacts, tasks, suggestions, conversations
- belongs on the timeline
- source for processing pipelines

---

## 10. Artifact

### Purpose
A durable piece of content or output.

Examples:

- note
- transcript
- audio file
- summary
- PDF
- markdown doc
- code snapshot reference

### Required fields

- `artifact_id`
- `artifact_type`
- `title`
- `created_at`
- `updated_at`

### Optional fields

- `content_hash`
- `mime_type`
- `storage_uri`
- `storage_tier`
- `sync_class`
- `source_capture_id`
- `source_system`
- `external_ref`
- `privacy_class`
- `metadata_json`

### Relationships

- linked to projects, conversations, people, timeline events, suggestions
- may derive from other artifacts
- may be indexed semantically and lexically

---

## 11. Suggestion

### Purpose
A surfaced prompt, reminder, insight, or recommendation from Vel.

Examples:

- You have not worked on the memoir in 18 days
- This idea may relate to your current project
- You promised to send this draft

### Required fields

- `suggestion_id`
- `suggestion_type`
- `created_at`
- `status`

### Optional fields

- `priority`
- `reason_code`
- `source_rule`
- `source_llm_run_id`
- `target_object_id`
- `target_object_type`
- `presented_at`
- `dismissed_at`
- `accepted_at`
- `feedback`
- `metadata_json`

### Relationships

- linked to goals, projects, commitments, conversations, timeline events
- feeds behavior tuning

---

## 12. Reflection

### Purpose
A summary or evaluative view across a time period.

Examples:

- daily summary
- weekly review
- monthly planning reflection
- year in review

### Required fields

- `reflection_id`
- `reflection_type`
- `time_range_start`
- `time_range_end`
- `created_at`

### Optional fields

- `summary_artifact_id`
- `container_id`
- `project_id`
- `privacy_class`
- `metadata_json`

### Relationships

- linked to timeline windows
- linked to goals/projects/behavior metrics

---

## 13. Timeline Event

### Purpose
A normalized event in Vel’s time model.

Examples:

- task completed
- meeting occurred
- artifact created
- suggestion accepted
- diary entry logged
- git activity detected

### Required fields

- `timeline_event_id`
- `event_type`
- `occurred_at`
- `created_at`

### Optional fields

- `object_id`
- `object_type`
- `container_id`
- `project_id`
- `person_id`
- `importance`
- `metadata_json`

### Relationships

- central to planning, replay, reflection, analytics
- may point to any primary object

---

## 14. Behavior Config

### Purpose
Configurable preferences governing how Vel behaves.

Examples:

- nag level
- suggestion frequency
- quiet hours
- health reminders
- memoir reminder cadence
- initiative rules

### Required fields

- `behavior_config_id`
- `scope_type`
- `scope_id`
- `created_at`
- `updated_at`

### Optional fields

- `key`
- `value_json`
- `source`
- `editable_by_vel`
- `metadata_json`

### Relationships

- may apply globally or to:
  - a goal
  - project
  - container
  - reminder type
  - health rule
- modified through user feedback and Vel self-tuning workflows

---

## 15. Diary Entry

### Purpose
A first-person reflection or narrative entry.

### Required fields

- `diary_entry_id`
- `created_at`
- `entry_date`

### Optional fields

- `title`
- `artifact_id`
- `mood`
- `energy_level`
- `pain_level`
- `privacy_class`
- `metadata_json`

### Relationships

- linked to timeline
- may link to goals, containers, projects, behavior insights

---

## Relationship Model

Vel should support both:

- direct foreign-key style relationships
- flexible graph-style links

A generic relationship table is recommended.

Example fields:

- `relationship_id`
- `source_object_type`
- `source_object_id`
- `relationship_type`
- `target_object_type`
- `target_object_id`
- `created_at`
- `metadata_json`

Examples:

- project `relates_to` project
- person `collaborates_on` project
- artifact `derived_from` artifact
- commitment `originated_in` conversation
- goal `supported_by` project

This makes the system much more flexible.

---

## Revision / History Model

For important user-facing objects, keep revision history.

Recommended for:

- goals
- projects
- behavior configs
- reflections
- summaries
- diary entries

Suggested revision fields:

- `revision_id`
- `object_type`
- `object_id`
- `version_number`
- `changed_at`
- `changed_by`
- `change_reason`
- `snapshot_json`

This enables:

- rollback
- introspection
- behavioral debugging
- future git-style exports

---

## Privacy Model

Every sensitive object should carry a privacy class.

Initial values:

- `private`
- `work`
- `sensitive`
- `do_not_record`

Suggested rules:

- `do_not_record` artifacts should not persist raw capture unless explicitly approved
- `sensitive` objects should require stronger sync/retention controls
- privacy inheritance may apply:
  - conversation → transcript artifact
  - capture → derived note
  - container → project default

Privacy policy should be configurable later, but must exist from the beginning.

---

## Sync Model

Objects and artifacts should support sync policy fields.

Recommended fields:

- `sync_status`
- `sync_class`
- `last_synced_at`
- `device_availability`
- `storage_tier`

Suggested sync classes:

- `hot`
- `warm`
- `cold`

Examples:

- daily summaries: `hot`
- recent tasks/goals: `hot`
- transcripts: `warm`
- raw audio archive: `cold`

---

## Suggestion Engine Data Model

Vel’s suggestion system should be hybrid.

### Rule-based stage
Deterministic detectors generate candidate events.

Examples:

- dormant project > N days
- due commitment approaching
- zero progress on high-priority goal
- repeated task deferral
- health reminder threshold

### LLM-based stage
An LLM can:

- rank candidate suggestions
- contextualize tone
- suppress noisy suggestions
- phrase nudges appropriately
- adapt to user behavior

This means the data model should preserve:

- detector source
- candidate score
- presentation decision
- user feedback

---

## Self-Tuning Model

Vel should support feedback on its own behavior.

Example feedback:

- too naggy
- more reminders for this
- don’t surface this at night
- be more direct about health
- stop asking about this project for now

This implies storing:

- behavior configs
- feedback events
- suggestion outcomes
- user preference adjustments

Vel can later use these records to recommend changes to its own behavior settings.

---

## Minimal v0 Schema Recommendation

For v0, do **not** implement everything at once.

Start with:

- containers
- projects
- goals
- tasks
- commitments
- captures
- artifacts
- conversations
- suggestions
- timeline events
- behavior configs
- relationships

This gives enough structure for:

- capture
- recall
- daily summaries
- basic planning
- contextual suggestions

Add later:

- reflections
- diary entries
- richer revision model
- advanced analytics
- scenario simulation

---

## Recommended v0 Table Strategy

### Core tables
- `containers`
- `projects`
- `goals`
- `tasks`
- `commitments`
- `people`
- `captures`
- `artifacts`
- `conversations`
- `suggestions`
- `timeline_events`
- `behavior_configs`
- `relationships`

### Shared utility tables
- `revisions`
- `sync_state`
- `processing_jobs`
- `embedding_index_refs`

---

## Migration Strategy

Vel should assume the model will evolve.

Recommended practices:

- every table includes `created_at`, `updated_at`
- use explicit migration files
- avoid over-normalizing early
- use `metadata_json` to protect iteration speed
- keep import/export paths open
- periodically snapshot key objects for backup/recovery

---

## Final Guidance

The point of this model is not perfection.

The point is to give Vel:

- enough structure to reason
- enough flexibility to evolve
- enough history to reflect
- enough identity to connect everything over time

Vel is building a memory-and-alignment system for a messy life.

The schema should therefore be **disciplined, but forgiving**.
