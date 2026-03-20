# Phase 31 Research

## Domain

Extend the already-shipped planning-profile model across real product entry surfaces so routine and planning-constraint management no longer depends on the web shell alone.

## Locked Inputs

- Phase 29 made durable routine blocks and bounded planning constraints real backend-owned planning inputs.
- Phase 30 shipped the canonical planning-profile management seam plus web `Settings` management over `GET /v1/planning-profile` and `PATCH /v1/planning-profile`.
- Assistant, voice, Apple, and CLI are already meaningful operator entry paths for daily use.
- Product policy still requires one backend-owned planning substrate, explicit confirmation for meaningful edits, and fail-closed trust behavior.

## Problem

Vel now has a good canonical planning-profile seam, but access to it is uneven:

- web `Settings` can inspect and mutate the profile
- CLI does not yet expose the same profile as a first-class inspect/manage surface
- Apple does not yet have a bounded way to inspect or participate in routine/profile management
- assistant and voice entry can help with daily loop, reflow, and staged actions, but not yet with bounded routine/profile edits

That creates a shell split around a core same-day planning substrate that should stay shared.

## Required Truths

1. One planning profile
   - Web, CLI, Apple, and assistant entry must all read or mutate the same backend-owned `RoutinePlanningProfile`.
   - This phase must not create shell-local routine records or parallel planner state.

2. Assistant edits stay supervised
   - Assistant or voice-driven routine/profile edits should route through the typed mutation seam.
   - Ambiguous or consequential edits should preserve confirmation, provenance, and thread continuity.

3. Shells stay thin
   - CLI and Apple can gain useful inspect/manage affordances, but planning semantics stay in Rust.
   - Voice and assistant entry should not invent their own routine-edit grammar outside the typed mutation model.

4. Bounded same-day scope remains
   - This phase should widen access to the current planning profile.
   - It should not widen into multi-day planning, broad calendar mutation, or a second routine-management product.

## Recommended Execution Shape

Phase 31 should be executed in four slices:

1. publish the parity/assistant-edit contract and widen shared transport seams where needed
2. ship CLI and Apple planning-profile inspection/parity over the canonical backend seam
3. route assistant and voice-driven routine/profile edits through the typed mutation model with confirmation/provenance
4. close with docs and verification for the cross-surface planning-profile model

## Code Context

- `crates/vel-core/src/planning.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/chat/`
- `crates/veld/src/services/apple_voice.rs`
- `crates/veld/src/routes/planning_profile.rs`
- `crates/vel-cli/src/commands/`
- `clients/apple/`
- `clients/web/src/components/SettingsPage.tsx`
- `docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md`
- `docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md`
- `docs/api/runtime.md`
- `docs/user/daily-use.md`

## Risks

- drifting into shell-specific routine state instead of reusing the canonical profile
- allowing assistant/voice edits to bypass explicit confirmation or typed mutation provenance
- widening the planning-profile seam into a second planner or an autonomous calendar editor
- making Apple or CLI parity so bespoke that cross-surface behavior drifts again

## Success Condition

Phase 31 is complete when the product can honestly say:

- the same planning profile is inspectable across web, CLI, and Apple pathways
- assistant and voice can help stage bounded routine/profile edits through the canonical mutation seam
- confirmation, provenance, and continuity stay explicit for planning-profile edits
- cross-surface docs teach one shared planning-profile story and its current limits
