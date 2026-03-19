# Roadmap: Vel

## Overview

Phase 1 and Phase 3 are complete. Phase 2 and Phase 4 are closed historical baselines with unfinished original-scope work explicitly re-scoped into Phases 5, 6, and 8. There is no remaining active roadmap work before Phase 5. The active roadmap now begins with the product-shaping sequence focused on `Now + Inbox`, project substrate, high-value write-back integrations, Apple action loops, coding-centric supervised execution, backup-first trust surfaces, a strict daily-loop MVP, agent grounding over real Vel data/tools, and operator-shell/onboarding ergonomics (Phases 5-12). Each remaining phase produces a verifiable capability boundary before the next begins.

## Phases

**Phase Numbering:**
- Phases 2–4 continue from completed Phase 1
- Integer phases only; decimal phases created via `/gsd:insert-phase` if urgent work is needed

- [x] **Phase 1: Structural Foundation** - Layered crates, auth hardening, canonical schemas, self-awareness (COMPLETE)
- [x] **Phase 1.1: Preflight — Pre-Phase 2 Hardening** (INSERTED) - Integration startup panic fixes, SQLite WAL mode, app.rs decomposition (COMPLETE)
- **Phase 2: Distributed State, Offline Clients & System-of-Systems** - Closed historical baseline; unfinished sync ordering, external connect transport, and guided node-linking work moved to Phases 5, 6, and 8
- [x] **Phase 3: Deterministic Verification & Continuous Alignment** - Day-simulation harness, LLM-as-a-Judge eval, execution tracing, user documentation (COMPLETE)
- **Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution** - Closed historical baseline; unfinished semantic graph expansion, direct WASM guest runtime, and external limb transport work moved to Phases 6 and 8
- [x] **Phase 5: Now + Inbox core and project substrate** - Keep `Now + Inbox` primary while adding durable project structure and shared workspace contracts (COMPLETE)
- [x] **Phase 6: High-value write-back integrations and lightweight people graph** - Add notes, reminders, GitHub, email, transcripts, and minimal people identity with upstream write-back (COMPLETE)
- [x] **Phase 7: Apple action loops and behavioral signal ingestion** - Prioritize fast iOS/watch actions and directly useful behavior signals (COMPLETE)
- [x] **Phase 8: Coding-centric supervised execution with GSD and local agents** - Launch and supervise coding-first runtimes with direct GSD integration and local-agent support (COMPLETE)
- [x] **Phase 9: Backup-first trust surfaces and simple operator control** - Add backup-first trust workflows and keep control/config surfaces simple (COMPLETE)
- [x] **Phase 10: Daily-loop morning overview and standup commitment engine** - Turn `Now`, calendar, Todoist, commitments, and voice into a bounded daily prioritization loop (completed 2026-03-19)
- [x] **Phase 11: Agent grounding and operator-relevant data/tool awareness** - Make supervised agents aware of real Vel state, projects, people, commitments, and bounded tool surfaces (completed 2026-03-19)
- [x] **Phase 12: Operator shell, onboarding, and connector ergonomics** - Make the daily loop and integration surfaces easier to adopt, navigate, and trust (completed 2026-03-19)

## Phase Details

### Phase 1.1: Preflight — Pre-Phase 2 Hardening (INSERTED)
**Goal**: Eliminate crash-risk technical debt and close infrastructure gaps that will become more expensive once distributed complexity grows. This phase has no new feature surface — it is purely hardening. Must complete before Phase 2 execution begins.
**Depends on**: Phase 1 (complete)
**Success Criteria** (what must be TRUE):
  1. Daemon starts cleanly with missing or corrupt Todoist/Google Calendar settings — no panics, graceful degradation with warning logs
  2. SQLite WAL mode is enabled; concurrent readers do not block background writers
  3. `app.rs` auth middleware and exposure gate logic extracted to a separate module; file is under 3,000 lines
