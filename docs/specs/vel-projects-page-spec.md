---
title: Vel projects page and multi-agent active chat workspace spec
status: proposed
owner: product+engineering
priority: P0
created: 2026-03-16
related:
  - docs/specs/vel-chat-interface-implementation-brief.md
  - docs/specs/vel-addendum-calendar-todoist-workflows.md
  - docs/specs/vel-task-hud-spec.md
  - docs/status.md
---

# Vel Projects Page — Spec

## Summary

Vel needs a first-class **Projects** surface that sits between the current `Now` dashboard and the existing thread/chat shell.

Today, the repo already has:

- Todoist sync into **commitments** and `external_task` signals.
- A web app with `Now`, `Inbox`, `Threads`, and `Settings`.
- A native Vel chat runtime with conversations/messages/interventions.
- Transcript ingestion for external assistants via `assistant_transcripts`.

What it does **not** yet have is a coherent project workspace where the user can:

- see all projects in one place,
- browse project-scoped tasks backed by Todoist,
- create and tag tasks without leaving Vel,
- see active agent/chat work across Vel, Codex, Claude, OpenCode, ChatGPT, etc,
- queue messages to those agents,
- steer, annotate, or critique those agent sessions,
- manage per-agent settings from a project lens.

This spec defines that surface.

## Current repo truth

Grounding against the attached codebase matters, otherwise this turns into decorative vaporware.

### Already present

- **Todoist integration**
  - `/api/integrations/todoist` credential plumbing exists.
  - `/v1/sync/todoist` exists.
  - direct Todoist API sync exists in `crates/veld/src/services/integrations.rs`.
  - snapshot ingestion also exists in `crates/veld/src/adapters/todoist.rs`.
  - tasks currently normalize into `commitments` plus `external_task` signals.
- **Project-ish data already exists, but weakly**
  - commitments have a nullable `project` field.
  - project synthesis route exists: `POST /v1/synthesis/project/:slug`.
- **Web shell exists**
  - current top-level web navigation in `clients/web/src/App.tsx` only exposes `Now`, `Inbox`, `Threads`, and `Settings`.
- **External assistant material exists, but is passive**
  - transcript ingestion lands in `assistant_transcripts` via `migrations/0021_assistant_transcripts.sql` and `crates/veld/src/adapters/transcripts.rs`.
  - external assistant messages become signals, but there is no first-class operator surface for them.

### Missing today

- No Projects page.
- No project registry or stable project identity model.
- No Todoist write path for task creation, tagging, rescheduling, completion, or project moves.
- No first-class external chat session model.
- No queue/outbox model for agent-bound messages.
- No steering / feedback / control plane for non-Vel chats.
- No shared "rich view vs CLI view" contract for project workspaces.

That is the gap this pack closes.

## Product thesis

The Projects page should be a **workbench**, not a museum.

It should answer, per project:

1. what work exists,
2. what is active,
3. what is blocked or drifting,
4. what conversations are currently moving the work,
5. what message or steering action should be sent next.

In psychoanalytic terms: this page should not just archive the signifiers of work; it should reveal where desire, avoidance, delegation, and symbolic commitment are currently attaching. If a project page cannot expose drift, displacement, and pseudo-work, it is just a prettier inbox with better shoes.

## Goals

- Add a top-level **Projects** surface to Vel web.
- Make **Todoist-backed project tasks** visible and editable from Vel.
- Incorporate **Codex-workspace project semantics** into project grouping and tagging.
- Show **active chats/sessions** per project across Vel and external agent systems.
- Support **queued outbound messages** to those agents.
- Support **steering, feedback, and per-session controls**.
- Provide one canonical backend/view-model that can power:
  - a rich web page,
  - a CLI/TUI view,
  - future ambient/mobile surfaces.

## Non-goals

Do not turn this into a bloated Notion cosplay.

Specifically out of scope for v1:

- full collaborative multi-user PM system,
- arbitrary kanban/roadmap/Gantt madness,
- bidirectional real-time control of every external agent product,
- speculative autonomous agent execution without explicit audit trail,
- replacing Todoist as the authority for user-authored tasks.

## Design principles

### 1. External systems remain authoritative where they already are

- **Todoist** remains the primary authority for user task data.
- **Vel** remains authority for internal chats, interventions, provenance, and policy decisions.
- External systems like Codex/Claude/OpenCode remain authorities for their native sessions unless and until a specific adapter supports mutation.

