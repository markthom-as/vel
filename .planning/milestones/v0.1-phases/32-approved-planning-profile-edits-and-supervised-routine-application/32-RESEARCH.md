# Phase 32 Research

## Domain

Turn staged planning-profile proposals into a real supervised application lane over the canonical backend-owned routine/planning profile.

## Locked Inputs

- Phase 30 shipped the canonical planning-profile management seam via `GET /v1/planning-profile` and `PATCH /v1/planning-profile`.
- Phase 31 extended read parity across web, CLI, and Apple and let assistant/voice entry stage bounded `PlanningProfileEditProposal` values with explicit `Threads` continuity.
- Product policy still requires one backend-owned planning substrate, explicit approval for consequential conversational edits, and no silent planner mutation from shells.
- Assistant action phases already established a proposal lifecycle shape (`staged`, approved/apply follow-through, continuity in `Threads`) that should be reused rather than duplicated.

## Problem

Vel can now do two adjacent things, but not the bridge between them:

- direct operator edits can already mutate the canonical planning profile
- assistant and Apple voice can already stage bounded planning-profile proposals

What is missing is the supervised application lane that resolves a staged proposal through review and then applies it through the same canonical backend mutation seam as direct edits.

Without that bridge:

- conversational planning help remains incomplete
- operators must manually recreate approved changes elsewhere
- `Threads` continuity can say what was proposed, but not what actually became durable profile state

## Required Truths

1. One canonical mutation seam
   - approved conversational proposals must apply through the existing backend-owned planning-profile mutation model
   - no shell should gain a privileged direct-write shortcut

2. Proposal lifecycle stays explicit
   - staged, approved, applied, failed, and reversed or dismissed outcomes should remain inspectable
   - `Threads` continuity should remain the durable source of follow-through truth

3. Shells stay thin
   - web, Apple, CLI, and conversational entry surfaces may review or reflect outcomes
   - planner semantics, validation, and persistence remain in Rust backend layers

4. Scope stays bounded
   - this phase is about routine blocks and bounded planning constraints already covered by the canonical planning-profile vocabulary
   - it must not widen into autonomous planner mutation, broad calendar editing, or a second review system outside the existing supervised lanes

## Recommended Execution Shape

Phase 32 should be executed in four slices:

1. publish the approved-application contract and lifecycle transitions for planning-profile proposals
2. implement backend approval/application over the canonical mutation seam
3. expose review pressure and applied-outcome continuity across shipped surfaces without turning them into planners
4. close with docs/examples/verification for the supervised planning-profile application model

## Code Context

- `crates/vel-core/src/planning.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/services/apple_voice.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/planning_profile.rs`
- `crates/veld/src/routes/threads.rs`
- `crates/veld/src/services/operator_queue.rs`
- `clients/web/src/components/`
- `clients/web/src/types.ts`
- `clients/apple/`
- `crates/vel-cli/src/commands/`
- `docs/cognitive-agent-architecture/architecture/`
- `docs/api/`
- `docs/user/`

## Risks

- accidentally creating a planner-specific approval model instead of reusing the existing supervised proposal/application vocabulary
- letting conversational flows apply planning-profile edits without clear approval and continuity
- duplicating review state between `Settings`, `Threads`, and assistant/voice shells
- drifting into a broader planner/editor scope while trying to make the flow feel convenient

## Success Condition

Phase 32 is complete when the product can honestly say:

- bounded planning-profile proposals from assistant or Apple voice can move from `staged` to an approved/applicable backend-owned change
- applied outcomes resolve through the same canonical mutation seam used by direct Settings edits
- `Threads` continuity and summary-first surfaces show what was proposed, approved, applied, failed, or dismissed
- cross-surface docs teach one supervised planning-profile application story and its current limits
