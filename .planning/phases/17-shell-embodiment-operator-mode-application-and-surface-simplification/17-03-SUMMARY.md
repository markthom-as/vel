# 17-03 Summary

## Outcome

Completed the advanced/support disclosure slice for the web shell so `Projects`, `Settings`, trust/setup, and passive detail surfaces match the approved Phase 14 product boundaries without introducing shell-owned behavior.

## What Changed

- Reframed [ProjectsView.tsx](/home/jove/code/vel/clients/web/src/components/ProjectsView.tsx) as contextual drill-down instead of a co-equal daily-use destination:
  - changed the lead framing to `Project context and durable roots`
  - added explicit `Secondary surface` and `Project-owned context` guidance
  - kept durable project identity and local-root detail visible without making `Projects` read like another inbox
- Updated [ProjectsView.test.tsx](/home/jove/code/vel/clients/web/src/components/ProjectsView.test.tsx) to assert the new secondary/contextual posture.
- Tightened [SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) around progressive disclosure:
  - added summary-first intro copy that points daily-use triage back to `Now`, `Inbox`, and `Threads`
  - added an `Advanced operator setup` card ahead of deeper runtime/internal controls
  - clarified runtime copy so implementation-aware controls read as internal detail rather than first-contact setup
- Updated [SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx) to cover the new summary-first operator/setup framing.
- Reworked passive detail headers in [StatsView.tsx](/home/jove/code/vel/clients/web/src/components/StatsView.tsx) and [SuggestionsView.tsx](/home/jove/code/vel/clients/web/src/components/SuggestionsView.tsx) so they read as drill-down/support views instead of peer daily-use categories.

## Verification

- `npm --prefix clients/web test -- --run src/components/ProjectsView.test.tsx`
- `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx`
- `rg -n "Project context and durable roots|Secondary surface|Advanced operator setup|Passive detail and observability|Reviewable suggestion detail|summary-first|first-contact" clients/web/src/components/ProjectsView.tsx clients/web/src/components/SettingsPage.tsx clients/web/src/components/StatsView.tsx clients/web/src/components/SuggestionsView.tsx`

All passed.

## Notes

- This slice intentionally preserved existing typed backend projections and route structure.
- `Stats` and `Suggestions` remain reachable, but the shell no longer teaches them as first-contact operator destinations.
