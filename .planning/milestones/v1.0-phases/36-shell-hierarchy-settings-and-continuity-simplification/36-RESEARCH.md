# Phase 36 Research

## Problem

After Phase 34 and Phase 35, `Now` is substantially more correct, but the surrounding shell still feels noisy and internally framed. The remaining friction is mostly hierarchy and affordance debt:

- sidebar context is still too structurally important for something that should be optional
- `Settings` still reads as a dense mixed-purpose management surface
- `Threads` risks surfacing as clutter instead of continuity
- iconography and button/link choices are inconsistent with actual operator actions

## Inputs

- operator interview guidance captured before this phase
- `Vel.csv` pressure around collapsible/sidebar behavior, icon-driven UI, settings/docs access, freshness noise, and thread continuity
- existing shell policy in `docs/product/operator-mode-policy.md`
- current web surfaces in `clients/web/src/components`

## Constraints

- no shell-owned product logic
- `Now` remains primary and low-noise
- `Threads` stays continuity-first, not chat-first
- `Settings` stays advanced management rather than onboarding prose or runtime dump
- sidebar should become available-but-ignorable

## Architectural direction

- publish hierarchy and affordance rules first
- use the existing backend-owned seams rather than inventing new shell-local models
- keep continuity bounded and contextual
- compress sync/freshness posture into secondary surfaces

## Execution order

1. hierarchy/sidebar/continuity contract
2. `Settings` restructuring
3. `Threads`/action-affordance simplification
4. `Vel.csv` closeout verification
