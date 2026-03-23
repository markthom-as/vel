# 70-01 Summary

Completed the `v0.5.1` `Threads` and `System` reconnection slice and removed the remaining top-level shell drift that kept the web client pretending `Inbox` and `Settings` were still real surfaces.

## Delivered

- added [SystemView.tsx](/home/jove/code/vel/clients/web/src/views/system/SystemView.tsx)
- added [SystemView.test.tsx](/home/jove/code/vel/clients/web/src/views/system/SystemView.test.tsx)
- added [index.ts](/home/jove/code/vel/clients/web/src/views/system/index.ts)
- updated [App.tsx](/home/jove/code/vel/clients/web/src/App.tsx)
- updated [MainPanel.tsx](/home/jove/code/vel/clients/web/src/shell/MainPanel/MainPanel.tsx)
- updated [MainPanel.test.tsx](/home/jove/code/vel/clients/web/src/shell/MainPanel/MainPanel.test.tsx)
- updated [NavbarNavLinks.tsx](/home/jove/code/vel/clients/web/src/shell/Navbar/NavbarNavLinks.tsx)
- updated [Navbar.test.tsx](/home/jove/code/vel/clients/web/src/shell/Navbar/Navbar.test.tsx)
- updated [operatorSurfaces.ts](/home/jove/code/vel/clients/web/src/data/operatorSurfaces.ts)
- updated [operator.ts](/home/jove/code/vel/clients/web/src/data/operator.ts)
- updated [NowView.tsx](/home/jove/code/vel/clients/web/src/views/now/NowView.tsx)
- updated [NowView.test.tsx](/home/jove/code/vel/clients/web/src/views/now/NowView.test.tsx)
- updated [nowModel.ts](/home/jove/code/vel/clients/web/src/views/now/nowModel.ts)
- updated [ThreadView.tsx](/home/jove/code/vel/clients/web/src/views/threads/ThreadView.tsx)
- updated [ThreadView.test.tsx](/home/jove/code/vel/clients/web/src/views/threads/ThreadView.test.tsx)
- updated [DocumentationPanel.tsx](/home/jove/code/vel/clients/web/src/views/context/DocumentationPanel.tsx)
- updated [README.md](/home/jove/code/vel/clients/web/src/README.md)

## What Changed

- the shell route model now exposes only `Now`, `Threads`, and `System`
- assistant entries that previously routed to `Inbox` now return users to truthful `Now` triage instead of reopening a dead surface
- `Threads` no longer loads or mutates inbox interventions and no longer sorts thread lists with local recency semantics
- `Threads` now stays read-only with explicit guidance that workflow invocation requires a canonical object binding rather than floating client-side execution
- `/system` now exists as one top-level structural surface with the fixed `Domain`, `Capabilities`, and `Configuration` section set
- `/system` uses canonical reads for grounding, integrations, and account connections instead of dragging `SettingsPage` forward whole
- `System > Integrations` renders only named canonical single-step actions that are actually used here: `refresh` and `disconnect`
- legacy `open_settings` nudge actions now map to the canonical `/system` surface rather than a removed settings shell
- the web surface guide and documentation hints now reflect the new three-surface model instead of advertising `Inbox` and `Settings` as active products

## Verification

- `npm test -- src/views/system/SystemView.test.tsx src/views/threads/ThreadView.test.tsx src/views/now/NowView.test.tsx src/shell/MainPanel/MainPanel.test.tsx src/shell/Navbar/Navbar.test.tsx`
- `npm run build`

## Outcome

Phase 70 leaves the web client with a truthful top-level shape: `Now` remains operational, `Threads` is narrowed to canonical conversation truth plus explicit non-invocation guidance, and `System` replaces `Settings` with a single structural surface that stays read-heavy and action-narrow.
