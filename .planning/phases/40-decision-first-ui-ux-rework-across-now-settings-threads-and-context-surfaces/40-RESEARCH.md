# Phase 40 Research

## Problem

Phase 40 is no longer a UI-first cleanup phase. It is the contract-and-architecture lock for milestone `v0.2`.

The prior cycle drifted because scope, product behavior, and authority boundaries were not specified tightly enough before implementation widened. This phase must therefore produce a durable MVP specification for the fixed loop:

`overview -> commitments -> reflow -> threads -> review`

The research task is to turn the current milestone decisions into an implementation-ready boundary:

- what the true MVP includes and excludes
- which Rust-owned contracts must become canonical
- where shell behavior stops and backend authority begins
- how same-day reflow and threads interact without widening into a generic assistant product
- what documents, examples, and validation artifacts are required so later phases stop guessing

## Inputs

- milestone authority in `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, and `.planning/STATE.md`
- locked Phase 40 decisions in [40-CONTEXT.md](/home/jove/code/vel/.planning/phases/40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces/40-CONTEXT.md)
- durable repo rules in `AGENTS.md`
- implementation truth in `docs/MASTER_PLAN.md`
- current Rust seams in:
  - `crates/veld/src/services/now.rs`
  - `crates/veld/src/services/daily_loop.rs`
  - `crates/veld/src/services/reflow.rs`
  - `crates/veld/src/routes/threads.rs`
  - `crates/vel-api-types/src/lib.rs`

## Constraints

- do not widen beyond the fixed MVP loop
- do not add local-calendar milestone work to `v0.2`
- keep MVP logic Rust-owned and portable across web and Apple
- treat threads as bounded continuation, not generic chat
- write durable authority into `docs/`, not only `.planning/`
- specify degraded and failure states, not just happy-path UI

## Architectural Direction

- contract-first before implementation-first
- one canonical overview/read-model seam that can express:
  - dominant action
  - compact timeline
  - one visible top nudge
  - optional additional context behind `Why + state` affordances
  - 1-3 suggestions when no dominant action exists
- one bounded commitment flow with explicit operator outcomes
- one Rust-owned reflow model with proposal state, provenance, and degraded behavior
- one thread-escalation rule for genuinely multi-step work
- one review/closure seam that explains what changed and what remains unresolved

## Key Research Conclusions

1. The largest risk is not missing implementation; it is boundary ambiguity.
2. Phase 40 should publish canonical models, transitions, and failure states before any shell refresh or service migration work.
3. Existing `now`, `daily_loop`, `reflow`, and thread seams should be reconciled into the MVP loop, not replaced wholesale.
4. Durable docs must separate:
   - MVP product behavior
   - Rust/domain contracts
   - transport and shell consumption rules
5. Later phases need examples and acceptance checklists, not just prose, or drift will reappear.

## Recommended Execution Order

1. define the strict MVP boundary and acceptance rules
2. reconcile and publish canonical Rust-owned contracts and read models
3. refine architecture and shell-boundary documentation around those contracts
4. publish examples/templates/owner guidance and validation criteria before implementation widens
