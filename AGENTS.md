# AGENTS.md

This document defines durable repository rules and the **Agentic Implementation Protocol** for AI coding agents working in Vel.

## Authority

- **Canonical Truth**: Repo-wide implementation status and architectural roadmap live in **`docs/MASTER_PLAN.md`**.
- **Workflow Protocol**: Standardized implementation steps for agents live in **`docs/templates/agent-implementation-protocol.md`**.
- If `AGENTS.md` and the Master Plan appear to conflict, treat **`docs/MASTER_PLAN.md`** as canonical.

## Durable Repository Rules

### 1. Domain & Layering
- **`vel-core`**: Owns domain semantics, domain types, and system invariants. It must remain a "pure" library (no IO/Network).
- **`vel-storage`**: Must not depend on `vel-api-types`. Use the **Repository Pattern** and **Unit of Work** for transactional safety.
- **`vel-api-types`**: Contains transport DTOs only.
- **Boundary Rule**: Services return domain entities; Routes map to DTOs. No DTOs in the service layer.

### 2. State & Consistency
- **Context Integrity**: The `current_context` must be handled as a versioned, strictly typed Rust struct, not a raw JSON blob.
- **Distributed Identity**: Use prefixed UUIDs (e.g., `wrkreq_`, `run_`) and Hybrid Logical Clocks (HLC) for LWW conflict resolution.
- **Event Sourcing**: Prefer append-only logs (`RunEvent`, `WorkAssignmentReceipt`) for distributed state changes.

### 3. Verification & Safety
- **Surgical Edits**: Use targeted `replace` calls. Minimize full-file `write_file` unless creating new modules.
- **Test-First**: Every logic change must be accompanied by a unit test in the affected module or an integration test in `crates/veld/tests/`.
- **Simulation**: Use the **Day-Simulation Harness** (`Phase 3`) to verify long-term reasoning stability.

## Agent Workflow (ADX)

Before starting work, an agent MUST:

1.  Read **`docs/MASTER_PLAN.md`** to understand current status and phase goals.
2.  Locate the relevant ticket in **`docs/tickets/`**.
3.  Follow the **`docs/templates/agent-implementation-protocol.md`** strictly:
    -   **Research**: Use `grep_search` and `glob` to map symbols.
    -   **Strategy**: Formulate a concise plan.
    -   **Act**: Apply surgical changes.
    -   **Validate**: Run `cargo test` and `vel doctor`.

## Development Principles

- **Local-First**: All core functionality must work without internet access.
- **Explainability**: Every nudge and synthesis must link back to its source signals (Provenance).
- **Zero-Trust (Phase 4)**: Agent skills must operate within WASM sandboxes.

## Priority Order (The Master Plan)

1.  **Phase 1**: Foundations (Monolith decomposition, Repository pattern, Typed context).
2.  **Phase 2**: Swarm & Sync (HLCs, Signal Reducers, Connect Launch).
3.  **Phase 3**: Verification (Simulation harness, LLM-as-a-Judge).
4.  **Phase 4**: Autonomy (Semantic Graph RAG, WASM Sandboxing, P2P Sync).

## Early Non-Goals

- Premature UI polish (prefer functionality and state correctness).
- Cloud-only features or proprietary vendor lock-in.
- Speculative integrations not defined in the Master Plan.
