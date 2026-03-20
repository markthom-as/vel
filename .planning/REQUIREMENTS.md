# Requirements: Vel

**Defined:** 2026-03-18
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.

## v1 Requirements

Requirements for milestone Phases 2–17 plus milestone closeout integrity. Phase 1 is complete. Each requirement maps to a master plan ticket, roadmap phase contract, or closeout slice.

### Signal Ingestion & Sync (Phase 2)

- [ ] **SIG-01**: System ingests signals from pluggable sources via a context reducer pipeline (ticket 004)
- [ ] **SIG-02**: Signal sources can be composed and reduced into unified context state (ticket 004)
- [ ] **SYNC-01**: Distributed nodes achieve consistent ordering via Hybrid Logical Clocks (ticket 005)
- [ ] **SYNC-02**: Conflict resolution is deterministic and reproducible given the same event log (ticket 005)

### Agent Connect (Phase 2)

- [ ] **CONN-01**: Agent processes can be launched via a supervised connect protocol (ticket 006)
- [ ] **CONN-02**: Launched agents run under defined lifecycle supervision (start, heartbeat, terminate) (ticket 006)
- [ ] **CONN-03**: New nodes can discover and onboard to the system without manual configuration (ticket 012)
- [ ] **CONN-04**: Onboarding flow exposes tester-readiness checks and diagnostics (ticket 012)

### Capability & Security (Phase 2)

- [x] **CAP-01**: Agent capabilities are brokered — agents request scoped permissions, not raw credentials (ticket 016)
- [x] **CAP-02**: Secrets are mediated through indirection; prompt-visible raw credentials are prohibited (ticket 016)

### Operator Experience (Phase 2)

- [x] **OPS-01**: Operator surfaces (CLI, web) expose effective configuration state clearly (ticket 019)
- [x] **OPS-02**: Accessibility requirements are met for the operator dashboard (ticket 019)

### Deterministic Verification (Phase 3)

- [x] **VERIFY-01**: A day-simulation harness can replay a recorded day deterministically (ticket 007)
- [x] **VERIFY-02**: Simulation output is stable across runs given the same input log (ticket 007)
- [x] **EVAL-01**: An LLM-as-a-Judge pipeline evaluates agent reasoning outputs (ticket 008)
- [x] **EVAL-02**: Eval results are stored and queryable for regression tracking (ticket 008)

### Execution Observability (Phase 3)

- [x] **TRACE-01**: Agent runs produce stable run IDs with associated traces (ticket 017)
- [x] **TRACE-02**: Handoffs between agents are recorded with event linkage (ticket 017)
- [x] **TRACE-03**: Execution history is reviewable from the operator dashboard (ticket 017)

### Documentation (Phase 3)

- [x] **DOCS-01**: Comprehensive user documentation covers all operator-facing workflows (ticket 013)
- [x] **DOCS-02**: A support wiki exists and is indexed for search (ticket 013)

### Semantic Memory (Phase 4)

- [ ] **MEM-01**: The system maintains a semantic memory graph over captured entities (ticket 009)
- [ ] **MEM-02**: Graph RAG retrieval surfaces contextually relevant memories during reasoning (ticket 009)

### Agent Sandboxing (Phase 4)

- [ ] **SAND-01**: Untrusted agents execute in WASM sandbox with zero-trust defaults (ticket 010)
- [ ] **SAND-02**: Sandboxed agents interact with the Brain only via declared capability contracts (ticket 010)

### Swarm SDK (Phase 4)

- [ ] **SDK-01**: A first-class `vel-agent-sdk` exists for building external agent Limbs (ticket 014)
- [ ] **SDK-02**: SDK agents communicate with veld via the standardized swarm execution contract (ticket 014)
- [ ] **SDK-03**: SDK is documented and includes at least one reference implementation (ticket 014)

### Now + Inbox Core And Project Substrate (Phase 5)

- [x] **NOW-01**: `Now` exposes backend-owned ranked actions and intervention pressure as the primary operator orientation surface
- [x] **NOW-02**: `Now` carries typed review and continuity state without client-local queue heuristics
- [x] **INBOX-01**: `Inbox` exposes typed triage rows with explicit available actions over shared backend-owned state
- [x] **INBOX-02**: `Inbox` reuses canonical thread handoff and persisted triage mutations rather than a separate message-only model
- [x] **ACTION-01**: A canonical typed action/intervention model exists across `Now`, `Inbox`, sync/bootstrap, and review surfaces
- [x] **REVIEW-01**: Review outputs reuse typed review snapshots and project/action context instead of raw count blobs
- [x] **CONTINUITY-01**: Sync/bootstrap preserves backend-owned projects, action items, and linking state for multi-client continuity
- [x] **CONTINUITY-02**: Web, CLI, and Apple surfaces stay aligned to one shared continuity model
- [x] **PROJ-01**: Typed local-first projects can be created, listed, and inspected through backend-owned APIs
- [x] **PROJ-02**: Synthesis and review resolve against canonical typed project records
- [x] **PROJ-03**: Projects preserve durable roots, metadata, and grouping state
- [x] **FAMILY-01**: `Personal`, `Creative`, and `Work` are canonical project families

### Write-Back, Conflict, And People Closure (Phase 6)