**Work items**:
  - Fix `expect()` panics on integration settings load in `integrations_todoist.rs` and `integrations_google.rs`
  - Enable WAL mode in `vel-storage/src/infra.rs` database initialization
  - Extract auth middleware + `HttpExposurePolicy` from `app.rs` to `crates/veld/src/middleware/`
**Plans**: 1 plan
Plans:
- [x] 1.1-01-PLAN.md — TDD: Wave 0 failing tests + WAL mode + integration settings hardening + middleware extraction

### Phase 2: Distributed State, Offline Clients & System-of-Systems
**Goal**: The system can ingest signals from pluggable sources, maintain consistent distributed state across nodes, launch and supervise agent processes, broker capabilities without exposing raw credentials, and present clear effective configuration to the operator.
**Depends on**: Phase 1
**Requirements**: SIG-01, SIG-02, SYNC-01, SYNC-02, CONN-01, CONN-02, CONN-03, CONN-04, CAP-01, CAP-02, OPS-01, OPS-02
**Tickets**: `docs/tickets/phase-2/` — 004, 005, 006, 012, 016, 019
**Parallel board**: `docs/tickets/phase-2/parallel-execution-board.md`
**Status**: Closed and re-scoped — partial baseline shipped; unfinished original-scope work moved forward so no active roadmap work remains before Phase 5
**Success Criteria** (what must be TRUE):
  1. Operator can register a new signal source and see its data flowing into unified context state without code changes to the core pipeline
  2. Two nodes with divergent event logs converge to the same state after sync, with no manual conflict resolution required
  3. An agent process can be launched via the connect protocol, heartbeats are recorded, and the process is cleanly terminated on operator command
  4. A new node discovers the system and completes onboarding diagnostics without manual configuration
  5. Agent capability requests resolve to scoped tokens; no raw credentials appear in prompts or logs
  6. The operator CLI and web dashboard both display effective (resolved) configuration state, not raw config file values
**Plans**: 7 plans
Plans:
- [x] 02-01-PLAN.md — SP1: Contract alignment, operator diagnostics, connect surface consistency (Wave 1)
- [ ] 02-02-PLAN.md — SP2 Lane A: Signal reducer extraction — SignalReducer trait + ReducerRegistry (Wave 2, TDD)
- [ ] 02-03-PLAN.md — SP2 Lane B: Connect lifecycle MVP — launch/heartbeat/terminate/expiry (Wave 2)
- [ ] 02-04-PLAN.md — SP2 Lane C: Capability broker MVP — CapabilityDescriptor + BrokerService (Wave 2, TDD)
- [ ] 02-05-PLAN.md — SP3 Lane A+B: HLC sync ordering primitive + node link CLI/web/Apple (Wave 3)
- [ ] 02-05b-PLAN.md — SP3 Pairing backend: POST /api/node/pair/issue token generation + storage (Wave 3, after 02-05)
- [ ] 02-06-PLAN.md — SP3 Lane C: Accessibility/config clarity — vel config show + canonical terminology (Wave 4)

Residual work moved forward:
- Phase 5 absorbs guided node-linking, cross-surface continuity, and user-facing multi-client setup closure.
- Phase 6 absorbs deterministic reconciliation follow-on (`NodeIdentity`/ordering/conflict policy) and upstream-vs-local conflict handling.
- Phase 8 absorbs external `/v1/connect` transport exposure and explicit delegated-runtime launch/handoff surfaces.

