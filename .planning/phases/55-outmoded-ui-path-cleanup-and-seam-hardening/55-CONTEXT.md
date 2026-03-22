# Phase 55 Context

## Why This Phase Exists

Phase 52 and Phase 54 left the active shell in the correct `Now`/`Inbox`/`Threads`/`Settings` posture, but a few superseded UI paths were still hanging around:

- legacy hidden `Suggestions` and `Stats` shell lanes
- placeholder routing logic in `MainPanel`
- legacy `SettingsPage` tab compatibility kept only to translate old section names

Phase 55 exists to remove those outmoded behavior lanes so the web client presents one honest MVP shell instead of an active shell plus dormant compatibility chrome.

## Inputs

- `.planning/ROADMAP.md`
- `.planning/phases/54-final-ui-cleanup-and-polish-pass/54-CONTEXT.md`
- `.planning/phases/54-final-ui-cleanup-and-polish-pass/54-VERIFICATION.md`
- `clients/web/src/shell/MainPanel/MainPanel.tsx`
- `clients/web/src/data/operatorSurfaces.ts`
- `clients/web/src/views/settings/SettingsPage.tsx`
- `clients/web/src/README.md`

## Bounded Cleanup Targets

- remove hidden shell lanes that are no longer part of the approved MVP surface taxonomy
- remove placeholder routing branches made obsolete by the narrower route set
- remove legacy settings-tab compatibility that no current caller uses
- remove dead view files that only existed to support the removed shell lanes

## Non-Goals

- no new shell surfaces
- no settings feature redesign
- no Rust/backend API contract changes
- no milestone closeout or manual verification work yet
