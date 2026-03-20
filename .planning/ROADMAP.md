# Roadmap: Vel

## Overview

Phase 1 and Phase 3 are complete. Phase 2 and Phase 4 are closed historical baselines with unfinished original-scope work explicitly re-scoped into Phases 5, 6, and 8. There is no remaining active roadmap work before Phase 5. The active roadmap now begins with the product-shaping sequence focused on `Now + Inbox`, project substrate, high-value write-back integrations, Apple action loops, coding-centric supervised execution, backup-first trust surfaces, a strict daily-loop MVP, agent grounding over real Vel data/tools, and operator-shell/onboarding ergonomics (Phases 5-12). Each remaining phase produces a verifiable capability boundary before the next begins.

## Phases

**Phase Numbering:**
- Phases 2–4 continue from completed Phase 1
- Integer phases only; decimal phases created via `/gsd:insert-phase` if urgent work is needed

- [x] **Phase 1: Structural Foundation** - Layered crates, auth hardening, canonical schemas, self-awareness (COMPLETE)
- [x] **Phase 1.1: Preflight — Pre-Phase 2 Hardening** (INSERTED) - Integration startup panic fixes, SQLite WAL mode, app.rs decomposition (COMPLETE)
- [x] **Phase 2: Distributed State, Offline Clients & System-of-Systems** - Closed historical baseline; unfinished sync ordering, external connect transport, and guided node-linking work moved to Phases 5, 6, and 8 (BASELINE CLOSED)
- [x] **Phase 3: Deterministic Verification & Continuous Alignment** - Day-simulation harness, LLM-as-a-Judge eval, execution tracing, user documentation (COMPLETE)
- [x] **Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution** - Closed historical baseline; unfinished semantic graph expansion, direct WASM guest runtime, and external limb transport work moved to Phases 6 and 8 (BASELINE CLOSED)
- [x] **Phase 5: Now + Inbox core and project substrate** - Keep `Now + Inbox` primary while adding durable project structure and shared workspace contracts (COMPLETE)
- [x] **Phase 6: High-value write-back integrations and lightweight people graph** - Add notes, reminders, GitHub, email, transcripts, and minimal people identity with upstream write-back (COMPLETE)
- [x] **Phase 7: Apple action loops and behavioral signal ingestion** - Prioritize fast iOS/watch actions and directly useful behavior signals (COMPLETE)
- [x] **Phase 8: Coding-centric supervised execution with GSD and local agents** - Launch and supervise coding-first runtimes with direct GSD integration and local-agent support (COMPLETE)
- [x] **Phase 9: Backup-first trust surfaces and simple operator control** - Add backup-first trust workflows and keep control/config surfaces simple (COMPLETE)
- [x] **Phase 10: Daily-loop morning overview and standup commitment engine** - Turn `Now`, calendar, Todoist, commitments, and voice into a bounded daily prioritization loop (completed 2026-03-19)
- [x] **Phase 11: Agent grounding and operator-relevant data/tool awareness** - Make supervised agents aware of real Vel state, projects, people, commitments, and bounded tool surfaces (completed 2026-03-19)
- [x] **Phase 12: Operator shell, onboarding, and connector ergonomics** - Make the daily loop and integration surfaces easier to adopt, navigate, and trust (completed 2026-03-19)
- [x] **Phase 13: Cross-surface core architecture and adapter boundaries** - Lock the canonical Rust-owned product-core architecture and adapter boundaries across shells (completed 2026-03-19)
- [x] **Phase 14: Product discovery, operator modes, and milestone shaping** - Define the real operator product shape, mode boundaries, and roadmap direction before wider UI growth (completed 2026-03-19)
- [x] **Phase 15: Incremental core migration and canonical Rust service seams** - Move product logic toward canonical backend-owned services and shared transport seams (completed 2026-03-19)
- [x] **Phase 16: Logic-first product closure on canonical core surfaces** - Close the next wave of operator behavior on Rust-owned commands, queries, and read models (completed 2026-03-19)
- [x] **Phase 17: Shell embodiment, operator-mode application, and surface simplification** - Apply the product taxonomy consistently across web, Apple, and CLI shells (completed 2026-03-19)
- [x] **Phase 18: Milestone verification backfill and requirement reconciliation** - Backfill durable verification evidence and restore a truthful milestone requirement ledger (completed 2026-03-20)
- [x] **Phase 19: Archive readiness, re-audit, and milestone closeout** - Repair milestone metadata, add milestone-level integration/flow evidence, and rerun closeout audit (completed 2026-03-20)
- [x] **Phase 20: Grounded assistant entry and daily-use usability closure** - Make the grounded assistant the practical default text/capture entry while tightening daily-use shells (completed 2026-03-20)
- [x] **Phase 21: Cross-surface voice assistant parity and desktop push-to-talk** - Unify browser, desktop, and Apple voice onto the same assistant seam (completed 2026-03-20)
- [x] **Phase 22: Assistant-supported daily loop, closeout, and thread resolution** - Extend the assistant seam into morning, standup, closeout, and multi-step thread resolution (completed 2026-03-20)
- [x] **Phase 23: Safe assistant-mediated actions and supervised write lanes** - Let the assistant stage bounded actions through existing trust and review lanes (completed 2026-03-20)
- [x] **Phase 24: Approved assistant action application and reversible write execution** - Turn approved assistant proposals into applied, inspectable outcomes (completed 2026-03-20)
- [x] **Phase 25: Local recall, semantic memory, and grounded assistant context** - Improve explainable recall quality and backend-owned assistant context assembly (completed 2026-03-20)
- [x] **Phase 26: Real day-plan reflow and schedule reconciliation** - Make same-day schedule recovery backend-owned, typed, and explainable (completed 2026-03-20)
- [x] **Phase 27: Canonical scheduler facets and commitment rule normalization** - Normalize scheduling semantics into canonical Vel-backed fields and facets (completed 2026-03-20)
- [x] **Phase 28: Routine blocks and commitment-aware day planning** - Add bounded backend-owned day planning over routines, commitments, and calendar anchors (completed 2026-03-20)
- [x] **Phase 29: Durable routine blocks and operator-managed planning constraints** - Persist routine/planning inputs as durable backend-owned records (completed 2026-03-20)
- [x] **Phase 30: Routine and planning-profile management surfaces** - Add thin shipped surfaces over the canonical planning-profile substrate (completed 2026-03-20)
- [x] **Phase 31: Cross-surface planning-profile parity and assistant-managed routine edits** - Extend the planning-profile seam into CLI, Apple, and assistant flows (completed 2026-03-20)
- [x] **Phase 32: Approved planning-profile edits and supervised routine application** - Let approved routine/profile proposals apply through canonical backend mutations (completed 2026-03-20)
- [x] **Phase 33: Approved day-plan and reflow application over commitment scheduling** - Apply bounded same-day planning outcomes through supervised backend scheduling seams (completed 2026-03-20)
- [x] **Phase 34: Now-view simplification, current-day truth, and Vel.csv acceptance** - Repair `Now` into a compact, current-day control surface and use `Vel.csv` as regression pressure (completed 2026-03-20)
- [x] **Phase 35: Sleep-relative day boundary and today-lane correctness** - Make `Now`, next-event truth, and today-lane ordering share one sleep-relative day model (completed 2026-03-20)
- [x] **Phase 36: Shell hierarchy, settings, and continuity simplification** - Reduce shell slop across `Now`, `Threads`, `Settings`, and sidebar behavior (completed 2026-03-20)
- [x] **Phase 37: iPhone embedded Rust core and Apple FFI foundation** - Introduce the real embedded-capable iPhone Rust bridge without forking product logic (completed 2026-03-20)
- [x] **Phase 38: Local-first iPhone voice continuity and offline action lane** - Make queued voice continuity and offline quick actions feel local-first on iPhone (completed 2026-03-20)
- [x] **Phase 39: Vel.csv regression sweep and daily-use closeout** - Close the repaired daily-use arc with cross-surface regression evidence and explicit deferred limits (completed 2026-03-20)

