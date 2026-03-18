# Requirements: Vel

**Defined:** 2026-03-18
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.

## v1 Requirements

Requirements for Phases 2–4 (Phase 1 is complete). Each maps to a master plan ticket.

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

- [ ] **CAP-01**: Agent capabilities are brokered — agents request scoped permissions, not raw credentials (ticket 016)
- [ ] **CAP-02**: Secrets are mediated through indirection; prompt-visible raw credentials are prohibited (ticket 016)

### Operator Experience (Phase 2)

- [ ] **OPS-01**: Operator surfaces (CLI, web) expose effective configuration state clearly (ticket 019)
- [ ] **OPS-02**: Accessibility requirements are met for the operator dashboard (ticket 019)

### Deterministic Verification (Phase 3)

- [ ] **VERIFY-01**: A day-simulation harness can replay a recorded day deterministically (ticket 007)
- [ ] **VERIFY-02**: Simulation output is stable across runs given the same input log (ticket 007)
- [ ] **EVAL-01**: An LLM-as-a-Judge pipeline evaluates agent reasoning outputs (ticket 008)
- [ ] **EVAL-02**: Eval results are stored and queryable for regression tracking (ticket 008)

### Execution Observability (Phase 3)

- [ ] **TRACE-01**: Agent runs produce stable run IDs with associated traces (ticket 017)
- [ ] **TRACE-02**: Handoffs between agents are recorded with event linkage (ticket 017)
- [ ] **TRACE-03**: Execution history is reviewable from the operator dashboard (ticket 017)

### Documentation (Phase 3)

- [ ] **DOCS-01**: Comprehensive user documentation covers all operator-facing workflows (ticket 013)
- [ ] **DOCS-02**: A support wiki exists and is indexed for search (ticket 013)

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
| SIG-01 | Phase 2 | 004 | Pending |
| SIG-02 | Phase 2 | 004 | Pending |
| SYNC-01 | Phase 2 | 005 | Pending |
| SYNC-02 | Phase 2 | 005 | Pending |
| CONN-01 | Phase 2 | 006 | Pending |
| CONN-02 | Phase 2 | 006 | Pending |
| CONN-03 | Phase 2 | 012 | Pending |
| CONN-04 | Phase 2 | 012 | Pending |
| CAP-01 | Phase 2 | 016 | Pending |
| CAP-02 | Phase 2 | 016 | Pending |
| OPS-01 | Phase 2 | 019 | Pending |
| OPS-02 | Phase 2 | 019 | Pending |
| VERIFY-01 | Phase 3 | 007 | Pending |
| VERIFY-02 | Phase 3 | 007 | Pending |
| EVAL-01 | Phase 3 | 008 | Pending |
| EVAL-02 | Phase 3 | 008 | Pending |
| TRACE-01 | Phase 3 | 017 | Pending |
| TRACE-02 | Phase 3 | 017 | Pending |
| TRACE-03 | Phase 3 | 017 | Pending |
| DOCS-01 | Phase 3 | 013 | Pending |
| DOCS-02 | Phase 3 | 013 | Pending |
| MEM-01 | Phase 4 | 009 | Pending |
| MEM-02 | Phase 4 | 009 | Pending |
| SAND-01 | Phase 4 | 010 | Pending |
| SAND-02 | Phase 4 | 010 | Pending |
| SDK-01 | Phase 4 | 014 | Pending |
| SDK-02 | Phase 4 | 014 | Pending |
| SDK-03 | Phase 4 | 014 | Pending |

**Coverage:**
- v1 requirements: 28 total
- Mapped to phases: 28
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-18*
*Last updated: 2026-03-18 after initial definition from existing docs/tickets*
