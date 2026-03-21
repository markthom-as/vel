## 44-03 Summary

Removed the last misleading MVP shell language that still suggested local planner fallback instead of backend-owned inference.

### What changed

- Updated `clients/web/src/components/NowView.tsx` so the compact day-plan card now labels non-durable routine shaping as `inferred routine blocks` instead of `inferred fallback`.
- Tightened the matching explanation text in `NowView` to say the routine blocks are backend-inferred from current context until durable routines are configured.
- Updated `clients/web/src/components/NowView.test.tsx` to verify the new wording.
- Updated `docs/user/daily-use.md` so the user-facing guidance describes inferred routine shaping as backend-owned output, not shell-local planner fallback.

### Verification

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/ThreadView.test.tsx`
- `rg -n "legacy|secondary|detail|Threads|Inbox|Now|Settings|fallback|inferred routine blocks|backend-inferred" clients/web/src/components/NowView.tsx clients/web/src/components/InboxView.tsx clients/web/src/components/ThreadView.tsx clients/apple/Apps/VeliOS/ContentView.swift clients/apple/Apps/VelMac/ContentView.swift docs/user/daily-use.md`

### Outcome

The shipped MVP surfaces now describe this remaining planning seam truthfully: shells render backend-owned inference and state, and do not imply a separate local planner authority.