## Phase Details

### Phase 1: Structural Foundation

**Goal**: Establish the canonical layered Rust architecture, auth-by-default exposure model, machine-readable contracts, and documentation truth needed for every later phase.
**Depends on**: none
**Status**: Complete historical foundation for the milestone
**Plans**: historical pre-GSD implementation baseline

**Foundation note:** Phase 1 predates the current phase-planning artifact pattern, so it does not have per-plan files in `.planning/phases/`. Its completion truth is preserved through shipped code, the validated requirement ledger in [PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md), and the Phase 1 requirements in [REQUIREMENTS.md](/home/jove/code/vel/.planning/REQUIREMENTS.md).

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
Remaining execution order: 40

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
| 13. Cross-surface core architecture and adapter boundaries | 4/4 | Complete | 2026-03-19 |
| 14. Product discovery, operator modes, and milestone shaping | 4/4 | Complete | 2026-03-19 |
| 15. Incremental core migration and canonical Rust service seams | 5/5 | Complete | 2026-03-19 |
| 16. Logic-first product closure on canonical core surfaces | 5/5 | Complete | 2026-03-19 |
| 17. Shell embodiment, operator-mode application, and surface simplification | 4/4 | Complete | 2026-03-19 |
| 18. Milestone verification backfill and requirement reconciliation | 4/4 | Complete | 2026-03-20 |
| 19. Archive readiness, re-audit, and milestone closeout | 4/4 | Complete | 2026-03-20 |
| 20. Grounded assistant entry and daily-use usability closure | 4/4 | Complete | 2026-03-20 |
| 21. Cross-surface voice assistant parity and desktop push-to-talk | 4/4 | Complete | 2026-03-20 |
| 22. Assistant-supported daily loop, closeout, and thread resolution | 4/4 | Complete | 2026-03-20 |
| 23. Safe assistant-mediated actions and supervised write lanes | 4/4 | Complete | 2026-03-20 |
| 24. Approved assistant action application and reversible write execution | 4/4 | Complete | 2026-03-20 |
| 25. Local recall, semantic memory, and grounded assistant context | 4/4 | Complete | 2026-03-20 |
| 26. Real day-plan reflow and schedule reconciliation | 4/4 | Complete | 2026-03-20 |
| 27. Canonical scheduler facets and commitment rule normalization | 4/4 | Complete | 2026-03-20 |
| 28. Routine blocks and commitment-aware day planning | 4/4 | Complete | 2026-03-20 |
| 29. Durable routine blocks and operator-managed planning constraints | 4/4 | Complete | 2026-03-20 |
| 30. Routine and planning-profile management surfaces | 4/4 | Complete | 2026-03-20 |
| 31. Cross-surface planning-profile parity and assistant-managed routine edits | 4/4 | Complete | 2026-03-20 |
| 32. Approved planning-profile edits and supervised routine application | 4/4 | Complete | 2026-03-20 |
| 33. Approved day-plan and reflow application over commitment scheduling | 4/4 | Complete | 2026-03-20 |
| 34. Now-view simplification, current-day truth, and Vel.csv acceptance | 4/4 | Complete | 2026-03-20 |
| 35. Sleep-relative day boundary and today-lane correctness | 4/4 | Complete | 2026-03-20 |
| 36. Shell hierarchy, settings, and continuity simplification | 4/4 | Complete | 2026-03-20 |
| 37. iPhone embedded Rust core and Apple FFI foundation | 4/4 | Complete | 2026-03-20 |
| 38. Local-first iPhone voice continuity and offline action lane | 4/4 | Complete | 2026-03-20 |
| 39. Vel.csv regression sweep and daily-use closeout | 4/4 | Complete | 2026-03-20 |
| 40. Decision-first UI/UX rework across Now, Settings, Threads, and context surfaces | 0/4 | Planned | - |

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
**Plans:** 4/4 plans complete

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
**Plans:** 4/4 plans complete

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
**Plans:** 5/5 plans complete

**Migration note:** This phase should favor a sequence of proof-bearing seam migrations over a broad crate shuffle. Structural moves are justified only when they materially reduce shell-owned logic, boundary confusion, or transport coupling.
**Included from thread decisions:** do the minimum structural work needed for the next real logic slices, avoid refactor theater, and move seams only when the result clearly improves product-core ownership or portability.
**Phase 14 carry-forward:** migration should create the backend-owned seams needed for canonical action records, summary-first trust/readiness projections, check-in flows, and reflow planning without re-opening shell-boundary debates.

