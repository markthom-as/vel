# Vel Ticket Object Spec

Status: planned
Audience: core, storage, API, sync, project-surface, and integration implementers
Purpose: define `ticket` as a first-class Vel object with a native backend plus provider-backed integrations such as GitHub Issues, Linear, Jira, and Todoist

## 1. Why This Exists

Vel already has a durable actionable object in `commitments`.

That is useful, but it is not enough.

Real project and operations work often lives in systems like:

- Vel-native local work queues
- GitHub Issues
- Linear
- Jira
- Todoist

Those systems are not interchangeable with a commitment.

A commitment is:

- personal
- review-oriented
- about what matters to the user
- often smaller or more immediate than an upstream work item

A ticket is:

- a durable work object
- usually provider-backed
- often shared with teams or systems outside Vel
- richer in workflow metadata such as status, assignee, labels, board/sprint, backlinks, and external URLs

Today Vel leans on commitments and Todoist mirroring for task-shaped work. That works for a narrow slice, but it flattens the distinction between:

- external backlog objects
- personal obligations
- local Vel-only operator tasks

Vel should therefore add **ticket** as a first-class object.

## 2. Goals

- Add `ticket` as a first-class Vel object with stable identity.
- Support a native Vel ticket backend.
- Support provider-backed ticket sources:
  - GitHub Issues
  - Linear
  - Jira
  - Todoist
- Preserve provider metadata without leaking provider semantics into every core workflow.
- Keep `ticket` distinct from `commitment`.
- Allow links between tickets, commitments, projects, sessions, runs, captures, artifacts, and people.
- Support both provider-owned and Vel-owned tickets.
- Make project surfaces, CLI, and APIs ticket-aware.

## 3. Non-Goals

- Replacing every provider UI.
- Building a generic Jira clone inside Vel.
- Eliminating commitments.
- Full bidirectional support for every provider in the first slice.
- Assuming every ticket must map to exactly one commitment.

## 4. Core Domain Decision

Vel should have both:

- `ticket`
- `commitment`

They are related but not identical.

### 4.1 Ticket

A ticket is a canonical work object representing a backlog item, issue, task, bug, feature, or operational unit.

Examples:

- GitHub issue `#123`
- Linear issue `ENG-42`
- Jira issue `PLAT-199`
- Todoist task mirrored as a ticket
- local Vel-native “fix transcript provider contract” item

### 4.2 Commitment

A commitment is a personal or operational obligation that matters to the user.

Examples:

- “prep for meeting with Dimitri”
- “take meds”
- “finish risk engine cleanup”
- “reply to Marta”

### 4.3 Relationship

Tickets and commitments should link, not collapse.

Examples:

- one ticket may produce multiple commitments
- one commitment may point to one ticket as its upstream work object
- some tickets have no commitment because they are merely reference backlog
- some commitments have no ticket because they are life or personal obligations

This distinction is critical.

Do not continue flattening all task-shaped work into commitments alone.

## 5. Canonical Ticket Model

Minimum canonical fields:

- `ticket_id`
- `backend`
- `provider`
- `provider_ticket_id`
- `provider_project_id`
- `provider_project_key`
- `human_ref`
  - `#123`
  - `ENG-42`
  - `PLAT-199`
- `title`
- `body_markdown`
- `state`
- `ticket_type`
  - bug
  - feature
  - task
  - chore
  - docs
  - research
  - incident
  - followup
- `priority`
- `assignee_person_ids`
- `reporter_person_id`
- `labels`
- `project_slug`
- `milestone`
- `sprint`
- `estimate`
- `url`
- `created_at`
- `updated_at`
- `closed_at`
- `metadata_json`

Minimum backend fields:

- `backend`
  - `vel`
  - `github`
  - `linear`
  - `jira`
  - `todoist`
- `connection_id`
- `external_id`
- `sync_state`
- `write_authority`
  - `vel_authoritative`
  - `provider_authoritative`
  - `mirror_read_only`
  - `write_through`

## 6. Ticket Backend Model

Vel should support two broad backend classes.

### 6.1 Native Vel Backend

Vel-native tickets are created and stored directly by Vel.

Use cases:

- local operator backlog
- personal project tickets not owned by GitHub/Linear/Jira/Todoist
- bridge objects before export to an external provider
- fallback when a provider is unavailable or intentionally not used

Required abilities:

- create, update, close, reopen
- assign project slug
- link commitments
- link sessions, captures, artifacts, and runs
- optional later export to a provider

### 6.2 Provider-Backed Backend

Provider-backed tickets are mirrored from or synchronized with external systems.

Initial providers:

- `github`
- `linear`
- `jira`
- `todoist`

Each provider must map into the canonical ticket model while preserving provider-specific metadata in structured form.

## 7. Provider Notes

### 7.1 GitHub Issues

GitHub issues fit naturally as tickets.

Provider-specific metadata to preserve:

- repository
- issue number
- labels
- author
- assignees
- milestone
- linked PRs
- state reason

GitHub issues remain useful both as:

- self-knowledge evidence
- project/work tickets

These roles should share the same canonical ticket identity instead of creating one issue object for self-knowledge and another task object for project work.

### 7.2 Linear

Preserve:

- team key
- issue identifier
- workflow state
- cycle
- estimate
- parent/child relationship
- project linkage

### 7.3 Jira

Preserve:

