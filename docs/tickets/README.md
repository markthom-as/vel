---
title: Vel Ticket Packs
status: active
owner: agent
labels:
  - planning
  - tickets
created: 2026-03-15
---

# Vel — Ticket Pack Index

This directory is a **ticket inventory and triage index**.

It is **not** the canonical implementation ledger.

For repo-wide truth about what is actually implemented, partial, planned, or deferred, see:

- [docs/status.md](../status.md)

Use this file to answer:

- what ticket packs exist,
- what each pack is for,
- how mature/speculative each pack is,
- where to start reading.

Do **not** use this file alone to decide what has shipped.

## How to read this directory

Ticket packs fall into three practical maturity classes:

### 1. Active convergence work

These packs are closest to current implementation and are most suitable for near-term execution.

### 2. Near-term design / expansion

These packs are structured and useful, but they extend the system beyond the current convergence core.

### 3. Speculative / future architecture

These packs may contain good ideas, but they should not outrank current convergence, runtime correctness, or repo-truth work without an explicit decision.

## Pack Inventory

### Active convergence work

#### 1. Chat Interface (`001–036` in this directory)

Sequenced tickets for the Vel chat interface: conversations, messages, interventions, inbox, provenance, settings, tests, and richer chat interactions.

- **Files:** `001-initialize-monorepo.md` … `036-rich-chat-interactions-and-markdown-rendering.md`
- **Use when:** working on the current web/chat surface or reconciling the shipped chat stack with remaining polish.
- **Caution:** implementation truth for chat lives in [docs/status.md](../status.md) and chat-specific detail docs, not in ticket completion vibes.

#### 2. Repo feedback (`repo-feedback/`)

Architecture and convergence tickets derived from repo review: evaluate/read boundary, inference reducers, risk authority, nudge lifecycle, API contracts, web state management, CI, docs rationalization.

- **Index:** [repo-feedback/README.md](repo-feedback/README.md)
- **Tickets:** `001-enforce-evaluate-read-boundary.md` … `009-rationalize-docs-status-and-implementation-roadmap.md`
- **Use when:** tightening current architecture, reducing drift, or choosing the highest-value cleanup work.
- **Caution:** some tickets may be partly implemented already; verify against [docs/status.md](../status.md).

#### 3. Vel docs reconciliation (`vel-docs/`)

Documentation quality tickets: canonical status, API/doc alignment, README refresh, doc-drift guardrails.

- **Tickets:** `VEL-DOC-001-canonical-status-ledger.md` … `VEL-DOC-007-doc-drift-guardrails.md`
- **Use when:** improving repo truth, doc authority, and implementation/status consistency.

### Near-term design / expansion

#### 4. Agentic engineering (`agentic/`)

Current-state pack regenerated from the live repo snapshot. Focus: agentic dev loop, knowledge hoard, walkthroughs, CI guardrails, inference refactor, suggestion loop hardening, example/skill extraction.

- **Index:** [agentic/000_INDEX.md](agentic/000_INDEX.md)
- **Tickets:** `001_agentic_bootstrap_first_run_the_tests.md` … `010_record_example_and_skill_extraction_cli.md`

#### 5. Agent runtime (`agent-runtime/`)

Agent runtime spec and tickets: runtime skeleton, executor integration, memory contracts, introspection HUD, replay and reflection.

- **Spec:** [docs/specs/vel-agent-runtime-spec.md](../specs/vel-agent-runtime-spec.md)
- **Tickets:** `TICKET-001-runtime-skeleton.md` … `TICKET-005-replay-reflection.md`
- **Related specs:** [vel-cognitive-loop-spec.md](../specs/vel-cognitive-loop-spec.md), [vel-stavrobot-integration-spec.md](../specs/vel-stavrobot-integration-spec.md)

#### 6. iOS/watch monorepo (`ios-watch-monorepo/`)

