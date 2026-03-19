# Requirements: Vel

**Defined:** 2026-03-18
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.

## v1 Requirements

Requirements for Phases 2–4 plus the later roadmap slices for backup/trust, the daily loop, and agent grounding. Phase 1 is complete. Each requirement maps to a master plan ticket or phase contract slice.

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

- [x] **OPS-01**: Operator surfaces (CLI, web) expose effective configuration state clearly (ticket 019)
- [x] **OPS-02**: Accessibility requirements are met for the operator dashboard (ticket 019)

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

### Backup Trust Surfaces (Phase 9)

- [ ] **BACKUP-01**: Operators can create and inspect a typed local backup/export pack that captures the database snapshot, artifact coverage, config coverage, explicit omissions, and verification summary for the Vel core state (ticket 09-01)
- [ ] **BACKUP-02**: Backup/export confidence is surfaced clearly through inspectable status and coverage data, while restore automation remains secondary to manual inspection and operator trust (ticket 09-01)
- [ ] **CTRL-01**: Backup/control surfaces remain simple, typed, and bounded to high-value runtime state instead of becoming a generic configuration editor (ticket 09-01)
- [ ] **CTRL-02**: Safety state and trust posture are visible so operators can inspect what is safe to trust before taking action (ticket 09-01)

### Daily Loop MVP (Phase 10)

- [ ] **MORNING-01**: Operators can manually start a daily morning session now, with the contract shaped for future automatic start, using next-12h calendar inputs plus Todoist today/overdue inputs (ticket 10-01)
- [ ] **MORNING-02**: Morning Overview returns a short passive snapshot, no more than two friction callouts, and one to three intent-gathering questions without becoming a verbose dashboard or coaching flow (ticket 10-01)
- [ ] **MORNING-03**: Morning Overview captures signals only and must not create durable commitments before Standup runs (ticket 10-01)
- [ ] **STANDUP-01**: Standup can resume after Morning Overview or start directly, and must reconcile calendar plus compress tasks into must/should/stretch buckets (ticket 10-01)
- [ ] **STANDUP-02**: Standup must end with one to three bounded daily commitments plus explicit deferred tasks, confirmed calendar state, and proposed focus blocks (ticket 10-01)
- [ ] **STANDUP-03**: Standup enforces the three-commitment cap, reprompts once when no commitments are defined, and stays interruptible/skippable/resumable throughout (ticket 10-01)
- [ ] **SESSION-01**: Daily-loop state is typed, durable, inspectable, and resumable without deepening untyped context blobs or client-local policy state (ticket 10-01)
- [ ] **VOICE-01**: Voice-first entry and resume reuse backend-owned Apple/runtime seams while remaining available through text-capable operator shells too (ticket 10-01)

### Agent Grounding (Phase 11)

- [ ] **AGENT-CTX-01**: A typed grounding bundle exposes `Now`, projects, people, commitments, review obligations, and execution handoffs for supervised agents (ticket 11-01)
- [ ] **AGENT-CTX-02**: Agent grounding remains inspectable, traceable, and bounded to persisted records plus explicit explain/reference fields rather than raw unbounded dumps (ticket 11-01)
- [ ] **AGENT-TOOLS-01**: Operator-visible capability summaries describe bounded read, review, and mutation affordances in operator terms instead of low-level internal tool names (ticket 11-01)
- [ ] **AGENT-TOOLS-02**: Missing grants, blocked mutations, or unsupported requests fail closed and expose the narrow escalation or review gate required to proceed (ticket 11-01)
- [ ] **AGENT-REVIEW-01**: Existing review queues, SAFE MODE constraints, and approval flows remain intact while being surfaced in agent-relevant terms (ticket 11-01)
- [ ] **AGENT-TRUST-01**: Operator surfaces show what an agent can currently see and do, plus why, from one backend-owned policy contract shared across API, CLI, and web surfaces (ticket 11-01)

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
| BACKUP-01 | Phase 9 | 09-01 | Pending |
| BACKUP-02 | Phase 9 | 09-01 | Pending |
| CTRL-01 | Phase 9 | 09-01 | Pending |
| CTRL-02 | Phase 9 | 09-01 | Pending |
| MORNING-01 | Phase 10 | 10-01 | Pending |
| MORNING-02 | Phase 10 | 10-01 | Pending |
| MORNING-03 | Phase 10 | 10-01 | Pending |
| STANDUP-01 | Phase 10 | 10-01 | Pending |
| STANDUP-02 | Phase 10 | 10-01 | Pending |
| STANDUP-03 | Phase 10 | 10-01 | Pending |
| SESSION-01 | Phase 10 | 10-01 | Pending |
| VOICE-01 | Phase 10 | 10-01 | Pending |
| AGENT-CTX-01 | Phase 11 | 11-01 | Pending |
| AGENT-CTX-02 | Phase 11 | 11-01 | Pending |
| AGENT-TOOLS-01 | Phase 11 | 11-01 | Pending |
| AGENT-TOOLS-02 | Phase 11 | 11-01 | Pending |
| AGENT-REVIEW-01 | Phase 11 | 11-01 | Pending |
| AGENT-TRUST-01 | Phase 11 | 11-01 | Pending |

**Coverage:**
- v1 requirements: 46 total
- Mapped to phases: 46
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-18*
*Last updated: 2026-03-19 — Phases 10 and 11 requirement contracts added; all 46 requirements confirmed mapped across Phases 2–4 and 9–11*
