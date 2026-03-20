# Phase 2: Distributed State, Offline Clients & System-of-Systems - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 2 delivers the distributed runtime seams that enable the system to: (1) ingest signals from pluggable sources via a reducer pipeline, (2) maintain consistent ordering across nodes via HLC, (3) launch and supervise agent processes via a typed connect protocol, (4) broker capabilities without leaking raw credentials, and (5) present resolved effective configuration to the operator across CLI, web, and native surfaces.

Authoritative specifications are in `docs/tickets/phase-2/` — tickets 004, 005, 006, 012, 016, 019.
The parallel execution board (`docs/tickets/phase-2/parallel-execution-board.md`) defines the three-sub-phase rollout and lane ownership.

</domain>

<decisions>
## Implementation Decisions

### Execution Strategy
- Follow the parallel execution board sub-phase structure: SP1 (contract alignment) → SP2 (core runtime) → SP3 (onboarding + conflict hardening + config closure)
- SP1 executes first as a Wave 0 contract-alignment pass before feature work
- SP2 lanes A/B/C execute in parallel within a single wave (non-overlapping write scopes)
- SP3 lanes execute in parallel within a single wave after SP2 gate passes

### Ticket Specifications
- All ticket acceptance criteria are authoritative — do not re-derive or change scope
- Ticket 006 (Connect): current state is shell routes returning 403; full lifecycle (launch/heartbeat/lease-expiry/terminate) must be implemented
- Ticket 004 (Signal Reducer): extract `inference.rs` into a reducer trait + registry; keep replay deterministic
- Ticket 016 (Capability Broker): agents-only scope (confirmed); integrations deferred to later milestone
- Ticket 005 (HLC): NodeIdentity type prerequisite; deterministic ordering primitive required
- Ticket 012 (Onboarding): `vel node link` command + pairing flow + trust/config visibility
- Ticket 019 (Operator Accessibility): effective config display (not raw config) across all operator surfaces

### Test Strategy
- Multi-node sync: mock/simulated nodes within a single process for determinism
- HLC tests: timestamp injection for deterministic replay
- Connect lifecycle: end-to-end testable via test-app pattern (existing 160 auth tests as model)
- Use `cargo test -p veld`, `cargo test -p vel-cli`, `cargo test -p vel-storage` as verification commands

### Frontend Depth
- Web dashboard changes: functional production quality following existing component patterns
- Apple client: include `vel node link` pairing flow (SP3 scope per ticket 012); defer cosmetic polish

### Claude's Discretion
- Exact storage schema additions for HLC timestamps, lease tracking, capability token table
- Internal service function signatures within the prescribed external contracts
- Error message text in CLI output (follow existing conventions)
- Test helper structure within each test module

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/veld/src/services/client_sync.rs` — worker heartbeat TTL infrastructure (basis for connect lifecycle lease tracking)
- `crates/veld/src/services/inference.rs` (1,796 lines) — existing inference loop to be extracted into reducer trait + registry (ticket 004)
- `crates/vel-storage/src/repositories/` — per-entity repository pattern; new tables follow this pattern
- `crates/veld/src/middleware/mod.rs` — auth middleware (just extracted in Phase 1.1); `ExposureGate` pattern for new routes
- `vel-api-types/src/lib.rs` — `ApiResponse<T>` wrapper; all new routes use this

### Established Patterns
- Route handlers: thin (parse → auth → service call → DTO → error map)
- Services: return domain types from `vel-core`, never HTTP DTOs
- Storage: per-entity repository modules, compile-time verified sqlx queries
- Authentication: `ExposureGate` with `OperatorAuthenticated` for operator routes, `WorkerAuthenticated` for agent routes
- Error handling: `AppError` with `StorageError` propagation via `?`

### Integration Points
- New `/v1/connect/*` routes → `app.rs` operator/worker route builders
- New signal reducer trait → `vel-core/src/` (domain type) + `veld/src/services/` (registry)
- HLC type → `vel-core/src/` (new primitive)
- Capability broker service → `veld/src/services/broker.rs` (new)
- `vel node link` command → `vel-cli/src/commands/node.rs` (new)

</code_context>

<specifics>
## Specific Ideas

- Ticket 006 baseline section was captured as a pending todo: document the current state (heartbeat infra, 403 shell routes, CLI stubs) before implementing the full lifecycle
- Ticket 016 broker scope decision locked: agents-only (confirmed by user 2026-03-18)
- Ticket 005 NodeIdentity prereq documented in pending todo
- Follow `docs/templates/agent-implementation-protocol.md` for every ticket per CLAUDE.md instructions

</specifics>

<deferred>
## Deferred Ideas

- Integration-level capability brokering (integrations delegating to broker) — deferred to later milestone per scope decision
- Apple client cosmetic/accessibility polish beyond the `vel node link` pairing flow
- Multi-process multi-node integration tests (single-process mocks sufficient for Phase 2)

</deferred>
