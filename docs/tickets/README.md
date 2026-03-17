---
title: Vel Ticket Packs
status: active
owner: agent
class: convergence
authority: design
status_model:
  - active
  - normalized
  - partial
  - archived
source_of_truth: docs/status.md
labels:
  - planning
  - tickets
created: 2026-03-15
updated: 2026-03-17
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

Shared pack schema:

- [docs/tickets/pack-schema.md](pack-schema.md)

## How to read this directory

This index is itself a schema-driven classification layer, not a shipped-behavior ledger.

Pack schema for this index:

- `class: convergence`
- `authority: design`
- `status_model: active | normalized | partial | archived`
- `source_of_truth: docs/status.md`

Entry criteria:

- use this file to decide which pack to consult,
- use this file to compare pack maturity and authority,
- use this file when multiple packs appear to overlap.

Overlap protocol:

- start from the pack `README.md` before drilling into ticket files
- prefer packs with explicit `Current status` or `Current focus` sections when multiple packs are adjacent
- use [repo-audit-hardening/README.md](repo-audit-hardening/README.md) for current repo-convergence work
- use caution notes in this index to avoid duplicating overlapping architecture lanes

Exit criteria:

- every actively used pack is classified by maturity and authority,
- the index points to the current normalization rules,
- pack readers know when to prefer `docs/status.md` instead.

Ticket packs fall into three practical maturity classes:

### 1. Active convergence work

These packs are closest to current implementation and are most suitable for near-term execution.

### 2. Near-term design / expansion

These packs are structured and useful, but they extend the system beyond the current convergence core.

### 3. Speculative / future architecture

These packs may contain good ideas, but they should not outrank current convergence, runtime correctness, or repo-truth work without an explicit decision.

## Pack Inventory

### Active convergence work

#### 1. Chat Interface (`001–037` in this directory)

Sequenced tickets for the Vel chat interface: conversations, messages, interventions, inbox, provenance, settings, tests, and richer chat interactions.

- **Files:** `001-initialize-monorepo.md` … `037-chat-remote-fallback-for-assistant-generation.md`
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

#### 4. Repo audit hardening (`repo-audit-hardening/`)

Audit-derived convergence pack for repo truth, hermetic tests, ticket-pack normalization, and big-picture architecture decomposition planning.

- **Index:** [repo-audit-hardening/README.md](repo-audit-hardening/README.md)
- **Spec:** [docs/specs/vel-repo-audit-hardening-spec.md](../specs/vel-repo-audit-hardening-spec.md)
- **Use when:** reconciling docs/tickets/specs/code state and turning audit findings into sequenced cleanup work.
- **Caution:** this pack is for convergence and modularization planning; use `docs/status.md` for shipped behavior.

Normalization note:

- this is the current reference pack for the shared schema in [pack-schema.md](pack-schema.md)

#### Web UI convergence (`web-ui-convergence/`)

Execution pack for the global web shell, shared operator UX rules, surface-specific read models, query/realtime architecture, and project workspace integration.

- **Index:** [web-ui-convergence/README.md](web-ui-convergence/README.md)
- **Spec:** [docs/specs/vel-web-operator-surface-spec.md](../specs/vel-web-operator-surface-spec.md)
- **Tickets:** `WUI-001-shell-ia-and-route-ownership.md` … `WUI-011-docs-tests-and-pack-reconciliation.md`
- **Use when:** changing the web shell, shared UI/state architecture, global operator UX, or the Projects web surface.
- **Caution:** this pack consolidates older `ui-v4`, `now-page-fixes`, `projects`, and frontend-cleanup packets for execution; use [docs/status.md](../status.md) for shipped behavior.

#### Client connect + sync milestone (`client-connect-sync/`)

Cross-pack execution index for the immediate milestone of getting clients connected to one authority node, bootstrapped, stale-aware, and operator-visible.

- **Index:** [client-connect-sync/README.md](client-connect-sync/README.md)
- **Spec:** [docs/specs/vel-client-connection-and-sync-milestone-spec.md](../specs/vel-client-connection-and-sync-milestone-spec.md)
- **Primary source tickets:** `APPLE-003`, `WUI-005`, `WUI-006`, `SWARM-009`
- **Use when:** prioritizing the concrete client connection/sync slice across daemon, Apple, web, and docs work.
- **Caution:** this pack consolidates execution for the immediate milestone; broader swarm replication, authority handoff, and distribution work remain in their existing packs/specs.

### Near-term design / expansion

#### 5. Agentic engineering (`agentic/`)