Plans:
- [x] 15-01-PLAN.md — Tighten the canonical operator-action/read-model contract and migration map before behavior widens
- [x] 15-02-PLAN.md — Introduce the first backend-owned `check_in` seam through core, service, and `Now` read-model boundaries
- [x] 15-03-PLAN.md — Introduce the first backend-owned `reflow` seam near daily-loop ownership and drift inputs
- [x] 15-04-PLAN.md — Compose summary-first trust/readiness projections from existing backup, freshness, and review evidence
- [x] 15-05-PLAN.md — Preserve project-scoped operator-action ownership across core, storage/service, DTO, and read-model seams

### Phase 16: Logic-first product closure on canonical core surfaces

**Goal:** Implement the next wave of operator product behavior as Rust-owned commands, queries, policies, and read models on top of the migrated seams, so later Apple/web/desktop UI phases become embodiment work rather than product-logic design work.
**Requirements**: LOGIC-01, FLOW-01, MODE-02, READMODEL-02, SHELL-ARCH-01
**Depends on:** Phase 15
**Plans:** 5/5 plans complete

**Delivery note:** This phase is where the product logic discovered in Phase 14 should become canonical backend/application behavior, with UI phases following behind instead of leading product definition.
**Included from thread decisions:** business logic should be defined and implemented before broad shell expansion, with later UI phases focused on embodiment, interaction quality, and surface-specific affordances rather than inventing product semantics.
**Phase 14 carry-forward:** Phase 16 should implement the action-model and operator journey logic directly, including check-ins, heavier reflow semantics, summary-first trust/readiness, and backend-owned routing across `Now`, `Inbox`, `Threads`, and project-scoped actions.

Plans:
- [x] 16-01-PLAN.md — Ratify the canonical operator-action transition contract and logic entry points before behavior widens
- [x] 16-02-PLAN.md — Implement backend-owned `check_in` accept/bypass/completion behavior over daily-loop/session seams
- [x] 16-03-PLAN.md — Implement backend-owned `reflow` confirm/apply/edit behavior and follow-up state generation
- [x] 16-04-PLAN.md — Tighten trust/readiness follow-through so degraded posture yields canonical backend-owned recovery/review actions
- [x] 16-05-PLAN.md — Close project-scoped action behavior and typed thread escalation/routing without reopening shell debates

### Phase 17: Shell embodiment, operator-mode application, and surface simplification

**Goal:** Apply the Phase 14 product taxonomy and Phase 15-16 backend ownership decisions across web, Apple, CLI, and future desktop-ready shells so the default operator experience is simpler, advanced/runtime concerns are progressively disclosed, and internal implementation categories stop leaking into everyday use.
**Requirements**: SHELL-MODE-01, SHELL-MODE-02, TRUST-SUMMARY-01, APPLE-SHELL-01
**Depends on:** Phase 16
**Plans:** 4/4 plans complete

**Embodiment note:** This phase exists so UI/surface simplification does not get mixed into migration or backend logic closure. It should apply the approved product-mode policy and shell boundaries rather than invent new product semantics.
**Phase 14 carry-forward:** Phase 17 should embody minimal `Now`, triage-first `Inbox`, archive/search-first `Threads`, secondary-but-real `Projects`, inline `check_in`, and heavier `reflow` treatment without reopening the taxonomy or action-model decisions.

Plans:
- [x] 17-01-PLAN.md — Stabilize shared web shell classification and top-level routing around the approved taxonomy
- [x] 17-02-PLAN.md — Embody minimal `Now`, triage-first `Inbox`, and archive/search-first `Threads` in the web shell
- [x] 17-03-PLAN.md — Apply progressive disclosure to `Projects`, `Settings`, trust/setup, and passive support surfaces
- [x] 17-04-PLAN.md — Align Apple and CLI shells to the same taxonomy and disclosure rules without widening backend semantics

### Phase 18: Milestone verification backfill and requirement reconciliation

**Goal:** Close the milestone audit blockers by backfilling missing verification evidence across the shipped phases, reconciling `REQUIREMENTS.md` against completed summaries and verification outcomes, and restoring one truthful milestone ledger before archival.
**Requirements**: CLOSEOUT-01, CLOSEOUT-02
**Depends on:** Phase 17
**Gap Closure:** Closes `MILESTONE-VERIFY-01` and `MILESTONE-REQS-01` from [v0.1-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.1-MILESTONE-AUDIT.md)
**Plans:** 4/4 plans complete

**Closeout note:** This phase is intentionally bookkeeping-heavy but still product-critical. The milestone cannot be archived honestly until verification coverage and requirement status are backed by durable artifacts instead of summaries alone.

Plans:
- [x] 18-01-PLAN.md — Create the milestone closeout inventory and explicit reconciliation rules
- [x] 18-02-PLAN.md — Backfill verification artifacts for historical/baseline phases 2-4 without rewriting the re-scope history
- [x] 18-03-PLAN.md — Backfill verification artifacts for fully shipped product phases 5-17 using existing summary evidence
- [x] 18-04-PLAN.md — Reconcile REQUIREMENTS.md against the new verification truth and prepare the Phase 19 handoff

### Phase 19: Archive readiness, re-audit, and milestone closeout

**Goal:** Make the milestone archive inputs internally consistent, rerun the milestone audit against the repaired verification/requirements state, and complete archival/tag readiness without introducing new product work.
**Requirements**: CLOSEOUT-03, CLOSEOUT-04
**Depends on:** Phase 18
**Gap Closure:** Closes `MILESTONE-ROADMAP-01`, `MILESTONE-INTEGRATION-01`, and `MILESTONE-FLOW-01` from [v0.1-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.1-MILESTONE-AUDIT.md)
**Plans:** 4/4 plans complete

**Closeout note:** This phase exists so the archive/release record is based on a passing milestone audit and coherent roadmap/requirements state, not on optimistic completion summaries.

