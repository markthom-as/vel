# Phase 21 Validation

## Goal

Make voice a first-class path into the same grounded assistant/runtime authority across web/desktop and Apple, with local STT preferred where practical and thread continuity preserved.

## Required Truths

- Typed and voiced assistant entry share one backend-owned product seam.
- Web/desktop push-to-talk does not invent separate capture/chat policy.
- Apple voice reduces shell-specific policy where the shared backend seam can own behavior.
- Transcript provenance and fallback behavior remain explicit and inspectable.

## Plan Shape

Phase 21 should be executed in four slices:

1. shared backend voice assistant contract and migration seam
2. web/desktop push-to-talk and availability/fallback polish
3. Apple voice alignment onto the shared seam while preserving bounded offline/cache behavior
4. cross-surface parity docs and verification closure

## Block Conditions

Block if any plan:

- introduces a second parallel voice product lane
- pushes assistant policy back into Swift or TS when the backend can own it
- hides unsupported local STT states behind silent failure
- widens mutation authority for voice beyond existing assistant/writeback rules

## Exit Condition

Phase 21 is complete when the product can honestly say:

- voice and typed entry share one backend-owned assistant path
- web/desktop and Apple differ in shell affordances, not in core semantics
- transcript provenance, continuity, and fallback behavior are explicit