### Phase 3: Deterministic Verification & Continuous Alignment
**Goal**: The system can replay any recorded day deterministically to verify correctness, evaluate agent reasoning outputs via an LLM judge, and expose complete execution traces to the operator — giving the operator confidence that agent behavior is auditable and regressions are detectable.
**Depends on**: Phase 2
**Requirements**: VERIFY-01, VERIFY-02, EVAL-01, EVAL-02, TRACE-01, TRACE-02, TRACE-03, DOCS-01, DOCS-02
**Tickets**: `docs/tickets/phase-3/` — 007, 008, 017, 013
**Parallel board**: `docs/tickets/phase-3/parallel-execution-board.md`
**Status**: Complete and closed — retained as historical implementation record only; no remaining roadmap work precedes Phase 5
**Success Criteria** (what must be TRUE):
  1. Running the day-simulation harness twice against the same recorded event log produces identical output both times
  2. Operator can submit an agent reasoning output for evaluation and receive a structured judge verdict stored in the database
  3. LLM eval results are queryable by date range and model so regressions can be spotted across runs
  4. Every agent run has a stable run ID; the operator can pull a full trace including inter-agent handoffs from the dashboard
  5. Operator-facing workflows are covered by documentation accessible from a searchable wiki
**Plans**: 5 plans
Plans:
- [x] 03-01-PLAN.md — SP1 Lane A/B entry: shared trace contract + run inspection linkage + contract docs
- [x] 03-02-PLAN.md — SP1 Lane B: CLI/web trace inspection surfaces
- [x] 03-03-PLAN.md — SP1 Lane C: user docs/support parity + recovery architecture
- [x] 03-04-PLAN.md — SP2: deterministic day-simulation harness + replay assertions
- [x] 03-05-PLAN.md — SP3: eval runner, judge integration, and reporting gates

### Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution
**Goal**: The system maintains a semantic memory graph over captured entities, uses graph-based retrieval to surface relevant context during reasoning, executes untrusted agents in WASM sandboxes with zero-trust defaults, and provides a first-class SDK for building external agent Limbs that communicate via a standardized swarm contract.
**Depends on**: Phase 3
**Requirements**: MEM-01, MEM-02, SAND-01, SAND-02, SDK-01, SDK-02, SDK-03
**Tickets**: `docs/tickets/phase-4/` — 009, 010, 014
**Parallel board**: `docs/tickets/phase-4/parallel-execution-board.md`
**Status**: Closed and re-scoped — baseline contracts/runtime slices shipped, but unfinished original-scope work moved forward so no active roadmap work remains before Phase 5
**Success Criteria** (what must be TRUE):
  1. Captured entities are reflected in the semantic memory graph; a reasoning step can retrieve contextually relevant memories via graph RAG
  2. An untrusted agent binary executes inside the WASM sandbox and can only interact with veld through declared capability contracts — any attempt to exceed declared permissions is rejected at the boundary
  3. A developer can import `vel-agent-sdk`, implement the swarm execution contract, and have their agent communicate with veld without touching veld internals
  4. The SDK ships with at least one working reference implementation and documentation covering the full integration contract
**Plans**: 5 plans
Plans:
- [x] 04-01-PLAN.md — SP1: semantic-memory, sandbox-ABI, and swarm-protocol contract foundations
- [x] 04-02-PLAN.md — SP2 Lane A: semantic index backend seam, provenance-preserving records, and retrieval lifecycle
- [x] 04-03-PLAN.md — SP2 Lane B: WASM sandbox runtime, deny-by-default policies, and operator-visible decisions
- [x] 04-04-PLAN.md — SP3 Lane A: swarm protocol crate, fixtures, and versioned serialization/validation
- [x] 04-05-PLAN.md — SP3 Lane B: reference SDK limb and end-to-end scoped capability flow

Residual work moved forward:
- Phase 6 absorbs semantic graph expansion beyond the shipped capture-backed baseline, including richer entity/link indexing for projects, notes, GitHub, and people.
- Phase 8 absorbs direct WASM guest runtime embedding, external connect/auth transport exposure, and broader external-limb execution hardening beyond the shipped host-executor baseline.

## Progress