Plans:
- [x] 19-01-PLAN.md — Repair roadmap/state/archive metadata drift and establish the archive-readiness baseline
- [x] 19-02-PLAN.md — Write milestone-level integration verification tying backend, web, Apple, and CLI surfaces together
- [x] 19-03-PLAN.md — Write milestone-level end-to-end flow verification for closeout-critical operator flows
- [x] 19-04-PLAN.md — Rerun milestone audit, update closeout requirements, and leave the milestone ready for archival

**Completion note (2026-03-20):** Phase 19 was initially deferred so feature work could repair real daily-use usability first. The later daily-use arc completed, and the closeout work is now finished retroactively with repaired milestone metadata, milestone-level integration/flow evidence, and a rerun audit that leaves the milestone ready for archival.

### Phase 20: Grounded assistant entry and daily-use usability closure

**Goal:** Make Vel materially more usable for repeated daily operation by turning the grounded Rust-owned assistant into the default operator entry for text, capture, and thread continuity, while tightening the `Now` / `Inbox` / `Threads` loop and reducing setup friction enough for real daily use.
**Requirements**: USABLE-01, USABLE-02, NOW-UX-01, INBOX-UX-01, THREADS-UX-01, ENTRY-01, SETTINGS-UX-01, ASSIST-01, ASSIST-02, THREADS-02
**Depends on:** Phase 17 shipped behavior; Phase 19 closeout was temporarily deferred during execution and later completed retroactively
**Plans:** 4/4 plans complete

Plans:
- [x] 20-01-PLAN.md — Stabilize the thread/composer contract and repair the known ThreadView placeholder drift
- [x] 20-02-PLAN.md — Add the backend-owned grounded assistant entry seam over the existing conversation/message stack
- [x] 20-03-PLAN.md — Embody assistant-first entry in `Now` and shared thread/composer web flows
- [x] 20-04-PLAN.md — Close Inbox, Threads, and Settings daily-use friction without breaking surface boundaries

**Priority note:** This phase was promoted ahead of milestone archival because the operator explicitly chose usability and repeated daily use over more closeout bookkeeping; the closeout lane was completed later without reopening product scope here.

**Scope note:** This phase should stay focused on real operator usability:

- `Now` as compact, urgent-first, contextual action surface
- better `Inbox` and `Threads` ergonomics for triage versus continuity
- a unified entry/capture/conversation path backed by one grounded assistant seam
- bounded remote LLM routing through configured profiles, including localhost `openai_oauth`, without weakening local-first core behavior
- reduction of settings/setup friction in the default experience

It should not widen into broad new provider expansion or another architecture phase.

### Phase 21: Cross-surface voice assistant parity and desktop push-to-talk

**Goal:** Make voice a first-class path into the same grounded assistant/runtime authority across web, desktop, and Apple surfaces, preferring local speech-to-text where practical while keeping backend-owned product semantics and thread continuity.
**Requirements**: VOICE-02, VOICE-03, APPLE-VOICE-02, DESKTOP-VOICE-01, DESKTOP-VOICE-02
**Depends on:** Phase 20
**Plans:** 4/4 plans complete

**Scope note:** This phase should unify voice-facing behavior rather than growing a separate voice product:

- browser/local desktop voice should feed the same grounded assistant path as typed input
- Apple voice should stop carrying more product logic than the shared assistant seam
- push-to-talk and transcript provenance should stay explicit and inspectable
- local STT is preferred for desktop usability, with remote inference remaining optional and replaceable

### Phase 22: Assistant-supported daily loop, closeout, and thread resolution

**Goal:** Extend the grounded assistant seam so morning briefing, standup, end-of-day closeout, and multi-step resolution of action items can all happen through one backend-owned conversation/thread model instead of separate ad hoc flows.
**Requirements**: DAILY-AI-01, DAILY-AI-02, EOD-01, EOD-02, THREAD-RES-01, THREAD-RES-02
**Depends on:** Phase 21
**Plans:** 4/4 plans complete

**Scope note:** This phase should reuse the existing daily-loop and operator-action contracts rather than inventing a parallel assistant-only planning system:

- morning and standup should be assistant-capable without abandoning the typed daily-loop authority
- end-of-day should become a first-class assistant-capable closure flow
- longer check-in, reflow, and item-resolution work should escalate into durable threads cleanly
- thread history should preserve why an item was resolved, deferred, edited, or left pending

### Phase 23: Safe assistant-mediated actions and supervised write lanes

**Goal:** Open the next step beyond read-only grounding by letting the assistant propose or stage bounded actions through the existing review, trust, and writeback gates, so the assistant can help resolve real work without bypassing operator control.
**Requirements**: ASSIST-ACT-01, ASSIST-ACT-02, REVIEW-02, TRUST-02
**Depends on:** Phase 22
**Plans:** 4/4 plans complete

Plans:
- [x] 23-01-PLAN.md — Publish the assistant proposal contract and first backend staging seam
- [x] 23-02-PLAN.md — Integrate assistant proposals with review/trust surfaces and fail-closed SAFE MODE gates
- [x] 23-03-PLAN.md — Connect thread continuity to staged approvals and confirmations with preserved provenance
- [x] 23-04-PLAN.md — Close Phase 23 with shell/docs verification and honest shipped limits

**Scope note:** This phase is intentionally later than the conversation and voice phases. The assistant should become deeply useful before it becomes mutation-capable:

- assistant-proposed actions must reuse existing review and SAFE MODE lanes
- no raw credential widening or silent background writes
- thread-based resolution should be able to hand off into explicit approval or confirmation paths

### Phase 24: Approved assistant action application and reversible write execution

**Goal:** Turn staged assistant proposals into explicitly approved, applied, and inspectable outcomes by reusing the existing confirmation, execution-review, and writeback seams instead of leaving assistant actions permanently stuck at staging.
**Requirements**: ASSIST-APPLY-01, ASSIST-APPLY-02, REVIEW-03, TRUST-03
**Depends on:** Phase 23
**Plans:** 4/4 plans complete

Plans:
- [x] 24-01-PLAN.md — Publish the approved-application contract and canonical proposal state transitions
- [x] 24-02-PLAN.md — Complete review-gated execution and writeback application for approved assistant proposals
- [x] 24-03-PLAN.md — Preserve applied outcome provenance, reversibility, and thread/Now follow-through
- [x] 24-04-PLAN.md — Close Phase 24 with shell/docs verification and honest shipped limits

