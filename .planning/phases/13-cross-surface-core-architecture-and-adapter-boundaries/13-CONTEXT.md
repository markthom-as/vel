# Phase 13: Cross-surface core architecture and adapter boundaries - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** User-provided Rust core integration spec + current roadmap/codebase analysis

<domain>
## Phase Boundary

Phase 13 defines how Vel should preserve one canonical Rust-owned product core across Apple, web, and future desktop shells without forcing a mass rewrite or a premature UI-first product shape.

This phase is about architecture truth, adapter boundaries, and migration intent. It should establish the canonical command/query/read-model language, the allowed shell integration patterns, the current-state-to-target-state map for the existing crate graph, and the future migration path for Apple embedded Rust / FFI without requiring that full migration to ship now.

This phase is not broad UI work, not a full Apple FFI implementation, not a Tauri product launch, and not a big-bang crate rename.

</domain>

<decisions>
## Implementation Decisions

### Locked product/architecture direction
- [locked] Vel should use one real Rust product core across shells rather than letting Apple, web, or future desktop clients own their own product logic.
- [locked] The important goal is universal business logic and stable cross-surface contracts, not crate-name purity.
- [locked] Contracts and canonical read models should be defined before broad new UI work widens.
- [locked] Tauri/desktop should be planned for, but it can wait as an implementation target.

### Current-repo alignment
- [locked] The existing repo is not a greenfield workspace. Planning must align with the current split around `vel-core`, `vel-storage`, `vel-api-types`, `veld`, `vel-cli`, and the current Apple/web clients.
- [locked] Phase 13 should prefer incremental evolution of current crates and seams over a mass rename into a brand-new crate taxonomy.
- [locked] Existing architectural drift should not be normalized, but nearby good-enough seams should be extended rather than rewritten for aesthetics.

### Apple integration stance
- [locked] Apple currently uses an HTTP-first model through `VelAPI` talking to `veld`; that remains the active integration model.
- [locked] Phase 13 should document the future Apple embedded-Rust / FFI migration path, but should not require a full Apple FFI migration in this phase.
- [locked] Apple-native presentation, App Intents, widgets, notifications, complications, and lifecycle glue remain shell-owned concerns; product logic should remain Rust-owned.

### Web and desktop stance
- [locked] Web should continue to consume Rust through authenticated HTTP/JSON contracts and streaming where helpful, not through direct browser exposure to internal core structs.
- [locked] Future desktop/Tauri work should be designed as a shell over the same Rust-owned contracts, ideally compatible with either local-daemon or in-process adapter patterns.
- [locked] Phase 13 should plan with embedded, daemon, and server runtime topologies in mind, even if not all are implemented now.

### Sequencing and scope discipline
- [locked] Phase 13 is architecture-first.
- [locked] Phase 14 should handle product discovery, operator modes, advanced/dev gating, and milestone shaping.
- [locked] Later phases should do incremental architecture migration first, then canonical Rust business logic, then broader shell/UI embodiment.
- [locked] UX discovery should not be left to emerge accidentally from current UI work.

### What this phase should likely produce
- [auto] one canonical architecture doc for cross-surface core/adapters/topologies
- [auto] a current-state to target-state mapping for existing crates and shell boundaries
- [auto] explicit ownership rules for commands, queries, events, read models, and transport DTOs
- [auto] a documented future Apple FFI/embedded path
- [auto] a documented future desktop/Tauri adapter path
- [auto] at least one proof-oriented vertical slice plan that shows how a real flow should cross the new boundary

### What this phase should avoid
- [auto] broad UI redesign or shell-specific interaction work
- [auto] crate churn that does not remove a concrete architecture problem
- [auto] duplicating planning logic in Swift, React, or Tauri handlers
- [auto] turning adapter crates into business-logic ownership layers

### Claude's Discretion
- Exact contract names for canonical commands, queries, read models, capability summaries, and adapter surfaces
- Whether the proof slice should center on the daily loop, dashboard snapshot, agent grounding, or another already-shipped seam
- Whether the first migration artifact is doc-only, contract-only, or includes one narrow code-backed adapter seam, as long as the architecture-first purpose remains intact

</decisions>

<specifics>
## Specific Ideas

- The user’s integration spec is directionally correct: one Rust substrate, multiple shells, with support for embedded, daemon, and server modes.
- The current repo already behaves this way at a high level for web and Apple HTTP: `veld` is the authority and clients should not own business logic.
- The main architectural risk is not missing a perfect crate diagram; it is letting shell-specific flows continue to define product truth before command/query/read-model contracts are stabilized.
- A good proof seam for this phase likely builds on a flow already present in the system, such as:
  - daily loop / morning flow
  - `Now` / dashboard snapshot
  - agent inspect / grounding
  - operator review queue

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and project authority
- `.planning/ROADMAP.md` — Phase 13-16 sequencing and current post-Phase-12 roadmap shape
- `.planning/PROJECT.md` — accepted product-direction decisions
- `.planning/STATE.md` — current milestone state and accumulated decisions
- `docs/MASTER_PLAN.md` — canonical implementation truth and historical phase status
- `README.md` — repo entrypoint and current system framing

### Architecture and workflow guidance
- `AGENTS.md` — durable repository rules, layering, and workflow expectations
- `docs/templates/agent-implementation-protocol.md` — normative planning/execution protocol
- `docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md` — broader concept framing
- `docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md` — subsystem and cross-cutting discipline
- `config/README.md` — config/schema/manifest maintenance rules for any durable contract output

### Current runtime and transport boundaries
- `docs/api/runtime.md` — actual mounted runtime/API authority and auth classes
- `crates/veld/src/app.rs` — route mounting and authority boundary
- `crates/vel-api-types/src/lib.rs` — current transport DTO ownership
- `crates/vel-core/src/lib.rs` and nearby modules — current domain/core ownership
- `crates/vel-storage/src/lib.rs` — current storage boundary

### Apple and shell boundary evidence
- `clients/apple/README.md` — explicit statement that Apple clients currently talk to the same daemon over HTTP
- `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift` — active Apple transport boundary
- `clients/web/src/data/` and `clients/web/src/types.ts` — active web transport/read-model consumption pattern

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `veld` already acts as the authority/runtime host for Apple and web clients.
- `vel-api-types` already gives the repo a shared transport DTO seam.
- Phase 10 and Phase 11 already introduced backend-owned daily-loop and agent-inspect contracts that multiple shells can consume.
- Apple currently has a clean enough boundary in `VelAPI` to document a future alternative integration path without guessing.

### Missing or Thin Areas
- There is no single canonical doc that explains how embedded, daemon, and server topologies relate to the current codebase.
- There is no explicit current-state-to-target-state crate responsibility map for cross-surface evolution.
- There is no documented future Apple FFI/embedded migration path.
- Future desktop/Tauri integration is not yet given an adapter/ownership plan.
- Product logic and shell logic are directionally aligned today, but the long-term contract vocabulary is still implicit rather than explicitly ratified.

</code_context>

<deferred>
## Deferred Ideas

- Full Apple FFI / UniFFI migration
- A real Tauri/desktop shell implementation
- Broad UI redesign, navigation overhaul, or shell-specific product-definition work
- Multi-phase crate reorganization that is not justified by a narrow seam migration

</deferred>

---

*Phase: 13-cross-surface-core-architecture-and-adapter-boundaries*
*Context gathered: 2026-03-19*
