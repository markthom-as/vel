# Phase 40 Research

## Problem

The daily-use repair arc improved correctness and continuity, but the primary operator shell still feels harder to use than it should because hierarchy, actionability, and reliability are drifting together:

- too many surfaces still narrate state instead of driving action
- visual hierarchy is too flat for the operator to know what matters first
- `Now`, `Threads`, `Settings`, and context panels still overlap in job and meaning
- internal/runtime/debug concepts still leak into default views
- some web and mobile affordances appear present but are broken, inert, or misrouted in practice

This phase must therefore start with discovery, not implementation assumptions.

## Inputs

- operator-supplied Phase 40 UI/UX spec in [40-CONTEXT.md](/home/jove/code/vel/.planning/phases/40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces/40-CONTEXT.md)
- repaired daily-use shell truth from Phases 34-39
- current web surfaces in `clients/web/src/components`
- Apple current-use shell in `clients/apple/Apps/VeliOS/ContentView.swift` and shared Apple transport/state layers
- existing operator shell policy in `docs/product/operator-mode-policy.md`

## Constraints

- preserve backend-owned product logic; this is shell/interaction work, not a new planner
- one screen = one job:
  - `Now` → act
  - `Threads` → think
  - `Settings` → configure
- `Now` must stay execution-first and current-day oriented
- broken interaction paths are in scope as product failures, not polish backlog
- avoid widening this phase into a broad full visual redesign detached from behavior

## Architectural Direction

- discovery first: write a cross-surface interaction audit before changing structure
- separate:
  - true functionality failures
  - hierarchy/clarity failures
  - debug leakage
- make the top-level shell decision-first:
  - one dominant action
  - few next actions
  - background/system info demoted
- keep editing and decomposition inline where possible
- move debug/raw model state behind explicit disclosure rather than deleting it from the product entirely

## Key Planning Questions

1. What replaces the primary action strip when there is no active commitment?
2. Are primary CTA verbs universal across item types or commitment/task-specific?
3. Is routine drag-edit required now, or should the phase stop at timeline visualization plus inline edit?
4. Should Apple parity be immediate for the new `Threads` mental model, or follow after web proves the shell?
5. Are right-panel `State / Why / Debug` tabs shell-global or surface-specific?
6. How far should inline decomposition go in this phase?

## Recommended Execution Order

1. cross-surface discovery and broken-interaction audit
2. `Now` decision-first restructure and reliability repair
3. `Settings`, `Threads`, and context-panel model cleanup
4. cross-surface verification, docs truth, and explicit remaining limits