**Scope note:** This phase should make approved assistant work real without weakening supervision:

- bounded confirmations should be able to apply through the canonical operator-action lane
- supervised write work should advance only after existing review gates are satisfied
- applied assistant actions should remain inspectable, explainable, and reversible where the underlying product contract already requires it
- no ambient widening of assistant authority, credentials, or background mutation behavior

### Phase 25: Local recall, semantic memory, and grounded assistant context

**Goal:** Improve local recall quality and assistant grounding by tightening semantic retrieval, recall-oriented context assembly, and explainable memory-backed answers over existing Vel data.
**Requirements**: RECALL-01, RECALL-02, SEM-02, GROUND-CTX-01, GROUND-CTX-02
**Depends on:** Phase 24
**Plans:** 4/4 plans complete

Plans:
- [x] 25-01-PLAN.md — Tighten the recall and grounding contract around canonical semantic retrieval and bounded assistant context inputs
- [x] 25-02-PLAN.md — Improve local semantic retrieval quality, ranking, and durable provenance across core runtime entities
- [x] 25-03-PLAN.md — Assemble stronger backend-owned assistant context from recall results instead of ad hoc tool responses
- [x] 25-04-PLAN.md — Close Phase 25 with shell/docs verification and honest shipped recall limits

**Scope note:** This phase should improve retrieval quality and assistant grounding without turning into a broad RAG platform rewrite:

- local-first semantic recall should get better across captures, notes, projects, people, threads, and transcripts
- retrieved context should stay explainable from persisted records, scores, and provenance
- assistant grounding should prefer bounded recall/context packs over repeated raw storage-shaped tool calls
- do not widen into new providers, hosted memory infrastructure, or speculative multi-agent memory systems

### Phase 26: Real day-plan reflow and schedule reconciliation

**Goal:** Turn `reflow` into a real backend-owned day-repair lane that can recompute the remaining day, explain what changed, and reconcile stale schedule reality instead of only surfacing warnings.
**Requirements**: REFLOW-REAL-01, REFLOW-REAL-02, SCHED-RECON-01, SCHED-RECON-02
**Depends on:** Phase 25
**Plans:** 4/4 plans complete

Plans:
- [x] 26-01-PLAN.md — Publish the canonical reflow/reconciliation contract and scheduler-rule mapping seam
- [x] 26-02-PLAN.md — Implement backend-owned remaining-day recomputation and explicit moved/unscheduled outcomes
- [x] 26-03-PLAN.md — Embody reflow and recovery posture in `Now`, `Threads`, and `Settings`
- [x] 26-04-PLAN.md — Close the phase with docs and verification for the real recovery story

**Scope note:** This phase should make daily-use recovery materially more useful without turning into a speculative planner rewrite:

- stale schedule, missed event, and slipped-block recovery should become one canonical backend-owned reflow path
- reflow should be able to explain what moved, what no longer fits, and what still needs the operator
- shell surfaces should stay thin and consume typed reflow output instead of re-deriving plan logic
- do not widen into broad autonomous calendar mutation or multi-day planning yet

### Phase 27: Canonical scheduler facets and commitment rule normalization

**Goal:** Normalize scheduling and tagging rules into canonical Vel-backed fields and facets so agents, recall, reflow, and future planning logic can reason over them without depending on raw provider labels or title syntax.
**Requirements**: SCHED-FACET-01, SCHED-FACET-02, AGENT-SCHED-01, RECALL-SCHED-01
**Depends on:** Phase 26
**Plans:** 4/4 plans complete

Plans:
- [x] 27-01-PLAN.md — Define canonical scheduler facet schema, storage shape, and ingest mapping rules
- [x] 27-02-PLAN.md — Persist normalized scheduler facets for commitments and expose them through backend/domain seams
- [x] 27-03-PLAN.md — Use normalized scheduler facets in agent, recall, and reflow paths instead of ad hoc raw-label parsing
- [x] 27-04-PLAN.md — Close Phase 27 with docs, examples, and verification for the canonical rule model

**Scope note:** This phase should make scheduling semantics trustworthy and agent-usable without turning into a broad planner rewrite:

- raw upstream tags and freeform keywords remain compatibility/search inputs, not durable product truth
- canonical fields/facets should intentionally capture the proven `codex-workspace` rule system
- SQL-backed normalized scheduler semantics should improve explainability, filtering, and agent reasoning
- do not widen into speculative multi-day optimization or broad external calendar mutation yet

### Phase 28: Routine blocks and commitment-aware day planning

**Goal:** Turn canonical scheduler rules plus real calendar reality into a bounded backend-owned day-planning lane that can shape the day before it drifts, not just recover after drift.
**Requirements**: DAYPLAN-01, DAYPLAN-02, ROUTINE-01, ROUTINE-02
**Depends on:** Phase 27
**Plans:** 4/4 plans complete

Plans:
- [x] 28-01-PLAN.md — Publish the canonical routine-block and day-plan contract over commitments, calendar anchors, and scheduler rules
- [x] 28-02-PLAN.md — Implement backend-owned initial day-plan shaping with explicit scheduled, deferred, and did-not-fit outcomes
- [x] 28-03-PLAN.md — Embody planned-day output in `Now`, `Threads`, and `Settings` without creating a shell-owned planner
- [x] 28-04-PLAN.md — Close the phase with docs, examples, and verification for bounded day planning and routine handling

**Scope note:** This phase should make the morning loop materially more useful without turning into a speculative autonomous planner:

- canonical scheduler rules should shape the initial day plan before `reflow` is needed
- routine blocks, calendar anchors, and open commitments should all remain backend-owned typed inputs
- the planner should explain what was scheduled, what was deferred, and what did not fit
- do not widen into multi-day optimization, automatic upstream calendar mutation, or opaque shell-local heuristics

### Phase 29: Durable routine blocks and operator-managed planning constraints

**Goal:** Replace bounded inferred routine inputs with durable backend-owned routine blocks and operator-managed planning constraints that can shape the day-plan substrate consistently across surfaces.
**Requirements**: ROUTINE-03, ROUTINE-04, DAYPLAN-03, DAYPLAN-04
**Depends on:** Phase 28
**Plans:** 4/4 plans complete

