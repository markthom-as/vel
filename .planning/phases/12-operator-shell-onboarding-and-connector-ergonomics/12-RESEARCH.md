# Phase 12: Operator shell, onboarding, and connector ergonomics - Research

**Date:** 2026-03-19
**Mode:** Local codebase research

## Question

What do we need to know to plan Phase 12 well without widening product scope beyond shell/onboarding/connector ergonomics?

## Findings

### 1. The shell seams are already narrow and testable

- `clients/web/src/App.tsx`, `Sidebar.tsx`, and `MainPanel.tsx` still use a lightweight local view-state router.
- This makes shell polish feasible without immediately introducing a full routing framework.
- Existing tests already cover sidebar ordering and main-panel dispatch, so Phase 12 can add navigation metadata, default-thread behavior, and shell affordances with focused `vitest` coverage.

### 2. Settings is the current connector and trust hub

- `clients/web/src/components/SettingsPage.tsx` already owns:
  - integration cards,
  - linking issue/redeem/revoke flows,
  - sync/runtime diagnostics,
  - SAFE MODE and trust messaging.
- This is the right place to improve contextual help, template editing/viewing, connector icons, hidden internal paths, and guided setup sequencing.
- The risk is turning Settings into an unstructured mega-surface, so planning should slice contracts and rendering cleanup before broad UI edits.

### 3. Most onboarding primitives already exist in backend routes and docs

- `docs/api/runtime.md` shows stable routes for:
  - `/v1/projects`,
  - `/v1/linking/*`,
  - `/v1/threads*`,
  - `/v1/daily-loop/*`,
  - diagnostics and freshness surfaces.
- `docs/user/setup.md`, `docs/user/integrations/*.md`, and `docs/user/troubleshooting.md` already document endpoint resolution, Apple/local-source paths, and sync setup.
- Phase 12 therefore needs contextual path-discovery and operator guidance more than new backend infrastructure.

### 4. Project and thread ergonomics are the most obvious thin areas

- The roadmap explicitly calls out project detail/edit surfaces and threads defaulting to the latest thread.
- The top-level shell currently treats threads as just another static nav item.
- Planning should treat project detail/edit and thread-entry ergonomics as operator-shell work, not as separate product phases.

### 5. Freshness/Todoist UX is already present but still blunt

- `NowView.tsx` already exposes freshness state, degraded-source warnings, and Todoist backlog cards/actions.
- `SettingsPage.tsx` also surfaces freshness diagnostics and Todoist settings.
- The roadmap's "softer auto-refresh freshness UX" and "richer Todoist rendering" can therefore be shipped as improvements to existing typed data rather than net-new backends.

## Planning Implications

- Start with a contract/docs slice so shell/help/onboarding additions stay typed and reviewable.
- Keep shell/nav work primarily in web code with typed metadata and focused tests.
- Keep connector/onboarding work centered on existing runtime routes and user docs, not on new provider integrations.
- Separate project/settings ergonomics from onboarding/linking ergonomics so write scopes stay reviewable.

## Recommended Phase Breakdown

1. Contracts and contextual help baseline
2. Shell/navigation and freshness ergonomics
3. Project detail/edit plus Settings clarity improvements
4. Onboarding, linking, and Apple/local-source discovery closure

---

*Output intended to guide planning only; not implementation authority by itself.*
