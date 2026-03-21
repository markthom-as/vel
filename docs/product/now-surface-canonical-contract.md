---
title: Canonical Now Surface Contract
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-21
updated: 2026-03-21
keywords:
  - now
  - cross-platform
  - threads
  - tasks
  - voice
  - mesh
index_terms:
  - now surface
  - now contract
  - cross-platform parity
  - apple watch reduced now
related_files:
  - docs/MASTER_PLAN.md
  - docs/product/mvp-operator-loop.md
  - docs/product/now-inbox-threads-boundaries.md
  - docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - crates/vel-api-types/src/lib.rs
summary: Canonical cross-platform Now surface contract for the post-v0.2 execution-first surface and its thread-backed continuity rules.
---

# Purpose

Define the canonical behavior of the `Now` surface across:

- web
- iPhone
- iPad
- Mac
- Apple Watch (reduced)

This document is product authority for the `Now` surface after milestone `v0.2`.

It defines:

- the visible structure of `Now`
- which interactions stay inline versus escalate to `Threads`
- what parity means across clients
- what reduced-watch support looks like

This is a strict contract. Shells may adapt density and layout, but must not invent alternate product behavior.

# Product Rule

`Now` is an execution-first compressed operational surface.

It is not a dashboard and not a second inbox.

The backing principles are:

- `Threads` is the canonical continuity layer
- `task` is the canonical work object
- `day` is the canonical context container
- backend-owned Rust logic defines truth
- shells reflect that truth without re-deriving policy

# Header

## Title

- default: `{FirstName}'s Now`
- source: `user.first_name`
- fallback: `Now`
- configurable via governed settings: `settings.display.now_title`

## Header Icon Bar

### Purpose

- primary: status and filtering
- secondary: global system controls

### Allowed global actions in v1

- `force_refresh_all`
- `agents_play_pause`

### Minimum buckets

- `threads_by_type`
- `needs_input`
- `new_nudges`
- `search_filter`

### Interaction rules

- tapping a bucket opens `Threads` filtered to that bucket
- opening a bucket must not mutate read/open state
- counts show only when `count > 0` by default
- urgency uses subtle visual signal for new urgent items or escalations

### Sync / offline indicator

Header must surface:

- sync/offline state
- last sync timestamp when offline
- queued local-write count when non-zero

Tapping the sync/offline indicator opens settings or sync details.

# Top Status Row

The densest row on the surface is:

`[date] | [time] | [context_label] | [elapsed_time]`

Rules:

- fields never collapse
- empty states render explicit fallbacks
- time refreshes every 60 seconds
- context updates on relevant state change

Context resolution priority:

1. active started task
2. active calendar event
3. upcoming event within 60 minutes
4. `No active context`

Elapsed time source priority:

1. task timer
2. event start
3. inferred work session

Fallback:

- `No active task`

# Context One-Liner

## Source priority

1. backend LLM summary
2. deterministic local summarizer

The deterministic fallback is required and must include:

- current context
- upcoming commitments
- unresolved nudges

## Update rules

- update immediately on relevant state change
- background refresh every 5 minutes

## Interaction

Tap opens a thread using this priority:

1. object `primary_thread_id`
2. active container thread
3. day thread
4. create a thread if none exists

`Now` should not normally render a blank one-liner.

# Thread Model

`Thread` is a first-class object with:

- `thread_id`
- lifecycle
- metadata
- participants
- linked artifacts and actions
- optional embedded LLM chat content
- optional embedded human-chat snippets

Objects such as `task`, `day`, and `event` may nominate a `primary_thread_id`.

When opening from `Now`, use this priority:

1. object `primary_thread_id`
2. active container thread
3. day thread
4. create thread if none exists

`Now` is compressed operational state. `Threads` is full continuity, history, and context.

# Nudge / Action Bars

## Supported types

- `nudge`
- `needs_input`
- `review_request`
- `reflow_proposal`
- `thread_continuation`
- `trust_warning`
- `freshness_warning`

## Ordering

- backend-defined only in v1
- clients preserve order

## Allowed inline actions

- `accept`
- `deny`
- `snooze`
- `open_thread`
- `expand`
- `close`

Rules:

- `close` removes the bar from `Now` only and is not destructive
- `expand` routes to `Threads`; no inline expansion in v1
- all active bars are visible in v1
- snoozed bars leave the active stack and remain available through filtered thread views
- snoozed counts appear in the icon bar

### Snooze presets

