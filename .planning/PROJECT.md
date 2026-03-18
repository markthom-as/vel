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

### Active

**Phase 2 — Distributed State, Offline Clients & System-of-Systems:**
- [ ] Pluggable signal ingestion & context reducer pipeline (ticket 004)
- [ ] Sync ordering & conflict resolution via HLC (ticket 005)
- [ ] Agent Connect launch protocol & supervision (ticket 006)
- [ ] Tester-readiness onboarding & node discovery (ticket 012)
- [ ] Capability broker & secret mediation (ticket 016)
- [ ] Operator surface accessibility & effective config clarity (ticket 019)

**Phase 3 — Deterministic Verification & Continuous Alignment:**
- [ ] Deterministic replay engine / day-simulation harness (ticket 007)
- [ ] LLM-as-a-Judge evaluation pipeline (ticket 008)
- [ ] Execution tracing, handoff telemetry & reviewability (ticket 017)
- [ ] Comprehensive user documentation & support wiki (ticket 013)

**Phase 4 — Autonomous Swarm, Graph RAG & Zero-Trust Execution:**
- [ ] Semantic memory & Graph RAG (ticket 009)
- [ ] Zero-trust WASM agent sandboxing (ticket 010)
- [ ] Swarm execution SDK & contract (ticket 014)

### Out of Scope

- Multi-tenant cloud hosting — local-first, single operator by design
- Fine-grained RBAC — token-based auth sufficient for v1 swarm
- Mobile-first UI — web dashboard is primary operator surface; Apple clients are secondary

## Context

This is a mid-migration brownfield project. The codebase has been structurally decomposed (Phase 1 complete): layered Rust crates (`vel-core` → `vel-storage` → `vel-api-types` → `veld`), auth hardening, canonical schemas, and self-awareness contracts are all in place. Phase 2 has partial work in flight on Agent Connect and operator accessibility; Phase 3 has partial work on tracing and user docs.

The existing ticket files in `docs/tickets/phase-{2,3,4}/` are the authoritative implementation specifications. GSD phases map directly to the master plan phases.

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
| GSD phases match master plan phases (2, 3, 4) | Tickets already grouped; avoids re-scoping existing work | — Pending |
| Skip GSD domain research | Domain is well-understood; tickets are prescriptive | — Pending |
| Ticket files are requirements source | `docs/tickets/` are agent-optimized specs; no need to re-derive | — Pending |

---
*Last updated: 2026-03-18 after GSD initialization from existing docs*