Vel should orchestrate, contextualize, and cache — not fork reality into seven mutually hostile truths.

### 2. Projects are a lens, not necessarily a new source of truth

Do **not** introduce a heavyweight `projects` ontology unless the repo can justify it.

Initial project identity should be resolved from:

- Todoist project names / IDs,
- commitment `project` values,
- codex-workspace tags and project markers,
- transcript metadata such as `project_hint`,
- future thread/project synthesis artifacts.

A registry may be needed, but it should start narrow and boring.

### 3. Shared view model, multiple renderers

The web page and CLI page should render the same canonical `ProjectWorkspaceData` contract.

That avoids the classic split-brain bug where the CLI becomes the grimy truth and the rich UI becomes the lying brochure.

### 4. Queueing is first-class

If Vel cannot yet send a message directly to an external system, the user should still be able to:

- draft it,
- queue it,
- mark intended target,
- review it,
- copy/export/dispatch manually,
- record completion or failure.

Graceful degradation beats fake integration bravado.

## Functional requirements

## A. Projects index

The Projects page must provide:

- all active projects,
- project health summary,
- task counts,
- top risks / drift indicators,
- last activity timestamp,
- active chat/session count,
- one-line next suggested action.

### Minimum project sources

- Todoist project mapping
- commitment `project`
- transcript metadata `project_hint`
- optional codex-workspace project extraction

### Minimum project states

- active
- paused
- dormant
- archived

These are Vel projection states, not necessarily Todoist states.

## B. Project detail workspace

Each project detail page/view must show four primary panels.

### 1. Work panel

Shows task-like work items derived from Todoist-backed commitments.

Required capabilities:

- list tasks by state: now, soon, waiting, done recently
- show labels/tags
- add a task
- edit title
- complete/reopen
- apply/remove labels
- reschedule due date
- move between projects where supported
- show source + sync state

### 2. Active chats panel

Shows project-relevant sessions across:

- Vel chats
- Codex chats
- Claude chats
- OpenCode chats
- ChatGPT or other future transcript sources

Each session card should show:

- source
- title or inferred summary
- project association confidence
- last speaker / last message age
- queue depth
- unread/review-needed indicator
- steering/feedback affordances

### 3. Queue / outbox panel

Shows outbound messages not yet dispatched or awaiting adapter execution.

Required states:

- draft
- queued
- sending
- sent
- failed
- cancelled
- needs_manual_dispatch

### 4. Controls / settings panel

Per project and per session controls:

- preferred agent / routing target
- tone/style presets
- autonomy level
- approval requirement
- project tags and aliases
- transcript ingestion enable/disable
- external adapter capabilities and health

## C. Add-and-tag task flow

The user explicitly asked that Vel allow adding and tagging tasks from this page.

That requires a real write path.

### Required behavior

- create Todoist task from Vel UI/CLI
- set title/content
- set Todoist project
- set labels
- set due date/time
- optionally attach Vel metadata in task description or comment if needed
- reconcile created task back into local commitment projection

### Boundary rule

Vel should not create a separate local-only task unless:

- Todoist is unavailable, or
- the user intentionally chooses local draft mode.

If Todoist is unavailable, create a local outbox/draft task with clear unsynced status. No sneaky shadow systems.

## D. Active chat and steering flow

### Required user actions

For any active session, the user must be able to:

- queue a message,
- mark a message as steering,
- attach project-scoped feedback,
- set or update session tags,
- change agent/session settings,
- mark a session as active, paused, done, or noisy,
- link/unlink the session from a project.

### Steering examples

- “stay within repo truth; do not invent endpoints”
- “optimize for implementation tickets, not architecture poetry”
- “prioritize docs reconciliation before adding new surfaces”

The system should treat steering as a structured control object, not just raw chat text lost in the soup.

## E. CLI view parity

The user said this could be CLI or rich chat view. The correct move is: **both, with one backend contract**.

### CLI minimum

Add a CLI/TUI-friendly projects command family:

- `vel projects list`
- `vel projects show <slug>`
- `vel projects task add`
- `vel projects task tag`
- `vel projects chat queue`
- `vel projects chat steer`
- `vel projects chat feedback`

The CLI may start as plain terminal output plus action commands before any full TUI work. No need to summon ncurses from the abyss on day one.

## Domain model

## 1. Project registry

A narrow registry is recommended.

