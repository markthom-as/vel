# Phase 33 Research

## Domain

Turn bounded same-day `day_plan` and `reflow` outputs into a real supervised application lane over canonical commitment scheduling.

## Locked Inputs

- Phases 26 through 29 shipped the bounded same-day planning substrate: canonical scheduler rules, explainable `reflow`, `day_plan`, and durable routine/planning inputs.
- Phase 30 shipped canonical planning-profile management via backend-owned transport and persisted profile mutation.
- Phases 31 and 32 extended supervised conversational parity and approved apply flow for planning-profile edits, with explicit lifecycle continuity in `Threads`.
- Product policy still requires one backend-owned write seam, explicit supervision for consequential assistant-originated changes, and summary-first shells that report continuity instead of becoming planners.

## Problem

Vel can now explain same-day scheduling decisions well, but it still stops short of applying them:

- `day_plan` can say what should be scheduled, deferred, or left out
- `reflow` can say how the remaining day should be repaired
- direct planning-profile edits can already apply through a supervised backend path

What is missing is the supervised bridge from approved same-day planning output to durable commitment scheduling mutation.

Without that bridge:

- operators must manually recreate approved schedule changes
- same-day planning remains advisory rather than operational
- `Threads` continuity can show proposed day-shaping but not what actually became the durable schedule

## Required Truths

1. One canonical commitment-scheduling seam
   - approved `day_plan` and `reflow` changes must apply through backend-owned commitment mutation paths
   - shells must not gain planner-write shortcuts

2. Scope stays same-day and bounded
   - this phase applies same-day commitment schedule changes only
   - it must not widen into multi-day planning or broad autonomous calendar editing

3. Proposal lifecycle stays explicit
   - staged, approved, applied, failed, and reversed or dismissed outcomes should remain inspectable
   - `Threads` continuity should remain the durable source of follow-through truth

4. Shells stay summary-first
   - `Now`, `Threads`, CLI, Apple, and web `Settings` may reflect pending/applied scheduling continuity
   - planner semantics, validation, and persistence remain in Rust backend layers

## Recommended Execution Shape

Phase 33 should be executed in four slices:

1. publish the approved day-plan/reflow application contract and lifecycle over commitment scheduling
2. implement backend application of bounded same-day planning changes through canonical commitment mutation seams
3. expose pending/applied/failure continuity across shipped surfaces without creating a second planner
4. close with docs/examples/verification for supervised same-day plan application

## Code Context

- `crates/vel-core/src/`
- `crates/vel-storage/src/repositories/commitments_repo.rs`
- `crates/veld/src/services/day_plan.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/routes/threads.rs`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/components/`
- `clients/web/src/types.ts`
- `crates/vel-cli/src/commands/`
- `clients/apple/`
- `docs/cognitive-agent-architecture/architecture/`
- `docs/api/`
- `docs/user/`

## Risks

- accidentally creating a planner-specific write path instead of reusing canonical commitment mutation seams
- letting `day_plan` or `reflow` apply changes without explicit supervised follow-through
- duplicating day-shaping authority across `Now`, `Threads`, CLI, Apple, and web `Settings`
- widening into multi-day planning or hidden calendar mutation while trying to make same-day scheduling convenient

## Success Condition

Phase 33 is complete when the product can honestly say:

- bounded same-day `day_plan` and `reflow` changes can move from proposal to supervised applied scheduling change
- applied outcomes resolve through the same backend-owned commitment scheduling seam, not shell-local heuristics
- `Threads` continuity and summary-first surfaces show what was proposed, approved, applied, failed, or dismissed
- docs teach one supervised same-day schedule-application story and its current limits
