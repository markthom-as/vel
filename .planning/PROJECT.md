# Vel

## What This Is

Vel is a local-first personal cognition runtime centered on a strict daily operator loop. It runs as a daemon (`veld`) that captures, recalls, orients the operator, and supports supervised continuation through bounded thread flows. Operators interact through CLI (`vel`), the web client, and Apple clients over the same Rust-owned product core.

## Core Value

Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.

## Current State

**Shipped version:** `0.3.0`

`v0.2` established the true MVP as one strict daily operator loop:

`overview -> commitments -> reflow -> threads -> review`

**Shipped truths:**
- overview, commitments, reflow, thread continuation, and review are backed by Rust-owned contracts and services
- web and Apple now act as thin MVP shells over the same backend-owned loop
- same-day reflow remains bounded, explainable, and review-gated
- threads remain a supervised continuation lane rather than a generic chat product
- the MVP now has an explicit post-MVP roadmap instead of implied carryover scope

## Latest Shipped Milestone: 0.3.0 Canonical Now Surface and Client Mesh

**Goal achieved:** `Now` is now one strict cross-platform control surface backed by platform-portable Rust core, and clients mesh more cleanly around the same authority runtime.

**Shipped features:**
- canonical `Now` contract across web, iPhone, iPad, Mac, and reduced watch
- `Now` semantics owned by Rust core and shared transport contracts
- one canonical task, nudge, thread, and day model feeding `Now`
- docked capture and voice entry with thread artifact continuity where practical
- client mesh, linking, sync/offline visibility, and connection recovery as shared product behavior
- governed `Now` config, deterministic ranking, and approval posture in the same Rust-owned product-core lane

## Latest Closed Release Line: 0.5 Backend Core Rewrite

**Goal achieved:** Vel now has a canonical object-centered backend authority with typed action/policy/audit infrastructure, governed module/bootstrap lanes, manual workflow runtime, native calendar semantics, Todoist and Google Calendar proving adapters, and canonical write-path cutover backed by execution evidence.

**Versioning rule:**
- shipped releases use semver, starting from the current `0.3.0` baseline
- `0.5.0-beta` is the latest completed backend rewrite line
- in-flight planning slices use `<major>.<minor>.<phase>.<plan>` identifiers such as `0.5.57.1`, while the existing phase numbers remain the canonical directory and history keys

**Milestone priorities achieved:**
- canonical object kernel is now the backend system of record
- action, ownership, confirmation, and audit are mandatory backend infrastructure
- module, skill, and workflow primitives load through governed paths
- Todoist and Google Calendar prove the new substrate against real task/calendar complexity
- live write authority now flows through canonical `WriteIntent` routes

## Prior Closed Release Line: 0.4.x Now/UI MVP Conformance Closure

**Goal achieved:** the shipped `Now` experience was tightened into the compact, operator-corrected MVP shell, then closed with a clean web build and focused regression evidence.

## Latest Closed Release Line: 0.5.1 Canonical Client Reconnection

**Goal achieved:** the web client now speaks the frozen `0.5` backend truth through exactly three first-class surfaces: `Now`, `Threads`, and `System`.

**Delivered:**
- truthful-surface doctrine and canonical transport boundary
- `Now` rebound to canonical task/calendar truth with direct canonical completion flow
- `Threads` rebound to canonical conversation truth with explicit invocation gating
- authoritative `/system` surface with bounded reads and allow-listed actions
- deleted `Settings` / `Projects` web surfaces
- browser-executed proof of read, mutation, degraded-state, and no-silent-fallback behavior
- Apple handoff/spec packet only

**Accepted debt:**
- live browser workflow dispatch remains deferred until a future milestone ships canonical workflow invocation transport

## Latest Closed Release Line: 0.5.2 Operator Surface Embodiment

**Goal achieved:** the truthful `0.5.1` client line now has the intended embodied operator surfaces on web without reopening backend law.

**Delivered:**
- approved operator-surface doctrine and UI contract
- repaired shared shell rhythm, disclosure model, and three-surface framing
- focus-first `Now` embodiment with local-first completion reconcile
- bound-object-first `Threads` embodiment with browser-proven grounded reading
- grouped sidebar/detail `System` embodiment with bounded canonical actions only
- cross-surface browser proof and refreshed Apple handoff/spec packet

**Accepted debt:**
- live browser workflow dispatch remains deferred until a future milestone ships canonical workflow invocation transport