**Execution Order:**
Remaining execution order: 13 → 14 → 15 → 16

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Structural Foundation | - | Complete | 2026-03-18 |
| 1.1. Preflight — Pre-Phase 2 Hardening | 1/1 | Complete | 2026-03-18 |
| 2. Distributed State, Offline Clients & System-of-Systems | 1/7 | Closed / Re-scoped | 2026-03-19 |
| 3. Deterministic Verification & Continuous Alignment | 5/5 | Complete | 2026-03-18 |
| 4. Autonomous Swarm, Graph RAG & Zero-Trust Execution | 5/5 | Closed / Re-scoped | 2026-03-19 |
| 5. Now + Inbox core and project substrate | 9/9 | Complete | 2026-03-19 |
| 6. High-value write-back integrations and lightweight people graph | 7/7 | Complete | 2026-03-19 |
| 7. Apple action loops and behavioral signal ingestion | 4/4 | Complete | 2026-03-19 |
| 8. Coding-centric supervised execution with GSD and local agents | 6/6 | Complete | 2026-03-19 |
| 9. Backup-first trust surfaces and simple operator control | 4/4 | Complete | 2026-03-19 |
| 10. Daily-loop morning overview and standup commitment engine | 5/5 | Complete   | 2026-03-19 |
| 11. Agent grounding and operator-relevant data/tool awareness | 3/3 | Complete   | 2026-03-19 |
| 12. Operator shell, onboarding, and connector ergonomics | 4/4 | Complete | 2026-03-19 |

### Phase 5: Now + Inbox core and project substrate

**Goal:** Keep `Now + Inbox` as the primary operator shell while establishing a typed project substrate, project families, and a unified action/intervention model that can safely anchor work across tasks, notes, messages, suggestions, conflicts, multi-client continuity, and future execution flows.
**Requirements**: NOW-01, NOW-02, INBOX-01, INBOX-02, ACTION-01, REVIEW-01, CONTINUITY-01, CONTINUITY-02, PROJ-01, PROJ-02, PROJ-03, FAMILY-01
**Depends on:** Phase 4
**Plans:** 9 plans

Plans:
- [x] 05-01-PLAN.md — Publish typed Phase 05 contracts for projects, action items, and linking scopes
- [x] 05-02-PLAN.md — Implement the persisted project substrate and local-first project workspace API
- [x] 05-03-PLAN.md — Close the backend linking path with scoped pairing tokens and durable trust state
- [x] 05-04-PLAN.md — Add the CLI fallback and runtime docs for guided node linking
- [x] 05-05-PLAN.md — Build the backend action/intervention projection plus Inbox triage mutations and sync state
- [x] 05-06-PLAN.md — Add typed web data contracts, project/linking loaders, and Inbox mutation helpers
- [x] 05-07-PLAN.md — Ship the web Now/Inbox/Projects/linking views on top of the new data layer
- [x] 05-08-PLAN.md — Bring Apple clients to Phase 05 continuity parity without adding client-owned policy
- [x] 05-09-PLAN.md — Make review outputs and operator docs align with the typed project/action model

### Phase 6: High-value write-back integrations and lightweight people graph

**Goal:** Deliver safe write-back for the highest-value integrations, make upstream systems authoritative with explicit conflict prompts, translate Todoist label syntax into Vel-native typed fields, and add a practical people registry tied to commitments, scheduling, messages, intervention loops, and cross-client reconciliation.
**Requirements**: WB-01, WB-02, WB-03, CONFLICT-01, PROV-01, RECON-01, TODO-01, NOTES-01, REMIND-01, GH-01, EMAIL-01, PEOPLE-01, PEOPLE-02
**Depends on:** Phase 5
**Plans:** 7/7 plans executed

Plans:
- [x] 06-01-PLAN.md — Publish typed Phase 06 contracts, schemas, and owner docs for write-back, conflicts, and people
- [x] 06-02-PLAN.md — Install deterministic ordering, conflict queue, write-back history, and upstream ownership foundations
- [x] 06-03-PLAN.md — Close the Todoist lane with typed write-back, project linkage, and conflict handling
- [x] 06-04-PLAN.md — Add scoped notes write-back, transcript-under-notes folding, and reminder intent execution tracking
- [x] 06-05-PLAN.md — Ship the minimal people registry and provenance-bearing graph expansion over durable Phase 06 entities
- [x] 06-06-PLAN.md — Add bounded GitHub and email provider slices with typed project/people linkage
- [x] 06-07-PLAN.md — Surface write-back, conflicts, provenance, and people status through operator views, CLI, and docs

