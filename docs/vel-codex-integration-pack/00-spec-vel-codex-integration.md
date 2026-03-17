---
title: Vel × Codex Workspace Integration Spec
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Summary

Vel already has the beginnings of external awareness:
- `signals`
- `commitments`
- `current_context`
- Todoist and calendar adapters
- settings-backed integration state

But the current repo still treats external systems mostly as:
- adapter-specific feeds
- thin sync status panels
- direct commitment updaters

Codex Workspace adds a more disciplined external ontology:
- Todoist is canonical for tasks
- Google Calendar is canonical for events
- `schemas/project-registry.md` is canonical for project naming and project-level taxonomy
- the Brain vault is canonical for long-form context
- `schemas/unified-schema-map.md` already explains how tasks, calendar, projects, inbox, and context roll into a daily plan

Vel should extend from the current repo state into a structure where:

```text
external system -> raw snapshot / API item -> normalized external item -> signal(s) -> linked commitment(s) -> current_context -> suggestions / nudges / questions / writeback proposals
```

# Non-goals

This spec does **not** make Vel the source of truth for:
- Todoist tasks
- Google Calendar events
- Codex Workspace project registry
- Brain vault note content

Vel may cache, normalize, infer, rank, and propose.  
Vel should not silently fork those systems into its own alternate reality.

# Canonical source-of-truth model

## Tasks
Canonical source:
- Codex Workspace / Todoist model (`schemas/task-schema.md`, `tools/todoist`)

Vel role:
- ingest
- normalize
- link to commitments
- infer risk / momentum / neglect / project drift
- propose changes

## Calendar
Canonical source:
- Codex Workspace / Google Calendar model (`schemas/calendar-schema.md`, `tools/google-calendar`)

Vel role:
- ingest
- normalize
- attach prep/travel semantics
- infer schedule pressure and context transitions
- propose changes

## Projects
Canonical source:
- `schemas/project-registry.md`
- `schemas/project-schema.md`

Vel role:
- maintain stable project identity and aliases locally
- attach commitments and events to project clusters
- infer active, blocked, drifting, dormant project state

## Long-form context
Canonical source:
- Brain vault / Codex notes

Vel role:
- store summaries and extracted artifacts
- reference, not replace

# Required ontology additions

Vel should explicitly add the following domain types:

## `ExternalSourceKind`
Represents the source family:
- `todoist`
- `google_calendar`
- `codex_project_registry`
- `brain_vault`
- `metrics`
- future: `gmail`, `messages`, `github`, `healthkit`

## `ExternalItemKind`
Represents the normalized external object class:
- `task`
- `calendar_event`
- `project`
- `note`
- `metric_rollup`

## `ExternalItemRef`
Stable reference to canonical data:
- source kind
- external id
- snapshot hash / sync version
- canonical URL or path when available

## `ProjectIdentity`
Stable project identity independent of source-specific IDs:
- project slug
- canonical display name
- aliases
- source mappings

# Schema plan

Add four new tables.

## 1. `external_items`
Normalized cache of canonical-source objects.

Suggested columns:
- `external_item_id TEXT PRIMARY KEY`
- `source_kind TEXT NOT NULL`
- `item_kind TEXT NOT NULL`
- `external_id TEXT NOT NULL`
- `project_slug TEXT`
- `title TEXT`
- `state TEXT`
- `start_at INTEGER`
- `end_at INTEGER`
- `due_at INTEGER`
- `url TEXT`
- `payload_json TEXT NOT NULL`
- `fingerprint TEXT NOT NULL`
- `first_seen_at INTEGER NOT NULL`
- `last_seen_at INTEGER NOT NULL`
- `last_changed_at INTEGER NOT NULL`
- `archived_at INTEGER`

Unique index:
- `(source_kind, external_id)`

## 2. `external_item_links`
Cross-links external items to Vel objects.

Suggested columns:
- `id INTEGER PRIMARY KEY`
- `external_item_id TEXT NOT NULL`
- `commitment_id TEXT`
- `artifact_id TEXT`
- `signal_id TEXT`
- `link_kind TEXT NOT NULL`
- `created_at INTEGER NOT NULL`

This prevents Vel from pretending a commitment *is* a Todoist task. It is linked, not identical.

## 3. `projects`
Local stable project registry derived from Codex canonical project names.

Suggested columns:
- `project_id TEXT PRIMARY KEY`
- `slug TEXT NOT NULL UNIQUE`
- `display_name TEXT NOT NULL`
- `status TEXT NOT NULL`
- `project_type TEXT`
- `owner TEXT`
- `description TEXT`
- `metadata_json TEXT NOT NULL`
- `created_at INTEGER NOT NULL`
- `updated_at INTEGER NOT NULL`

## 4. `sync_watermarks`
Tracks incremental ingestion state.

Suggested columns:
- `source_kind TEXT PRIMARY KEY`
- `cursor_json TEXT`
- `last_snapshot_fingerprint TEXT`
- `last_success_at INTEGER`
- `last_attempt_at INTEGER`
- `last_error TEXT`

