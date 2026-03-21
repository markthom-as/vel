## 43-01 Summary

Locked the bounded thread continuation contract and exposed it through typed thread transport so shells no longer need to infer continuation posture from raw metadata alone.

### What changed

- Updated [mvp-loop-contracts.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md) so `ThreadEscalation` now explicitly requires `bounded_capability_state` alongside escalation reason, continuation context, and review requirements.
- Updated [now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md) so thread detail now explicitly owns escalation reason, continuation context, and remaining review gate for bounded MVP follow-through.
- Extended [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with typed `ThreadContinuationData` on `ThreadData`.
- Updated [threads.rs](/home/jove/code/vel/crates/veld/src/routes/threads.rs) to map existing proposal-style thread metadata into typed continuation fields for `assistant_proposal`, `planning_profile_edit`, `reflow_edit`, and `day_plan_apply`.
- Updated [command_lang.rs](/home/jove/code/vel/crates/veld/src/routes/command_lang.rs) so non-thread-continuation command-language thread DTOs default `continuation` safely to `None`.

### Verification

- `cargo test -p veld routes::threads::tests -- --nocapture`

### Outcome

Phase 43 now starts from one explicit continuation vocabulary: why the work escalated, what bounded context came with it, what review gate still exists, and what capability posture remains in force.
