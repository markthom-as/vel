# Phase 23 Context

## Purpose

Phase 23 exists because read-only grounding and assistant-guided resolution can make Vel much more useful, but eventually the assistant also needs to help stage real bounded actions. That should only happen after the conversation, voice, daily-loop, and thread-resolution seams are trustworthy enough that adding mutations does not blur control boundaries.

## Product Direction

The assistant should become more useful without becoming less supervised.

This phase should therefore:

- reuse existing review and approval lanes
- respect SAFE MODE and writeback grants
- stage bounded actions rather than silently applying ambient mutations
- preserve provenance for assistant-originated proposals

## Expected Focus

1. Assistant-staged actions
   - proposals flow through the canonical operator-action model
   - assistant-generated actions are typed, inspectable, and reviewable
   - operators can see what was proposed and why

2. Review and trust integration
   - assistant-originated proposals land in existing review/trust surfaces
   - blocked or unapproved mutations fail closed with explicit guidance
   - no prompt-visible credential widening

3. Thread-to-action continuity
   - thread resolution can hand off into explicit approval or confirmation paths
   - accepted actions preserve history across thread, review, and writeback surfaces

## Non-Goals

- broad autonomous writeback
- silent background actions
- bypassing operator review because a model feels confident

## Inputs

- [docs/product/operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md)
- [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md)
- [docs/api/chat.md](/home/jove/code/vel/docs/api/chat.md)
- the shipped SAFE MODE, writeback, and review seams from Phases 6, 9, 11, 15, and 16

## Exit Condition

Phase 23 is successful when assistant-mediated actions can be proposed and supervised through existing trust/review contracts without weakening operator control or architectural boundaries.
