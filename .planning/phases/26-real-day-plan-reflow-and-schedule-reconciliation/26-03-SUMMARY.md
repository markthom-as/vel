# 26-03 Summary

## Outcome

Embodied the new backend-owned reflow/recovery posture across the web shell without moving planning logic into the client.

The web surfaces now:

- render typed reflow proposal counts, change rows, and rule-facet chips in `Now`
- keep longer schedule shaping framed as `Threads` continuity instead of expanding `Now` into a planner
- surface recovery posture in `Settings` as part of summary-first trust/freshness guidance

## Main Files

- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/components/NowView.test.tsx`
- `clients/web/src/components/ThreadView.test.tsx`
- `clients/web/src/components/SettingsPage.test.tsx`

## Verification

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/ThreadView.test.tsx src/components/SettingsPage.test.tsx src/types.test.ts`

## Notes

- The client still only renders typed backend output. It does not derive schedule changes or recovery rules locally.
- `Settings` recovery posture is now tolerant of partial `loadNow()` payloads so existing shell states do not regress while reflow data widens gradually.
