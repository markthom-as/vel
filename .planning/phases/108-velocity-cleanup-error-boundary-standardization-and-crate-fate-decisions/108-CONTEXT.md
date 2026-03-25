# Phase 108: Velocity Cleanup - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning
**Source:** `docs/VELOCITY-DRIFT-CLEANUP.md`, `docs/MASTER_PLAN.md`, and current CLI/runtime seams

<domain>
## Phase Boundary

This phase closes two related cleanup lines:

- `VD-10` make error handling more coherent across route, service, and storage layers
- `VD-11` keep `vel-sim` and `vel-agent-sdk`, integrate them through existing CLI surfaces, and document what they are for

The crate work is not a new top-level CLI feature lane. It should hook into existing surfaces such as `vel evaluate`, `vel exec`, or other already-shipped supervised-execution seams.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** `vel-sim` stays. It already underpins deterministic replay/eval work and aligns with the verified simulation/evals line.
- **D-02:** `vel-agent-sdk` stays. `docs/MASTER_PLAN.md` still calls for a unified agent SDK and existing Phase 8/14 research points at it as part of supervised execution closure.
- **D-03:** CLI integration should land under existing command families rather than adding new top-level commands for now.
- **D-04:** Error-boundary cleanup should prefer one clear forward pattern applied to a meaningful seam instead of a shallow repo-wide partial sweep.

## Agent Discretion

- the chosen CLI entry points may be helper flags, nested subcommands, or richer output under existing command families, as long as they stay under existing surfaces
- the error-normalization target seam may be chosen based on the best leverage/reviewability tradeoff, but it should include a real route → service → storage path where possible

</decisions>

<canonical_refs>
## Canonical References

- `docs/MASTER_PLAN.md`
- `docs/VELOCITY-DRIFT-CLEANUP.md`
- `docs/api/runtime.md`
- `docs/user/coding-workflows.md`
- `.planning/ROADMAP.md`
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/commands/evaluate.rs`
- `crates/vel-cli/src/commands/exec.rs`
- `crates/vel-agent-sdk/src/lib.rs`
- `crates/vel-sim/src/lib.rs`
- `crates/veld-evals/src/lib.rs`
- `crates/veld/tests/agent_sdk.rs`
- `.planning/milestones/v0.1-phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-RESEARCH.md`

</canonical_refs>

<deferred>
## Deferred Ideas

- broad new public CLI product surface for simulation/SDK work
- repo-wide error-enum unification if a narrower forward pattern is enough for this phase
- Phase 109 file-split work

</deferred>