Tickets for Apple platform work inside the Vel monorepo under `clients/apple`: bootstrap, shared contracts, sync spine, iOS shell, reminders/meds/pre-meeting, notifications, watch quick actions, widgets/complications, voice capture, design system, integration tests, CI and boundary rules.

- **Index:** [ios-watch-monorepo/README.md](ios-watch-monorepo/README.md)
- **Tickets:** `00-epic-apple-platform-monorepo.md`, `01-apple-monorepo-bootstrap.md` … `13-monorepo-boundary-rules.md`

#### 7. Visual interface (`visual-interface/`)

Affect-driven visual system: affect core, morphology mapper, sync protocol, debug harness, web renderer, watch basis state, phone-watch sync, QA/performance, runtime event integration, polish/presets/capture.

- **Tickets:** `001-foundation-affect-core.md` … `010-polish-presets-and-capture.md`
- **Docs:** [docs/visual-interface/](../visual-interface/) and [docs/specs/visual-interface-README.md](../specs/visual-interface-README.md)
- **Packages:** repo root `packages/` (vel-affect-core, vel-visual-morphology, vel-protocol, vel-render-web, vel-render-watch)

#### 8. Uncertainty & clarification (`uncertainty/`)

Uncertainty as first-class runtime: domain model, confidence scoring, clarification policy engine, ledger persistence, clarification resolvers, uncertainty panel, assumption review, telemetry/calibration, output contract.

- **Spec:** [docs/specs/vel-uncertainty-architecture-spec.md](../specs/vel-uncertainty-architecture-spec.md)
- **Tickets:** `TICKET-001-uncertainty-domain-model.md` … `TICKET-012-agent-output-contract-update.md`

#### 9. Adaptive configuration (`adaptive-configuration/`)

Adaptive settings and effective config behavior: typed settings, dynamic policy-driven overrides, deterministic merge, explainability, auditability, and runtime profiles.

- **Spec:** [docs/specs/vel-adaptive-configuration-spec.md](../specs/vel-adaptive-configuration-spec.md)
- **Tickets:** `001-schema-and-migrations.md` … `010-client-sdk-and-surface-wiring.md`

#### 10. Metadata enrichment (`metadata-enrichment/`)

Metadata hygiene across integrated sources: schema and domain, gap detection, enrichment candidates, source adapter writeback, consent/risk controls, and review workflows.

- **Spec:** [docs/specs/vel-metadata-enrichment-spec.md](../specs/vel-metadata-enrichment-spec.md)
- **Tickets:** `001-schema-and-domain-model.md` … `012-tests-fixtures-and-rollout.md`

#### 11. Self-knowledge system (`self-knowledge/`)

Documentation and source-code awareness: repo indexer, self-knowledge graph, evidence-backed claims, doc/code drift detection, navigation APIs, system map, change hotspots, maintenance loop.

- **Spec:** [docs/specs/vel-self-knowledge-system-spec.md](../specs/vel-self-knowledge-system-spec.md)
- **Tickets:** `SK-001-knowledge-indexer.md` … `SK-010-maintenance-automation-loop.md`

### Speculative / future architecture

#### 12. Context reasoning (`context-reasoning/`)

Context/decision tickets: belief store, inference engine, inspector UI, decision trace logging, explanation UI, feedback learning loop, belief expiration, confidence calibration, introspection report.

- **Spec:** [docs/specs/vel-context-decision-spec.md](../specs/vel-context-decision-spec.md)
- **Tickets:** `TICKET-001-context-belief-store.md` … `TICKET-010-introspection-report.md`
- **Use with caution:** this pack is conceptually rich, but may overlap with already-implemented current-context and explainability systems. Validate boundaries against [docs/status.md](../status.md) before execution.

#### 13. Task HUD (`task-hud/`)

Task HUD subsystem: task core crate, DB schema/migrations, actions engine, ranking engine, HUD policy, view model, desktop HUD UI, inference engine, ritual tasks, risk integration, voice bridge, glance API, ambient mode, AR protocol spec.

