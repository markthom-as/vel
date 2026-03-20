# Phase 21 Context

## Purpose

Phase 21 exists because voice is already present in multiple shells, but it is not yet one product path. Web/browser microphone input now feeds the grounded assistant through local speech-to-text, while Apple voice still uses a more specialized backend route with shell-specific intent handling. The next step is to make voice a first-class path into the same assistant/runtime seam rather than letting each surface grow its own voice product.

## Product Direction

The operator explicitly wants:

- voice to work with the assistant
- local speech-to-text on desktop if that is easier
- cross-surface continuity instead of separate voice-only behavior

This phase should preserve the existing product boundaries:

- Rust/backend owns the assistant semantics, thread continuity, and policy
- shells own microphone permissions, push-to-talk UX, waveform/presentation, and local STT where appropriate
- remote LLM use remains optional and replaceable

## Expected Focus

1. Shared voice-facing assistant seam
   - the same grounded assistant behavior for typed and voiced input
   - shared transcript provenance and fallback rules
   - no parallel voice-only planning logic

2. Desktop/browser push-to-talk
   - fast local STT path where available
   - same thread continuity and operator entry semantics as typed input
   - clear failure states when speech support is unavailable

3. Apple voice alignment
   - reduce Apple-specific intent logic where the shared assistant seam can own behavior
   - preserve bounded offline/cache behavior on Apple without creating local authority
   - keep typed daily-loop and action contracts as the source of truth

## Non-Goals

- broad new audio platform work
- always-listening ambient runtime behavior
- speculative multimodal expansion beyond transcript-driven voice entry
- giving voice surfaces broader mutation authority than typed assistant surfaces

## Inputs

- [docs/api/chat.md](/home/jove/code/vel/docs/api/chat.md)
- [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md)
- [docs/product/now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md)
- the current backend chat assistant and Apple voice service seams

## Exit Condition

Phase 21 is successful when voice is no longer a special side path: web/desktop and Apple can both enter the same grounded assistant workflow, with local STT preferred where practical and thread continuity preserved.
