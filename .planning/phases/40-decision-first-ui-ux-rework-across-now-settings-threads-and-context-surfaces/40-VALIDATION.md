# Phase 40 Validation

## Required Proofs

- a discovery artifact exists that distinguishes:
  - working behavior
  - broken/inert/misrouted affordances
  - hierarchy/clarity problems
- `Now` clearly exposes one dominant current action and a bounded next-action stack
- `Settings` reads as configuration, not a second dashboard
- `Threads` reads as open loops needing deeper thought, not generic chat
- the context/right panel defaults to human-readable state/explanation and hides raw internals by default
- surfaced web/mobile operator actions that were visibly broken in discovery are either repaired or explicitly deferred

## Verification Approach

- focused web component tests:
  - `NowView.test.tsx`
  - `ThreadView.test.tsx`
  - `SettingsPage.test.tsx`
  - shell/right-panel tests as needed
- Apple shell and shared Swift package checks:
  - `make check-apple-swift`
- targeted docs `rg` checks for `Now`/`Threads`/`Settings`/context-panel wording
- direct discovery/audit artifact review against shipped behavior

## Acceptance Markers

- the operator can tell what to do next without reading multiple competing cards
- no primary screen defaults to narrating raw system/debug state
- dead states are replaced by action or suggestion
- visibly present actions behave like real actions instead of dead affordances
- discovery findings map cleanly to the implementation slices and closeout evidence