Plans:
- [x] 29-01-PLAN.md — Publish the durable routine-block and planning-constraint contract over the existing day-plan substrate
- [x] 29-02-PLAN.md — Persist operator-declared routine blocks and bounded planning constraints in backend/storage seams
- [x] 29-03-PLAN.md — Use durable routine blocks in day-plan shaping and expose summary-first management in shipped surfaces
- [x] 29-04-PLAN.md — Close the phase with docs, examples, and verification for durable routine-backed day planning

**Scope note:** This phase should strengthen same-day planning inputs without jumping to speculative planner autonomy:

- routine blocks should become durable backend-owned records rather than only inferred context hints
- operators should be able to manage bounded planning constraints without creating shell-owned planning logic
- `day_plan` and `reflow` should continue to share one planning substrate
- do not widen into multi-day optimization, automatic broad upstream mutation, or a full habit/routine product

### Phase 30: Routine and planning-profile management surfaces

**Goal:** Let operators inspect and manage durable routine blocks and bounded planning constraints through typed backend seams and thin shipped surfaces, without creating a second planner or shell-owned scheduling logic.
**Requirements**: ROUTINE-05, ROUTINE-06, PLANPROFILE-01, PLANPROFILE-02
**Depends on:** Phase 29
**Plans:** 4/4 plans complete

Plans:
- [x] 30-01-PLAN.md — Publish the typed planning-profile management contract and transport seam over the durable routine/planning substrate
- [x] 30-02-PLAN.md — Implement backend/storage planning-profile mutation seams with validation and durable persistence
- [x] 30-03-PLAN.md — Expose summary-first planning-profile management in shipped surfaces over the canonical backend-owned profile
- [x] 30-04-PLAN.md — Close the phase with docs, examples, and verification for the operator-managed planning-profile model

**Scope note:** This phase should make the durable planning substrate operable in daily use without widening its authority:

- operators should be able to inspect, create, edit, and remove durable routine blocks through backend-owned typed seams
- bounded planning constraints should become manageable from shipped surfaces, especially Settings, without shifting planning semantics into the client
- `Now`, `Threads`, and `Settings` should stay summary-first and explainable over the same backend-owned planning profile
- do not widen into broad calendar editing, multi-day planning, or a separate routine-management product

### Phase 31: Cross-surface planning-profile parity and assistant-managed routine edits

**Goal:** Extend the canonical planning-profile seam into CLI, Apple, and assistant/voice flows so routine and planning-constraint management works across real entry surfaces without creating a second planner.
**Requirements**: PLANPROFILE-03, PLANPROFILE-04, ROUTINE-07, VOICE-04
**Depends on:** Phase 30
**Plans:** 4/4 plans complete

Plans:
- [x] 31-01-PLAN.md — Publish the cross-surface planning-profile parity contract and assistant-edit transport seam
- [x] 31-02-PLAN.md — Ship CLI and Apple planning-profile inspection/parity over the canonical backend seam
- [x] 31-03-PLAN.md — Route assistant and voice-driven routine/profile edits through the typed mutation seam with confirmation and provenance
- [x] 31-04-PLAN.md — Close the phase with docs, examples, and verification for the cross-surface planning-profile model

**Scope note:** This phase should widen access to the already-shipped planning-profile model without widening its authority:

- CLI and Apple should be able to inspect the same durable planning profile the web Settings surface now manages
- assistant and voice entry should be able to stage bounded routine/profile edits through the typed planning-profile mutation seam
- confirmation, provenance, and thread continuity should remain explicit for planning-profile edits just like other supervised assistant actions
- do not widen into autonomous planner mutation, broad calendar editing, or a shell-specific planning system

### Phase 32: Approved planning-profile edits and supervised routine application

**Goal:** Let approved planning-profile edit proposals apply through the canonical backend-owned mutation seam with explicit review, thread continuity, and cross-surface follow-through, without creating a second planner or silent conversational writes.
**Requirements**: PLANPROFILE-05, PLANPROFILE-06, ROUTINE-08, VOICE-05
**Depends on:** Phase 31
**Plans:** 4/4 plans complete

Plans:
- [x] 32-01-PLAN.md — Publish the approved planning-profile application contract and proposal lifecycle transitions
- [x] 32-02-PLAN.md — Implement backend approval/application of staged planning-profile proposals over the canonical mutation seam
- [x] 32-03-PLAN.md — Expose proposal review and applied outcome continuity across shipped surfaces without turning them into planners
- [x] 32-04-PLAN.md — Close the phase with docs, examples, and verification for supervised planning-profile application

**Scope note:** This phase should make staged routine/planning edits resolvable in real use without widening planner authority:

- approved planning-profile proposals should apply through the same backend-owned mutation seam that direct Settings edits already use
- proposal state, approval, failure, and applied outcome should remain explicit in `Threads` continuity and operator-visible surfaces
- Apple voice, assistant entry, web, CLI, and Apple read surfaces should all preserve the same supervised story about what was merely proposed versus what actually changed
- do not widen into autonomous planner mutation, broad calendar editing, or a second planning model outside the canonical profile

### Phase 33: Approved day-plan and reflow application over commitment scheduling

**Goal:** Let bounded same-day `day_plan` and `reflow` outcomes move from explainable proposal state into supervised backend-owned commitment scheduling changes, without widening into broad autonomous calendar editing.
**Requirements**: DAYPLAN-05, DAYPLAN-06, REFLOW-REAL-03, SCHED-APPLY-01
**Depends on:** Phase 32
**Plans:** 4/4 plans complete

Plans:
- [x] 33-01-PLAN.md — Publish the approved day-plan/reflow application contract and lifecycle over commitment scheduling
- [x] 33-02-PLAN.md — Implement backend application of bounded day-plan/reflow changes through canonical commitment mutation seams
- [x] 33-03-PLAN.md — Expose pending and applied day-plan/reflow continuity across `Now`, `Threads`, CLI, and Apple without creating a second planner
- [x] 33-04-PLAN.md — Close the phase with docs, examples, and verification for supervised same-day plan application

**Scope note:** This phase should make same-day planning materially actionable while preserving trust and bounded authority:

- `day_plan` and `reflow` changes should apply through backend-owned commitment scheduling seams, not shell-local heuristics
- applied outcomes, failures, and pending review should remain explicit in `Threads` continuity and summary-first surfaces
- the lane should stay bounded to same-day commitment scheduling and explicit operator approval where required
- do not widen into autonomous multi-day optimization, broad upstream calendar mutation, or a second planner model

### Phase 34: Now-view simplification, current-day truth, and Vel.csv acceptance

**Goal:** Turn `Now` into a compact current-day control surface that orients the operator around what is happening now, what matters next, and one dominant ask/capture/talk affordance, while repairing broken calendar/Todoist rendering, duplicated UI, and low-value status clutter.
**Requirements**: NOW-UX-02, NOW-UX-03, CAL-UX-01, TODO-UX-01, VELCSV-01
**Depends on:** Phase 33
**Plans:** 4/4 plans complete

**Operator-input note:** This phase is anchored to explicit operator interview decisions, prior `Now` specs, and `~/Downloads/Vel.csv`. `Vel.csv` is a regression and acceptance input, not the primary product authority.
**Layout note:** The shipped `Now` order should follow perception → action → execution: compact context bar, current status, dominant ask/capture/talk, next event, unified today lane, compressed attention strip.

Plans:
- [x] 34-01-PLAN.md — Publish the compact `Now` contract, ranking rules, and `Vel.csv`-anchored acceptance baseline before UI changes widen
- [x] 34-02-PLAN.md — Repair current status, next-event truth, and aggressive calendar noise filtering over the current-day surface
- [x] 34-03-PLAN.md — Rebuild the unified commitment-first today lane, primary quick actions, and low-noise attention strip
- [x] 34-04-PLAN.md — Close duplicate/sloppy `Now` presentation issues, broken actions, and verification against `Vel.csv` pressure items

**Scope note:** This phase should make `Now` materially trustworthy and low-noise:

- `Now` should be a current-day control surface, not a dashboard or status dump
- calendar events and tasks should render through one unified today lane, but commitments remain primary
- duplicated data, verbose sync posture, and non-actionable status blocks should move out of the default surface
- the dominant ask/capture/talk affordance should remain inline-first and thread-backed, not chat-first

### Phase 35: Sleep-relative day boundary and today-lane correctness

**Goal:** Make `Now`, `next event`, commitments, tasks, and thread resurfacing agree on one sleep-relative current-day model instead of a midnight-bound or surface-local interpretation of “today.”
**Requirements**: DAYBOUND-01, DAYBOUND-02, NOW-ORDER-01, CONTEXT-01
**Depends on:** Phase 34
**Plans:** 4/4 plans complete

**Behavior note:** The day should extend across midnight until the sleep boundary is crossed, so unfinished work, night events, and routine continuity still belong to the same operator day when appropriate.
**Ranking note:** Commitment-first execution ordering should be backend-owned and shared across `Now`, thread resurfacing, and follow-through surfaces.

Plans:
- [x] 35-01-PLAN.md — Publish the sleep-relative day-boundary contract and current-day precedence rules before behavior widens
- [x] 35-02-PLAN.md — Implement multi-signal day-boundary handling and current-day event filtering over backend-owned time/context seams
- [x] 35-03-PLAN.md — Enforce commitment-first today-lane ordering, active-item precedence, and next-up semantics across `Now`
- [x] 35-04-PLAN.md — Close resurfacing, recency, and correctness gaps so one current-day truth holds across web, Apple, and thread continuity

**Scope note:** This phase should make “today” trustworthy:

- active item precedence must remain calendar event > active commitment > routine block > inferred activity
- next event should remain strictly calendar-driven while excluding routine/noise blocks
- tasks should not rise above commitments unless promoted
- if ranking confidence for contextual thread resurfacing is low, `Now` should show none rather than cluttering the surface

### Phase 36: Shell hierarchy, settings, and continuity simplification

**Goal:** Apply the corrected product hierarchy across `Now`, `Threads`, `Settings`, and the web shell so the daily-use experience is simpler, action-oriented, and less internally noisy without reintroducing shell-owned product logic.
**Requirements**: SHELL-03, SETTINGS-UX-02, THREADS-03, SIDEBAR-01
**Depends on:** Phase 35
**Plans:** 4/4 plans complete

**Hierarchy note:** `Now` remains the default action surface, `Threads` is continuity-first and should not become a chat inbox, `Settings` is advanced management, and the sidebar should become a thin icon rail instead of a required information column.
**Vel.csv note:** `Vel.csv` pressure around simplification, richer context, continuity, and de-emphasizing sync clutter should be treated as acceptance pressure for this phase.

Plans:
- [x] 36-01-PLAN.md — Publish the simplified shell hierarchy, sidebar role, and zero-or-one thread resurfacing rules
- [x] 36-02-PLAN.md — Restructure `Settings` into clearer subcategories with lower text clutter and summary-first defaults
- [x] 36-03-PLAN.md — Tighten `Threads`, action affordances, icon usage, and button-vs-link behavior around continuity rather than clutter
- [x] 36-04-PLAN.md — Verify the simplified shell against `Vel.csv` friction points and close daily-use slop across primary surfaces

**Scope note:** This phase should remove slop without widening scope:

- runtime/sync posture should compress into optional secondary affordances rather than dominating primary views
- `Settings` should manage durable profile/integration concerns without becoming onboarding prose or a debug dump
- sidebar context should be optional and ignorable, not mandatory for core operation
- continuity should resurface contextually rather than by promoting thread-centric UI everywhere

### Phase 37: iPhone embedded Rust core and Apple FFI foundation

**Goal:** Introduce the real iPhone embedded-core / Apple FFI path behind explicit platform and feature gates so Apple can start consuming the same Rust-owned product logic locally where it materially improves responsiveness, offline behavior, and future distribution posture.
**Requirements**: APPLE-EMBED-01, APPLE-EMBED-02, FFI-01, OFFLINE-01
**Depends on:** Phase 36
**Plans:** 4/4 plans complete

**Topology note:** This phase should add the embedded-capable Apple path described in Phase 13 without pretending the daemon-backed HTTP model is gone. The embedded path should be additive and iPhone-first.
**Boundary note:** Heavy recall, long-running jobs, integrations/sync, shared thread sync, and supervised apply/review lanes remain daemon-backed in this phase.