Current-state pack regenerated from the live repo snapshot. Focus: agentic dev loop, knowledge hoard, walkthroughs, CI guardrails, inference refactor, suggestion loop hardening, example/skill extraction.

- **Index:** [agentic/000_INDEX.md](agentic/000_INDEX.md)
- **Tickets:** `001_agentic_bootstrap_first_run_the_tests.md` … `010_record_example_and_skill_extraction_cli.md`

#### 6. Agent runtime (`agent-runtime/`)

Agent runtime spec and tickets: runtime skeleton, executor integration, memory contracts, introspection HUD, replay and reflection.

- **Spec:** [docs/specs/vel-agent-runtime-spec.md](../specs/vel-agent-runtime-spec.md)
- **Tickets:** `TICKET-001-runtime-skeleton.md` … `TICKET-005-replay-reflection.md`
- **Related specs:** [vel-cognitive-loop-spec.md](../specs/vel-cognitive-loop-spec.md), [vel-stavrobot-integration-spec.md](../specs/vel-stavrobot-integration-spec.md)

#### 7. iOS/watch monorepo (`ios-watch-monorepo/`)

Tickets for Apple platform work inside the Vel monorepo under `clients/apple`: bootstrap, shared contracts, sync spine, iOS shell, reminders/meds/pre-meeting, notifications, watch quick actions, widgets/complications, voice capture, design system, integration tests, CI and boundary rules.

- **Index:** [ios-watch-monorepo/README.md](ios-watch-monorepo/README.md)
- **Tickets:** `00-epic-apple-platform-monorepo.md`, `01-apple-monorepo-bootstrap.md` … `13-monorepo-boundary-rules.md`

#### 8. Visual interface (`visual-interface/`)

Affect-driven visual system: affect core, morphology mapper, sync protocol, debug harness, web renderer, watch basis state, phone-watch sync, QA/performance, runtime event integration, polish/presets/capture.

- **Tickets:** `001-foundation-affect-core.md` … `010-polish-presets-and-capture.md`
- **Docs:** [docs/visual-interface/](../visual-interface/) and [docs/specs/visual-interface-README.md](../specs/visual-interface-README.md)
- **Packages:** repo root `packages/` (vel-affect-core, vel-visual-morphology, vel-protocol, vel-render-web, vel-render-watch)

#### 9. Uncertainty & clarification (`uncertainty/`)

Uncertainty as first-class runtime: domain model, confidence scoring, clarification policy engine, ledger persistence, clarification resolvers, uncertainty panel, assumption review, telemetry/calibration, output contract.

- **Spec:** [docs/specs/vel-uncertainty-architecture-spec.md](../specs/vel-uncertainty-architecture-spec.md)
- **Tickets:** `TICKET-001-uncertainty-domain-model.md` … `TICKET-012-agent-output-contract-update.md`

#### 10. Adaptive configuration (`adaptive-configuration/`)

Adaptive settings and effective config behavior: typed settings, dynamic policy-driven overrides, deterministic merge, explainability, auditability, and runtime profiles.

- **Spec:** [docs/specs/vel-adaptive-configuration-spec.md](../specs/vel-adaptive-configuration-spec.md)
- **Tickets:** `001-schema-and-migrations.md` … `010-client-sdk-and-surface-wiring.md`

#### 11. Metadata enrichment (`metadata-enrichment/`)

Metadata hygiene across integrated sources: schema and domain, gap detection, enrichment candidates, source adapter writeback, consent/risk controls, and review workflows.

- **Spec:** [docs/specs/vel-metadata-enrichment-spec.md](../specs/vel-metadata-enrichment-spec.md)
- **Tickets:** `001-schema-and-domain-model.md` … `012-tests-fixtures-and-rollout.md`

#### 12. Projects page (`projects/`)

Project workspace and multi-surface contract for project registry, commitments, agent sessions, task steering, and operator/workspace parity.

- **Spec:** [docs/specs/vel-projects-page-spec.md](../specs/vel-projects-page-spec.md)
- **Tickets:** `01-project-boundary-and-registry.md` … `13-tests-docs-rollout.md`

#### 13. Self-knowledge system (`self-knowledge/`)

Documentation and source-code awareness: repo indexer, self-knowledge graph, evidence-backed claims, doc/code drift detection, navigation APIs, system map, change hotspots, maintenance loop.

- **Spec:** [docs/specs/vel-self-knowledge-system-spec.md](../specs/vel-self-knowledge-system-spec.md)
- **Related spec:** [docs/specs/vel-github-issues-spec.md](../specs/vel-github-issues-spec.md)
- **Tickets:** `SK-001-knowledge-indexer.md` … `SK-011-github-issue-awareness.md`