### Phase 7: Apple action loops and behavioral signal ingestion

**Goal:** Make Vel useful from iPhone/watch first through fast capture and response loops, while ingesting lightweight behavioral signals that improve daily orientation without making health or astrology core dependencies.
**Requirements**: IOS-01, IOS-02, IOS-03, HEALTH-01, HEALTH-02, APPLE-01
**Depends on:** Phase 6
**Plans:** 4 plans

Plans:
- [x] 07-01-PLAN.md — Publish typed Apple voice, schedule, and behavior-summary contracts before implementation widens
- [x] 07-02-PLAN.md — Move Apple voice, schedule answers, and safe action execution into backend-owned routes/services
- [x] 07-03-PLAN.md — Add bounded step/stand/exercise ingestion and explainable backend behavior summaries
- [x] 07-04-PLAN.md — Wire Apple clients and docs to the new backend-owned loops while removing local query synthesis authority

### Phase 8: Coding-centric supervised execution with GSD and local agents

**Goal:** Extend Vel from daily orientation into supervised execution for coding-first work by letting projects carry repo/GSD context, generating repo-local planning artifacts that GSD can consume, and routing work by token budget, agent profile, task type, and explicit handoff boundaries.
**Requirements**: EXEC-01, EXEC-02, GSD-01, GSD-02, HANDOFF-01, HANDOFF-02, LOCAL-01, POLICY-01
**Depends on:** Phase 7
**Plans:** 6 plans

**Parallelization note:** `08-01` through `08-03` are intentionally scoped to project/protocol/runtime seams and can be executed in parallel with Phase 07 implementation after its contract slice (`07-01`) is in place. Final Phase 08 closure still waits on its own later slices and the overall roadmap order.

Plans:
- [x] 08-01-PLAN.md — Publish typed execution-context, handoff, routing-policy, and local-agent manifest contracts
- [x] 08-02-PLAN.md — Persist project execution context and generate bounded repo-local GSD artifact packs
- [x] 08-03-PLAN.md — Activate authenticated `/v1/connect` transport and supervised local runtime lifecycle
- [x] 08-04-PLAN.md — Surface explicit routing and handoff review across operator CLI/web surfaces
- [x] 08-05-PLAN.md — Add direct WASM guest-runtime execution behind the same mediated connect boundary
- [x] 08-06-PLAN.md — Close the loop with SDK, repo-local workflow docs, and execution-backed operator guidance

### Phase 9: Backup-first trust surfaces and simple operator control

**Goal:** Add lightweight backup/export and simple control surfaces that reduce fear of loss, while keeping restore/recovery and advanced policy surfaces intentionally smaller than the core daily loop.
**Requirements**: BACKUP-01, BACKUP-02, CTRL-01, CTRL-02
**Depends on:** Phase 8
**Plans:** 4 plans

Plans:
- [x] 09-01-PLAN.md — Ratify Phase 09 requirements and publish backup manifest/trust contracts before runtime work widens
- [x] 09-02-PLAN.md — Implement the snapshot-backed backup service, persisted history, and real CLI/API trust path
- [x] 09-03-PLAN.md — Surface backup freshness and safety state through doctor, settings, CLI, and web runtime views
- [x] 09-04-PLAN.md — Close with manual restore guidance, non-destructive verification, and narrow validation evidence

### Phase 10: Daily-loop morning overview and standup commitment engine