- `3m`
- `5m`
- `10m`
- `15m`

### Lifecycle

Supported state fields:

- `expires_at`
- `stale_after`
- `resurface_policy`

Nudge lifecycle states:

- `new`
- `seen`
- `acted`
- `snoozed`
- `expired`

# Task Model

## Canonical rule

Everything is a `task`.

Commitments are a task subtype, not a separate top-level work object.

Use one canonical task model with `task_kind`:

- `task`
- `commitment`
- `routine`
- `reminder`

## Active work definition

Any backend-ranked work item may become current in this priority order:

1. explicit active task
2. active commitment
3. next ranked item

## Timing and activity sources

Supports:

- explicit task start
- inferred work session
- calendar-derived context
- future high-confidence activity sources

If inferred active-state conflict persists for 2-5 minutes:

- prompt the operator
- keep provisional-state markers
- avoid destructive automation

# Task List

The default structure is:

1. active task
2. next pending tasks
3. 1-2 most recent completed tasks
4. expand affordance

## Task row metadata minimum

- title
- project or source container
- due time
- status
- duration estimate
- thread link indicator
- source badge

## Interaction rules

- inline on `Now`: lightweight actions only
- deep input or editing: `Threads`
- completion is optimistic and reversible with visible undo
- completed items render crossed out, faded, and compact

## Empty state

When there are no tasks, `Now` prompts:

- `What are you working on?`

Offer:

- `start_task`
- `capture`
- `open_threads`
- `voice_input`

# Docked Capture / Voice Bar

The bottom input is a unified docked bar.

Rules:

- text routes automatically as capture/request input
- mic starts live transcription automatically
- backend decides final routing:
  - inline
  - inbox work
  - thread continuation or creation

## Live bubble

- show live transcription while speaking
- bubble may show compact response preview during processing
- bubble persists through processing
- bubble collapses only once routing is stable
- tapping the bubble opens the associated thread
- if no thread exists, create a thread artifact and open it

## Continuity rule

Every input creates a thread artifact.

## Closed v1 intent taxonomy

- `task`
- `question`
- `note`
- `command`
- `continuation`
- `reflection`
- `scheduling`

# Thread Routing Categories

`Now` uses filtered thread views over shared continuation metadata:

- `new_nudges`
- `needs_input`
- `snoozed`
- `review_apply`
- `reflow`
- `follow_up`

When content escalates to `Threads`, `Now` keeps a compact status chip or bar until resolution, snooze, or removal from `Now`.

Allowed direct actions from `Now`:

- `accept`
- `deny`
- `snooze`
- `complete`
- `undo`

Deep work still belongs in `Threads`.

# Offline / Sync Model

`Now` must preserve local-first behavior:

- cached or replicated data remains visible
- visual distinction exists for `synced`, `local_only`, and `stale`
- offline writes may include task create/complete, thread response, and supported local actions
- queued writes are inspectable through the sync/offline affordance
- failed actions are inspectable and retryable

Conflict policy in v1:

- simple atomic state: latest user input generally wins
- text/content edits: simple merge plus explicit conflict UI when ambiguous

# Day Object

`Day` is a first-class object.

Minimum fields:

- `day_id`
- timezone
- boundary timestamps
- `primary_thread_id`
- summary and status metadata
- links to tasks, events, and artifacts

Boundary rule:

- default rollover at local midnight in user timezone
- must support user-configurable day boundary
- future-compatible with sleep/wake anchored boundary

# Cross-Platform Parity

Parity means:

- same backend contract
- same information architecture
- same core actions
- density adapts by device

## Watch version

Watch is reduced, not divergent.

Minimum watch support:

- status row
- top nudge
- current task
- voice entry
- thread response or confirmation

Mac and iPad may use denser layouts and hotkeys, but interaction contract must still match.

# Visual Direction

The visual direction is:

- sparse
- spartan
- minimal

Rules:

- highest density stays in the top status row
- the rest of the surface stays clean and separated
- use mostly monochrome styling with selective color for severity, urgency, and sync state

# Governance

The surface is governed state, not shell-local convention.

Config must be:

- UI-editable
- user-editable in structured form
- versioned

Agents may propose config mutations, but user approval is required before apply in v1.

# Core Principles

- no dead states
- always show something meaningful
- everything reversible where possible
- threads are canonical continuity
- tasks are canonical work objects
- day is a canonical context container
- backend owns truth
- voice and text parity exist everywhere practical
- live sync is preferred, cached local resilience is required