#### 14. Multi-client swarm (`multi-client-swarm/`)

Parallel multi-client orchestration and cluster-aware sync: explicit swarm task/work-unit model, append-only cluster sync, authority epochs, worker presence, DAG scheduler, supervisor integration, load balancing, replay, and failover testing.

- **Index:** [multi-client-swarm/README.md](multi-client-swarm/README.md)
- **Specs:** [docs/specs/vel-multi-client-swarm-spec.md](../specs/vel-multi-client-swarm-spec.md), [docs/specs/vel-cluster-sync-spec.md](../specs/vel-cluster-sync-spec.md)
- **Tickets:** `SWARM-001-task-and-work-unit-model.md` … `SWARM-008-observability-replay-and-failover-tests.md`
- **Use when:** implementing supervised parallel execution across clients/workers, or adding the cluster sync substrate needed for swarm scheduling and load balancing.
- **Caution:** this pack is orchestration-heavy and should not outrank current core convergence work unless there is an explicit decision to prioritize runtime/swarm infrastructure.

#### 15. Integration expansion (`integration-expansion/`)

Provider- and connection-aware integration architecture: multi-vendor messaging/notes/transcripts/tasks, person identity, Apple bridge prep, Steam activity, Google Workspace convergence, and standards-aware ingest/export strategy.

- **Index:** [integration-expansion/README.md](integration-expansion/README.md)
- **Spec:** [docs/specs/vel-multi-vendor-integration-and-person-identity-spec.md](../specs/vel-multi-vendor-integration-and-person-identity-spec.md)
- **Tickets:** `INTG-001-foundation-family-provider-connection-model.md` … `INTG-012-fixtures-tests-docs-and-rollout.md`
- **Use when:** expanding Vel beyond single-provider-per-family assumptions or introducing person-native multi-source identity and provenance.
- **Caution:** this pack should extend the existing adapter/runtime foundations, not bypass them with provider-specific core logic.

#### 16. Ticket object (`ticket-object/`)

First-class ticket subsystem for native Vel-backed and provider-backed work objects across GitHub Issues, Linear, Jira, Todoist, and future backends, with explicit linkage to commitments and projects.

- **Index:** [ticket-object/README.md](ticket-object/README.md)
- **Spec:** [docs/specs/vel-ticket-object-spec.md](../specs/vel-ticket-object-spec.md)
- **Tickets:** `VTKT-001-ticket-domain-schema-and-storage.md` … `VTKT-010-tests-fixtures-and-rollout.md`
- **Use when:** implementing durable backlog/work objects that must outgrow commitment-only modeling.
- **Caution:** this pack changes planned direction for project/task modeling; treat [docs/status.md](../status.md) as canonical for what is currently shipped.

#### 17. UI v4 redesign (`ui-v4/`)

Screenshot-backed operator-surface redesign pack focused on action-first `Now`, explicit context modes, a first-class `Stats` surface, clearer thread/inbox/suggestion roles, and stronger integration/settings IA.

- **Index:** [ui-v4/README.md](ui-v4/README.md)
- **Spec:** [docs/specs/vel-ui-v4-spec.md](../specs/vel-ui-v4-spec.md)
- **Tickets:** `UI-V4-001-context-panel-state-why-debug.md` … `UI-V4-010-settings-ia-rework.md`
- **Use when:** redesigning the main operator UI without changing the core runtime/domain model.
- **Caution:** this pack is planning work derived from imported screenshots and notes; use [docs/status.md](../status.md) for shipped UI truth.

#### 18. Storage backup sync (`storage-backup-sync/`)

Artifact backup and storage-target planning: manifest-driven backup model, verification, restore planning, and optional targets such as `rsync`, `s3`, `icloud_drive`, `google_drive`, and `dropbox`.

- **Index:** [storage-backup-sync/README.md](storage-backup-sync/README.md)
- **Spec:** [docs/specs/vel-storage-backup-sync-spec.md](../specs/vel-storage-backup-sync-spec.md)
- **Tickets:** `STOR-001-foundation-storage-target-and-backup-manifest-model.md` … `STOR-008-restore-plan-verification-and-operator-surfaces.md`
- **Use when:** planning or implementing artifact backup targets, off-device storage, verification, or restore workflows.
- **Caution:** this pack is about trust and recovery surfaces, not broader cluster/client sync or runtime authority.

#### 19. Connect-backed agent launch (`connect-agent-launch/`)

