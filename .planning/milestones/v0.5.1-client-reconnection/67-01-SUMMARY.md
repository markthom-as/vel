# 67-01 Summary

Completed the `v0.5.1` client contract audit and named deprecated-seam kill list.

## Delivered

- published [0.5.1-CLIENT-CONTRACT-AUDIT.md](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/0.5.1-CLIENT-CONTRACT-AUDIT.md)
- refined [0.5.1-DEPRECATED-ROUTE-KILL-LIST.md](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/0.5.1-DEPRECATED-ROUTE-KILL-LIST.md) into an implementation-grade disposition table
- updated [STATE.md](/home/jove/code/vel/.planning/STATE.md)
- updated [ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/ROADMAP.md)

## Audit Findings

- the current web shell still treats `Inbox` and `Settings` as first-class surfaces, which directly violates the frozen `Now` / `Threads` / `System` model
- `clients/web/src/data/operator.ts` is the main mixed-transport seam, combining canonical `v1` reads with quarantined `/api/*` configuration and integration routes
- `clients/web/src/views/settings/SettingsPage.tsx` still contains direct-fetch escapes that bypass the canonical transport boundary
- a large amount of current runtime/admin UI surface area (`components`, `cluster`, `linking`, `runs`, `loops`, `planning-profile`, `projects`) is out of `v0.5.1` scope and should be removed from client usage rather than rehabilitated
- only a narrow subset of integration/config actions survives into `/system`, and that subset is already pre-frozen by doctrine

## Verification

- `rg -n "inbox|settings|projects|threads|operatorSurfaces|MainView" clients/web/src/App.tsx clients/web/src/shell/MainPanel/MainPanel.tsx clients/web/src/data/operatorSurfaces.ts clients/web/src/views/inbox clients/web/src/views/settings clients/web/src/views/projects`
- `rg -n "/api/|/v1/" clients/web/src/data/operator.ts clients/web/src/data/chat.ts clients/web/src/data/context.ts clients/web/src/views/settings/SettingsPage.tsx`
- `rg -n "route\\(\"/api/settings|/api/integrations|/api/components|/api/diagnostics|/api/inbox|/api/conversations|/v1/projects|/v1/runs|/v1/loops|/v1/linking|/v1/cluster|/v1/execution/handoffs|/v1/agent/inspect" crates/veld/src/app.rs crates/veld/src/routes`

## Outcome

Phase 67 now leaves `v0.5.1` with a named seam inventory and a stable rewrite/quarantine/delete boundary. Later phases no longer need to rediscover whether a given client/backend seam is lawful; they only need to execute the frozen dispositions.