- [x] **WB-01**: High-value integrations support bounded typed write-back flows rather than raw provider mutations
- [x] **WB-02**: Write-back attempts persist durable records with explicit status and provenance
- [x] **WB-03**: Operators can review pending write-back pressure through shared trust/queue surfaces
- [x] **CONFLICT-01**: Upstream-vs-local conflicts are persisted and surfaced for operator review rather than silently overwritten
- [x] **PROV-01**: Write-back and linked-entity flows preserve provenance and upstream ownership state
- [x] **RECON-01**: Deterministic reconciliation primitives exist for upstream/local ordering and follow-on repair
- [x] **TODO-01**: Todoist writes use typed Vel-native fields and explicit upstream mapping rules
- [x] **NOTES-01**: Notes writes stay scoped to allowed roots and preserve blocked-write evidence when out of bounds
- [x] **REMIND-01**: Reminder intents execute through a bounded typed lifecycle with pending/conflict visibility
- [x] **GH-01**: GitHub write-back stays bounded and linked to project/operator context
- [x] **EMAIL-01**: Email/provider write-back stays bounded and linked to project/operator context
- [x] **PEOPLE-01**: A lightweight people registry exists with durable identity and alias linkage
- [x] **PEOPLE-02**: People-linked review and evidence surfaces remain explainable in operator views

### Apple Action Loops And Behavioral Signals (Phase 7)

- [x] **IOS-01**: iPhone surfaces consume backend-owned `Now`/voice/action-loop data instead of Swift-local policy
- [x] **IOS-02**: Apple quick loops preserve offline cache/use patterns without inventing new local authority
- [x] **IOS-03**: Apple action flows stay aligned to authenticated shared transport boundaries
- [x] **HEALTH-01**: Behavioral signal ingestion is bounded and explainable rather than a new opaque planning engine
- [x] **HEALTH-02**: Apple behavior summaries surface through backend-owned summaries and typed transport
- [x] **APPLE-01**: Watch/iPhone shells share one backend-owned Apple quick-loop model

### Coding-Centric Supervised Execution (Phase 8)

- [x] **EXEC-01**: Projects can carry execution context suitable for supervised coding work
- [x] **EXEC-02**: Supervised execution stays reviewable through explicit handoff and routing surfaces
- [x] **GSD-01**: Repo-local workflow artifacts and docs support GSD-aware supervised execution
- [x] **GSD-02**: GSD-oriented workflow surfaces stay aligned to the shipped runtime contracts
- [x] **HANDOFF-01**: Human-to-agent and agent-to-agent handoffs remain explicit and reviewable
- [x] **HANDOFF-02**: Execution handoff surfaces preserve bounded context and operator review state
- [x] **LOCAL-01**: Local-agent and guest-runtime launch paths are available through the supervised connect boundary
- [x] **POLICY-01**: Execution routing and launch policy remain explicit and bounded instead of ambient

### Backup Trust Surfaces (Phase 9)

- [x] **BACKUP-01**: Operators can create and inspect a typed local backup/export pack that captures the database snapshot, artifact coverage, config coverage, explicit omissions, and verification summary for the Vel core state (ticket 09-01)
- [x] **BACKUP-02**: Backup/export confidence is surfaced clearly through inspectable status and coverage data, while restore automation remains secondary to manual inspection and operator trust (ticket 09-01)
- [x] **CTRL-01**: Backup/control surfaces remain simple, typed, and bounded to high-value runtime state instead of becoming a generic configuration editor (ticket 09-01)
- [x] **CTRL-02**: Safety state and trust posture are visible so operators can inspect what is safe to trust before taking action (ticket 09-01)

### Daily Loop MVP (Phase 10)

- [x] **MORNING-01**: Operators can manually start a daily morning session now, with the contract shaped for future automatic start, using next-12h calendar inputs plus Todoist today/overdue inputs (ticket 10-01)
- [x] **MORNING-02**: Morning Overview returns a short passive snapshot, no more than two friction callouts, and one to three intent-gathering questions without becoming a verbose dashboard or coaching flow (ticket 10-01)
- [x] **MORNING-03**: Morning Overview captures signals only and must not create durable commitments before Standup runs (ticket 10-01)
- [x] **STANDUP-01**: Standup can resume after Morning Overview or start directly, and must reconcile calendar plus compress tasks into must/should/stretch buckets (ticket 10-01)
- [x] **STANDUP-02**: Standup must end with one to three bounded daily commitments plus explicit deferred tasks, confirmed calendar state, and proposed focus blocks (ticket 10-01)
- [x] **STANDUP-03**: Standup enforces the three-commitment cap, reprompts once when no commitments are defined, and stays interruptible/skippable/resumable throughout (ticket 10-01)
- [x] **SESSION-01**: Daily-loop state is typed, durable, inspectable, and resumable without deepening untyped context blobs or client-local policy state (ticket 10-01)
- [x] **VOICE-01**: Voice-first entry and resume reuse backend-owned Apple/runtime seams while remaining available through text-capable operator shells too (ticket 10-01)

### Agent Grounding (Phase 11)

- [x] **AGENT-CTX-01**: A typed grounding bundle exposes `Now`, projects, people, commitments, review obligations, and execution handoffs for supervised agents (ticket 11-01)
- [x] **AGENT-CTX-02**: Agent grounding remains inspectable, traceable, and bounded to persisted records plus explicit explain/reference fields rather than raw unbounded dumps (ticket 11-01)
- [x] **AGENT-TOOLS-01**: Operator-visible capability summaries describe bounded read, review, and mutation affordances in operator terms instead of low-level internal tool names (ticket 11-01)
- [x] **AGENT-TOOLS-02**: Missing grants, blocked mutations, or unsupported requests fail closed and expose the narrow escalation or review gate required to proceed (ticket 11-01)
- [x] **AGENT-REVIEW-01**: Existing review queues, SAFE MODE constraints, and approval flows remain intact while being surfaced in agent-relevant terms (ticket 11-01)
- [x] **AGENT-TRUST-01**: Operator surfaces show what an agent can currently see and do, plus why, from one backend-owned policy contract shared across API, CLI, and web surfaces (ticket 11-01)

