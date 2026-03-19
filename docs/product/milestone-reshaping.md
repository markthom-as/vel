# Milestone Reshaping After Phase 14

This document records the milestone-shaping outcome of Phase 14.

Its purpose is to preserve the discovery decisions from product taxonomy, onboarding/trust journeys, action modeling, and operator-mode policy before implementation work widens again.

## Core Outcome

Phase 14 confirms the roadmap should keep the post-discovery sequence:

1. incremental migration
2. logic-first product closure
3. shell embodiment and simplification

That means:

- Phase 15 remains migration-focused
- Phase 16 remains logic-focused
- Phase 17 remains shell-focused

This is the correct split because the product is now clear enough to avoid mixing backend seam work, product-logic work, and shell embodiment into one ambiguous phase.

## Why This Split Still Matters

Phase 14 discovered several product truths that would be easy to lose if later phases were too broad:

- `Now` should stay minimal and action-focused
- `Inbox` should remain the explicit triage queue
- `Threads` should remain archive/search-first and serve as the escalation path for longer interactions
- `Projects` is secondary in navigation, but may still own project-specific action semantics
- onboarding, trust, check-in, freshness recovery, and reflow all need summary-first routing
- `reflow` is a distinct, heavier action
- the action model needs separate axes for urgency, importance, blocking state, and disruption level

Those decisions are stable enough to guide later phases, but not yet embodied in the product code everywhere.

## Phase 15: What It Should Own

Phase 15 should own the minimum structural migration needed so new product logic lands in the right backend/application seams.

Focus:

- canonical action ownership
- read-model seams
- application-service boundaries
- transport boundary tightening
- preventing shell-local policy or action semantics from spreading

It should not own:

- broad UI simplification
- final navigation changes
- shell-specific presentation polish

## Phase 16: What It Should Own

Phase 16 should implement the newly clarified product logic on the canonical core surfaces.

Focus:

- Rust-owned action generation
- check-in logic
- reflow logic
- summary-first trust/readiness projections
- daily-loop integration with the action model
- connector/setup/recovery decision logic

It should not own:

- major shell restructuring
- visual information architecture cleanup
- frontend-first product semantics

## Phase 17: What It Should Own

Phase 17 should apply the Phase 14 product policy and the Phase 15-16 backend seams across shells.

Focus:

- simplify the default shell
- apply progressive disclosure consistently
- move non-urgent work out of `Now`
- embody inline `check_in` and heavier `reflow` treatment
- keep `Threads` archive/search-first while supporting escalation
- keep `Projects` secondary in navigation without erasing project-specific actions
- align Apple, web, and CLI embodiment around the same product-mode rules

It should not reopen:

- whether `Now`, `Inbox`, and `Threads` are distinct
- whether the action taxonomy is backend-owned
- whether urgency, importance, blocking, and disruption are separate axes

## Future Discovery Left Open

Phase 14 resolves a lot, but not everything.

Open areas that may still generate later follow-on phases:

- richer project-scoped action systems
- deeper scheduling and reflow porting from `codex-workspace`
- desktop/Tauri shell embodiment once local-runtime seams mature
- broader provider/platform expansion after the core operator product is stable

These are valid future lanes, but they should not derail the current 15 → 16 → 17 sequence.

## Acceptance Criteria

1. The roadmap keeps migration, logic, and shell embodiment separate.
2. Later phases are constrained by the Phase 14 taxonomy and action model instead of re-opening them.
3. Future product work can port complex behaviors such as reflow and check-in without redefining shell boundaries first.
