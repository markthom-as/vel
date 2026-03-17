---
title: Vel Projects page spec
status: ready
owner: product+engineering
priority: P0
---

# Vel Projects page

## Summary
Vel needs a first-class Projects surface that lets the user operate on a project as a live working field rather than a loose pile of commitments, chats, and sync artifacts.

Companion backend planning for project registry metadata, normalized tags, routines, and dependency projection lives in [vel-project-operations-substrate-spec.md](vel-project-operations-substrate-spec.md).

The page should:
- show all projects in one place
- provide a Todoist-backed project/task view
- allow creating, tagging, and updating tasks from Vel
- show active agent/chat sessions for the project across Vel, Codex, Claude, OpenCode, and future sources
- allow launching supported agent runtimes on compatible Connect instances from the project workspace
- allow queueing messages, steering work, recording feedback, and changing per-session or per-project settings
- render in both a rich web surface and a CLI workspace mode using the same underlying contracts

This is not a generic PM suite. It is an operator cockpit for project continuity.

## Why this exists
Right now Vel has pieces of the substrate but not the project-shaped operator surface:
- commitments already carry a `project` field
- Todoist sync already materializes tasks into commitments
- transcript sync already ingests external assistant/chat artifacts into `assistant_transcripts`
- chat UI exists, but is centered on Vel-native conversations rather than project operations across agents
- settings and sync control planes exist, but not a unified project view

So the problem is not total absence. It is fragmentation.

## Product goals
- Make project state legible at a glance.
- Preserve a single actionable truth for tasks.
- Let the user steer multi-agent work without digging through disconnected tools.
- Keep the same mental model across web and CLI.
- Stay local-first and auditable.

## Non-goals
- Replacing Todoist as a full task system.
- Building a full Jira/Linear clone.
- Treating raw transcript rows as a sufficient agent/session model.
- Adding speculative autonomous execution without explicit control surfaces.

## Boundary decisions

### 1. Tasks remain commitment-backed
Vel already has a durable actionable object: `commitments`.

The Projects page should therefore treat **commitments as the canonical Vel task object**.

Todoist-backed tasks should remain write-through synchronized where possible:
- if a task originates in Todoist, Vel writes changes back to Todoist and mirrors the result into commitments
- if a task is created in Vel for a Todoist-backed project, Vel should create it in Todoist first when the integration is connected, then persist/update the mirrored commitment
- if Todoist is unavailable, the UI may support a local-only commitment path, but it must be explicit and visually marked

Do not introduce a parallel durable `Task` truth unless the repo later proves commitments are structurally insufficient.

### 2. Projects are a read/write registry, not just a free-text field
Today `commitments.project` is effectively a string bucket.
That is good enough for synthesis filters, but not good enough for a coherent operator surface.

Vel should add a first-class **project registry** that can:
- normalize slug vs display name
- map external source identifiers (Todoist project ids, transcript conversation hints, thread links)
- hold project-scoped settings and display preferences
- define project status and ordering

The registry becomes the canonical project directory, while `commitments.project` remains the lightweight foreign-key-like slug.

### 3. External assistant activity needs a first-class session model
`assistant_transcripts` is useful evidence, but it is not a workable operator abstraction.

The Projects page needs a first-class **agent session registry** for things like:
- active Codex workspace thread
- recent Claude conversation for this repo
- OpenCode run or queue
- Vel-native conversation

A session should capture source, title, state, recency, last message summary, queue depth, project association, and operator controls.

### 4. Web and CLI share one workspace contract
The same backend projection should feed:
- `/projects` rich web surface
- `vel project ...` workspace/TTY views

No split-brain UX contracts.

## User stories
- As a user, I can open Projects and immediately see my active projects and their task pressure.
- As a user, I can filter to one project and see all open tasks regardless of whether they originated locally or from Todoist.
- As a user, I can create a task and attach tags/metadata without leaving Vel.
- As a user, I can see which chats/agents are actively working on a project.
- As a user, I can queue a message to a project-linked session instead of hunting through external tabs.
- As a user, I can leave steering/feedback for a specific agent session and audit what happened.
- As a user, I can use a CLI workspace instead of the web page and keep the same project model.