**Goal:** Turn the existing `Now`, commitments, calendar/Todoist input, and Apple/backend voice seams into a strict daily loop: a short morning overview that produces signals but no commitments, followed by a bounded standup that compresses work into 1-3 daily commitments, explicit deferrals, and focus-time protection.
**Requirements**: MORNING-01, MORNING-02, MORNING-03, STANDUP-01, STANDUP-02, STANDUP-03, SESSION-01, VOICE-01
**Depends on:** Phase 7 foundations; sequenced after Phase 9 in roadmap order
**Plans:** 5/5 plans complete

**Priority note:** This is the highest-value product phase after the current backup/trust lane because the repo already has partial `Now`, commitment, calendar, Todoist, and Apple voice primitives, but not yet one coherent <3 minute daily decision loop.
**Included from CSV triage:** morning overview, standup/commitment compression, focus-time shaping, bounded voice-or-text flow, and action-stack quality work such as deduplicating repeated suggestions so the daily loop stays trustworthy.
**Parallelization note:** `10-04` and `10-05` can run in parallel after the shared backend standup engine in `10-03` is in place.

Plans:
- [ ] 10-01-PLAN.md — Publish typed daily-loop session contracts and durable persistence before behavior widens
- [ ] 10-02-PLAN.md — Implement the backend-owned Morning Overview engine, bounded inputs, and dedicated daily-loop routes
- [ ] 10-03-PLAN.md — Build the bounded standup engine plus CLI text fallback over the shared session-turn API
- [ ] 10-04-PLAN.md — Expose daily-loop start/resume/outcome rendering through the existing web `Now` shell
- [ ] 10-05-PLAN.md — Extend the transcript-first Apple voice seam and docs to the shared daily-loop authority

### Phase 11: Agent grounding and operator-relevant data/tool awareness

**Goal:** Make supervised agents meaningfully aware of the operator's real Vel state by grounding them in current context, projects, people, commitments, review queues, and bounded tool affordances, so they can act on actual product data rather than behaving like generic assistants with weak repo-only context.
**Requirements**: AGENT-CTX-01, AGENT-CTX-02, AGENT-TOOLS-01, AGENT-TOOLS-02, AGENT-REVIEW-01, AGENT-TRUST-01
**Depends on:** Phase 10
**Plans:** 3/3 plans complete

**Priority note:** This phase is promoted ahead of shell ergonomics because the current codebase already has the raw ingredients for grounding agents in real Vel state (`/v1/now`, projects, people, review data, execution context, and handoff review), but it does not yet package that into a trustworthy operator-visible agent product path.
**Included from CSV triage:** stronger agent awareness of Vel data, tighter operator-relevant tool access, and grounding over shipped product state rather than generic ambient assistant behavior.

Plans:
- [ ] 11-01-PLAN.md — Publish typed grounding and capability-inspect contracts, schemas, examples, and owner docs
- [ ] 11-02-PLAN.md — Build the backend grounding service, authenticated inspect route, and supervised execution export reuse
- [ ] 11-03-PLAN.md — Add thin CLI/web trust surfaces over the shared inspect contract without moving policy client-side

### Phase 12: Operator shell, onboarding, and connector ergonomics

**Goal:** Make Vel easier to adopt and operate daily by tightening the web/operator shell, onboarding path, contextual docs/help, project detail surfaces, integration/status affordances, and path-discovery ergonomics around the daily-loop product direction.
**Requirements**: SHELL-01, SHELL-02, DOCS-01, ONBOARD-01, INTEGR-UX-01, PROJ-UX-01
**Depends on:** Phase 11
**Plans:** 4/4 plans executed

**Scope note:** This phase intentionally narrows the raw backlog to operator-shell and integration ergonomics. Broad new-provider expansion (full Google suite, LLM-provider routing, Dropbox-style picker proliferation, SaaS auth scaffolding, client-to-client file transfer, and reading/media systems) remains deferred until the daily loop and agent grounding are clearly working.
**Included from CSV triage:** app routes, top-nav/shell polish, icon-driven and collapsible navigation, softer auto-refresh freshness UX, project detail/edit surfaces, template viewing/editing in Settings, contextual docs/help routing, threads defaulting to the latest thread, upcoming-event ordering/pagination, richer Todoist rendering, connected-service icons, hidden internal integration paths, Apple/local-source path discovery/validation, and guided onboarding/linking/autodiscovery ergonomics.

