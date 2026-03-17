# Phase 4 Parallel Execution Board

This board defines Phase 4 parallel execution ownership with non-overlapping primary write scopes and a three-sub-phase rollout.

This board is execution guidance, not shipped-behavior authority.
Shipped behavior remains anchored in `docs/MASTER_PLAN.md`.

## Sub-Phase 1: Contract & Safety Foundations

Goal: lock semantic-memory, sandbox, and external-SDK contracts before broad runtime implementation.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Semantic Memory Contracts | `009` | `crates/vel-core/src/`, `docs/cognitive-agent-architecture/cognition/` | queued | Define semantic query/index and provenance contracts. |
| B: Sandbox Host ABI Contracts | `010` | `crates/vel-core/src/`, `docs/cognitive-agent-architecture/agents/` | queued | Define deny-by-default host ABI and policy boundaries. |
| C: External SDK/Protocol Contracts | `014` | `docs/`, `config/`, protocol fixture/schema surfaces | queued | Define versioned protocol envelopes and capability negotiation semantics. |

Sub-phase 1 merge gate:

- contract schemas are explicit and versioned for all three tickets
- security boundaries (mediation, deny-by-default, no self-escalation) are documented and testable
- protocol and host ABI fixtures parse in automated checks

## Sub-Phase 2: Core Runtime Implementations

Goal: implement semantic indexing/retrieval and sandbox runtime with traceable policy enforcement.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Semantic Index Runtime | `009` | `crates/vel-storage/src/`, `crates/veld/src/services/` | queued | Build index lifecycle and hybrid retrieval paths. |
| B: WASM Sandbox Runtime | `010` | `crates/veld/src/`, `crates/vel-api-types/src/` | queued | Embed sandbox host with policy hooks and lifecycle events. |
| C: Observability & Hardening | `009`, `010` | `crates/veld/src/routes/`, operator diagnostics surfaces | queued | Expose provenance, denials, and degraded-mode inspection. |

Sub-phase 2 merge gate:

- semantic retrieval is functional and provenance-aware
- sandbox execution enforces host ABI + capability broker mediation
- operators can inspect policy denials and runtime outcomes

## Sub-Phase 3: SDK Delivery & End-to-End Swarm Closure

Goal: ship external SDK protocol path integrated with connect/sandbox/memory capabilities.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Protocol Crate | `014` | `crates/vel-protocol/` (new) | queued | Implement envelope serialization/validation/versioning. |
| B: Reference SDKs | `014` | `crates/vel-agent-sdk/` (new), TS SDK surface | queued | Deliver heartbeat, capability negotiation, and action submission helpers. |
| C: End-to-End Integration | `014`, `010`, `009` | `crates/veld/src/services/`, integration tests, docs | queued | Validate delegated flows from SDK to runtime policy enforcement. |

Sub-phase 3 merge gate:

- SDK clients can connect, heartbeat, and submit scoped actions
- protocol contracts are fixture-tested and versioned
- end-to-end flows preserve mediation and traceability invariants

## Dependency Order

1. Sub-phase 1 establishes contract and security boundaries.
2. Sub-phase 2 ships runtime implementations behind those boundaries.
3. Sub-phase 3 ships external SDK surfaces and end-to-end integration.

## Coordination Rules

- Keep protocol, sandbox, and semantic-memory contracts versioned and fixture-backed.
- Do not allow sandbox or SDK lanes to bypass capability broker enforcement.
- If a lane modifies operator-visible behavior, update docs and diagnostics in the same slice.
- Every lane includes command-backed verification evidence before merge.

## Suggested Verification Commands

- `cargo test -p veld`
- `cargo test -p vel-storage`
- `cargo test -p vel-core`
- `node scripts/verify-repo-truth.mjs`

## First PR Batches

### Lane A: Semantic Memory Contracts (`009`)

1. Define semantic record/query contract types in `vel-core` plus fixture examples.
2. Add semantic-index backend trait seam in `vel-storage` (no backend implementation yet).
3. Document provenance requirements and hybrid ranking policy contract.

### Lane B: Sandbox Host ABI Contracts (`010`)

1. Define host ABI call envelope and deny-by-default policy schema.
2. Add capability mediation hooks and denial record contract fields.
3. Add sandbox policy docs covering resource limits and no-self-escalation rules.

### Lane C: External SDK/Protocol Contracts (`014`)

1. Define `vel-protocol` envelope/versioning schema with parse tests.
2. Define capability negotiation and heartbeat lease message contracts.
3. Publish protocol fixture set and integration contract docs for Rust/TS SDK consumers.