- **Spec:** [docs/specs/vel-task-hud-spec.md](../specs/vel-task-hud-spec.md)
- **Tickets:** `01-task-core-crate.md` … `14-ar-hud-protocol-spec.md`
- **Use with caution:** this pack introduces a new task-centric subsystem and should be reconciled carefully with existing commitments, nudges, threads, and risk semantics first.

#### 14. Orchestration Navs (`orchestration/`)

Core orchestration stack for Nav-based execution: task model, Nav trait, capability model, Nav registry, delegation engine, context scoping, result integration, persistent task store, trust profiles, reflection, observability, initial Nav implementations.

- **Tickets:** `001_task_model.md` … `012_initial_navs.md`

#### 15. Self-modification system (`vel-self-modification/`)

Governed self-modification pipeline: protected surface registry, patch proposal schema, self-improvement service skeleton, change ledger, validation orchestrator, sandbox execution, rollback control, autonomy budgets, rollout support, metrics, constitutional workflow.

- **Index:** [vel-self-modification/_ticket-index.md](vel-self-modification/_ticket-index.md)
- **Tickets:** `VSM-001-protected-surface-registry.md` … `VSM-020-constitutional-change-workflow.md`

#### 16. iOS/watch standalone (`ios-watch/`)

Tickets for iOS + watchOS as a separate repo/workspace (`vel-apple`): bootstrap, shared models/API, app shell, timeline/check-in/reminder flows, background refresh, widgets, voice, offline-first sync, privacy/observability, integration roadmap.

- **Index:** [ios-watch/README.md](ios-watch/README.md)
- **Tickets:** `TKT-001-apple-platform-bootstrap.md` … `TKT-013-apple-integration-roadmap.md`
- **Use with caution:** this pack assumes a separate-repo stance that may diverge from current same-repo Apple bootstrap guidance.

#### 17. Predicate system (`predicate-system/`)

Predicate/rule-oriented architecture tickets for store, rule engine, and observation ingest.

- **Tickets:** `VEL-201-predicate-store.md` … `VEL-203-observation-ingest.md`

## Product spec pack

Structured product/surface/engine specs live in [docs/product-spec-pack/](../product-spec-pack/) with a guide at [docs/product-spec-pack/README.md](../product-spec-pack/README.md).

## Chat file list

For convenience, the chat pack in this directory includes:

- `001-initialize-monorepo.md`
- `002-configure-tooling.md`
- `003-create-rust-crates.md`
- `004-implement-core-id-types.md`
- `005-implement-message-domain-model.md`
- `006-implement-intervention-model.md`
- `007-create-sqlite-migration-system.md`
- `008-implement-initial-database-schema.md`
- `009-implement-conversation-repository.md`
- `010-implement-message-repository.md`
- `011-implement-intervention-repository.md`
- `012-implement-event-log-repository.md`
- `013-create-axum-server-skeleton.md`
- `014-implement-conversation-api.md`
- `015-implement-message-api.md`
- `016-implement-inbox-api.md`
- `017-implement-intervention-actions-api.md`
- `018-implement-websocket-server.md`
- `019-broadcast-message-and-intervention-events.md`
- `020-initialize-react-client.md`
- `021-build-app-shell.md`
- `022-implement-conversation-list.md`
- `023-implement-thread-view.md`
- `024-implement-message-composer.md`
- `025-implement-card-renderer.md`
- `026-implement-inline-actions.md`
- `027-build-inbox-view.md`
- `028-implement-context-panel.md`
- `029-implement-provenance-api.md`
- `030-implement-provenance-drawer.md`
- `031-implement-settings-api.md`
- `032-implement-settings-ui.md`
- `033-add-seed-data-script.md`
- `034-add-backend-tests.md`
- `035-add-frontend-tests.md`
- `036-rich-chat-interactions-and-markdown-rendering.md`

## Status words used inside ticket files

Individual ticket files may use local workflow labels such as:

- `todo`
- `in_progress`
- `blocked`
- `review`
- `done`

Those labels are useful within a pack, but they do **not** override [docs/status.md](../status.md).