# How this maps onto the current repo

## Existing assets to preserve
- `signals` is still the evidence bus
- `commitments` remains the unit of user-relevant obligation
- `current_context` remains the "what matters now" boundary
- `suggestions` remains a proposal layer
- existing `integrations` settings should continue to store credentials / selection state

## Existing assets to stop overloading
- `commitments.source_id` is currently too overloaded for external identity
- `signal.payload_json` is currently being used as the only external cache
- `project` as a freeform string is too weak for project identity
- Todoist adapter currently writes commitments directly from snapshots without a durable normalized external-item layer

# Ingestion pipeline

## Step 1: source fetch
Per source:
- load snapshot or fetch API data
- compute snapshot metadata
- store/update source watermark

## Step 2: normalize to `ExternalItem`
All sources normalize into a shared struct.

Suggested new file:
- `crates/vel-core/src/external.rs`

Suggested shape:

```rust
pub enum ExternalSourceKind {
    Todoist,
    GoogleCalendar,
    CodexProjectRegistry,
    BrainVault,
}

pub enum ExternalItemKind {
    Task,
    CalendarEvent,
    Project,
    Note,
}

pub struct ExternalItem {
    pub external_item_id: String,
    pub source_kind: ExternalSourceKind,
    pub item_kind: ExternalItemKind,
    pub external_id: String,
    pub project_slug: Option<String>,
    pub title: Option<String>,
    pub state: Option<String>,
    pub start_at: Option<OffsetDateTime>,
    pub end_at: Option<OffsetDateTime>,
    pub due_at: Option<OffsetDateTime>,
    pub url: Option<String>,
    pub payload_json: serde_json::Value,
    pub fingerprint: String,
}
```

## Step 3: emit signals
Transform external items into one or more signals:
- task imported
- task completed
- event scheduled
- project activated
- project paused
- event start window approaching
- overdue task persists

## Step 4: reconcile commitments
Rules:
- not every task must become a commitment
- future calendar events that represent real obligations should usually create or update commitments
- project items do **not** create commitments; they enrich project identity

## Step 5: enrich current context
Context service reads:
- active commitments
- soon/upcoming external events
- active project clusters
- unresolved uncertainty / questions
- recent suggestion outcomes

# Project awareness model

Codex Workspace already has:
- canonical project names
- project path mapping
- project sync flags
- a stable registry table in markdown

Vel should import that into a real local project table and use `project_slug` everywhere internally.

Project awareness means Vel can answer:
- what project is this task part of?
- which projects are active but neglected?
- which calendar events belong to which projects?
- which suggestions should be grouped by project?

Project awareness should not depend on fragile string equality at runtime.

# Writeback model

Vel should not directly mutate canonical systems from core evaluation services.

Instead use:
- `sync_operations` table
- explicit proposal state
- approval gate when required

Suggested proposal states:
- `pending`
- `approved`
- `rejected`
- `applied`
- `failed`

Examples:
- reorder Todoist tasks
- move an event start time
- add a calendar prep block
- add missing project label to a task

# Current-context changes

Extend `current_context.context_json` shape to include stable sections:

```json
{
  "generated_at": "...",
  "projects": {
    "active": [],
    "blocked": [],
    "drifting": []
  },
  "tasks": {
    "due_today": [],
    "overdue": [],
    "in_progress": []
  },
  "calendar": {
    "next_event": null,
    "prep_windows": [],
    "free_windows": []
  },
  "suggestions": [],
  "uncertainties": [],
  "sync": {
    "todoist": {},
    "google_calendar": {},
    "project_registry": {}
  }
}
```

# Suggested new/changed files

## New
- `crates/vel-core/src/external.rs`
- `crates/vel-core/src/project.rs`
- `crates/veld/src/services/external_sync.rs`
- `crates/veld/src/services/project_awareness.rs`
- `crates/veld/src/services/sync_proposals.rs`
- `crates/veld/src/routes/projects.rs`
- `migrations/0025_external_items.sql`
- `migrations/0026_projects.sql`
- `migrations/0027_sync_watermarks.sql`
- `migrations/0028_sync_operations.sql`
- `docs/specs/vel-codex-integration.md`

## Changed
- `crates/veld/src/adapters/todoist.rs`
- `crates/veld/src/adapters/calendar.rs`
- `crates/veld/src/services/inference.rs`
- `crates/veld/src/services/integrations.rs`
- `crates/veld/src/routes/integrations.rs`
- `crates/vel-storage/src/db.rs`
- `crates/vel-core/src/lib.rs`
- `crates/vel-api-types/src/lib.rs`

# Acceptance criteria

- Todoist, calendar, and project registry each have a durable normalized representation in Vel
- Vel can answer project-aware questions without freeform string guessing
- commitments link to canonical external items instead of pretending to own them
- current_context exposes external awareness in a stable typed structure
- all writeback actions are represented as explicit proposals / sync operations
- integration ingestion is incremental, observable, and idempotent
