---
id: VEL-PROJ-002
title: Add project registry, agent sessions, and outbox migrations
status: proposed
priority: P0
estimate: 2-4 days
dependencies:
  - VEL-PROJ-001
labels:
  - projects
  - storage
  - migrations
---

# Goal

Add the minimum durable tables needed to support project identity, active agent sessions, queued messages, and structured control records.

# Scope

Add migrations and storage mappings for:

- `project_registry`
- `project_links` (optional but recommended)
- `agent_sessions`
- `agent_outbox`
- `agent_steering`
- `agent_feedback`

Do **not** add a new durable `tasks` table in this ticket.

# Suggested schema fields

## `project_registry`
- `id`
- `slug`
- `title`
- `status`
- `primary_source`
- `aliases_json`
- `tags_json`
- `metadata_json`
- `created_at`
- `updated_at`

## `agent_sessions`
- `id`
- `source`
- `external_conversation_id`
- `title`
- `project_id`
- `project_confidence`
- `status`
- `capabilities_json`
- `settings_json`
- `last_message_at`
- `created_at`
- `updated_at`

## `agent_outbox`
- `id`
- `session_id`
- `project_id`
- `author_kind`
- `body`
- `message_kind`
- `dispatch_state`
- `requires_manual_dispatch`
- `metadata_json`
- `created_at`
- `updated_at`

## `agent_steering`
- `id`
- `session_id`
- `project_id`
- `steering_kind`
- `content`
- `applies_until`
- `created_at`

## `agent_feedback`
- `id`
- `session_id`
- `project_id`
- `feedback_kind`
- `rating`
- `notes`
- `created_at`

# Deliverables

- new migration SQL file(s)
- `vel-storage` record/insert/update structs
- row mappers and CRUD/query methods in `crates/vel-storage/src/db.rs`
- migration tests or at least integration coverage proving tables are usable

# Acceptance criteria

- Schema migrates cleanly on empty and existing DBs.
- New tables have indexes on common lookup paths.
- Storage methods exist for create/list/update flows required by later tickets.
- No table duplicates current chat message authority.

# Notes for agent

The temptation to create eleven join tables is strong. Resist. This needs to stay tractable for a single-operator SQLite system.
