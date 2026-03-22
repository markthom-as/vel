# Workflow Development Phases (MVP-first)

## Phase 0 — Design groundwork

Goal: define the substrate so the implementation does not immediately become cursed.

Deliverables:

- workflow package spec (`workflow.yaml`)
- JSON Schemas for workflow manifest and run record
- explicit separation of tools / skills / workflows
- runtime state model and step type model
- trigger taxonomy and normalized trigger payload spec
- context binding rules for project/task/nudge/thread
- policy requirements and confirmation model

Exit criteria:

- docs approved
- schema linting works
- two example workflows authored on paper

## Phase 1 — Workflow MVP

Goal: make workflows real with the minimum architecture that is actually useful.

Scope:

- filesystem registry for workflow packages
- manifest parsing + validation
- manual trigger only
- sequential steps only
- step types: `skill`, `hook`, `gate`, `emit`
- typed context packet input
- CLI surface: `list`, `inspect`, `run`, `test`
- run logs + artifacts
- checkpointing between steps
- basic approval gate for write actions

Not yet:

- complex branching
- arbitrary event triggers
- cron scheduler
- parallel fanout
- multi-run recovery semantics beyond simple resume

Use cases to support:

1. Run a morning orientation workflow manually from CLI or UI
2. Run a thread follow-up workflow in an existing thread context
3. Run a task metadata enrichment workflow with approval before writes

Exit criteria:

- at least three working example workflows
- runs are resumable after restart at step boundaries
- audit log is inspectable
- workflow outputs are linked to context entities

## Phase 2 — Automation triggers and scheduling

Goal: let workflows fire without manual initiation.

Scope:

- scheduled triggers (cron/interval)
- durable scheduler
- event bus integration for domain events
- dedupe keys and cool-downs
- quiet hours / DND
- trigger preview and last-run UI
- replay trigger for debugging

Use cases:

- weekday morning orientation at 8:00 local time
- run follow-up workflow when thread idle > N hours
- run overdue rescue workflow when task becomes overdue

Exit criteria:

- scheduled jobs survive restart
- event triggers can be tested in development
- deduping prevents workflow storms

## Phase 3 — Conditional routing and graph execution

Goal: move from simple sequences to real orchestration.

Scope:

- DAG/graph execution
- branch conditions
- success/failure edges
- retries and compensating actions
- reusable sub-workflows
- workflow calling workflow
- concurrency controls

Use cases:

- if no calendar events, skip reconcile step
- if confidence low, route to human review
- if thread already exists, resume it; else create one

Exit criteria:

- branch traces visible in logs
- graph validation catches cycles/invalid refs
- sub-workflow composition works

## Phase 4 — Deep context intelligence and policy

Goal: make workflow decisions more context-aware and safer.

Scope:

- policy engine integration for approval rules by action type
- confidence thresholds and escalation
- richer context resolver strategies
- state/condition triggers
- project/task/nudge/thread lifecycle hooks
- partial replay from checkpoint

Use cases:

- if project is in quiet mode, suppress nudges
- if task is high-risk, require stricter confirmation
- if thread is unresolved and deadline near, escalate from nudge to task

## Phase 5 — Ecosystem maturity

Goal: make workflows distributable, inspectable, and shareable.

Scope:

- versioned registries
- signing/trust model
- import/export adapters
- visual workflow inspector/editor
- performance budgets and simulation mode
- governance dashboards

This is later. Do not start here unless you want to major in platform procrastination.
