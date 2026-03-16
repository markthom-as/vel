---
title: Vel Ticket Packs
status: todo
owner: agent
labels:
  - planning
  - tickets
created: 2026-03-15
---

# Vel — Ticket Packs

This directory holds sequenced implementation ticket packs for Vel.

## Packs

### 1. Chat Interface (this directory, 001–035)

Sequenced tickets for the Vel chat interface: conversations, messages, interventions, inbox, provenance, settings.

- **Files:** `001-initialize-monorepo.md` … `035-add-frontend-tests.md`
- **Index:** see list below.

### 2. Agentic engineering (`agentic/`)

Current-state pack regenerated from the live repo snapshot. Focus: agentic dev loop, knowledge hoard, walkthroughs, CI guardrails, inference refactor, suggestion loop hardening, example/skill extraction.

- **Index:** [agentic/000_INDEX.md](agentic/000_INDEX.md)
- **Tickets:** 001–010 in `agentic/` (bootstrap, knowledge hoard, walkthroughs, canonical examples, prompt library, CI guardrails, context reducer refactor, next-event/commitment selection, suggestion evidence/policy, record-example CLI).

### 3. iOS/watch monorepo (`ios-watch-monorepo/`)

Tickets for the Apple platform inside the Vel monorepo under `clients/apple`: epic, bootstrap, shared contracts, sync spine, iOS shell, reminders/meds/pre-meeting, notifications, watch quick actions, widgets/complications, voice capture, design system, integration tests, CI and boundary rules.

- **Index:** [ios-watch-monorepo/README.md](ios-watch-monorepo/README.md)
- **Tickets:** `00-epic-apple-platform-monorepo.md`, `01-apple-monorepo-bootstrap.md` … `13-monorepo-boundary-rules.md`.

### 4. iOS/watch standalone (`ios-watch/`)

Tickets for iOS + watchOS as a separate repo/workspace (`vel-apple`): bootstrap, shared models/API, app shell, today timeline, check-in/completion, meds/meeting-aware reminders, notifications/background refresh, watchOS quick actions, widgets/Live Activities/complications, voice capture, offline-first sync, privacy/observability, integration roadmap.

- **Index:** [ios-watch/README.md](ios-watch/README.md)
- **Tickets:** `TKT-001-apple-platform-bootstrap.md` … `TKT-013-apple-integration-roadmap.md`.

### 5. Visual interface (`visual-interface/`)

Affect-driven visual system: affect core, morphology mapper, sync protocol, debug harness, web renderer, watch basis state, phone–watch state sync, QA/performance, runtime event integration, polish/presets/capture.

- **Tickets:** `001-foundation-affect-core.md` … `010-polish-presets-and-capture.md`
- **Docs:** [docs/visual-interface/](../visual-interface/) and [docs/specs/visual-interface-README.md](../specs/visual-interface-README.md)
- **Packages:** repo root `packages/` (vel-affect-core, vel-visual-morphology, vel-protocol, vel-render-web, vel-render-watch).

### 6. Repo feedback (`repo-feedback/`)

Architecture and convergence tickets from repo review: evaluate/read boundary, inference reducers, risk as single authority, nudge lifecycle, API time fields, web client state, shell cleanup, CI, docs rationalization.

- **Index:** [repo-feedback/README.md](repo-feedback/README.md)
- **Tickets:** `001-enforce-evaluate-read-boundary.md` … `009-rationalize-docs-status-and-implementation-roadmap.md`

### 7. Context reasoning (`context-reasoning/`)

Context/decision spec and tickets: belief store, context inference engine, inspector UI, decision trace logging, explanation UI, debug inspector, feedback learning loop, belief expiration, confidence calibration, introspection report.

- **Spec:** [docs/specs/vel-context-decision-spec.md](../specs/vel-context-decision-spec.md)
- **Tickets:** `TICKET-001-context-belief-store.md` … `TICKET-010-introspection-report.md`

### 8. Agent runtime (`agent-runtime/`)

Agent runtime spec and tickets: runtime skeleton (spec loader, lifecycle, spawn validation, return contracts), executor integration (capability tokens, RPC, tool logging), memory contracts (typed read/write, provenance), introspection + HUD, replay + reflection.

- **Spec:** [docs/specs/vel-agent-runtime-spec.md](../specs/vel-agent-runtime-spec.md)
- **Tickets:** `TICKET-001-runtime-skeleton.md` … `TICKET-005-replay-reflection.md`
- **Related specs:** [vel-cognitive-loop-spec.md](../specs/vel-cognitive-loop-spec.md), [vel-stavrobot-integration-spec.md](../specs/vel-stavrobot-integration-spec.md)

### 9. Uncertainty & clarification (`uncertainty/`)