## Information architecture

### Projects index
Shows all projects with compact operational summaries.

Per project card/row:
- display name
- slug
- status (`active`, `paused`, `archived`, `proposed`)
- source badges (`todoist`, `manual`, `thread`, `transcript`)
- open task count
- overdue count
- due soon count
- waiting/blocked count when derivable
- active session count
- last activity at
- quick actions: open, add task, queue message, sync

### Project detail workspace
Tabs or panes:
1. **Overview**
2. **Tasks**
3. **Chats / Agents**
4. **Activity**
5. **Settings**

#### Overview
- project summary block
- top risks / nudges / suggestions when available
- top open commitments
- active sessions snapshot
- recent events / transcript evidence

#### Tasks
- list grouped by status or semantic buckets (`now`, `soon`, `waiting`, `done` optional)
- task creation composer
- tag editor
- project-specific filters (kind, tag, due state, source)
- quick actions: done, cancel, snooze later if semantics exist, retag, move project

#### Chats / Agents
- active session list
- per-session controls
- outbox / queued messages
- steering notes
- operator feedback history
- session settings

#### Activity
- merged timeline of task changes, transcript imports, run updates, and queued/sent operator messages

#### Settings
Project-scoped knobs such as:
- default task tags
- default source preference for new tasks (`todoist_first`, `local_only`, `ask`)
- default chat target for queued messages
- auto-link transcript project hints
- preferred layout (`rich`, `dense`, `cli_like`)

## Data model

### A. Project registry
Add a project registry table and typed DTOs.

Candidate fields:
```rust
pub struct ProjectRecord {
    pub slug: String,
    pub display_name: String,
    pub description: Option<String>,
    pub status: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub source_type: String,
    pub source_ref: Option<String>,
    pub todoist_project_id: Option<String>,
    pub default_task_tags: serde_json::Value,
    pub settings_json: serde_json::Value,
    pub metadata_json: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub archived_at: Option<OffsetDateTime>,
}
```

Notes:
- `slug` is the canonical project key used by commitments and APIs.
- `todoist_project_id` is nullable because not all projects come from Todoist.
- `settings_json` is acceptable initially; do not prematurely explode into dozens of columns.

### B. Commitment tagging
Current commitments have `metadata` but no first-class tags field.

Near-term approach:
- store tags under `metadata.tags: string[]`
- normalize in API/service layer
- provide explicit DTO fields so the UI does not spelunk raw JSON

Candidate transport additions:
```rust
pub struct CommitmentTaskData {
    pub commitment: CommitmentData,
    pub tags: Vec<String>,
    pub blocked_by: Vec<String>,
    pub waiting_on: Vec<String>,
    pub external_write_state: Option<String>,
}
```

Do not force a `commitment_tags` join table in the first slice unless query pressure actually demands it.

### C. Agent session registry
Add a durable session registry for project-linked work across assistant systems.

Candidate fields:
```rust
pub struct AgentSessionRecord {
    pub id: String,
    pub project_slug: String,
    pub source: String,          // vel | codex | claude | opencode | chatgpt | other
    pub source_ref: Option<String>,
    pub title: String,
    pub status: String,          // active | idle | blocked | done | archived
    pub mode: Option<String>,    // chat | code | research | review
    pub queue_depth: u32,
    pub last_message_at: Option<OffsetDateTime>,
    pub last_operator_action_at: Option<OffsetDateTime>,
    pub latest_summary: Option<String>,
    pub settings_json: serde_json::Value,
    pub metadata_json: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
```

This is separate from `assistant_transcripts`.
Transcripts remain evidence/history; sessions are the operator abstraction.

### D. Session outbox and feedback
Need explicit write surfaces for operator control.

Candidate tables:
- `agent_session_outbox`
- `agent_session_feedback`