### Operator Shell, Onboarding, And Connector Ergonomics (Phase 12)

Note: Phase 12 also reinforced `DOCS-01` through contextual setup/help work, but the shared docs requirement remains tracked once under Phase 3.

- [x] **SHELL-01**: The operator shell teaches the primary surfaces and help paths more clearly
- [x] **SHELL-02**: Shell navigation and freshness/status framing align better to daily use
- [x] **ONBOARD-01**: Settings surfaces provide a usable onboarding and recovery checklist from current runtime truth
- [x] **INTEGR-UX-01**: Connector and local-source ergonomics provide clearer next steps and recovery guidance
- [x] **PROJ-UX-01**: Project detail and settings surfaces are easier to navigate without widening client-owned policy

### Cross-Surface Core Architecture (Phase 13)

- [x] **ARCH-XS-01**: Cross-surface architecture docs define one canonical Rust-owned product core across shells
- [x] **ARCH-XS-02**: Current-state and target-state migration guidance are explicit for the existing codebase
- [x] **ADAPT-01**: Adapter boundaries for Apple, web, and future desktop surfaces are documented clearly
- [x] **ADAPT-02**: Contract vocabulary for commands, queries, events, and read models is stabilized
- [x] **APPLE-ARCH-01**: The future Apple embedded/FFI path is documented without rewriting current HTTP-first truth
- [x] **API-ARCH-01**: One real proof flow demonstrates the architecture against shipped backend behavior

### Product Discovery And Operator Modes (Phase 14)

- [x] **PROD-01**: The default operator product shape is defined before broader UI proliferation
- [x] **MODE-01**: Operator-mode boundaries distinguish default, advanced, and internal concerns
- [x] **UX-CORE-01**: `Now`, `Inbox`, `Threads`, and `Projects` boundaries are explicitly defined
- [x] **TRUST-UX-01**: Trust, check-in, and reflow behaviors are defined as product concepts rather than UI accidents
- [x] **ONBOARD-02**: Onboarding and recovery journeys are shaped as operator product flows
- [x] **ROADMAP-01**: Future phases and milestone shaping reflect the product taxonomy rather than pre-Phase-14 UI drift

### Incremental Core Migration (Phase 15)

- [x] **MIGRATE-01**: Existing operator seams are incrementally migrated toward canonical backend ownership instead of duplicated in shells
- [x] **MIGRATE-02**: Migration preserves the live codebase shape instead of forcing a large crate rewrite
- [x] **SERVICE-01**: Canonical service seams exist for operator actions like `check_in` and `reflow`
- [x] **DTO-01**: Shared transport DTOs reflect the canonical operator action model cleanly
- [x] **READMODEL-01**: Shared read-model ownership stays backend-owned rather than client-derived

### Logic-First Product Closure (Phase 16)

- [x] **LOGIC-01**: Core operator logic closes on backend-owned seams before broad shell expansion
- [x] **FLOW-01**: `check_in`, `reflow`, trust follow-through, and project escalation become canonical product flows
- [x] **MODE-02**: Product-mode behavior is implemented against the shared action model
- [x] **READMODEL-02**: Canonical read models carry the product logic needed by shells without local policy invention
- [x] **SHELL-ARCH-01**: Later shell work can embody backend-owned logic instead of designing it ad hoc

### Shell Embodiment And Simplification (Phase 17)

- [x] **SHELL-MODE-01**: Web, Apple, and CLI shells apply the Phase 14 taxonomy consistently
- [x] **SHELL-MODE-02**: Default operator surfaces are simplified while advanced/detail surfaces are progressively disclosed
- [x] **TRUST-SUMMARY-01**: Trust/setup/runtime concerns are rendered as summary-first operator information instead of first-contact clutter
- [x] **APPLE-SHELL-01**: Apple shell wording and grouping align to the same product hierarchy as web and CLI

### Milestone Closeout Integrity (Phases 18-19)

- [x] **CLOSEOUT-01**: Milestone phases 2-17 have durable verification coverage sufficient to support milestone-level audit and archival claims
- [x] **CLOSEOUT-02**: `REQUIREMENTS.md` traceability and checked-off state are reconciled against phase summaries and verification evidence before milestone archival
- [ ] **CLOSEOUT-03**: `ROADMAP.md`, `STATE.md`, and milestone archive inputs are internally consistent and reflect the true shipped state before archival/tagging
- [ ] **CLOSEOUT-04**: Milestone re-audit passes with explicit cross-phase integration and end-to-end closeout evidence before `gsd-complete-milestone`

### Grounded Assistant Entry And Daily-Use Closure (Phase 20)

