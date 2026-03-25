# Phase 109: Velocity Cleanup - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning
**Source:** `docs/VELOCITY-DRIFT-CLEANUP.md` plus operator guidance on behavior-preserving splits

<domain>
## Phase Boundary

This phase closes the navigability and file-organization slice from the audit:

- `VD-07` split `crates/veld/src/app.rs`
- `VD-08` split `crates/veld/src/services/now.rs`
- `VD-09` reorganize `crates/vel-api-types/src/lib.rs`

The phase should be behavior-preserving by default. Small boundary cleanups are allowed only when they materially simplify the split and remain easy to review.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** Preserve runtime behavior and public API shape unless a narrow cleanup is clearly justified by the modularization itself.
- **D-02:** The goal is navigability and ownership clarity, not a disguised feature rewrite.
- **D-03:** Tests/build checks must prove the split is behavior-preserving.
- **D-04:** Re-export patterns are acceptable in `vel-api-types` so consumers do not need churn just because files moved.

## Agent Discretion

- exact submodule names may vary if they reflect the current code better
- one large file may be split in more than one pass if that reduces review risk
- small adjacent cleanup is allowed when it removes obvious friction introduced by the old file layout

</decisions>

<canonical_refs>
## Canonical References

- `docs/VELOCITY-DRIFT-CLEANUP.md`
- `docs/templates/agent-implementation-protocol.md`
- `.planning/ROADMAP.md`
- `crates/veld/src/app.rs`
- `crates/veld/src/routes/mod.rs`
- `crates/veld/src/services/now.rs`
- `crates/vel-api-types/src/lib.rs`

</canonical_refs>

<deferred>
## Deferred Ideas

- any behavior change that is not required by the split itself
- repo-wide renaming or transport contract churn
- unrelated cleanup in other large files not named by the audit

</deferred>