## Next Milestone Goals

The next queued milestone is `v0.5.6` — single-node MVP and polished web UI.

This line is intentionally scoped from direct operator feedback in [TODO.md](/home/jove/code/vel/TODO.md), copied verbatim into the planning packet at [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md).

Primary goals:
- make one local Vel node work as a credible MVP through the web UI
- make Google/Todoist integration, chat provider selection, and single-node settings/config truthful enough for real operator use
- polish `Now`, `Threads`, and `System` to the accepted web-quality bar
- defer additional future phases until new feedback arrives after this MVP line

Immediate planning packet:
- [v0.5.6 roadmap](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/ROADMAP.md)
- [v0.5.6 requirements](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/REQUIREMENTS.md)
- [v0.5.6 verbatim feedback copy](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md)

## Requirements

### Validated

- ✓ True MVP daily operator loop with Rust-owned overview, commitments, reflow, threads, and review — v0.2
- ✓ Minimal fresh web and Apple shells over the shared MVP transport contracts — v0.2
- ✓ Explicit post-MVP roadmap and deferred-scope boundary — v0.2
- ✓ Storage Repository Pattern with transaction lifecycles — Phase 1 (ticket 001)
- ✓ Pure Core & Typed Context via `vel-core` domain types — Phase 1 (ticket 002)
- ✓ Service/DTO Boundary & standardized error handling — Phase 1 (ticket 003)
- ✓ Documentation Truth Repair & architecture mapping — Phase 1 (ticket 011)
- ✓ Auth-by-default HTTP surfaces & deny-by-default routing — Phase 1 (ticket 015)
- ✓ Cross-cutting trait baseline (modularity, logging, composability) — Phase 1 (ticket 018)
- ✓ Documentation catalog as single source of truth — Phase 1 (ticket 020)
- ✓ Canonical schemas, config contracts & templates — Phase 1 (ticket 021)
- ✓ Canonical data sources, integrations & connector architecture — Phase 1 (ticket 022)
- ✓ Self-awareness, repo visibility & supervised self-modification — Phase 1 (ticket 023)
- ✓ Machine-readable contract manifest publication — Phase 1 (ticket 024)
- ✓ Config template and fixture parity — Phase 1 (ticket 025)

### Historical Architecture Program (Closed / Re-Scoped Where Needed)

**Phase 2 — Distributed State, Offline Clients & System-of-Systems:**
- [x] Pluggable signal ingestion & context reducer pipeline (ticket 004)
- [ ] Sync ordering & conflict resolution via HLC (ticket 005) — unfinished scope moved to Phase 6
- [ ] Agent Connect launch protocol & supervision (ticket 006) — unfinished external transport/route scope moved to Phase 8
- [ ] Tester-readiness onboarding & node discovery (ticket 012) — unfinished guided linking scope moved to Phase 5
- [x] Capability broker & secret mediation (ticket 016)
- [x] Operator surface accessibility & effective config clarity (ticket 019)

**Phase 3 — Deterministic Verification & Continuous Alignment:**
- [x] Deterministic replay engine / day-simulation harness (ticket 007)
- [x] LLM-as-a-Judge evaluation pipeline (ticket 008)
- [x] Execution tracing, handoff telemetry & reviewability (ticket 017)
- [x] Comprehensive user documentation & support wiki (ticket 013)

**Phase 4 — Autonomous Swarm, Graph RAG & Zero-Trust Execution:**
- [ ] Semantic memory & Graph RAG (ticket 009) — shipped capture-backed baseline; richer graph expansion moved to Phase 6
- [ ] Zero-trust WASM agent sandboxing (ticket 010) — shipped host-executor baseline; direct WASM guest runtime moved to Phase 8
- [ ] Swarm execution SDK & contract (ticket 014) — shipped Rust SDK/protocol baseline; external connect/auth transport moved to Phase 8

### Active

- `v0.5.2` operator-surface embodiment

### Out of Scope

- Multi-tenant cloud hosting — local-first, single operator by design
- Fine-grained RBAC — token-based auth sufficient for v1 swarm
- Consumer-grade astrology-first product logic — astrology may become an optional enrichment module later, not a core planning dependency
- Broad fully automatic upstream provisioning — creating external Todoist projects, note roots, or similar structures should remain operator-confirmed in the new-project workflow

## Context