- [ ] **USABLE-01**: The grounded assistant becomes a practical default operator entry instead of a side chat surface
- [ ] **USABLE-02**: Daily-use friction across `Now`, `Inbox`, `Threads`, and setup is reduced enough for repeated operator use
- [ ] **NOW-UX-01**: `Now` better balances urgent inline actions with subtle links into deeper surfaces
- [ ] **INBOX-UX-01**: `Inbox` better supports explicit triage over the shared action model without turning into a second thread archive
- [ ] **THREADS-UX-01**: `Threads` better support continuity, search, and assistant escalation over real product state
- [ ] **ENTRY-01**: Capture, text conversation, and operator intent entry converge on one clearer assistant-first path
- [ ] **SETTINGS-UX-01**: Default setup friction drops without expanding advanced/runtime complexity
- [ ] **ASSIST-01**: One backend-owned grounded assistant seam powers the default text/capture entry path over real Vel data and bounded tools
- [ ] **ASSIST-02**: Configured remote LLM routing, including localhost `openai_oauth`, remains optional, bounded, and compatible with the local-first core
- [ ] **THREADS-02**: Assistant continuity and escalation preserve thread ownership and product boundaries instead of inventing a separate assistant archive model

### Cross-Surface Voice Assistant Parity (Phase 21)

- [ ] **VOICE-02**: Voice input across web/desktop and Apple reuses the same grounded assistant behavior as typed input
- [ ] **VOICE-03**: Transcript provenance, intent routing, and fallback behavior remain explicit and inspectable across voice surfaces
- [ ] **APPLE-VOICE-02**: Apple voice stops carrying shell-specific product logic that should live in the shared assistant/runtime seam
- [ ] **DESKTOP-VOICE-01**: Desktop/browser push-to-talk remains fast and practical with local speech-to-text when available
- [ ] **DESKTOP-VOICE-02**: Voice surfaces preserve thread continuity and do not fork into a second-class “voice-only” interaction model

### Assistant-Supported Daily Loop, Closeout, And Thread Resolution (Phase 22)

- [ ] **DAILY-AI-01**: Morning overview and standup can be entered or resumed through the grounded assistant without bypassing the typed daily-loop authority
- [ ] **DAILY-AI-02**: Assistant-capable daily-loop flows preserve durable prompts, responses, and session state across shells
- [ ] **EOD-01**: End-of-day becomes a first-class assistant-capable closure flow over persisted Vel state
- [ ] **EOD-02**: End-of-day summaries and follow-ups remain explainable from persisted records rather than shell-local synthesis
- [ ] **THREAD-RES-01**: Longer check-in, reflow, and item-resolution work can escalate into durable threads cleanly
- [ ] **THREAD-RES-02**: Thread-based resolution preserves why an item was resolved, deferred, edited, or left pending

### Safe Assistant-Mediated Actions (Phase 23)

- [ ] **ASSIST-ACT-01**: The assistant can stage bounded actions through the existing operator-action and review model without bypassing trust gates
- [ ] **ASSIST-ACT-02**: Assistant-proposed mutations remain explicit, inspectable, and reversible where the existing product contract requires it
- [ ] **REVIEW-02**: Review queues and approval surfaces can accept assistant-originated proposals without losing provenance or supervision
- [ ] **TRUST-02**: SAFE MODE, writeback grants, and trust/readiness posture still fail closed when the assistant attempts a gated action
- [ ] **ASSIST-APPLY-01**: Explicitly approved assistant proposals can advance from staged to applied through the canonical operator-action, execution-review, or writeback lanes instead of stopping at proposal state

### Routine Blocks And Commitment-Aware Day Planning (Phase 28)

- [ ] **DAYPLAN-01**: Vel can shape an initial same-day plan from calendar anchors, open commitments, and canonical scheduler rules before schedule drift occurs
- [ ] **DAYPLAN-02**: Planned-day output remains explainable with explicit scheduled, deferred, and did-not-fit outcomes instead of opaque planner state
- [ ] **ROUTINE-01**: Routine blocks become typed backend-owned planning inputs rather than shell-only hints or raw labels
- [ ] **ROUTINE-02**: Morning/day-plan surfaces consume one shared backend-owned plan contract across CLI, web, assistant, and Apple pathways
- [ ] **ASSIST-APPLY-02**: Applied assistant outcomes preserve provenance, resulting state, and reversible follow-through where the existing product contract already supports reversal
- [ ] **REVIEW-03**: Review and approval surfaces can complete assistant-originated proposals into applied outcomes without losing thread, handoff, or action lineage
- [ ] **TRUST-03**: SAFE MODE, writeback grants, approval gates, and trust/readiness posture still fail closed during apply, retry, and reverse paths for assistant-mediated work

### Durable Routine Blocks And Operator-Managed Planning Constraints (Phase 29)

- [ ] **ROUTINE-03**: Routine blocks can persist as durable backend-owned planning records rather than only inferred current-context hints
- [ ] **ROUTINE-04**: Operators can manage bounded routine/planning constraints through typed backend-owned seams without creating shell-owned planning logic
- [ ] **DAYPLAN-03**: Day shaping can consume durable routine blocks and constraints over the same backend-owned planning substrate as `reflow`
- [ ] **DAYPLAN-04**: Shipped surfaces can summarize durable routine-backed planning posture without implying multi-day autonomy or a second planner

### Cross-Surface Planning-Profile Parity And Assistant-Managed Routine Edits (Phase 31)

- [ ] **PLANPROFILE-03**: CLI, Apple, and assistant-capable entry points can inspect the same canonical planning profile instead of relying on shell-local routine state
- [ ] **PLANPROFILE-04**: Assistant or voice-driven routine/profile edits route through the typed planning-profile mutation seam with explicit confirmation and provenance
- [ ] **ROUTINE-07**: Durable routine and planning-constraint management remains one backend-owned substrate across web, CLI, Apple, and assistant flows
- [ ] **VOICE-04**: Voice-capable planning-profile edits preserve the same bounded confirmation, thread continuity, and fail-closed behavior as typed assistant entry