- issue key
- project key
- issue type
- workflow state
- sprint / board references
- epic / parent relationships
- custom field metadata in structured form

### 7.4 Todoist

Todoist tasks can map into tickets when the user wants a project/work object, not just a lightweight reminder.

Important distinction:

- some Todoist items should still become commitments directly
- some Todoist items should become tickets with linked commitments

Vel must support policy deciding whether a Todoist import becomes:

- commitment only
- ticket only
- both ticket and commitment

## 8. Relationship Model

Tickets should support durable links to:

- commitments
- projects
- people
- sessions
- captures
- artifacts
- runs
- signals
- documents/specs

Required link examples:

- `ticket -> commitment`
- `ticket -> project`
- `ticket -> agent_session`
- `ticket -> github_pr`
- `ticket -> capture`
- `ticket -> run`
- `ticket -> person`

## 9. Ticket State Model

Canonical states should be simple and provider-agnostic:

- `open`
- `in_progress`
- `blocked`
- `in_review`
- `done`
- `cancelled`

Provider states map into canonical states plus provider metadata.

Examples:

- GitHub `open` -> `open`
- GitHub `closed` -> `done` or `cancelled` based on state reason when available
- Linear `in progress` -> `in_progress`
- Jira workflow states map via provider config
- Todoist checked -> `done`

Do not force provider state machines into the canonical core.

## 10. Commitments and Tickets

This subsystem requires a stricter rule than current planning docs.

### 10.1 New Rule

Commitments are not the canonical task object for all project work.

Tickets are the canonical backlog/work object.

Commitments remain the canonical personal obligation object.

### 10.2 Mapping Rules

Examples:

- a GitHub issue assigned to the user may create a linked commitment:
  - ticket: “fix websocket reconnect drift”
  - commitment: “finish websocket reconnect fix today”
- a Jira epic may have no commitment because it is reference planning
- a personal reminder may be commitment-only and never become a ticket
- a Vel-native local backlog item may be a ticket first and produce commitments later

### 10.3 Operator Surfaces

Projects page and future task surfaces should show:

- tickets
- commitments derived from or linked to tickets
- unlinked commitments

Those should be visible as related but distinct layers.

## 11. API Surface

Needed API concepts:

- `GET /v1/tickets`
- `POST /v1/tickets`
- `GET /v1/tickets/:id`
- `PATCH /v1/tickets/:id`
- `POST /v1/tickets/:id/link-commitment`
- `GET /v1/tickets/:id/links`
- `GET /v1/ticket-backends`
- `POST /v1/ticket-backends/:provider/sync`

Provider-aware query filters:

- `backend`
- `provider`
- `project`
- `state`
- `assignee`
- `ticket_type`
- `source_ref`

## 12. CLI Surface

Needed CLI concepts:

- `vel tickets`
- `vel ticket inspect <id>`
- `vel ticket add`
- `vel ticket update <id>`
- `vel ticket done <id>`
- `vel ticket link-commitment <ticket_id> <commitment_id>`
- `vel ticket sync <provider|connection>`

The CLI should support both:

- local Vel-native ticket workflows
- provider-backed sync and inspection

## 13. Project Surface Implications

The Projects page spec currently says “tasks remain commitment-backed.”

This new spec changes the planned direction:

- projects should become ticket-first for backlog/work tracking
- commitments remain visible and linkable as personal obligations
- project views should support mixed provider backends in one normalized list

This is a planned architectural correction, not a statement about current shipped behavior.

## 14. Storage Model

Introduce at minimum:

- `tickets`
- `ticket_links`
- `ticket_comments` or `ticket_events`
- `ticket_backend_connections`
- `ticket_sync_cursor` or equivalent sync state tracking

Candidate `tickets` columns:

- id
- backend
- provider
- connection_id
- external_id
- human_ref
- project_slug
- title
- body_markdown
- state
- ticket_type
- priority
- assignee_person_ids_json
- reporter_person_id
- labels_json
- milestone
- sprint
- estimate_json
- url
- sync_state
- write_authority
- metadata_json
- created_at
- updated_at
- closed_at

## 15. Explainability

Explain surfaces should answer:

- where did this ticket come from?
- which provider and connection own it?
- why did it map to this project?
- which commitments are linked to it?
- which runs, captures, or sessions reference it?
- if provider state changed, when and why?

## 16. Rollout Order

### Phase 1

- canonical ticket schema
- native Vel backend
- project linking
- commitment linking
- list/detail API + CLI

### Phase 2

- GitHub issues backend
- Todoist backend migration from commitment-only mirroring
- project page ticket-first read model

### Phase 3

- Linear backend
- Jira backend
- write-through sync policy
- comment/event timelines

## 17. Hard Rules

- `ticket` is a first-class object in `vel-core`.
- tickets and commitments must remain distinct types.
- provider-specific workflow state must map into canonical fields plus structured provider metadata.
- a native Vel ticket backend must exist, not only mirrored providers.
- any provider-backed ticket must carry stable provenance to its provider, connection, and external id.
- project surfaces must not assume Todoist is the only durable work backend.

## 18. Initial Ticket Pack

This spec should be executed through a dedicated ticket-object pack covering:

- schema and domain types
- native Vel backend
- commitments link model
- GitHub issues backend
- Todoist backend migration
- Linear backend
- Jira backend
- API/CLI
- project surface migration
- tests and rollout
