# Vel Web Operator Surface Spec

Status: canonical planning spec for web and global operator UX/UI work
Audience: coding agents and contributors implementing Vel's web/operator surfaces
Purpose: consolidate the overlapping web, UX/UI, and operator-surface architecture docs into one execution-oriented spec with clear boundaries and parallel work lanes

This document does not change shipped behavior by itself. Use [docs/status.md](../status.md) for implementation truth.

## 1. Why this exists

Vel's web and operator UX planning is currently spread across multiple useful but overlapping documents:

- [vel-ui-v4-spec.md](vel-ui-v4-spec.md)
- [vel-now-awareness-and-ux-repair-spec.md](vel-now-awareness-and-ux-repair-spec.md)
- [vel-projects-page-spec.md](vel-projects-page-spec.md)
- [vel-operator-cockpit-spec.md](vel-operator-cockpit-spec.md)
- [vel-system-architecture-map.md](vel-system-architecture-map.md)
- [vel-distributed-and-ambient-architecture-spec.md](vel-distributed-and-ambient-architecture-spec.md)
- [vel-chat-interface-implementation-brief.md](vel-chat-interface-implementation-brief.md)
- [docs/chat-interface-status-and-outstanding.md](../chat-interface-status-and-outstanding.md)
- [docs/tickets/repo-audit-hardening/007-frontend-surface-decomposition-plan.md](../tickets/repo-audit-hardening/007-frontend-surface-decomposition-plan.md)
- [docs/tickets/repo-feedback/006-harden-web-client-state-management-and-realtime-sync.md](../tickets/repo-feedback/006-harden-web-client-state-management-and-realtime-sync.md)

That spread is no longer helping. The repo needs one canonical source for:

- the global web surface map
- the UX role of each operator surface
- the web/client architectural boundaries
- the shared data and realtime model
- the execution sequence for backend, web, and docs work

## 2. Scope

This spec governs the global web/operator surface and the shared UX rules that cut across those surfaces.

Included:

- web app shell and global navigation
- `Now`, `Inbox`, `Threads`, `Suggestions`, `Projects`, `Stats`, and `Settings`
- shared context/explain/debug UX patterns
- shared query, decoder, cache, and realtime behavior for the web client
- read-model and view-model contracts needed to support those surfaces
- operator-facing terminology, freshness, degraded states, and error handling

Excluded:

- Apple-specific client UX beyond shared contract assumptions
- voice-specific interaction details
- visual embodiment work under `visual-interface/`
- speculative multi-client swarm behavior beyond the web shell's current contract needs
- domain semantics owned by `vel-core`

## 3. Authority and relationship to other docs

Use this document as the canonical planning and execution spec for web/global operator UX work.

Use these other docs as bounded inputs:

- [vel-ui-v4-spec.md](vel-ui-v4-spec.md): screenshot-derived IA and role clarity
- [vel-now-awareness-and-ux-repair-spec.md](vel-now-awareness-and-ux-repair-spec.md): Now-specific trust and freshness requirements
- [vel-projects-page-spec.md](vel-projects-page-spec.md): Projects workspace model
- [vel-operator-cockpit-spec.md](vel-operator-cockpit-spec.md): operator legibility principles
- [vel-system-architecture-map.md](vel-system-architecture-map.md): runtime flow and subsystem roles
- [vel-distributed-and-ambient-architecture-spec.md](vel-distributed-and-ambient-architecture-spec.md): client versus daemon responsibility split

Interpretation rule:

- if these sources disagree on web/global UX direction, this spec wins
- if this spec appears to conflict with shipped runtime behavior, [docs/status.md](../status.md) wins

## 4. Product position for the web surface

The web client is not a generic dashboard and not a generic chat shell.

It is Vel's primary operator surface for:

- seeing what matters now
- inspecting why Vel believes it
- steering suggestions and interventions
- operating project continuity
- managing integrations, loops, and system control

The web surface should feel like an operator console for continuity and daily orientation, not like a debug landfill or a general-purpose productivity suite.

## 5. Canonical global information architecture

The global web shell should expose exactly these top-level surfaces:

- `Now`
- `Inbox`
- `Threads`
- `Suggestions`
- `Projects`
- `Stats`
- `Settings`

Surface roles:

### 5.1 Now

Primary question:

- what should happen next?

Owns:

- summary state relevant to action right now
- next event and truly upcoming events
- prioritized actionable tasks and commitments
- concise attention/risk cues
- section-level freshness and degraded-state warnings