```rust
pub struct ProjectRecord {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub status: ProjectStatus,
    pub aliases: Vec<String>,
    pub source_refs: Vec<ProjectSourceRef>,
    pub primary_source: ProjectPrimarySource,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}
```

This registry is for identity resolution and UI stability, not for replacing Todoist projects.

## 2. Project workspace view model

```rust
pub struct ProjectWorkspaceData {
    pub project: ProjectSummary,
    pub work_items: Vec<ProjectWorkItem>,
    pub active_chats: Vec<AgentSessionSummary>,
    pub queued_messages: Vec<QueuedAgentMessage>,
    pub controls: ProjectControlState,
    pub health: ProjectHealth,
    pub warnings: Vec<String>,
}
```

This should live in `vel-api-types` and be renderer-neutral.

## 3. Work items

Initial recommendation: **derive from commitments**, not a new task table.

```rust
pub struct ProjectWorkItem {
    pub commitment_id: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub title: String,
    pub status: String,
    pub due_at: Option<String>,
    pub project: Option<String>,
    pub labels: Vec<String>,
    pub priority: Option<u8>,
    pub sync_state: SyncState,
    pub metadata: serde_json::Value,
}
```

Todoist-specific mutable fields can still round-trip through a dedicated adapter.

## 4. Agent sessions

Add a first-class session registry for both Vel-native and external agent sessions.

```rust
pub enum AgentSessionSource {
    Vel,
    Codex,
    Claude,
    OpenCode,
    ChatGpt,
    Other(String),
}

pub struct AgentSessionRecord {
    pub id: String,
    pub source: AgentSessionSource,
    pub external_conversation_id: String,
    pub title: Option<String>,
    pub project_id: Option<String>,
    pub project_confidence: Option<f32>,
    pub status: AgentSessionStatus,
    pub capabilities: AgentSessionCapabilities,
    pub settings_json: serde_json::Value,
    pub last_message_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}
```

This sits above raw `assistant_transcripts`.

## 5. Queued messages

```rust
pub struct QueuedAgentMessage {
    pub id: String,
    pub session_id: String,
    pub project_id: Option<String>,
    pub author_kind: String,
    pub body: String,
    pub message_kind: QueuedMessageKind,
    pub dispatch_state: DispatchState,
    pub requires_manual_dispatch: bool,
    pub metadata: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}
```

## 6. Steering and feedback

These should be structured, auditable records.

```rust
pub struct AgentSteeringRecord {
    pub id: String,
    pub session_id: String,
    pub project_id: Option<String>,
    pub steering_kind: String,
    pub content: String,
    pub applies_until: Option<i64>,
    pub created_at: i64,
}

pub struct AgentFeedbackRecord {
    pub id: String,
    pub session_id: String,
    pub project_id: Option<String>,
    pub feedback_kind: String,
    pub rating: Option<i32>,
    pub notes: Option<String>,
    pub created_at: i64,
}
```

## Data sources and mapping

## Todoist

Use the existing Todoist integration service as the write/read backbone.

Needed expansions beyond current code:

- fetch labels if Todoist API supports them in the chosen endpoint set,
- create task,
- update task,
- close/reopen task,
- list projects/labels for UI selectors,
- write-through reconciliation into commitments.

## Codex workspace semantics

The existing addendum already notes project/tag conventions. This page should explicitly incorporate those conventions.

At minimum, normalize:

- `project:` tags
- workspace project names/aliases
- tags embedded in task text or metadata
- transcript `metadata.project_hint`

This mapping should produce a stable project slug resolution flow.

## Vel-native chats

Use existing conversations/messages/interventions as one session source.

## External chats

Leverage ingested `assistant_transcripts` as the raw event source, but add a session registry above them so the UI can reason about:

- one active session vs many transcripts,
- capabilities,
- queueability,
- control state,
- project linkage.

## API design

## New endpoints

### Project read APIs

- `GET /api/projects`
- `GET /api/projects/:slug`
- `GET /api/projects/:slug/workspace`
- `GET /api/projects/:slug/chats`

### Project task mutation APIs

- `POST /api/projects/:slug/tasks`
- `PATCH /api/projects/:slug/tasks/:id`
- `POST /api/projects/:slug/tasks/:id/complete`
- `POST /api/projects/:slug/tasks/:id/reopen`
- `POST /api/projects/:slug/tasks/:id/tags`
- `DELETE /api/projects/:slug/tasks/:id/tags/:tag`

### Chat/session control APIs

