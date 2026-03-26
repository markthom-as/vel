# Phase 94 Verification

## Automated checks

- `npm --prefix clients/web test -- --run src/views/now/NowView.test.tsx src/types.test.ts`
- `npm --prefix clients/web test -- --run src/views/system/SystemView.test.tsx`
- `npm --prefix clients/web test -- --run src/shell/NudgeZone/NudgeZone.test.tsx src/views/system/SystemView.test.tsx`
- `npm --prefix clients/web test -- --run src/views/threads/ThreadView.test.tsx src/views/system/SystemView.test.tsx`
- `npm --prefix clients/web run build`

## Verified behaviors

- widened `Now` task DTOs decode and render through the web boundary
- `System` exposes real persisted operator settings fields and toggles through `/api/settings`
- intervention-backed nudge actions call real acknowledge/snooze seams instead of dead local handlers
- thread fallback selection updates shell truthfully and archive uses the real conversation patch seam
- composer explicit-intent fallback re-submits the preserved assistant-entry payload instead of leaving the operator in dead UI state

## Notes

- frontend tests are treated as regression checks only, not acceptance proof
- browser/manual review still governs final UI acceptance, which is the focus of Phase 95