Outbox row fields:
- id
- session_id
- project_slug
- payload_text
- payload_json
- state (`queued`, `sent`, `acked`, `failed`, `cancelled`)
- delivery_target
- created_at / sent_at / failed_at
- operator_note

Feedback row fields:
- id
- session_id
- project_slug
- feedback_type (`thumbs_up`, `thumbs_down`, `steer`, `constraint`, `settings_change`)
- content
- payload_json
- created_at

## Projection model
The frontend should not assemble this page from 14 independent ad hoc fetches like a raccoon building a radio.

Add a backend **project workspace projection**.

Candidate response:
```rust
pub struct ProjectWorkspaceData {
    pub project: ProjectData,
    pub summary: ProjectSummaryData,
    pub tasks: Vec<ProjectTaskData>,
    pub sessions: Vec<AgentSessionData>,
    pub queued_messages: Vec<AgentSessionOutboxData>,
    pub recent_activity: Vec<ProjectActivityEventData>,
    pub settings: ProjectSettingsData,
}
```

And index response:
```rust
pub struct ProjectIndexItemData {
    pub project: ProjectData,
    pub summary: ProjectSummaryData,
}
```

## APIs
Recommended routes:

### Projects
- `GET /v1/projects`
- `POST /v1/projects`
- `GET /v1/projects/:slug`
- `PATCH /v1/projects/:slug`
- `GET /v1/projects/:slug/workspace`

### Tasks within projects
- `POST /v1/projects/:slug/tasks`
- `PATCH /v1/projects/:slug/tasks/:id`
- `POST /v1/projects/:slug/tasks/:id/done`
- `POST /v1/projects/:slug/tasks/:id/cancel`
- `POST /v1/projects/:slug/tasks/:id/tags`

These should map back to commitments.
The `/tasks` naming is a surface affordance; backend ownership remains commitments.

### Agent sessions
- `GET /v1/projects/:slug/sessions`
- `POST /v1/projects/:slug/sessions`
- `PATCH /v1/projects/:slug/sessions/:id`
- `POST /v1/projects/:slug/sessions/:id/queue-message`
- `POST /v1/projects/:slug/sessions/:id/feedback`
- `POST /v1/projects/:slug/sessions/:id/settings`

### Connect-backed launch integration
Projects should also expose a launch path for live external agent runtimes on compatible Connect instances.

Minimum launch flow:
- choose project
- choose Connect instance
- choose supported runtime (`codex`, `copilot_agent`, `cursor_agent`, `claude_code`, `opencode`, `gemini_cli`, future runtimes)
- provide prompt / task / repo context
- create linked session record immediately, then reconcile live runtime state

The Projects page must not treat launched sessions as opaque external tabs. They should come back into the same session list, outbox, steering, and activity model.

See [vel-connect-agent-launch-spec.md](vel-connect-agent-launch-spec.md).

### Web transport equivalents if needed
If chat surfaces remain under `/api`, mirror the necessary routes there or keep the web client using `/v1` for this feature. Pick one convention and document it.

## Todoist behavior

### Read path
When Todoist is connected:
- existing sync continues to ingest Todoist tasks into commitments
- project registry should hydrate/refresh Todoist project mappings
- project summaries should expose Todoist connectivity and last sync state

### Write path
For Todoist-backed projects:
1. create/update task in Todoist
2. persist or reconcile commitment mirror locally
3. emit event + websocket update

If Todoist write fails:
- do not silently pretend success
- show failed write state in UI
- allow retry from activity/outbox-like affordance

### Tags
If Todoist labels exist, map them into Vel task tags.
Vel tags should preserve round-trip fidelity where possible.
If exact round-trip is not possible, keep both:
- normalized `tags: []`
- `metadata.todoist_labels_raw`

## Active chats / sessions UX
Each session card should show:
- source badge
- connect instance badge when session is instance-backed
- title
- active/idle/blocked status
- mode
- queue depth
- last activity timestamp
- latest summary or last message preview
- controls: open, queue message, steer, feedback, settings

