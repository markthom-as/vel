# Phase 84 Summary

## Outcome

Phase 84 landed the shared shell and token foundation in the real web client.

Implemented:

- warmer industrial graphite/copper token baseline in `clients/web/src/index.css`
- locked typography aliases in the live theme layer
- rebuilt top-band shell chrome
- docked bottom action bar with always-visible labeled actions
- shell-side nudge/trust zone backed by canonical `Now` data
- `Now` nudge-lane suppression hook so shell ownership can begin without duplicate top-priority signals

## Main Code Changes

- `clients/web/src/core/Theme/tokens.ts`
  - added shell chrome layout tokens
  - updated brand/theme utilities to use the new CSS token system
- `clients/web/src/index.css`
  - established the dark-first graphite/copper token ladder
  - locked typography families to `Geist / Inter / JetBrains Mono`
  - enabled tabular numeral posture in the base layer
- `clients/web/src/shell/AppShell/AppShell.tsx`
  - shell now owns workspace layout, side zone slot, and bottom action-bar slot
- `clients/web/src/shell/Navbar/*`
  - navbar now behaves as the durable top band instead of a thin header strip
- `clients/web/src/shell/ActionBar/*`
  - new shared docked action bar
- `clients/web/src/shell/NudgeZone/*`
  - new shared shell-side nudge/trust region backed by `loadNow()`
- `clients/web/src/shell/MainPanel/MainPanel.tsx`
  - integrated shell-owned nudge mode for `Now`
  - offset the floating composer above the new dock
- `clients/web/src/core/MessageComposer/MessageComposer.tsx`
  - added shell-friendly floating offset support
  - exposed focus hooks for action-bar affordances

## Transitional Notes

- `Now` still carries its legacy page body and metrics; Phase 84 only moved the shell-owned interruption lane and global chrome.
- Row/card doctrine is intentionally deferred to Phase 85.