Plans:
- [x] 37-01-PLAN.md — Publish the iPhone embedded-core / FFI contract, feature-gating rules, and daemon-vs-embedded boundary map
- [x] 37-02-PLAN.md — Stand up the first embedded Rust bridge and local authority seams for iPhone-safe read/write flows
- [x] 37-03-PLAN.md — Route the highest-value iPhone local flows through the embedded seam without forking product logic from the canonical Rust core
- [x] 37-04-PLAN.md — Close docs, examples, and verification for the new embedded-capable Apple topology while preserving daemon truth

**Scope note:** This phase should create the real Apple FFI foundation without overreaching:

- start on iPhone only; do not split effort across watch and Mac yet
- preserve one Rust-owned domain and policy model across embedded and daemon-backed paths
- prefer local responsiveness and offline capability for bounded high-frequency flows
- do not widen into full local-reasoning parity or a second Apple-specific product brain

### Phase 38: Local-first iPhone voice continuity and offline action lane

**Goal:** Make iPhone voice capture and quick action flows feel local-first and dependable by using the new embedded seam for offline-capable voice continuity, cached `Now`, queued quick actions, and local thread draft follow-through.
**Requirements**: APPLE-OFFLINE-01, APPLE-OFFLINE-02, VOICE-06, THREADS-04
**Depends on:** Phase 37
**Plans:** 4/4 plans complete

**Magic-flow note:** The proving flow for this phase is smooth queued voice continuity on iPhone: speak, get instant acknowledgment, survive offline, and later see the result correctly merged into thread and `Now` continuity without duplicates or sync confusion.
**Guardrail note:** This phase should not attempt full local heavy recall, local long-running job orchestration, or fully local review/apply parity.

Plans:
- [x] 38-01-PLAN.md — Publish the local-first iPhone voice continuity and offline action contract over embedded plus daemon-backed seams
- [x] 38-02-PLAN.md — Implement local voice capture, queued continuity, and offline-safe quick actions on iPhone
- [x] 38-03-PLAN.md — Align `Now`, thread drafts, and recovery flows so embedded/offline state merges cleanly back into canonical continuity
- [x] 38-04-PLAN.md — Verify the “magical” iPhone voice/offline loop and document remaining daemon-backed limits honestly

**Scope note:** This phase should prove local-first Apple value:

- minimum acceptable offline behavior is cached `Now`, queued voice capture, local quick actions, and local thread draft continuation
- voice/text continuity should remain one thread-backed model rather than separate offline and online interaction modes
- daemon sync remains responsible for broader shared history, integrations, and heavier reasoning
- the local lane should feel fast and trustworthy without claiming universal local parity

### Phase 39: Vel.csv regression sweep and daily-use closeout

**Goal:** Close the next milestone by sweeping the remaining `Vel.csv` usability pressure, verifying the corrected daily-use loop across web and Apple, and ensuring the operator can run a day from Vel without fighting the product.
**Requirements**: VELCSV-02, DAILY-USE-01, DAILY-USE-02, APPLE-PARITY-01
**Depends on:** Phase 38
**Plans:** 4/4 plans complete

**Acceptance note:** `Vel.csv` should be used here as an explicit regression and closeout checklist, subordinate to the product rules already locked through operator interview and prior specs.
**Milestone note:** This phase should validate the whole arc: compact `Now`, trusted current-day truth, simplified shell hierarchy, and an iPhone local-first loop that complements rather than replaces daemon-backed continuity.

Plans:
- [x] 39-01-PLAN.md — Turn `Vel.csv` into a structured regression/acceptance matrix for the Phase 34-38 arc
- [x] 39-02-PLAN.md — Close remaining daily-use friction across `Now`, continuity, richer context, and settings driven by the acceptance matrix
- [x] 39-03-PLAN.md — Verify cross-surface web and Apple daily-use parity against the “good Vel day” acceptance spine
- [x] 39-04-PLAN.md — Record milestone-level closeout truth for the new arc with docs, evidence, and remaining deferred items made explicit

**Scope note:** This phase should close the loop on daily use:

- web and Apple should both support the operator’s “good Vel day” flow without major friction
- richer context should help action rather than add slop
- `Vel.csv` should function as a regression guardrail and closeout check, not as an alternate source of product truth
- unresolved items after this phase should be consciously deferred rather than left as silent UX drift

### Phase 40: Decision-first UI/UX rework across Now, Settings, Threads, and context surfaces

**Goal:** Rework the primary operator shell around decision-first, action-oriented UI so `Now`, `Threads`, `Settings`, and contextual panels each have one clear job, strict hierarchy, materially lower cognitive load, and fewer broken operator interactions on web and mobile.
**Requirements**: UI-AUDIT-01, UI-ACT-01, UI-HIER-01, NOW-STACK-01, SETTINGS-MODEL-01, THREADS-THINK-01, CONTEXTPANEL-01, UI-RELIAB-01
**Depends on:** Phase 39
**Design note:** This phase is driven by an explicit operator-supplied UI/UX rework spec rather than incremental polish. The job is to reduce cognitive load, make primary actions obvious, and stop leaking internal/runtime model state into default views.
**Reliability note:** This phase also includes a focused web/mobile interaction audit. If an affordance is visibly present but unreliable or non-working in daily use, it belongs in scope here.
**Planning note:** Discovery is part of the phase, not pre-phase overhead. The first planning slice should produce an audit matrix of what works, what is broken, and what is merely confusing across web and mobile.
**Product note:** One screen = one job:
  - `Now` → act
  - `Threads` → think
  - `Settings` → configure
**Plans:** 4 plans

Plans:
- [ ] 40-01-PLAN.md — Audit current web/mobile behavior and produce the discovery matrix of working vs broken vs confusing interactions
- [ ] 40-02-PLAN.md — Rework `Now` into the decision-first execution surface and repair broken primary interactions
- [ ] 40-03-PLAN.md — Rework `Settings`, `Threads`, and context panels around one clear job each while fixing discovered affordance failures
- [ ] 40-04-PLAN.md — Close Phase 40 with cross-surface verification, docs truth, and explicit deferred interaction gaps
