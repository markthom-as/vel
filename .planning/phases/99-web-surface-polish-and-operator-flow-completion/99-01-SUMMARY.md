# Phase 99 Summary

status: complete

## What landed

- navbar chips and docs routing were normalized so status chrome and docs access read as one product family
- onboarding/Core setup flows were tightened so the disabled composer, onboarding nudge, and `System -> Preferences -> Accessibility -> Core settings` all point to the same truthful setup controls
- `System` now hides MVP-irrelevant technical control surfaces behind developer mode and keeps unavailable local services visible but collapsed by default
- `Now` was cleaned up to remove duplicate active-task ornament, and overdue work now reads as warning-state work inside visible task lanes instead of only in summary counts
- `Threads` now uses the intended latest-or-empty-state posture, labeled archive action, and tail-less chat bubble treatment with roomier transcript width
- the shared message chrome and nudge presentation were brought closer to the accepted visual direction without reopening Phase 98 truth seams

## Verification evidence

- targeted frontend tests stayed green across `System`, `MainPanel`, `Now`, `Threads`, `NudgeZone`, and `MessageRenderer`
- `npm --prefix clients/web run build` passed after the final polish slices and resulting type repairs

## Exit note

- manual desktop Chrome QA is still the final close authority for the milestone, so Phase 99 closes on implementation and automated proof while Phase 100 remains responsible for honest end-to-end MVP audit and manual proof capture