This is a mid-migration brownfield project. The codebase already has the right broad technical foundation: layered Rust crates (`vel-core` → `vel-storage` → `vel-api-types` → `veld`), auth hardening, canonical schemas, and substantial product behavior shipped through milestones `v0.1` and `v0.2`. `v0.2` deliberately narrowed the product to a true MVP and closed it with a hard post-MVP boundary. Future work should grow from that verified loop instead of reopening shell-owned product semantics or widening scope implicitly.

The existing ticket files in `docs/tickets/phase-{2,3,4}/` are historical implementation specifications for the shipped architecture queue. For active future work, `.planning/ROADMAP.md` and subsequent phase plans are the requirements source starting at Phase 5. For non-phase future work that is not yet scheduled, use `.planning/BACKLOG.md`. For execution-ready small work items, use `.planning/todos/pending/`.

**Existing source of truth:** `docs/MASTER_PLAN.md` — canonical status tracker
**Active tickets:** `docs/tickets/` — agent-optimized implementation specs
**Execution boards:** `docs/tickets/phase-*/parallel-execution-board.md`

## Constraints

- **Tech Stack**: Rust (Axum/SQLx/Tokio) backend, React 19/TypeScript frontend, SQLite persistence — no deviation
- **Crate Layering**: `vel-storage` must NOT depend on `vel-api-types`; services must not return HTTP DTOs
- **Implementation Protocol**: Follow `docs/templates/agent-implementation-protocol.md` for every ticket
- **Local-first**: All durable state in SQLite; no cloud sync assumptions

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| `Now + Inbox` remain the primary UI | Daily orientation matters more than project dashboards | Accepted |
| `Personal` / `Creative` / `Work` are project families, not projects | Keeps the project list practical while still supporting grouping | Accepted |
| Projects are local-first typed records with one primary repo and notes root plus optional secondary links | Matches the current workflow while preserving a clean substrate | Accepted |
| Upstream systems remain authoritative for their own records | Vel should add safe net-new writes and compatible updates, but prompt on conflicts | Accepted |
| New external project creation is operator-confirmed during project setup | Avoid accidental upstream sprawl and preserve trust | Accepted |
| Todoist labels remain supported at the adapter boundary, not as the core Vel model | Keeps compatibility without making tag syntax the internal domain language | Accepted |
| Vel should optimize equally for action and intervention | The product must answer both “what should I do now?” and “what is slipping, stale, blocked, or conflicted?” | Accepted |
| Multi-client continuity is a product feature, not just sync infrastructure | Web, Apple, CLI, and future worker surfaces must preserve one coherent action/intervention state | Accepted |
| Task handoff is a product feature, not just protocol plumbing | Human-to-agent and agent-to-agent delegation must be explicit, inspectable, and reviewable in real workflows | Accepted |
| Astrology is a toggleable enrichment module, not core planning logic | Useful for the operator, optional for most users, and deferrable | Accepted |
| GSD integration should begin through repo-local docs/context that GSD already consumes | Lowest-friction path to coding-first supervised execution | Accepted |
| Daily-loop value should be built from the shipped `Now`/calendar/Todoist/voice seams instead of inventing a separate planning subsystem | Maximizes product value from already-landed foundations and avoids a parallel architecture | Accepted |
| Agent awareness of Vel state is important enough to be committed roadmap work, not just backlog | The repo now has projects, people, `Now`, review data, execution context, and bounded tool surfaces; the missing work is productizing that grounding into a trustworthy agent path | Accepted |
| Interface and shell fixes should be planned, while broader provider/platform expansion stays backlog-only until daily use is strong | Keeps roadmap effort focused on adoption and repeated use rather than diffuse expansion | Accepted |
| One Rust-owned grounded assistant seam should power text, voice, daily-loop continuity, and thread escalation across shells | Prevents Apple/web/desktop from re-implementing assistant behavior and keeps future FFI/daemon/server topologies aligned | Accepted |
| Scheduler and tagging semantics should become canonical Vel-backed fields/facets instead of staying raw provider labels | Preserves the proven `codex-workspace` rule system while making it explainable, SQL-queryable, and agent-usable without provider drift | Accepted |
| Canonical scheduler rules should feed an explicit backend-owned day-planning lane before broader planner ambitions | Keeps morning/day shaping product-useful and explainable while avoiding a speculative multi-day planner rewrite | Accepted |
| Routine blocks should become durable backend-owned planning records before any richer planner or habit-system ambitions | Strengthens same-day planning trust and operator control without widening into a speculative multi-day or lifestyle-product rewrite | Accepted |
| After supervised planning-profile edits, the next planning value lane should be supervised application of bounded same-day day-plan and reflow changes | Turns explainable same-day planning into usable action without widening into broad autonomous calendar editing | Accepted |
| The next product arc should repair `Now` and current-day truth before widening Apple architecture | Prevents Apple FFI work from hardening the wrong daily-use model and makes the UI/UX correction the product priority | Accepted |
| `Now` should be an execution-first current-day control surface rather than a dashboard | Keeps the primary operator surface centered on what is happening now, what matters next, and one dominant capture/ask/talk affordance | Accepted |
| `Vel.csv` should validate and regress the next arc, not dictate product direction | Preserves operator interview decisions and prior specs as product authority while still using shipped feedback as an acceptance guardrail | Accepted |
| Apple embedded-core / FFI should start on iPhone, with voice continuity as the proving flow | Maximizes local-first value and offline differentiation without splitting effort across watch and Mac too early | Accepted |
| UI rework should be decision-first rather than dashboard-first | Reduces cognitive load by making each primary surface answer what to do next and hiding debug/runtime internals behind explicit disclosure | Accepted |
| Broken operator interactions belong in the UI rework lane, not in a separate polish backlog | If an action is surfaced but does not work reliably on web or mobile, that is a product failure and should be repaired alongside hierarchy cleanup | Accepted |
| v0.2 should define the true MVP in depth before widening implementation again | Prevents fake-MVP scope and keeps future work explicit instead of leaking into the milestone | Accepted |
| v0.2 should consolidate MVP behavior into one Rust-owned product-core lane rather than re-platforming the whole system | Preserves the existing architecture while removing shell policy drift where it matters | Accepted |
| v0.2 reflow should stay Rust-owned and explainable without adding milestone-local calendar ingestion work | Keeps the MVP loop narrow while preserving portable backend authority | Accepted |
| Fresh web and Apple UI should follow contract lock and Rust-core consolidation rather than lead it | Prevents a second round of shell-owned product semantics | Accepted |
| v0.2 must stay limited to the overview -> commitments -> reflow -> threads -> review loop | Keeps the milestone honest and blocks drift into adjacent but non-essential product work | Accepted |
| Anything not directly serving the MVP loop should be deferred unless it is required to keep MVP logic Rust-owned or the clients thin | Prevents architecture and UI cleanup from becoming open-ended | Accepted |
| The next milestone should treat the `Now` surface as one canonical cross-platform contract backed by platform-portable Rust core | Prevents web and Apple from drifting again and turns the new `Now` interaction model into product authority instead of shell-specific design | Accepted |
| Cross-client mesh and client-linking help are part of the product, not setup debris | Operators need clients to discover, connect to, and recover connection to the same authority runtime without platform-specific guesswork | Accepted |
| Latest operator clarification overrides the Downloads contract where they differ | The new milestone exists to close concrete conformance gaps, so explicit operator corrections are the highest product authority for this lane | Accepted |
| `Now` top-area content must be containerless micro-rows while tasks remain the only dominant container | Restores the execution-first operating-surface feel and removes dashboard drift from the primary daily-use entry point | Accepted |
| Inbox and `Now` must share the same actionable objects, with Inbox as the superset queue | Treats empty-inbox-with-actionable-now as a product/data bug rather than a cosmetic UI inconsistency | Accepted |
| Documentation should be top-level navigation, not buried inside Settings | Keeps Settings focused on profile, device/sync, state, and backups while reducing shell noise and route confusion | Accepted |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition**:
1. Requirements invalidated? Move them to out of scope with reason.
2. Requirements validated? Move them to validated with phase reference.
3. New requirements emerged? Add them to active scope.
4. Decisions to log? Add them to the key decisions table.
5. Product framing drifted? Update the project description and context.

**After each milestone**:
1. Review all sections for truthfulness.
2. Re-check the core value and MVP boundary.
3. Audit out-of-scope items and confirm they are still intentionally deferred.
4. Update context with the new shipped state and next-milestone seed.

---
*Last updated: 2026-03-23 after queuing `v0.5.6` as the single-node MVP and polished-web-UI milestone*