Uncertainty as first-class runtime: domain model, confidence scoring, clarification policy engine, ledger persistence, resolvers (user, agent, retrieval, validation), ask-before-acting preferences, uncertainty panel and clarification inbox, assumption review, telemetry/calibration, agent output contract.

- **Spec:** [docs/specs/vel-uncertainty-architecture-spec.md](../specs/vel-uncertainty-architecture-spec.md)
- **Tickets:** `TICKET-001-uncertainty-domain-model.md` … `TICKET-012-agent-output-contract-update.md`

### 10. Task HUD (`task-hud/`)

Task HUD subsystem: task core crate, DB schema/migrations, actions engine, ranking engine, HUD policy, HUD view model, desktop HUD UI, inference engine, ritual tasks, risk-engine integration, voice bridge, glance API for mobile/watch, ambient mode, AR HUD protocol spec.

- **Spec:** [docs/specs/vel-task-hud-spec.md](../specs/vel-task-hud-spec.md)
- **Tickets:** `01-task-core-crate.md` … `14-ar-hud-protocol-spec.md`

### 11. Orchestration Navs (`orchestration/`)

Core orchestration stack for Nav-based execution: task model, Nav trait, capability model, Nav registry, delegation engine, context scoping, result integration, persistent task store, Nav trust profiles, reflection engine, observability layer, initial Nav implementations.

- **Tickets:** `001_task_model.md` … `012_initial_navs.md`

### 12. Self-knowledge system (`self-knowledge/`)

Documentation and source-code awareness: repo indexer, self-knowledge graph, evidence-backed claims and confidence, doc/code drift detection, navigation APIs, system map + coverage CLI, reasoning context integration, self-awareness dashboard, git freshness and change hotspots, maintenance loop.

- **Spec:** [docs/specs/vel-self-knowledge-system-spec.md](../specs/vel-self-knowledge-system-spec.md)
- **Tickets:** `SK-001-knowledge-indexer.md` … `SK-010-maintenance-automation-loop.md`

---

## Product spec pack (docs)

Structured product/surface/engine specs are in **docs/product-spec-pack/** (architecture, surfaces, interaction, engines, design, flows). See [docs/product-spec-pack/README.md](../product-spec-pack/README.md).

---

## Chat Interface Pack (001–035) — file list

- `001-initialize-monorepo.md` — Initialize Vel Monorepo
- `002-configure-tooling.md` — Configure Development Tooling
- `003-create-rust-crates.md` — Create Rust Core Crates
- `004-implement-core-id-types.md` — Implement Core ID Types
- `005-implement-message-domain-model.md` — Implement Message Domain Model
- `006-implement-intervention-model.md` — Implement Intervention Model
- `007-create-sqlite-migration-system.md` — Create SQLite Migration System
- `008-implement-initial-database-schema.md` — Implement Initial Database Schema
- `009-implement-conversation-repository.md` — Implement Conversation Repository
- `010-implement-message-repository.md` — Implement Message Repository
- `011-implement-intervention-repository.md` — Implement Intervention Repository
- `012-implement-event-log-repository.md` — Implement Event Log Repository
- `013-create-axum-server-skeleton.md` — Create Axum Server Skeleton
- `014-implement-conversation-api.md` — Implement Conversation API
- `015-implement-message-api.md` — Implement Message API
- `016-implement-inbox-api.md` — Implement Inbox API
- `017-implement-intervention-actions-api.md` — Implement Intervention Actions API
- `018-implement-websocket-server.md` — Implement WebSocket Server
- `019-broadcast-message-and-intervention-events.md` — Broadcast Message and Intervention Events
- `020-initialize-react-client.md` — Initialize React Client
- `021-build-app-shell.md` — Build App Shell
- `022-implement-conversation-list.md` — Implement Conversation List
- `023-implement-thread-view.md` — Implement Thread View
- `024-implement-message-composer.md` — Implement Message Composer
- `025-implement-card-renderer.md` — Implement Card Renderer
- `026-implement-inline-actions.md` — Implement Inline Actions
- `027-build-inbox-view.md` — Build Inbox View
- `028-implement-context-panel.md` — Implement Context Panel
- `029-implement-provenance-api.md` — Implement Provenance API
- `030-implement-provenance-drawer.md` — Implement Provenance Drawer
- `031-implement-settings-api.md` — Implement Settings API
- `032-implement-settings-ui.md` — Implement Settings UI
- `033-add-seed-data-script.md` — Add Seed Data Script
- `034-add-backend-tests.md` — Add Backend Tests
- `035-add-frontend-tests.md` — Add Frontend Tests

## Status convention

- `todo`
- `in_progress`
- `blocked`
- `review`
- `done`

## Chat completion standard

Vel Chat V1 is ready when: conversations persist, structured cards render, inbox shows proactive interventions, actions mutate system state, provenance is visible, event log exists, realtime updates function.