- `POST /api/projects/:slug/chats/:session_id/queue`
- `POST /api/projects/:slug/chats/:session_id/steering`
- `POST /api/projects/:slug/chats/:session_id/feedback`
- `PATCH /api/projects/:slug/chats/:session_id/settings`
- `PATCH /api/projects/:slug/chats/:session_id/link`

### Capability/status APIs

- `GET /api/agent-sources`
- `GET /api/agent-sessions`
- `GET /api/agent-sessions/:id`

## Websocket events

Recommended new events:

- `projects:updated`
- `project:workspace_updated`
- `agent_sessions:updated`
- `agent_outbox:updated`

These should follow the existing typed websocket event pattern already used by chat.

## Storage design

## New tables

Recommended minimum additions:

### `project_registry`

Stable project identity, aliases, and mapping metadata.

### `agent_sessions`

Registry for Vel-native and external sessions.

### `agent_outbox`

Queued outbound messages.

### `agent_steering`

Structured steering instructions.

### `agent_feedback`

Structured feedback records.

### `project_links`

Optional mapping table to associate commitments, sessions, or transcripts with a stable project ID when source names drift.

## Avoid

Do **not** add a duplicate durable `tasks` table unless the implementation can prove commitments are insufficient.

That kind of duplication is how software develops neurosis: the ego says one thing, the symptom says another, and the operator gets to play analyst to a malfunctioning database.

## Web UX

## Top-level nav

Add `Projects` alongside `Now`, `Inbox`, and `Threads`.

## Projects index layout

Left/center split or stacked responsive layout:

- project list / filters
- selected project summary
- active health indicators

## Project detail layout

Recommended four-column mental model rendered responsively:

- work
- chats
- queue
- controls

On narrower screens, collapse to sections/tabs.

## Interaction rules

- optimistic UI only where rollback is well-defined,
- explicit unsynced badges for queued/manual states,
- visible source badges (`todoist`, `vel`, `claude`, etc.),
- per-action provenance where mutation affects external systems.

## CLI UX

Plain output first:

```bash
vel projects list
vel projects show vel
vel projects task add vel --title "write projects spec" --tag planning --tag vel
vel projects chat queue vel codex-session-1 --message "update the migration and API types"
vel projects chat steer vel codex-session-1 --message "respect repo truth and existing conventions"
```

Later, a TUI can consume the same APIs.

## Capability model

Not all external agents support all actions.

Each `AgentSessionRecord` should expose capability flags such as:

```rust
pub struct AgentSessionCapabilities {
    pub can_queue_message: bool,
    pub can_send_immediately: bool,
    pub can_apply_feedback: bool,
    pub can_change_settings: bool,
    pub can_read_transcripts: bool,
    pub requires_manual_dispatch: bool,
}
```

This avoids the pathetic UX of presenting buttons that are pure fiction.

## Rollout plan

### Phase 1

- project registry
- project workspace read APIs
- Todoist task creation/tagging/update write path
- web Projects page with work panel

### Phase 2

- agent sessions registry
- transcript-to-session mapping
- active chats panel
- queued message outbox

### Phase 3

- steering and feedback actions
- per-session settings controls
- websocket live updates
- CLI command family

### Phase 4

- richer adapters for dispatch into external systems
- smarter project association and confidence scoring
- synthesis-aware project health scoring

## Acceptance criteria

- A top-level `Projects` page exists in web.
- Project identity resolves deterministically from Todoist/commitments/transcripts/codex tags.
- User can create and tag Todoist-backed tasks from Vel.
- Project detail view shows active agent sessions from both Vel-native and external sources.
- User can queue messages and record steering/feedback for sessions.
- A shared API/view-model powers both web and CLI commands.
- Unsynced/manual-dispatch states are explicit, not hidden.
- Integration and UI tests cover the main read/write flows.

## Recommended file touch points

### Backend

- `crates/vel-api-types/src/lib.rs`
- `crates/vel-storage/src/db.rs`
- `crates/veld/src/app.rs`
- `crates/veld/src/routes/` (new `projects.rs`, `agent_sessions.rs`)
- `crates/veld/src/services/integrations.rs`
- `migrations/` (new project/session/outbox migrations)

### Frontend

- `clients/web/src/App.tsx`
- `clients/web/src/types.ts`
- `clients/web/src/data/resources.ts`
- `clients/web/src/components/` (new Projects page components)

### CLI

- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/` project command module(s)

## Ticket pack

See `docs/tickets/projects/`.
