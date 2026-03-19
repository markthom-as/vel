# Phase 12: Operator shell, onboarding, and connector ergonomics - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** Roadmap Phase 12 scope + CSV triage notes + current web/docs/runtime surface analysis

<domain>
## Phase Boundary

Phase 12 tightens Vel's daily operator experience around the surfaces that already exist: web shell navigation, project detail affordances, settings/integration clarity, contextual docs/help, and setup/discovery flows for linking and local-source paths.

This phase is about adoption, navigation, and trust. It should make the daily loop, projects, threads, and connector setup easier to find, understand, and recover when stale or partially configured.

This phase is not a new provider expansion phase, not a hosted-auth phase, and not a broad redesign of product logic. It should reuse the backend-owned daily-loop, linking, project, and inspect boundaries already shipped in earlier phases.

</domain>

<decisions>
## Implementation Decisions

### Product intent
- [locked] Phase 12 exists to make the current daily-loop and integration product direction easier to adopt, navigate, and trust.
- [locked] `Now + Inbox` remain the primary operator shell; Phase 12 improves shell ergonomics without replacing those surfaces.
- [locked] The phase should prefer repeated daily usability over broad new functionality.

### Included scope from roadmap triage
- [locked] Included shell work covers app routes, top-nav/shell polish, icon-driven and collapsible navigation, softer auto-refresh freshness UX, and threads defaulting to the latest thread.
- [locked] Included project/settings work covers project detail/edit surfaces, template viewing/editing in Settings, contextual docs/help routing, richer Todoist rendering, connected-service icons, and hidden internal integration paths.
- [locked] Included onboarding work covers Apple/local-source path discovery and validation plus guided onboarding/linking/autodiscovery ergonomics.
- [locked] Included calendar/task ergonomics cover upcoming-event ordering and pagination where the current shell is too thin or noisy.

### Architecture constraints
- [auto] Backend Rust layers remain the source of truth for linking state, diagnostics, project records, integration status, freshness, and daily-loop policy.
- [auto] Web and docs surfaces should expose backend-owned state more clearly rather than re-deriving product policy client-side.
- [auto] If this phase adds new shell/config/docs contracts, ship typed DTOs, docs, examples, and tests together before widening UI behavior.
- [auto] Settings should distinguish operator-usable connector paths from internal/diagnostic-only paths.

### UX constraints
- [auto] Navigation should reduce operator effort, not add more destinations than the current shell can explain.
- [auto] Freshness and stale-state UX should become calmer and more actionable, not more verbose or alarmist.
- [auto] Docs/help routing should be contextual: from the shell or settings, operators should land on the correct user-facing guide for the surface they are using.
- [auto] Onboarding should stay local-first and explicit about what is bootstrap, cached, offline-only, or requires daemon reachability.

### Claude's Discretion
- Exact contract/type names for shell navigation metadata, docs/help link payloads, setup guides, and project detail/editor affordances
- Whether shell ergonomics land through the current sidebar/app-shell structure or through a small routing/navigation refactor
- Exact slicing between shell polish, project/settings UX, and onboarding/linking flows so long as the dependency order stays reviewable

</decisions>

<specifics>
## Specific Ideas

- The current web shell already has canonical surfaces (`Now`, `Inbox`, `Projects`, `Threads`, `Suggestions`, `Stats`, `Settings`) but navigation is still plain-text and thread-centric behavior is only partially ergonomic.
- `SettingsPage.tsx` already owns a large amount of integration, linking, diagnostics, and trust rendering; this phase should clarify and segment it rather than creating duplicate setup surfaces.
- `NowView.tsx` already surfaces freshness and Todoist backlog state, so Phase 12 should refine the UX and path-discovery around that existing data instead of inventing a new dashboard.
- Runtime and user docs already describe linking, Apple endpoint resolution, local-source discovery, and Todoist boundaries. The missing layer is contextual routing from product surfaces to the right doc/help entrypoint.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and phase authority
- `.planning/ROADMAP.md` — Phase 12 goal, scope note, included triage items, and dependency on Phase 11
- `.planning/PROJECT.md` — accepted decision that interface/shell fixes should be prioritized ahead of broad provider expansion
- `.planning/STATE.md` — roadmap evolution and current sequence through Phase 12
- `docs/MASTER_PLAN.md` — canonical status tracker

### Existing operator shell and settings surfaces
- `clients/web/src/App.tsx` — current main-view state and thread/settings navigation entrypoints
- `clients/web/src/components/Sidebar.tsx` — current shell navigation structure
- `clients/web/src/components/MainPanel.tsx` — current top-level surface routing
- `clients/web/src/components/NowView.tsx` — current daily-loop adjacent shell, freshness, and Todoist rendering
- `clients/web/src/components/ProjectsView.tsx` — current project list/detail entry seam
- `clients/web/src/components/ThreadView.tsx` — current thread surface that should default to the latest useful conversation
- `clients/web/src/components/SettingsPage.tsx` — current integration, linking, diagnostics, and trust surface
- `clients/web/src/data/operator.ts` — existing linking/integration/operator loader and mutation seam

### User and runtime docs the phase should tighten
- `docs/user/daily-use.md` — repeated operator loop and trust guidance
- `docs/user/setup.md` — current setup, endpoint resolution, and local-source discovery guide
- `docs/user/integrations/README.md` — integration doc entrypoint
- `docs/user/integrations/local-sources.md` — local-source setup aids and path guidance
- `docs/user/integrations/apple-macos.md` — Apple/macOS local-source bridge guide
- `docs/user/integrations/todoist.md` — current Todoist operator guide
- `docs/user/troubleshooting.md` — recovery and setup failure guidance
- `docs/api/runtime.md` — mounted `/v1` routes for linking, projects, threads, diagnostics, and daily-loop authority

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- The web shell already has test coverage around sidebar order, main-panel routing, `Now` freshness, and Settings linking/integration behavior.
- Runtime routes already exist for projects, threads, linking, diagnostics, and daily-loop authority; Phase 12 can stay thin-shell if it extends those typed seams.
- The docs tree already has setup/integration/troubleshooting guides that can be reused through contextual links instead of writing duplicate help copy in the UI.

### Missing or Thin Areas
- There is no explicit Phase-12-level contract for shell navigation metadata, contextual docs/help affordances, or onboarding guidance surfaces.
- Project detail/edit ergonomics are still thinner than the rest of the product direction.
- Connector setup affordances exist, but Apple/local-source path discovery and validation are still split across docs and Settings without one clean operator path.
- The current shell/navigation experience is functional but not yet optimized for repeated daily use and fast recovery when trust/freshness degrades.

</code_context>

<deferred>
## Deferred Ideas

- Broad provider-family expansion such as full Google Workspace, Dropbox-style pickers, or SaaS auth scaffolding
- LLM-provider routing, local-vs-remote model policy, or budget-discovery product work
- Client-to-client file transfer, reading/media systems, and other non-daily-loop product surfaces
- Any client-owned policy fork that would move linking, freshness, or daily-loop logic out of backend authority

</deferred>

---

*Phase: 12-operator-shell-onboarding-and-connector-ergonomics*
*Context gathered: 2026-03-19*
