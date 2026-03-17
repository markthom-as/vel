# Vel Project Operations Substrate Spec

Status: planned companion spec for project/routine/tag/dependency substrate  
Audience: core, storage, API, scheduler, and project workspace implementers  
Purpose: define the backend substrate that makes the Projects surface operationally useful, based on concrete patterns already proven in local workspace tooling without importing that implementation wholesale

This spec does not change shipped behavior by itself. Use [docs/status.md](../status.md) for implementation truth.

## 1. Why this exists

Vel already has a solid page-level Projects plan in [vel-projects-page-spec.md](vel-projects-page-spec.md), but that spec is mostly about the operator surface.

The local `codex-workspace` implementation demonstrates that a usable project system usually needs more than:

- a project slug
- a task list
- a session list

It also needs:

- a durable project registry with external source mappings and sync participation rules
- normalized tags that can survive cross-source round trips
- recurring routines and schedule-block semantics
- dependency and blocker projection that can shape task and workspace views

Vel has pieces of this already:

- `commitments.project`
- `metadata.tags`
- `commitment_dependencies`
- runtime loops and sync controls
- ritual-oriented planning under the task HUD and nudge specs

What is missing is one canonical substrate spec that says how these pieces fit together without creating conflicting authorities.

## 2. Scope

Included:

- project registry extension fields and source mappings
- normalized tag substrate for project tasks and related objects
- routine and schedule-block substrate for recurring anchors
- dependency and blocker projection for commitments in project workspaces
- sequencing guidance for storage, API, and projection work

Excluded:

- the full web/CLI Projects page IA, which remains owned by [vel-projects-page-spec.md](vel-projects-page-spec.md)
- replacing calendar providers with a second calendar system
- replacing Todoist or other ticket/task providers outright
- speculative autonomous scheduling that writes back without explicit policy

## 3. Design inputs

This spec is informed by concrete patterns already used in the local sibling `codex-workspace` repository:

- `schemas/project-registry.md`
- `schemas/project-schema.md`
- `schemas/chatgpt-tag-rules.md`
- `docs/scheduler.md`
- `docs/schedules.md`
- `workflows/project-sync.md`

Those files are design input, not Vel authority.

## 4. Core decisions

### 4.1 Projects are first-class records, not just slugs

Vel should keep `commitments.project` as the lightweight foreign-key-like project reference, but it should no longer treat projects as free-text buckets alone.

The project registry should additionally hold:

- display information
- owner/responsible person
- source mappings
- sync participation flags
- local/external workspace hints
- project-scoped defaults

This is similar to the role played by `codex-workspace`'s registry table, but Vel should store it as typed local-first records rather than a manually maintained markdown table.

### 4.2 Tags are normalized substrate, not UI glitter

Vel should treat tags as lightweight operational classifiers that can apply to:

- commitment-backed project tasks
- tickets later
- project defaults
- transcript/project matching rules

Near-term storage can remain `metadata.tags: string[]` for commitments, but the service and transport layers should expose tags as explicit typed fields.

Normalization rules:

- store canonical lowercase slugs
- preserve original provider labels in metadata when round-trip fidelity matters
- collapse duplicates deterministically
- treat tags as operator-facing semantic hints, not as a replacement for project identity

### 4.3 Routines are recurring anchors, not a second calendar authority

`codex-workspace` uses routine windows and schedule blocks to constrain daily planning.

Vel should adopt the same underlying idea, but with stricter boundaries:

- routines are recurring anchors such as morning prep, meds, shutdown, commute prep, writing block
- routines may define preferred windows, block semantics, and project affinity
- routines are not the canonical source of event truth when calendar integrations already own that

Routine substrate should support:

- recurring definitions
- optional project association
- block semantics such as `busy`, `free`, `preferred`, `ritual`
- derivation of due ritual tasks or scheduling hints
- projection into `Now`, HUD, and project workspaces

### 4.4 Dependencies stay commitment-backed

Vel already has `commitment_dependencies`.

That should remain the canonical blocker/dependency substrate for project task views until a stronger task/ticket authority explicitly replaces it.

Project workspace contracts should therefore expose:

- `blocked_by`
- `blocking`
- `waiting_on`
- dependency type
- whether the blocking item is inside or outside the active project

Do not invent a second dependency graph for Projects.

### 4.5 Scheduling policy must stay explainable

The useful lesson from `codex-workspace` is not "copy the scheduler."

The useful lesson is:

- schedule blocks should be explicit
- routine constraints should be inspectable
- tags and dependencies should influence prioritization and fit
- unschedulable work should remain visible rather than silently dropped

Vel should only adopt routine-aware scheduling through explicit policy-backed projections and explain surfaces.

## 5. Candidate data model

### 5.1 Project registry extensions

Extend the registry in [vel-projects-page-spec.md](vel-projects-page-spec.md) with optional fields such as:

```rust
pub struct ProjectRecord {
    pub slug: String,
    pub display_name: String,
    pub description: Option<String>,
    pub status: String,
    pub project_type: Option<String>,
    pub owner_person_id: Option<String>,
    pub website_url: Option<String>,
    pub repo_url: Option<String>,
    pub local_workspace_path: Option<String>,
    pub notes_ref: Option<String>,
    pub todoist_project_id: Option<String>,
    pub transcript_match_rules_json: serde_json::Value,
    pub sync_policies_json: serde_json::Value,
    pub default_task_tags_json: serde_json::Value,
    pub settings_json: serde_json::Value,
    pub metadata_json: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
```

Notes:

- `local_workspace_path` is a hint, not proof of existence or authority
- `sync_policies_json` can capture things like `todoist_write_through`, `transcript_auto_link`, or future source participation
- transcript/tag matching rules belong here or in a dedicated helper table, not in frontend-only config

### 5.2 Tag substrate

Near-term typed projection:

```rust
pub struct NormalizedTagData {
    pub slug: String,
    pub label: String,
    pub source: String, // vel | todoist | github | transcript_rule | inferred
}
```

Commitment-backed workspace tasks should expose:

```rust
pub struct ProjectTaskData {
    pub commitment: CommitmentData,
    pub tags: Vec<NormalizedTagData>,
    pub blocked_by: Vec<String>,
    pub waiting_on: Vec<String>,
    pub external_write_state: Option<String>,
}
```

### 5.3 Routine definitions

Candidate local-first routine model:

```rust
pub struct RoutineDefinitionRecord {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub status: String, // active | paused | archived
    pub routine_kind: String, // meds | prep | shutdown | writing | health | custom
    pub project_slug: Option<String>,
    pub recurrence_json: serde_json::Value,
    pub preferred_window_json: serde_json::Value,
    pub block_mode: String, // busy | free | preferred | ritual
    pub default_tags_json: serde_json::Value,
    pub metadata_json: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
```

Important boundary:

- routines are definitions
- occurrences may be derived or materialized later
- they should not replace calendar events where a real provider event exists

### 5.4 Schedule blocks

If routine-aware scheduling is implemented, its derived blocks should be explicit:

```rust
pub struct ScheduleBlockData {
    pub id: String,
    pub source: String, // calendar | routine | inferred
    pub title: String,
    pub block_mode: String, // busy | free | preferred | ritual
    pub starts_at: OffsetDateTime,
    pub ends_at: OffsetDateTime,
    pub tags: Vec<String>,
    pub project_slug: Option<String>,
    pub metadata: serde_json::Value,
}
```

These blocks are projection inputs, not durable authorities over the source systems by default.

## 6. Project matching and tag rules

`codex-workspace` uses keyword rules for ChatGPT-to-project mapping.

Vel should support a more explicit and inspectable version:

- exact project slug or display-name matches
- configured transcript/project keyword rules
- provider-native labels or metadata
- optional later inferred matches, always inspectable

Priority order:

1. explicit operator link
2. source-native stable mapping
3. configured keyword/tag rule
4. explainable inference

The Projects surface and explain surfaces should be able to answer:

- why a transcript/session/task is linked to a project
- which tag or rule caused the link
- whether the link is durable, provider-native, or inferred

## 7. Dependency projection rules

Dependency data should shape workspace and planning behavior.

Minimum project-task projection should include:

- direct blockers
- children/unlocks
- dependency type
- blocked/waiting badges
- counts for blocked work on the project index

Future scheduler or HUD policy may consume:

- number of unresolved blockers
- cross-project dependency pressure
- routine prerequisites
- whether a commitment is blocked on a non-actionable external item

## 8. Relationship to existing Vel specs

This spec extends but does not replace:

- [vel-projects-page-spec.md](vel-projects-page-spec.md)
- [commitments.md](commitments.md)
- [vel-task-hud-spec.md](vel-task-hud-spec.md)
- [vel-ticket-object-spec.md](vel-ticket-object-spec.md)

Authority split:

- Projects page spec owns operator surface behavior
- this spec owns substrate and backend planning for projects/tags/routines/dependencies
- commitments spec remains canonical for current commitment object semantics
- task HUD spec remains the main planning home for glance/ranking/ritual rendering

## 9. Recommended implementation phases

### Phase A — Registry and tags

- extend project registry for source mappings and sync policies
- normalize tags in typed DTOs and workspace filters
- add project matching rules for transcript/session/task ingestion

### Phase B — Dependency-aware projection

- expose dependency data in project workspace payloads
- show blocked/waiting counts and relationships
- add explain/debug coverage for why a task is blocked

### Phase C — Routines and recurring anchors

- add routine definitions
- derive ritual/project affinity projections
- expose routine-backed tasks or schedule hints in `Now`, HUD, and project overview

### Phase D — Routine-aware scheduling

- add explicit schedule-block projection
- consume tags/dependencies/routines in policy-backed scheduling
- keep unschedulable work visible and explainable

## 10. Acceptance criteria

- Vel has one explicit backend plan for project registry metadata, tags, routines, and dependencies
- project workspace contracts do not need to invent ad hoc logic for these concerns
- routine and schedule semantics do not create a competing hidden calendar authority
- dependency and tag behavior are explainable and testable
- every major phase is represented in the ticket pack rather than implied by scattered specs