### Now Simplification, Current-Day Truth, And Vel.csv Acceptance (Phase 34)

- [ ] **NOW-UX-02**: `Now` becomes a compact execution-first current-day surface with a stable top-to-bottom order of context, current status, ask/capture/talk, next event, today lane, and compressed attention indicators
- [ ] **NOW-UX-03**: The primary `Now` surface removes duplicated UI, verbose sync/runtime clutter, and non-actionable status blocks from default view
- [ ] **CAL-UX-01**: Calendar rendering on `Now` defaults to aggressive relevance filtering and correct next-event truth for the current day
- [ ] **TODO-UX-01**: The today lane renders commitment-first work and Todoist-backed tasks with cleaner structure, correct ordering, and meaningful quick actions over the broader task abstraction
- [ ] **VELCSV-01**: The next UI cleanup lane uses `Vel.csv` as a regression and acceptance input while keeping operator interview decisions and prior specs as product authority

### Sleep-Relative Day Boundary And Today-Lane Correctness (Phase 35)

- [ ] **DAYBOUND-01**: `Now`, next-event truth, and today-lane membership use one sleep-relative day boundary instead of midnight-local heuristics
- [ ] **DAYBOUND-02**: Late-night unfinished work, night events, and routine continuity remain in the same operator day until the sleep boundary is crossed
- [ ] **NOW-ORDER-01**: The unified today lane stays commitment-first and execution-ordered, with tasks demoted unless explicitly promoted into commitments
- [ ] **CONTEXT-01**: Current-status precedence remains calendar event > active commitment > routine block > inferred activity across surfaces and follow-through logic

### Shell Hierarchy, Settings, And Continuity Simplification (Phase 36)

- [ ] **SHELL-03**: The daily-use shell hierarchy is simplified so `Now` stays primary, `Threads` stays continuity-first, `Settings` stays advanced, and shell noise drops materially
- [ ] **SETTINGS-UX-02**: `Settings` becomes a clearer summary-first management surface with stronger categories, less top-level clutter, and less unnecessary save/sync friction
- [ ] **THREADS-03**: `Now` resurfaces at most one highly relevant thread contextually, and `Threads` avoids becoming a default live queue or chat inbox
- [ ] **SIDEBAR-01**: The web sidebar becomes an optional thin icon rail that keeps secondary sync/debug/context state available without dominating core use

### iPhone Embedded Rust Core And Apple FFI Foundation (Phase 37)

- [ ] **APPLE-EMBED-01**: iPhone gains an explicit embedded-capable Rust path behind feature/platform gates while preserving the canonical daemon-backed model as current truth
- [ ] **APPLE-EMBED-02**: Embedded and daemon-backed Apple flows reuse one Rust-owned domain/policy model rather than diverging into separate product logic
- [ ] **FFI-01**: The first Apple FFI bridge and embedded boundary are documented, typed, and testable without claiming full local parity
- [ ] **OFFLINE-01**: The embedded-capable iPhone path materially improves bounded offline/local behavior for high-frequency flows without widening to heavy local reasoning

### Local-First iPhone Voice Continuity And Offline Action Lane (Phase 38)

- [ ] **APPLE-OFFLINE-01**: iPhone can render cached `Now`, queue voice capture, and preserve local quick actions while offline over the embedded-capable path
- [ ] **APPLE-OFFLINE-02**: Local thread draft continuation and later sync merge behave cleanly without duplicate, lost, or confusing continuity state
- [ ] **VOICE-06**: iPhone voice capture becomes the first “magical” local-first Apple flow with fast acknowledgment, queued continuity, and clean eventual thread/`Now` integration
- [ ] **THREADS-04**: Voice and text continuity remain one thread-backed model across offline and online transitions instead of forking into separate local modes

### Vel.csv Regression Sweep And Daily-Use Closeout (Phase 39)

- [ ] **VELCSV-02**: Remaining `Vel.csv` usability pressure is converted into a structured regression/acceptance sweep for the repaired daily-use loop
- [ ] **DAILY-USE-01**: Web and Apple can support a real wake-up-to-closeout daily-use flow without major friction in `Now`, continuity, and quick actions
- [ ] **DAILY-USE-02**: Richer context and follow-through surfaces help action rather than adding slop or status-heavy clutter
- [ ] **APPLE-PARITY-01**: Apple local-first flows and daemon-backed continuity tell one coherent product story across web, Apple, and backend-owned state

## v2 Requirements

Deferred beyond current milestone scope.

### Swarm Coordination
- **SWARM-01**: Peer-to-peer sync without central authority node
- **SWARM-02**: Multi-operator federation

### Mobile
- **MOB-01**: iOS Limb client reaches feature parity with web dashboard for daily capture

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multi-tenant cloud hosting | Local-first by design; single operator |
| Fine-grained RBAC | Token-based auth is sufficient for this swarm scale |
| Real-time collaborative editing | Single-operator model; no conflict UI needed |
| Public API / SaaS | Not a platform product |

## Traceability

