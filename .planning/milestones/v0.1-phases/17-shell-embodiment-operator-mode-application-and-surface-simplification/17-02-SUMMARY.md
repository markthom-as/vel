# 17-02 Summary

## Outcome

Completed the default-surface embodiment slice for the web shell so `Now`, `Inbox`, and `Threads` better match the approved Phase 14 product boundaries without adding shell-owned policy.

## What Changed

- Tightened [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) into a more minimal urgent-first surface:
  - added a compact top context strip
  - replaced the old full `Action stack` treatment with summary-first `Immediate pressure`
  - surfaced typed backend-owned `reflow`, `reflow_status`, `check_in`, and `trust_readiness`
  - demoted non-urgent queue pressure into a compact `Waiting elsewhere` summary instead of rendering `Now` like a second inbox
- Sharpened [InboxView.tsx](/home/jove/code/vel/clients/web/src/components/InboxView.tsx) copy so it reads as the explicit triage queue rather than a general work surface.
- Added a continuity/history header in [ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) so `Threads` reads as archive/search-first and escalation-friendly instead of another live queue.
- Updated [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx), [InboxView.test.tsx](/home/jove/code/vel/clients/web/src/components/InboxView.test.tsx), and [ThreadView.test.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx) to match the new shell posture and the newer typed `Now` contract fields from Phases 15-16.

## Verification

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`
- `npm --prefix clients/web test -- --run src/components/InboxView.test.tsx src/components/ThreadView.test.tsx`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/ThreadView.test.tsx`

All passed.

## Notes

- This slice intentionally did not introduce new web-owned routing heuristics or backend semantics.
- `Now` still does not perform live `reflow` transitions directly from the shell because that route wiring remains outside this embodiment slice; the view now reflects the typed backend state cleanly instead of inventing its own behavior.