Plans:
- [x] 12-01-PLAN.md — Publish the Phase 12 shell/help/onboarding contract and contextual-docs baseline before UI work widens
- [x] 12-02-PLAN.md — Tighten shell navigation, latest-thread entry behavior, and calmer freshness/connector UX in the daily shell
- [x] 12-03-PLAN.md — Add project detail/edit ergonomics and clarify Settings templates, connector status, and contextual help
- [x] 12-04-PLAN.md — Close guided onboarding, linking, and Apple/local-source path discovery with aligned setup/troubleshooting docs

### Phase 13: Cross-surface core architecture and adapter boundaries

**Goal:** Lock the cross-surface product-core architecture so Vel can keep one canonical Rust-owned behavior layer across Apple, web, and future desktop shells, with explicit adapter boundaries, migration seams, and future embedded/daemon/server topology guidance before broader shell expansion continues.
**Requirements**: ARCH-XS-01, ARCH-XS-02, ADAPT-01, ADAPT-02, APPLE-ARCH-01, API-ARCH-01
**Depends on:** Phase 12
**Plans:** 4/4 plans complete

**Architecture note:** This phase is explicitly about defining and proving the durable seams, not about mass crate renaming or immediate shell rewrites. The current `vel-core`/`vel-storage`/`vel-api-types`/`veld` split should evolve incrementally unless a narrower migration step proves a concrete structural defect.
**Future-topology note:** Tauri/desktop packaging should be planned here as a consumer of the same core/runtime contracts, but not treated as a required shipped shell in this phase.
**Included from thread decisions:** one canonical Rust-owned product core, command/query/read-model ownership rules, a truthful current-state-to-target-state map, a documented future Apple FFI path, a documented future desktop/Tauri path, and one proof flow over existing backend-owned architecture.

Plans:
- [ ] 13-01-PLAN.md — Ratify the canonical cross-surface architecture, topology modes, and current-to-target responsibility map
- [ ] 13-02-PLAN.md — Publish canonical command/query/read-model and transport ownership rules across core, runtime, and shell boundaries
- [ ] 13-03-PLAN.md — Document the future Apple embedded/FFI path and future desktop/Tauri adapter path without changing current authority
- [ ] 13-04-PLAN.md — Prove the architecture against one shipped multi-surface flow and record migration guardrails

### Phase 14: Product discovery, operator modes, and milestone shaping

**Goal:** Define the actual operator product shape after the architectural seams are clear: what belongs in the default daily-use experience, what moves behind advanced or developer modes, how onboarding and trust ergonomics should work, and which future milestones deserve investment before broader UI proliferation.
**Requirements**: PROD-01, MODE-01, UX-CORE-01, TRUST-UX-01, ONBOARD-02, ROADMAP-01
**Depends on:** Phase 13
**Plans:** 4/4 plans complete

**Discovery note:** This phase should reduce accidental product sprawl by deciding which currently exposed surfaces are core, advanced, internal, or deferred rather than letting those boundaries emerge ad hoc from current web or Apple UI.
**Included from thread decisions:** define the default core feature set, decide what moves behind menus or advanced/dev mode, settle onboarding/trust/help priorities, and produce milestone structure before broader UI or logic investment widens.
**Discovery follow-on note:** Early discovery indicates Phase 14 should evaluate adding a dedicated post-16 shell embodiment and surface-simplification phase instead of forcing that UI work into Phase 16.
**Taxonomy note:** Phase 14 concluded that `Now` and `Inbox` are the primary default surfaces, `Threads` is archive/search-first support, and `Projects` is secondary in navigation but may still own project-scoped actions.
**Action-model note:** Phase 14 also concluded that filters remain derived views over a canonical action model, with separate axes for urgency, importance, blocking state, and disruption level.

