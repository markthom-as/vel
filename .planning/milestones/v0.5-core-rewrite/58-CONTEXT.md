# Phase 58: Canonical Object Kernel And System-Of-Record Storage Rewrite - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 58 is the first implementation-heavy phase in milestone `0.5`.

Its job is to take the Phase 57 contract packet and build the durable substrate that makes Vel the live system of record for canonical objects, relations, integration accounts, sync links, runtime records, projections, and migration/cutover scaffolding.

This phase is still backend-only. It should not widen into action-membrane policy work, module loading, adapter feature work, or UI/client embodiment. Those belong to downstream phases.

</domain>

<decisions>
## Locked Dependencies From Phase 57

- **D-01:** Canonical `content`, `registry`, `read_model`, and `runtime` classes remain distinct.
- **D-02:** Canonical linkage truth remains external through `SyncLink` plus typed relations.
- **D-03:** `source_summary` is derived only.
- **D-04:** `Availability` remains a read model.
- **D-05:** `WriteIntent` remains a provider-agnostic runtime record outside canonical content storage.
- **D-06:** Domain logic must stay storage-agnostic behind traits.
- **D-07:** Typed Rust ID newtypes are the intended backend model.
- **D-08:** Projections remain rebuildable and non-authoritative.
- **D-09:** Bootstrap and seeding must be deterministic and idempotent.
- **D-10:** Secrets remain outside canonical account objects.

### Phase 58-specific implementation posture
- **D-11:** Phase 58 should build the minimum durable substrate first, not try to implement full provider semantics.
- **D-12:** New persistence seams should prefer dedicated repositories/stores over re-centralizing logic into `db.rs` or equivalent god-modules.
- **D-13:** Migration scaffolding in this phase should support cutover readiness without creating a permanent dual-write architecture.
- **D-14:** The first embedded backend target should remain local-first SQLite-backed storage behind explicit traits.
- **D-15:** Projection rebuild and query contracts should be storage-neutral, even if SQLite is the first concrete implementation.

</decisions>

<specifics>
## Specific Ideas

- The hardest mistake Phase 58 can make is collapsing back into provider-shaped or legacy-shaped persistence while calling it canonical.
- The second-hardest mistake is building storage so concretely around SQLite that later targets and test backends become ceremonial.
- This phase should leave behind a substrate that Phase 59 can put a membrane on, not a half-membrane baked directly into repositories.
- Migration and cutover scaffolding need to be real enough to prove feasibility, but not so invasive that Phase 58 becomes the full cutover phase.

</specifics>

<canonical_refs>
## Canonical References

### Milestone and phase authority
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/57-VALIDATION.md`
- `.planning/milestones/v0.5-core-rewrite/57-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/57-DEPENDENCY-AND-INVARIANTS.md`
- `.planning/milestones/v0.5-core-rewrite/57-RUST-BACKEND-CONTRACT-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/57-TEST-PROVING-LADDER.md`
- `.planning/milestones/v0.5-core-rewrite/57-RISK-SPIKES.md`

### Durable architecture contracts
- `docs/cognitive-agent-architecture/architecture/0.5-canonical-object-model.md`
- `docs/cognitive-agent-architecture/architecture/0.5-canonical-relations-and-linkage.md`
- `docs/cognitive-agent-architecture/architecture/0.5-action-membrane-and-policy.md`
- `docs/cognitive-agent-architecture/architecture/0.5-ownership-conflict-and-write-intent.md`
- `docs/cognitive-agent-architecture/architecture/0.5-module-skill-tool-and-workflow-registry.md`
- `docs/cognitive-agent-architecture/architecture/0.5-workflow-runtime-primitives.md`
- `docs/cognitive-agent-architecture/architecture/0.5-rust-backend-implementation-constraints.md`
- `docs/cognitive-agent-architecture/architecture/0.5-required-backend-traits-and-capability-matrix.md`
- `docs/cognitive-agent-architecture/architecture/storage-layer.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths
- `vel-storage` already uses a modular repository pattern and explicit storage facade language.
- Existing migration machinery and repository seams can inform the new substrate if they are extended rather than bypassed.
- The codebase already has local-first SQLite expectations and a clear preference for typed domain returns over DTO leakage.

### Existing weaknesses to avoid
- Current persistence remains shaped around historical product seams rather than the new canonical object-centered model.
- Existing integration and runtime storage are fragmented and not yet normalized around canonical IDs, relations, sync links, or runtime record families.
- `CurrentContext`-style JSON-heavy legacy patterns remain a cautionary example, not a precedent.

### High-risk implementation areas
- optimistic concurrency and version semantics across canonical objects and SyncLinks
- idempotent registry/bootstrap/migration interplay
- keeping projections rebuildable while still queryable and useful
- introducing new stores without accidentally hardwiring SQLite assumptions into domain contracts

</code_context>

<deferred>
## Deferred Ideas

- action membrane implementation details beyond the storage hooks Phase 59 needs
- module loader execution
- workflow runtime execution
- provider-specific sync behavior beyond what the substrate must store
- UI/client contract repair

</deferred>

---

*Context gathered: 2026-03-22 for Phase 58 planning*
