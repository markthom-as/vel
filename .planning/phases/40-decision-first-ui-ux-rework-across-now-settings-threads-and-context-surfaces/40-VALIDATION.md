# Phase 40 Validation

## Required Proofs

- the strict MVP loop is documented as `overview -> commitments -> reflow -> threads -> review`
- Phase 40 artifacts define explicit in-scope and out-of-scope rules that match milestone `v0.2`
- canonical Rust-owned contracts exist for overview, commitments, reflow, threads, and review
- those contracts describe user-visible behavior, state transitions, provenance, and degraded states
- durable docs in `docs/` describe the MVP authority model and shell boundaries
- the locked overview behavior is documented in durable docs: `action + timeline`, one dominant action, compact timeline, one visible nudge, `Why + state`, and 1-3 suggestions with `accept`, `choose`, `thread`, and `close`
- `.planning/` artifacts summarize planning state without becoming the only source of truth
- Phase 42 no longer depends on new local-calendar milestone work

## Verification Approach

- direct document review of:
  - `.planning/PROJECT.md`
  - `.planning/REQUIREMENTS.md`
  - `.planning/ROADMAP.md`
  - Phase 40 planning artifacts
  - `docs/product/mvp-operator-loop.md`
  - `docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md`
  - `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md`
  - `docs/templates/mvp-loop-contract-checklist.md`
- targeted `rg` checks to confirm stale local-calendar or UI-only Phase 40 wording is removed from active artifacts
- targeted `rg` checks to confirm the durable docs explicitly contain `action + timeline`, dominant action, compact timeline, single visible nudge, `Why + state`, 1-3 suggestions, and the `accept|choose|thread|close` fallback path
- traceability check that Phase 40 outputs map to the requirements and downstream roadmap phases

## Acceptance Markers

- downstream phases can implement without re-deciding MVP scope
- the overview/reflow/thread boundary is clear enough to reject shell-owned product logic
- degraded-state behavior is specified instead of left for later UI improvisation
- the spec packet is durable, navigable, and aligned with milestone `v0.2`