| Requirement | Phase | Ticket | Status |
|-------------|-------|--------|--------|
| SIG-01 | Phase 2 | 004 | Baseline only — not fully satisfied in live Phase 2 |
| SIG-02 | Phase 2 | 004 | Baseline only — not fully satisfied in live Phase 2 |
| SYNC-01 | Phase 2 | 005 | Deferred to Phase 6 |
| SYNC-02 | Phase 2 | 005 | Deferred to Phase 6 |
| CONN-01 | Phase 2 | 006 | Deferred to Phase 8 |
| CONN-02 | Phase 2 | 006 | Deferred to Phase 8 |
| CONN-03 | Phase 2 | 012 | Deferred to Phase 5 |
| CONN-04 | Phase 2 | 012 | Deferred to Phase 5 |
| CAP-01 | Phase 2 | 016 | Satisfied (Phase 2 baseline) |
| CAP-02 | Phase 2 | 016 | Satisfied (Phase 2 baseline) |
| OPS-01 | Phase 2 | 019 | Satisfied |
| OPS-02 | Phase 2 | 019 | Satisfied |
| VERIFY-01 | Phase 3 | 007 | Satisfied |
| VERIFY-02 | Phase 3 | 007 | Satisfied |
| EVAL-01 | Phase 3 | 008 | Satisfied |
| EVAL-02 | Phase 3 | 008 | Satisfied |
| TRACE-01 | Phase 3 | 017 | Satisfied |
| TRACE-02 | Phase 3 | 017 | Satisfied |
| TRACE-03 | Phase 3 | 017 | Satisfied |
| DOCS-01 | Phase 3 | 013 | Satisfied (reinforced by Phase 12) |
| DOCS-02 | Phase 3 | 013 | Satisfied |
| MEM-01 | Phase 4 | 009 | Baseline only — fuller graph expansion deferred to Phase 6 |
| MEM-02 | Phase 4 | 009 | Baseline only — fuller graph expansion deferred to Phase 6 |
| SAND-01 | Phase 4 | 010 | Baseline only — direct WASM guest runtime deferred to Phase 8 |
| SAND-02 | Phase 4 | 010 | Baseline only — direct WASM guest runtime deferred to Phase 8 |
| SDK-01 | Phase 4 | 014 | Baseline only — external transport closure deferred to Phase 8 |
| SDK-02 | Phase 4 | 014 | Baseline only — external transport closure deferred to Phase 8 |
| SDK-03 | Phase 4 | 014 | Baseline only — external transport closure deferred to Phase 8 |
| NOW-01 | Phase 5 | roadmap phase contract | Satisfied |
| NOW-02 | Phase 5 | roadmap phase contract | Satisfied |
| INBOX-01 | Phase 5 | roadmap phase contract | Satisfied |
| INBOX-02 | Phase 5 | roadmap phase contract | Satisfied |
| ACTION-01 | Phase 5 | roadmap phase contract | Satisfied |
| REVIEW-01 | Phase 5 | roadmap phase contract | Satisfied |
| CONTINUITY-01 | Phase 5 | roadmap phase contract | Satisfied |
| CONTINUITY-02 | Phase 5 | roadmap phase contract | Satisfied |
| PROJ-01 | Phase 5 | roadmap phase contract | Satisfied |
| PROJ-02 | Phase 5 | roadmap phase contract | Satisfied |
| PROJ-03 | Phase 5 | roadmap phase contract | Satisfied |
| FAMILY-01 | Phase 5 | roadmap phase contract | Satisfied |
| WB-01 | Phase 6 | roadmap phase contract | Satisfied |
| WB-02 | Phase 6 | roadmap phase contract | Satisfied |
| WB-03 | Phase 6 | roadmap phase contract | Satisfied |
| CONFLICT-01 | Phase 6 | roadmap phase contract | Satisfied |
| PROV-01 | Phase 6 | roadmap phase contract | Satisfied |
| RECON-01 | Phase 6 | roadmap phase contract | Satisfied |
| TODO-01 | Phase 6 | roadmap phase contract | Satisfied |
| NOTES-01 | Phase 6 | roadmap phase contract | Satisfied |
| REMIND-01 | Phase 6 | roadmap phase contract | Satisfied |
| GH-01 | Phase 6 | roadmap phase contract | Satisfied |
| EMAIL-01 | Phase 6 | roadmap phase contract | Satisfied |
| PEOPLE-01 | Phase 6 | roadmap phase contract | Satisfied |
| PEOPLE-02 | Phase 6 | roadmap phase contract | Satisfied |
| IOS-01 | Phase 7 | roadmap phase contract | Satisfied |
| IOS-02 | Phase 7 | roadmap phase contract | Satisfied |
| IOS-03 | Phase 7 | roadmap phase contract | Satisfied |
| HEALTH-01 | Phase 7 | roadmap phase contract | Satisfied |
| HEALTH-02 | Phase 7 | roadmap phase contract | Satisfied |
| APPLE-01 | Phase 7 | roadmap phase contract | Satisfied |
| EXEC-01 | Phase 8 | roadmap phase contract | Satisfied |
| EXEC-02 | Phase 8 | roadmap phase contract | Satisfied |
| GSD-01 | Phase 8 | roadmap phase contract | Satisfied |
| GSD-02 | Phase 8 | roadmap phase contract | Satisfied |
| HANDOFF-01 | Phase 8 | roadmap phase contract | Satisfied |
| HANDOFF-02 | Phase 8 | roadmap phase contract | Satisfied |
| LOCAL-01 | Phase 8 | roadmap phase contract | Satisfied |
| POLICY-01 | Phase 8 | roadmap phase contract | Satisfied |
| BACKUP-01 | Phase 9 | 09-01 | Satisfied |
| BACKUP-02 | Phase 9 | 09-01 | Satisfied |
| CTRL-01 | Phase 9 | 09-01 | Satisfied |
| CTRL-02 | Phase 9 | 09-01 | Satisfied |
| MORNING-01 | Phase 10 | 10-01 | Satisfied |
| MORNING-02 | Phase 10 | 10-01 | Satisfied |
| MORNING-03 | Phase 10 | 10-01 | Satisfied |
| STANDUP-01 | Phase 10 | 10-01 | Satisfied |
| STANDUP-02 | Phase 10 | 10-01 | Satisfied |
| STANDUP-03 | Phase 10 | 10-01 | Satisfied |
| SESSION-01 | Phase 10 | 10-01 | Satisfied |
| VOICE-01 | Phase 10 | 10-01 | Satisfied |
| AGENT-CTX-01 | Phase 11 | 11-01 | Satisfied |
| AGENT-CTX-02 | Phase 11 | 11-01 | Satisfied |
| AGENT-TOOLS-01 | Phase 11 | 11-01 | Satisfied |
| AGENT-TOOLS-02 | Phase 11 | 11-01 | Satisfied |
| AGENT-REVIEW-01 | Phase 11 | 11-01 | Satisfied |
| AGENT-TRUST-01 | Phase 11 | 11-01 | Satisfied |
| SHELL-01 | Phase 12 | roadmap phase contract | Satisfied |
| SHELL-02 | Phase 12 | roadmap phase contract | Satisfied |
| ONBOARD-01 | Phase 12 | roadmap phase contract | Satisfied |
| INTEGR-UX-01 | Phase 12 | roadmap phase contract | Satisfied |
| PROJ-UX-01 | Phase 12 | roadmap phase contract | Satisfied |
| ARCH-XS-01 | Phase 13 | roadmap phase contract | Satisfied |
| ARCH-XS-02 | Phase 13 | roadmap phase contract | Satisfied |
| ADAPT-01 | Phase 13 | roadmap phase contract | Satisfied |
| ADAPT-02 | Phase 13 | roadmap phase contract | Satisfied |
| APPLE-ARCH-01 | Phase 13 | roadmap phase contract | Satisfied |
| API-ARCH-01 | Phase 13 | roadmap phase contract | Satisfied |
| PROD-01 | Phase 14 | roadmap phase contract | Satisfied |
| MODE-01 | Phase 14 | roadmap phase contract | Satisfied |
| UX-CORE-01 | Phase 14 | roadmap phase contract | Satisfied |
| TRUST-UX-01 | Phase 14 | roadmap phase contract | Satisfied |
| ONBOARD-02 | Phase 14 | roadmap phase contract | Satisfied |
| ROADMAP-01 | Phase 14 | roadmap phase contract | Satisfied |
| MIGRATE-01 | Phase 15 | roadmap phase contract | Satisfied |
| MIGRATE-02 | Phase 15 | roadmap phase contract | Satisfied |
| SERVICE-01 | Phase 15 | roadmap phase contract | Satisfied |
| DTO-01 | Phase 15 | roadmap phase contract | Satisfied |
| READMODEL-01 | Phase 15 | roadmap phase contract | Satisfied |
| LOGIC-01 | Phase 16 | roadmap phase contract | Satisfied |
| FLOW-01 | Phase 16 | roadmap phase contract | Satisfied |
| MODE-02 | Phase 16 | roadmap phase contract | Satisfied |
| READMODEL-02 | Phase 16 | roadmap phase contract | Satisfied |
| SHELL-ARCH-01 | Phase 16 | roadmap phase contract | Satisfied |
| SHELL-MODE-01 | Phase 17 | roadmap phase contract | Satisfied |
| SHELL-MODE-02 | Phase 17 | roadmap phase contract | Satisfied |
| TRUST-SUMMARY-01 | Phase 17 | roadmap phase contract | Satisfied |
| APPLE-SHELL-01 | Phase 17 | roadmap phase contract | Satisfied |
| CLOSEOUT-01 | Phase 18 | milestone audit follow-up | Satisfied |
| CLOSEOUT-02 | Phase 18 | milestone audit follow-up | Satisfied |
| CLOSEOUT-03 | Phase 19 | milestone audit follow-up | Pending |
| CLOSEOUT-04 | Phase 19 | milestone audit follow-up | Pending |
| USABLE-01 | Phase 20 | roadmap phase contract | Pending |
| USABLE-02 | Phase 20 | roadmap phase contract | Pending |
| NOW-UX-01 | Phase 20 | roadmap phase contract | Pending |
| INBOX-UX-01 | Phase 20 | roadmap phase contract | Pending |
| THREADS-UX-01 | Phase 20 | roadmap phase contract | Pending |
| ENTRY-01 | Phase 20 | roadmap phase contract | Pending |
| SETTINGS-UX-01 | Phase 20 | roadmap phase contract | Pending |
| ASSIST-01 | Phase 20 | roadmap phase contract | Pending |
| ASSIST-02 | Phase 20 | roadmap phase contract | Pending |
| THREADS-02 | Phase 20 | roadmap phase contract | Pending |
| VOICE-02 | Phase 21 | roadmap phase contract | Pending |
| VOICE-03 | Phase 21 | roadmap phase contract | Pending |
| APPLE-VOICE-02 | Phase 21 | roadmap phase contract | Pending |
| DESKTOP-VOICE-01 | Phase 21 | roadmap phase contract | Pending |
| DESKTOP-VOICE-02 | Phase 21 | roadmap phase contract | Pending |
| DAILY-AI-01 | Phase 22 | roadmap phase contract | Pending |
| DAILY-AI-02 | Phase 22 | roadmap phase contract | Pending |
| EOD-01 | Phase 22 | roadmap phase contract | Pending |
| EOD-02 | Phase 22 | roadmap phase contract | Pending |
| THREAD-RES-01 | Phase 22 | roadmap phase contract | Pending |
| THREAD-RES-02 | Phase 22 | roadmap phase contract | Pending |
| ASSIST-ACT-01 | Phase 23 | roadmap phase contract | Pending |
| ASSIST-ACT-02 | Phase 23 | roadmap phase contract | Pending |
| REVIEW-02 | Phase 23 | roadmap phase contract | Pending |
| TRUST-02 | Phase 23 | roadmap phase contract | Pending |
| ASSIST-APPLY-01 | Phase 24 | roadmap phase contract | Pending |
| ASSIST-APPLY-02 | Phase 24 | roadmap phase contract | Pending |
| REVIEW-03 | Phase 24 | roadmap phase contract | Pending |
| TRUST-03 | Phase 24 | roadmap phase contract | Pending |
| RECALL-01 | Phase 25 | roadmap phase contract | Pending |
| RECALL-02 | Phase 25 | roadmap phase contract | Pending |
| SEM-02 | Phase 25 | roadmap phase contract | Pending |
| GROUND-CTX-01 | Phase 25 | roadmap phase contract | Pending |
| GROUND-CTX-02 | Phase 25 | roadmap phase contract | Pending |
| REFLOW-REAL-01 | Phase 26 | roadmap phase contract | Pending |
| REFLOW-REAL-02 | Phase 26 | roadmap phase contract | Pending |
| SCHED-RECON-01 | Phase 26 | roadmap phase contract | Pending |
| SCHED-RECON-02 | Phase 26 | roadmap phase contract | Pending |
| SCHED-FACET-01 | Phase 27 | roadmap phase contract | Pending |
| SCHED-FACET-02 | Phase 27 | roadmap phase contract | Pending |
| AGENT-SCHED-01 | Phase 27 | roadmap phase contract | Pending |
| RECALL-SCHED-01 | Phase 27 | roadmap phase contract | Pending |
| ROUTINE-05 | Phase 30 | roadmap phase contract | Pending |
| ROUTINE-06 | Phase 30 | roadmap phase contract | Pending |
| PLANPROFILE-01 | Phase 30 | roadmap phase contract | Pending |
| PLANPROFILE-02 | Phase 30 | roadmap phase contract | Pending |
| PLANPROFILE-03 | Phase 31 | roadmap phase contract | Pending |
| PLANPROFILE-04 | Phase 31 | roadmap phase contract | Pending |
| ROUTINE-07 | Phase 31 | roadmap phase contract | Pending |
| VOICE-04 | Phase 31 | roadmap phase contract | Pending |
| PLANPROFILE-05 | Phase 32 | roadmap phase contract | Pending |
| PLANPROFILE-06 | Phase 32 | roadmap phase contract | Pending |
| ROUTINE-08 | Phase 32 | roadmap phase contract | Pending |
| VOICE-05 | Phase 32 | roadmap phase contract | Pending |
| DAYPLAN-05 | Phase 33 | roadmap phase contract | Pending |
| DAYPLAN-06 | Phase 33 | roadmap phase contract | Pending |
| REFLOW-REAL-03 | Phase 33 | roadmap phase contract | Pending |
| SCHED-APPLY-01 | Phase 33 | roadmap phase contract | Pending |
| NOW-UX-02 | Phase 34 | roadmap phase contract | Pending |
| NOW-UX-03 | Phase 34 | roadmap phase contract | Pending |
| CAL-UX-01 | Phase 34 | roadmap phase contract | Pending |
| TODO-UX-01 | Phase 34 | roadmap phase contract | Pending |
| VELCSV-01 | Phase 34 | roadmap phase contract | Pending |
| DAYBOUND-01 | Phase 35 | roadmap phase contract | Pending |
| DAYBOUND-02 | Phase 35 | roadmap phase contract | Pending |
| NOW-ORDER-01 | Phase 35 | roadmap phase contract | Pending |
| CONTEXT-01 | Phase 35 | roadmap phase contract | Pending |
| SHELL-03 | Phase 36 | roadmap phase contract | Pending |
| SETTINGS-UX-02 | Phase 36 | roadmap phase contract | Pending |
| THREADS-03 | Phase 36 | roadmap phase contract | Pending |
| SIDEBAR-01 | Phase 36 | roadmap phase contract | Pending |
| APPLE-EMBED-01 | Phase 37 | roadmap phase contract | Pending |
| APPLE-EMBED-02 | Phase 37 | roadmap phase contract | Pending |
| FFI-01 | Phase 37 | roadmap phase contract | Pending |
| OFFLINE-01 | Phase 37 | roadmap phase contract | Pending |
| APPLE-OFFLINE-01 | Phase 38 | roadmap phase contract | Pending |
| APPLE-OFFLINE-02 | Phase 38 | roadmap phase contract | Pending |
| VOICE-06 | Phase 38 | roadmap phase contract | Pending |
| THREADS-04 | Phase 38 | roadmap phase contract | Pending |
| VELCSV-02 | Phase 39 | roadmap phase contract | Pending |
| DAILY-USE-01 | Phase 39 | roadmap phase contract | Pending |
| DAILY-USE-02 | Phase 39 | roadmap phase contract | Pending |
| APPLE-PARITY-01 | Phase 39 | roadmap phase contract | Pending |

**Coverage:**
- v1 requirements: 187 total
- Mapped to phases: 187
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-18*
*Last updated: 2026-03-20 — added Phase 34-39 requirements for `Now` repair, shell simplification, Apple embedded-core, and `Vel.csv`-driven closeout*
