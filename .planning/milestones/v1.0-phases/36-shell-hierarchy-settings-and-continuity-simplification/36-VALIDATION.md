# Phase 36 Validation

## Required proofs

- `Now`, `Threads`, `Settings`, and sidebar behavior all match one shell hierarchy
- sidebar is thin/secondary by default
- `Settings` categories are clearer and less text-heavy
- continuity is contextual and bounded rather than thread-cluttered
- `Vel.csv` simplification pressure is explicitly checked against shipped surfaces

## Verification approach

- focused web component tests:
  - `NowView.test.tsx`
  - `ThreadView.test.tsx`
  - `SettingsPage.test.tsx`
- targeted type/transport tests when DTOs change
- targeted docs `rg` checks for hierarchy/sidebar/settings wording
- direct code review of action affordances for button-vs-link/icon usage

## Acceptance markers

- `Now` still feels compact after the shell cleanup
- `Threads` is not promoted into a second inbox
- `Settings` can be navigated by category instead of reading top-to-bottom prose
- sidebar is clearly optional