For launchable sessions, add:
- launch/open-native-surface affordance
- instance/runtime metadata
- explicit "host agent linked" state when the main Vel host agent is supervising the session

### Queue message
Queueing is explicit. It does not imply immediate delivery unless the source adapter supports it.

States:
- queued
- sent
- acked
- failed
- cancelled

If an external source is not directly writable yet, the outbox still has value as a local operator queue.
That is not a bug; it is honest state.

### Steer
Steering is structured operator input attached to a session and optionally a project.
Examples:
- “stay within repo boundaries; no new crates unless justified”
- “prefer Todoist as task authority for this project”
- “summaries should include unresolved risks”

This should be stored durably, not vanish into vibes.

### Feedback
Feedback should support:
- quick binary reactions
- structured notes
- “apply as sticky preference” option where appropriate

### Settings
Per-session settings examples:
- verbosity
- risk tolerance
- autonomy level
- preferred artifact format
- model/backend profile

## Web UI design

### Navigation
Add `Projects` to the main shell navigation alongside Now / Inbox / Threads.

### Layout
Desktop split recommended:
- left rail: project list and filters
- center: current project workspace
- right rail: project context / activity / settings drawer depending on mode

### Key states
- no projects
- Todoist disconnected
- empty project with no tasks but active sessions
- active project with tasks but no sessions
- external sessions present but adapter read-only
- sync/write failure states

### Realtime
Use websocket invalidation for:
- project summary changes
- task mutations
- session state changes
- outbox state changes

Add specific event types rather than forcing the client to treat every change as generic chat noise.

## CLI workspace
Add a project-centric command surface.

Examples:
```bash
vel project list
vel project open vel
vel project tasks vel
vel project add-task vel "write project page spec" --tag docs --tag ui
vel project launch-agent vel --instance laptop-west --runtime codex "refactor workspace projection service"
vel project queue vel codex "refactor workspace projection service"
vel project steer vel codex "keep transport DTOs in vel-api-types only"
vel project feedback vel claude --type thumbs_down --note "too hand-wavy"
```

### CLI modes
1. **Command mode** for quick one-shot actions
2. **Workspace mode** for a richer TUI-like overview

Workspace mode should render:
- project summary
- top tasks
- active sessions
- queued items
- recent activity

Do not create a separate data model for CLI output.

## Events and auditability
Emit durable events for:
- project created/updated/archived
- project task created/updated/done/cancelled/tagged
- agent session created/updated
- outbox item queued/sent/acked/failed/cancelled
- feedback recorded
- steering/settings updated

This feature should be unusually inspectable because otherwise multi-agent orchestration turns into occult bookkeeping.

## Suggested implementation shape

### Backend
- `vel-storage`: new tables + queries
- `vel-api-types`: DTOs for project workspace/session/outbox
- `veld/services/projects.rs`: projection, mutation orchestration, Todoist write-through logic
- `veld/routes/projects.rs`: project/session/task APIs

### Web
- `clients/web/src/components/ProjectsPage.tsx`
- `clients/web/src/components/projects/*`
- shared query resources and websocket invalidation wiring

### CLI
- extend `vel-cli` with `project` subcommands and workspace rendering

## Rollout plan
1. Project registry + read-only project index/workspace projection
2. Task creation/tagging using commitment-backed semantics
3. Todoist write-through for project-backed task mutations
4. Agent session registry + read-only sessions view
5. Outbox / steering / feedback control plane
6. Web projects page
7. CLI project workspace
8. Realtime hardening and docs

## Acceptance criteria
- There is a first-class Projects navigation surface.
- Project identity is registry-backed, not only free-text.
- Tasks shown in Projects are commitment-backed and test-covered.
- Todoist-backed projects support task create/update with explicit failure semantics.
- Active assistant work is represented as sessions, not raw transcript rows alone.
- The operator can queue messages, record feedback, and steer sessions.
- Web and CLI consume the same project workspace contract.
- Events/docs/tests are updated.

## Implementation tickets
See [docs/tickets/projects/](../tickets/projects/README.md).
