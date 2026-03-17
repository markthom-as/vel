---
title: Establish project boundary and registry
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Create the minimal first-class project registry so Projects is not just a pretty wrapper over arbitrary strings.

## Scope
- add a `projects` registry table
- define the domain/storage/API boundary for project identity
- document how `commitments.project` maps to `projects.slug`
- add seed/bootstrap behavior for backfilling existing project strings into registry entries where needed

## Requirements
- `slug` is canonical and stable
- `display_name` is user-facing and editable
- support at least `active`, `paused`, `archived`, `proposed` statuses
- include optional external mapping fields such as `todoist_project_id`
- keep settings/metadata JSON-backed initially
- do not break existing synthesis or commitment filtering that already uses project slug strings

## Suggested paths
```text
crates/vel-storage/src/db.rs
crates/vel-api-types/src/lib.rs
crates/veld/src/services/projects.rs
crates/veld/src/routes/projects.rs
```

## Migration sketch
Add table:
- `slug TEXT PRIMARY KEY`
- `display_name TEXT NOT NULL`
- `description TEXT NULL`
- `status TEXT NOT NULL`
- `color TEXT NULL`
- `icon TEXT NULL`
- `source_type TEXT NOT NULL`
- `source_ref TEXT NULL`
- `todoist_project_id TEXT NULL`
- `default_task_tags_json TEXT NOT NULL`
- `settings_json TEXT NOT NULL`
- `metadata_json TEXT NOT NULL`
- timestamps

## API contract
Add DTOs for:
- `ProjectData`
- `ProjectCreateRequest`
- `ProjectUpdateRequest`
- `ProjectSummaryData`

## Done when
- project registry exists with migrations
- typed DTOs exist
- backfill strategy is documented and implemented for existing commitment project strings
- existing repo behavior remains compatible
