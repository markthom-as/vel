# Phase 106: Velocity Cleanup - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning
**Source:** `docs/VELOCITY-DRIFT-CLEANUP.md` plus operator clarification

<domain>
## Phase Boundary

This phase closes the lowest-risk cleanup items from the velocity-drift audit:

- `VD-01` remove the single-implementation `CapabilityResolver` trait
- `VD-02` remove the single-implementation `ToolRunner` trait
- `VD-03` delete dead policy structs/accessors in `policy_config.rs`
- `VD-04` reduce or remove blanket `dead_code` suppressions in the touched crates

The phase is intentionally narrow. It should simplify active code paths and make warning debt more truthful without widening into time-library migration, database cleanup, crate-retention work, or large module splits.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** Low-risk cleanup may be pulled forward when it materially simplifies active work, even though the cleanup lane is queued after `0.5.7`.
- **D-02:** Gradual warning cleanup is acceptable for now, but any remaining suppression must be explicit, local, and documented.
- **D-03:** Punts should be avoided; if any dead-code cluster remains deferred, the plan must record why it remains and what follow-on owns it.
- **D-04:** This phase should prefer no-behavior-change simplification. It should not reopen broader route, API, or product-surface decisions.

## Agent Discretion

- exact helper names may change if they improve readability and fit existing Rust style
- the warning-cleanup sequence may be split by crate or module if that reduces review risk
- targeted allows are acceptable where the code is intentionally staged future work and immediate activation would widen scope

</decisions>

<canonical_refs>
## Canonical References

- `docs/VELOCITY-DRIFT-CLEANUP.md`
- `docs/DEAD-CODE.md`
- `docs/templates/agent-implementation-protocol.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- `crates/veld/src/services/capability_resolver.rs`
- `crates/veld/src/services/tool_runner.rs`
- `crates/veld/src/policy_config.rs`
- `crates/veld/src/services/command_lang.rs`
- `crates/veld/src/lib.rs`
- `crates/veld/src/main.rs`
- `crates/vel-cli/src/client.rs`

</canonical_refs>

<deferred>
## Deferred Ideas

- `VD-05` chrono/time migration
- `VD-06` orphaned schema drop migration
- `VD-07` through `VD-09` monolithic file splits
- `VD-10` full cross-layer error-boundary normalization
- `VD-11` `vel-sim` and `vel-agent-sdk` CLI integration work

</deferred>
