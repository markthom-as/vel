# Vel

## What This Is

Vel is a local-first personal cognition runtime and autonomous agent orchestration platform. It runs as a daemon (`veld`) that captures, recalls, and aligns daily context, while providing a safe execution environment for autonomous agents operating as a distributed swarm. Operators interact via CLI (`vel`), a React web dashboard, and iOS/watchOS/macOS clients.

## Core Value

Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.

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

**Post-Phase-4 Product Direction:**
- [ ] Phase 5 — `Now + Inbox` remain primary; projects land first as typed substrate plus project-family structure
- [ ] Phase 6 — safe autonomous write-back for Todoist/notes/reminders/GitHub/email; lightweight people registry; upstream conflict prompts
- [ ] Phase 7 — iOS/watch action loops first: voice capture, current schedule, and nudge response; lightweight behavior signals
- [ ] Phase 8 — coding-first supervised execution with GSD-aware repo docs, token-budget awareness, and local-agent support
- [ ] Phase 9 — backup/export trust surfaces and simple operator control, with recovery deferred behind core usability
- [ ] Phase 10 — strict daily-loop MVP: morning overview, standup, 1-3 commitments, deferrals, and focus protection over existing `Now`/calendar/Todoist/voice seams
- [ ] Phase 11 — agent grounding over real Vel data and bounded tool surfaces: current context, projects, people, commitments, review queues, and operator-visible trust controls
- [ ] Phase 12 — shell, onboarding, and connector ergonomics: routes, docs/help surfaces, project detail UX, linking/discovery reliability, and settings polish

### Out of Scope

- Multi-tenant cloud hosting — local-first, single operator by design
- Fine-grained RBAC — token-based auth sufficient for v1 swarm
- Consumer-grade astrology-first product logic — astrology may become an optional enrichment module later, not a core planning dependency
- Broad fully automatic upstream provisioning — creating external Todoist projects, note roots, or similar structures should remain operator-confirmed in the new-project workflow

## Context

This is a mid-migration brownfield project. The codebase has been structurally decomposed (Phase 1 complete): layered Rust crates (`vel-core` → `vel-storage` → `vel-api-types` → `veld`), auth hardening, canonical schemas, and self-awareness contracts are all in place. Phase 3 is complete; Phases 2 and 4 shipped meaningful baselines but had unfinished original-scope work. That unfinished work has been explicitly re-scoped into Phases 5, 6, and 8 so no active roadmap work remains before Phase 5. The active planning lane is the product-shaping sequence centered on `Now + Inbox`, typed project substrate, safe write-back, Apple action loops, supervised execution, backup-first trust, a strict daily loop, stronger agent grounding, and a cleaner operator shell/onboarding path.

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

---
*Last updated: 2026-03-19 after auditing completed claims and re-scoping unfinished historical work*