Plans:
- [x] 14-01-PLAN.md — Publish the canonical operator-surface taxonomy for default, advanced operator, and internal/developer surfaces
- [x] 14-02-PLAN.md — Define onboarding, trust, and recovery journeys as summary-first operator flows
- [x] 14-03-PLAN.md — Publish the operator-mode and progressive-disclosure policy across web, CLI, and Apple assumptions
- [x] 14-04-PLAN.md — Close with milestone reshaping, roadmap updates, and any new future phase needed for shell embodiment

### Phase 15: Incremental core migration and canonical Rust service seams

**Goal:** Incrementally migrate the codebase toward the Phase 13 architecture by sharpening application-service seams, transport boundaries, and cross-surface read-model ownership, so new logic lands in canonical Rust-owned surfaces instead of being rederived in shells.
**Requirements**: MIGRATE-01, MIGRATE-02, SERVICE-01, DTO-01, READMODEL-01
**Depends on:** Phase 14
**Plans:** 0 plans

**Migration note:** This phase should favor a sequence of proof-bearing seam migrations over a broad crate shuffle. Structural moves are justified only when they materially reduce shell-owned logic, boundary confusion, or transport coupling.
**Included from thread decisions:** do the minimum structural work needed for the next real logic slices, avoid refactor theater, and move seams only when the result clearly improves product-core ownership or portability.
**Phase 14 carry-forward:** migration should create the backend-owned seams needed for canonical action records, summary-first trust/readiness projections, check-in flows, and reflow planning without re-opening shell-boundary debates.

Plans:
- [ ] TBD (run /gsd:plan-phase 15 to break down)

### Phase 16: Logic-first product closure on canonical core surfaces

**Goal:** Implement the next wave of operator product behavior as Rust-owned commands, queries, policies, and read models on top of the migrated seams, so later Apple/web/desktop UI phases become embodiment work rather than product-logic design work.
**Requirements**: LOGIC-01, FLOW-01, MODE-02, READMODEL-02, SHELL-ARCH-01
**Depends on:** Phase 15
**Plans:** 0 plans

**Delivery note:** This phase is where the product logic discovered in Phase 14 should become canonical backend/application behavior, with UI phases following behind instead of leading product definition.
**Included from thread decisions:** business logic should be defined and implemented before broad shell expansion, with later UI phases focused on embodiment, interaction quality, and surface-specific affordances rather than inventing product semantics.
**Phase 14 carry-forward:** Phase 16 should implement the action-model and operator journey logic directly, including check-ins, heavier reflow semantics, summary-first trust/readiness, and backend-owned routing across `Now`, `Inbox`, `Threads`, and project-scoped actions.

Plans:
- [ ] TBD (run /gsd:plan-phase 16 to break down)

### Phase 17: Shell embodiment, operator-mode application, and surface simplification

**Goal:** Apply the Phase 14 product taxonomy and Phase 15-16 backend ownership decisions across web, Apple, CLI, and future desktop-ready shells so the default operator experience is simpler, advanced/runtime concerns are progressively disclosed, and internal implementation categories stop leaking into everyday use.
**Requirements**: SHELL-MODE-01, SHELL-MODE-02, TRUST-SUMMARY-01, APPLE-SHELL-01
**Depends on:** Phase 16
**Plans:** 0 plans

**Embodiment note:** This phase exists so UI/surface simplification does not get mixed into migration or backend logic closure. It should apply the approved product-mode policy and shell boundaries rather than invent new product semantics.
**Phase 14 carry-forward:** Phase 17 should embody minimal `Now`, triage-first `Inbox`, archive/search-first `Threads`, secondary-but-real `Projects`, inline `check_in`, and heavier `reflow` treatment without reopening the taxonomy or action-model decisions.

Plans:
- [ ] TBD (run /gsd:plan-phase 17 to break down)
