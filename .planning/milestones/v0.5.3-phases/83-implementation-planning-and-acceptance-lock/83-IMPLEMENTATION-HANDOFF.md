# Phase 83 — Implementation Handoff

## Purpose

Turn the completed `0.5.3` design-definition milestone into an execution-ready follow-on implementation line.

## Design Packet To Carry Forward

- [78-PARADIGM-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md)
- [79-INTERACTION-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md)
- [80-FOUNDATION-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md)
- [81-COMPONENT-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/81-COMPONENT-SPEC.md)
- [82-NOW-UI-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/82-NOW-UI-SPEC.md)
- [82-THREADS-UI-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/82-THREADS-UI-SPEC.md)
- [82-SYSTEM-UI-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/82-SYSTEM-UI-SPEC.md)
- [82-MILESTONE-LOCK.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md)
- [prototypes/README.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/README.md)

## Follow-On Execution Order

1. Shared primitive rebuild
   - shell primitives first
   - row primitive and card taxonomy second
   - token layer and typography integration third

2. Surface implementation
   - `Now`
   - `Threads`
   - `System`

3. Proof and cleanup
   - browser proofs
   - screenshot/fixture evidence
   - retirement of temporary low-level wrappers

## Retirement Rules

### Aggressive removal

- shell/surface primitives:
  - `Now`
  - `Threads`
  - `System`
  - `TopBand`
  - `ActionBar`
  - `NudgeZone`

These should not receive backward-compatibility scaffolding.

### Gradual migration allowed

- lower-level shared components may be wrapped temporarily
- every wrapper must have:
  - explicit target replacement
  - named removal path
  - bounded scope

## Browser-Proof Expectations

The follow-on implementation line must produce proof artifacts for:

- `Now` normal
- `Now` degraded
- `Threads` normal
- `Threads` focused block
- `System` integrations issue
- `System` control view

Recommended artifact types:

- screenshots
- DOM summaries
- operation notes
- targeted regression tests where behavior is stable enough

## Anti-Drift Rules

The implementation line must explicitly defend against:

- `Now` expanding beyond its bounded surface law
- `Threads` absorbing `System` concerns
- `System` fragmenting into disconnected views
- modules leaking into shell chrome
- rows/cards forking into inconsistent variants

## Non-Negotiables

- `Now` remains bounded and non-inbox-like
- shell chrome remains instrument-like and spatially stable
- state color outranks brand/provider accent
- provider identity stays recognizable but subdued
- critical actions never hide on hover
- color never stands alone as the only state signal