Execution pack for launching external coding-agent runtimes on compatible Connect instances, representing them as live Vel sessions, and exposing those sessions through Projects, CLI, and host-agent supervision flows.

- **Index:** [connect-agent-launch/README.md](connect-agent-launch/README.md)
- **Spec:** [docs/specs/vel-connect-agent-launch-spec.md](../specs/vel-connect-agent-launch-spec.md)
- **Tickets:** `CAL-001-connect-instance-registry-and-capability-manifest.md` … `CAL-008-fixtures-tests-docs-and-rollout-guards.md`
- **Use when:** implementing instance capability discovery, launch APIs, live launched-session modeling, Projects launch UX, or host-agent supervision of external runtimes.
- **Caution:** this pack extends `integration-expansion/`, `projects/`, and `multi-client-swarm/`; it should not duplicate their core substrate work or imply that broad runtime/vendor support is already shipped.

### Speculative / future architecture

#### 20. Full spec pack (`full-spec-pack/`)

Imported workflow-first planning packet covering templates, workflows, media, integrations, policy, UI system, voice UX, and high-level architecture.

- **Spec packet:** [docs/product-spec-pack/imported/full-spec-pack-2026-03-16/](../product-spec-pack/imported/full-spec-pack-2026-03-16/)
- **Tickets:** `ticket_templates.md` … `ticket_voice.md`
- **Use with caution:** this pack is broad and partially overlaps current commitments/context/nudge architecture. Reconcile against [docs/status.md](../status.md) and existing specs before execution.

#### 21. Context reasoning (`context-reasoning/`)

Context/decision tickets: belief store, inference engine, inspector UI, decision trace logging, explanation UI, feedback learning loop, belief expiration, confidence calibration, introspection report.

- **Spec:** [docs/specs/vel-context-decision-spec.md](../specs/vel-context-decision-spec.md)
- **Tickets:** `TICKET-001-context-belief-store.md` … `TICKET-010-introspection-report.md`
- **Use with caution:** this pack is conceptually rich, but may overlap with already-implemented current-context and explainability systems. Validate boundaries against [docs/status.md](../status.md) before execution.

#### 21. Task HUD (`task-hud/`)

Task HUD subsystem: task core crate, DB schema/migrations, actions engine, ranking engine, HUD policy, view model, desktop HUD UI, inference engine, ritual tasks, risk integration, voice bridge, glance API, ambient mode, AR protocol spec.

- **Spec:** [docs/specs/vel-task-hud-spec.md](../specs/vel-task-hud-spec.md)
- **Tickets:** `01-task-core-crate.md` … `14-ar-hud-protocol-spec.md`
- **Use with caution:** this pack introduces a new task-centric subsystem and should be reconciled carefully with existing commitments, nudges, threads, and risk semantics first.

#### 22. Orchestration Navs (`orchestration/`)

Core orchestration stack for Nav-based execution: task model, Nav trait, capability model, Nav registry, delegation engine, context scoping, result integration, persistent task store, trust profiles, reflection, observability, initial Nav implementations.

- **Tickets:** `001_task_model.md` … `012_initial_navs.md`

#### 23. Self-modification system (`vel-self-modification/`)

Governed self-modification pipeline: protected surface registry, patch proposal schema, self-improvement service skeleton, change ledger, validation orchestrator, sandbox execution, rollback control, autonomy budgets, rollout support, metrics, constitutional workflow.

- **Index:** [vel-self-modification/_ticket-index.md](vel-self-modification/_ticket-index.md)
- **Tickets:** `VSM-001-protected-surface-registry.md` … `VSM-020-constitutional-change-workflow.md`

#### 24. iOS/watch standalone (`ios-watch/`)

Tickets for iOS + watchOS as a separate repo/workspace (`vel-apple`): bootstrap, shared models/API, app shell, timeline/check-in/reminder flows, background refresh, widgets, voice, offline-first sync, privacy/observability, integration roadmap.

- **Index:** [ios-watch/README.md](ios-watch/README.md)
- **Tickets:** `TKT-001-apple-platform-bootstrap.md` … `TKT-013-apple-integration-roadmap.md`
- **Use with caution:** this pack assumes a separate-repo stance that may diverge from current same-repo Apple bootstrap guidance.

#### 25. Predicate system (`predicate-system/`)

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
- `037-chat-remote-fallback-for-assistant-generation.md`

## Status words used inside ticket files

Individual ticket files may use local workflow labels such as:

- `todo`
- `in_progress`
- `blocked`
- `review`
- `done`

Those labels are useful within a pack, but they do **not** override [docs/status.md](../status.md).
