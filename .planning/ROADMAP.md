# Roadmap: Vel

## Overview

Phase 1 and Phase 3 are complete. Phase 2 and Phase 4 are closed historical baselines with unfinished original-scope work explicitly re-scoped into Phases 5, 6, and 8. There is no remaining active roadmap work before Phase 5. The active roadmap now begins with the product-shaping sequence focused on `Now + Inbox`, project substrate, high-value write-back integrations, Apple action loops, coding-centric supervised execution, and backup-first trust surfaces (Phases 5-9). Each remaining phase produces a verifiable capability boundary before the next begins.

## Phases

**Phase Numbering:**
- Phases 2–4 continue from completed Phase 1
- Integer phases only; decimal phases created via `/gsd:insert-phase` if urgent work is needed

- [x] **Phase 1: Structural Foundation** - Layered crates, auth hardening, canonical schemas, self-awareness (COMPLETE)
- [x] **Phase 1.1: Preflight — Pre-Phase 2 Hardening** (INSERTED) - Integration startup panic fixes, SQLite WAL mode, app.rs decomposition (COMPLETE)
- **Phase 2: Distributed State, Offline Clients & System-of-Systems** - Closed historical baseline; unfinished sync ordering, external connect transport, and guided node-linking work moved to Phases 5, 6, and 8
- [x] **Phase 3: Deterministic Verification & Continuous Alignment** - Day-simulation harness, LLM-as-a-Judge eval, execution tracing, user documentation (COMPLETE)
- **Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution** - Closed historical baseline; unfinished semantic graph expansion, direct WASM guest runtime, and external limb transport work moved to Phases 6 and 8
- [ ] **Phase 5: Now + Inbox core and project substrate** - Keep `Now + Inbox` primary while adding durable project structure and shared workspace contracts
- [ ] **Phase 6: High-value write-back integrations and lightweight people graph** - Add notes, reminders, GitHub, email, transcripts, and minimal people identity with upstream write-back
- [ ] **Phase 7: Apple action loops and behavioral signal ingestion** - Prioritize fast iOS/watch actions and directly useful behavior signals
- [ ] **Phase 8: Coding-centric supervised execution with GSD and local agents** - Launch and supervise coding-first runtimes with direct GSD integration and local-agent support
- [ ] **Phase 9: Backup-first trust surfaces and simple operator control** - Add backup-first trust workflows and keep control/config surfaces simple

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
Remaining execution order: 5 → 6 → 7 → 8 → 9

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Structural Foundation | - | Complete | 2026-03-18 |
| 1.1. Preflight — Pre-Phase 2 Hardening | 1/1 | Complete | 2026-03-18 |
| 2. Distributed State, Offline Clients & System-of-Systems | 1/7 | Closed / Re-scoped | 2026-03-19 |
| 3. Deterministic Verification & Continuous Alignment | 5/5 | Complete | 2026-03-18 |
| 4. Autonomous Swarm, Graph RAG & Zero-Trust Execution | 5/5 | Closed / Re-scoped | 2026-03-19 |
| 5. Now + Inbox core and project substrate | 9/9 | Complete | 2026-03-19 |
| 6. High-value write-back integrations and lightweight people graph | 3/7 | In Progress | - |
| 7. Apple action loops and behavioral signal ingestion | 0/0 | Not planned | - |
| 8. Coding-centric supervised execution with GSD and local agents | 0/0 | Not planned | - |
| 9. Backup-first trust surfaces and simple operator control | 0/0 | Not planned | - |

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
**Plans:** 3/7 plans executed

Plans:
- [x] 06-01-PLAN.md — Publish typed Phase 06 contracts, schemas, and owner docs for write-back, conflicts, and people
- [x] 06-02-PLAN.md — Install deterministic ordering, conflict queue, write-back history, and upstream ownership foundations
- [x] 06-03-PLAN.md — Close the Todoist lane with typed write-back, project linkage, and conflict handling
- [ ] 06-04-PLAN.md — Add scoped notes write-back, transcript-under-notes folding, and reminder intent execution tracking
- [ ] 06-05-PLAN.md — Ship the minimal people registry and provenance-bearing graph expansion over durable Phase 06 entities
- [ ] 06-06-PLAN.md — Add bounded GitHub and email provider slices with typed project/people linkage
- [ ] 06-07-PLAN.md — Surface write-back, conflicts, provenance, and people status through operator views, CLI, and docs

### Phase 7: Apple action loops and behavioral signal ingestion

**Goal:** Make Vel useful from iPhone/watch first through fast capture and response loops, while ingesting lightweight behavioral signals that improve daily orientation without making health or astrology core dependencies.
**Requirements**: IOS-01, IOS-02, IOS-03, HEALTH-01, HEALTH-02, APPLE-01
**Depends on:** Phase 6
**Plans:** 0 plans

Plans:
- [ ] TBD (phase should prioritize voice capture, current schedule retrieval, nudge response, and step/stand/exercise signal ingestion with explainable summaries)

### Phase 8: Coding-centric supervised execution with GSD and local agents

**Goal:** Extend Vel from daily orientation into supervised execution for coding-first work by letting projects carry repo/GSD context, generating repo-local planning artifacts that GSD can consume, and routing work by token budget, agent profile, task type, and explicit handoff boundaries.
**Requirements**: EXEC-01, EXEC-02, GSD-01, GSD-02, HANDOFF-01, HANDOFF-02, LOCAL-01, POLICY-01
**Depends on:** Phase 7
**Plans:** 0 plans

Plans:
- [ ] TBD (phase should define project execution context, human-to-agent and agent-to-agent handoff contracts for coding work, GSD handoff docs/contracts, token-budget-aware launch policy, supervised local coding-agent support, the re-scoped external connect transport/auth surface, and direct WASM-guest/runtime follow-on beyond the shipped host-executor baseline)

### Phase 9: Backup-first trust surfaces and simple operator control

**Goal:** Add lightweight backup/export and simple control surfaces that reduce fear of loss, while keeping restore/recovery and advanced policy surfaces intentionally smaller than the core daily loop.
**Requirements**: BACKUP-01, BACKUP-02, CTRL-01, CTRL-02
**Depends on:** Phase 8
**Plans:** 0 plans

Plans:
- [ ] TBD (phase should prioritize backup/export trust, inspectable control surfaces, and operator-visible safety state before deeper recovery automation)
