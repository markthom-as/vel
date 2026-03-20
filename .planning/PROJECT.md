# Vel

## What This Is

Vel is a local-first personal cognition runtime and autonomous agent orchestration platform. It runs as a daemon (`veld`) that captures, recalls, and aligns daily context, while providing a safe execution environment for autonomous agents operating as a distributed swarm. Operators interact via CLI (`vel`), a React web dashboard, and iOS/watchOS/macOS clients.

## Core Value

Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.

## Current Milestone: v0.2 True MVP and Rust-Core Closure

**Goal:** Establish the true MVP as one strict daily operator loop backed by platform-portable Rust core logic, with minimal fresh web and Apple shells over that core.

**Target features:**
- strict daily overview and commitment flow over one canonical current-day truth
- grounded suggestions, nudges, and same-day reflow with local-first input where possible
- threads, tools, context, and review over one Rust-owned continuation model
- canonical MVP data types, business logic docs, and refined cross-surface architecture docs
- explicit post-MVP roadmap and deferred-scope boundary

**MVP loop:**
`overview -> commitments -> reflow -> threads -> review`

**In scope for v0.2:**
- current-day operator behavior only
- Rust-owned contracts, business logic, and transport for the MVP loop
- bounded same-day reflow with local-first calendar input where possible
- bounded thread continuation for tools, context, and data
- minimal fresh web and Apple shells over the same Rust-owned loop
- explicit architecture and contract documentation needed to keep shells thin

**Out of scope for v0.2:**
- multi-day or autonomous planning
- broad UI polish outside the MVP loop screens
- broad provider or platform expansion
- shell-owned fallback logic, ranking logic, or planner logic
- broad FFI migration or Apple-local planning logic
- broad calendar write-back automation
- generic chat-first product direction

## Requirements

### Validated

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

**Milestone v0.2 Active Scope:**
- [ ] true MVP definition around one strict daily operator loop
- [ ] canonical Rust-owned data types, business logic, and architecture docs for that loop
- [ ] decision-first overview, bounded commitments, grounded nudges, and explainable same-day reflow
- [ ] thread-based continuation for tools, context, data, and follow-through
- [ ] lightweight review and closeout over the same Rust-owned loop
- [ ] fresh minimal web and Apple MVP shells over shared transport contracts
- [ ] explicit future roadmap for post-MVP work outside v0.2
- [ ] strict non-goals that prevent unrelated product expansion during MVP closure

### Out of Scope

- Multi-tenant cloud hosting — local-first, single operator by design
- Fine-grained RBAC — token-based auth sufficient for v1 swarm
- Consumer-grade astrology-first product logic — astrology may become an optional enrichment module later, not a core planning dependency
- Broad fully automatic upstream provisioning — creating external Todoist projects, note roots, or similar structures should remain operator-confirmed in the new-project workflow

## Context

This is a mid-migration brownfield project. The codebase already has the right broad technical foundation: layered Rust crates (`vel-core` → `vel-storage` → `vel-api-types` → `veld`), auth hardening, canonical schemas, and substantial product behavior shipped through milestone `v0.1`. The `v0.2` milestone is intentionally narrower than the breadth of what already exists. Its job is to define and ship the true MVP: one strict daily operator loop with Rust-owned authority, explainable same-day reflow, bounded thread continuation, lightweight review, and minimal fresh web and Apple shells that stop carrying local product policy.

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
| Local-first calendar support in v0.2 should mean Rust-owned reflow fed by narrow local calendar input, not Apple-local planner logic | Keeps same-day reflow portable, explainable, and bounded | Accepted |
| Fresh web and Apple UI should follow contract lock and Rust-core consolidation rather than lead it | Prevents a second round of shell-owned product semantics | Accepted |
| v0.2 must stay limited to the overview -> commitments -> reflow -> threads -> review loop | Keeps the milestone honest and blocks drift into adjacent but non-essential product work | Accepted |
| Anything not directly serving the MVP loop should be deferred unless it is required to keep MVP logic Rust-owned or the clients thin | Prevents architecture and UI cleanup from becoming open-ended | Accepted |

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
*Last updated: 2026-03-20 for milestone v0.2 true MVP planning*