Does not own:

- full observability
- raw system dumps by default
- broad integration management
- duplicated context explanation rails

### 5.2 Inbox

Primary question:

- what needs acknowledgement, triage, or direct action?

Owns:

- proactive interventions
- pending operator actions
- action-first cards
- direct open-thread or open-project handoff when relevant

Does not own:

- long-lived continuity state
- project workspace behavior

### 5.3 Threads

Primary question:

- what conversations or continuity threads are active, blocked, or dormant?

Owns:

- persistent thread history
- conversation context
- thread-level provenance and operator actions
- continuity-oriented state, not only message chronology

Does not own:

- broad project task management
- system observability

### 5.4 Suggestions

Primary question:

- what steering change is Vel proposing, and should I accept it?

Owns:

- suggestion decision
- evidence and rationale
- accept/reject/modify actions

UX rule:

- decision first
- evidence second
- raw payload only on explicit inspection

### 5.5 Projects

Primary question:

- what is the state of this project, and how do I steer work across tasks and agent sessions?

Owns:

- project registry and index
- project-scoped task workspace
- project-scoped agent/session workspace
- project activity timeline
- project-specific settings

Boundary rule:

- commitments remain the canonical task object
- sessions remain the operator abstraction over transcript evidence

### 5.6 Stats

Primary question:

- can I trust the system, and what is it doing?

Owns:

- source health
- context formation and freshness
- loop/runtime behavior
- run and sync visibility
- system diagnostics that do not belong on `Now`

### 5.7 Settings

Primary question:

- how is Vel configured and controlled?

Owns:

- runtime/operator settings
- integration credentials and policy participation
- loop controls
- component toggles where they are true control, not passive observability

Does not own:

- broad passive diagnostics that belong in `Stats`

## 6. Global UX rules

### 6.1 One surface, one primary question

Every top-level page must answer one primary operator question clearly.

### 6.2 Debug is available but not ambient

Raw keys, payloads, IDs, and provenance-heavy internals should be available through explicit reveal patterns such as drawers, tabs, or debug modes.

They should not dominate the default scanning path.

### 6.3 Freshness is first-class

The UI must expose when data is:

- `fresh`
- `aging`
- `stale`
- `error`
- `disconnected`

Stale information may remain visible, but it must never masquerade as live.

### 6.4 Human labels by default

Default UI uses operator-facing labels.

Raw enum keys or internal names belong in debug views only.

### 6.5 Global time behavior is user-local

All web timestamps, day-boundary logic, and freshness messaging must resolve against the same effective IANA timezone used by the runtime settings.

### 6.6 Empty states must be truthful

The UI must distinguish between:

- no data exists
- no source is configured
- source is stale
- source failed
- user filtered everything out

### 6.7 Action beats ornament

Visual polish is welcome, but it cannot outrank:

- actionability
- inspectability
- trust
- recovery guidance

## 7. Canonical cross-surface patterns

### 7.1 Context inspection model

Any shared context panel or drawer should use explicit modes:

- `State`
- `Why`
- `Debug`

Mode meanings:

- `State`: what Vel currently believes
- `Why`: the reasons, evidence summaries, and causal explanation
- `Debug`: raw keys, IDs, payloads, and low-level inspection details

### 7.2 Reason stack pattern

Normal views should present one canonical reason stack, not duplicate explanation blocks across multiple columns.

### 7.3 Recovery guidance pattern

When a surface detects degraded or missing inputs, it should offer the next useful operator move:

- sync
- inspect settings
- inspect source status
- retry a run
- acknowledge that no data exists yet

### 7.4 Shared loading/error/empty scaffolds

The web shell should use common page-state components and conventions rather than letting every page invent new loading and failure behavior.

## 8. Web architecture boundaries

### 8.1 Runtime and domain ownership

`vel-core` and runtime services own:

- domain semantics
- inference/risk/policy logic
- durable state transitions
- view-model assembly that depends on domain knowledge

The web client owns:

- presentation
- local interaction state
- cache/query orchestration
- optimistic mutation handling
- route-to-surface composition

The web client must not become a shadow inference or policy layer.

### 8.2 Read-model rule

Top-level operator surfaces should prefer purpose-built read models over client-side stitching of unrelated endpoints.

Examples:

- `Now` should consume `GET /v1/now`, not compose itself from multiple explain endpoints
- `Projects` should consume a dedicated project workspace projection
- `Stats` should consume dedicated observability-oriented contracts

### 8.3 Thin route rule

Route handlers remain thin:

- parse request
- call service
- map response/error

Read-model shaping belongs in services and DTOs, not route glue and not ad hoc web-side transforms.

### 8.4 Transport DTO rule

Transport DTOs live in `vel-api-types`.

Web runtime decoders should be organized by transport domain, not one monolithic `types.ts` choke point.

Suggested split:

- chat + websocket
- now + context + stats
- suggestions + provenance
- settings + integrations + loops
- projects

### 8.5 Query/cache rule

The web client should use one shared query/cache model for all operator surfaces.

Requirements:

- stable query keys by domain
- centralized invalidation rules
- optimistic mutation reconciliation where user action latency matters
- focus/background refresh only where appropriate

### 8.6 Realtime rule

Realtime should update operator state without turning every page into bespoke websocket code.

Use a shared event ingestion layer that:

- decodes websocket events once
- maps them to query invalidation or targeted cache updates
- keeps optimistic actions reconcilable with server truth

## 9. Surface-specific architectural contracts

### 9.1 Now

Required backend contract:

- one coherent snapshot endpoint
- operator labels plus raw debug keys where needed
- freshness state by source
- prioritized tasks for the page's purpose

Required frontend behavior:

- one dominant action path
- section-level degraded-state warnings
- debug hidden by default

### 9.2 Suggestions

Required backend contract:

- decision payload
- rationale
- evidence summaries and inspectable linked objects

Required frontend behavior:

- accept/reject/modify without needing to parse JSON
- evidence accessible but secondary

### 9.3 Projects

Required backend contract:

- project registry and workspace projection
- commitment-backed task data with explicit task fields
- session registry and outbox/feedback surfaces

Required frontend behavior:

- project shell, tasks, sessions, and activity share one project mental model
- no split-brain between web and CLI contracts

### 9.4 Stats

Required backend contract:

- source health
- context formation inputs
- loop/runtime summaries
- operator-legible diagnostic models

Required frontend behavior:

- one observability home
- drill-down paths without polluting operational pages

### 9.5 Settings

Required backend contract:

- explicit setting values
- integration status and policy participation
- loop controls

Required frontend behavior:

- separate control from passive observation
- surface policy participation, not just whether credentials exist

## 10. Shared design and copy guidance

- Prefer dense, legible operator layouts over roomy consumer-dashboard chrome.
- Use vocabulary aligned with [docs/vocabulary.md](../vocabulary.md) where terms are canonically defined.
- Avoid CLI-centric prose in primary web UX unless the surface is explicitly teaching recovery to an operator.
- Prefer consistent section names and badge semantics across pages.

## 11. Parallel execution model

The work should be operated in parallel across four main lanes with minimal write-set overlap.

### Lane A: Shared contracts and shell architecture

Owns:

- app-shell IA and navigation
- transport decoder decomposition
- shared query/resource boundaries
- websocket/realtime event ingestion

### Lane B: Operational surfaces

Owns:

- `Now`
- shared context inspection patterns
- `Stats`
- `Settings` / integration control alignment

### Lane C: Cognitive and continuity surfaces

Owns:

- `Inbox`
- `Threads`
- `Suggestions`
- project-continuity UX rules shared outside project detail

### Lane D: Project workspace

Owns:

- project registry and projection contracts
- Projects page shell
- task workspace
- session workspace

### Lane E: Docs, tests, and rollout

Owns:

- regression coverage
- docs/status/ticket reconciliation
- historical packet demotion or cross-link cleanup

## 12. Consolidated execution order

1. establish the global shell and shared transport/query architecture
2. lock `Now`, context inspection, `Stats`, and `Settings` around purpose-built contracts
3. clean up `Inbox`, `Threads`, and `Suggestions` to match their explicit roles
4. build the project workspace on top of the shared web architecture
5. reconcile docs, rollout notes, and regression coverage so the new surface map stays durable

## 13. Exit criteria

This consolidation succeeds when:

- there is one canonical planning spec for web/global operator UX work
- there is one execution-grade ticket pack for parallel implementation
- each top-level web surface has a clear role and does not duplicate another surface's job
- the web client boundaries are explicit enough that new work stops re-inventing fetch, decoder, and debug patterns
- historical planning packets remain available for context but are no longer the default execution entrypoint
