# Vel.csv Regression Matrix

## Purpose

Normalize `~/Downloads/Vel.csv` into a regression and acceptance matrix for the Phase 34-38 arc.

This artifact is a guardrail, not a second product authority. Final truth still comes from:

- operator decisions captured in the Phase 34-39 context docs
- shipped docs in `docs/user/`, `docs/api/runtime.md`, and `docs/product/operator-mode-policy.md`
- shipped web and Apple surfaces

## Status Vocabulary

- `validated`: the current Phase 34-38 arc materially addressed the item
- `open`: still relevant to the repaired daily-use arc and should be considered in Phase 39 follow-up work
- `deferred`: valid backlog, but outside the Phase 34-38 daily-use closeout scope
- `superseded`: the CSV wording conflicts with later operator decisions or shipped product rules and should not be used as authority

## Authority Anchors For This Matrix

- `Now` is execution-first, compact, and current-day focused
- current day is sleep-relative rather than midnight-bound
- `Threads` is continuity-first, not a default inbox
- `Settings` is advanced management, not a prose dump
- sidebar should be a thin icon rail, visible but ignorable
- Apple local-first voice continuity is bounded: cached `Now`, queued voice capture, local quick actions, local draft/recovery continuity

## Matrix

| CSV Row | Normalized Item | Bucket | Status | Evidence / Rationale |
|---|---|---|---|---|
| 03 | Node autodiscovery / LAN / Tailscale pairing still fails | Linking / onboarding | deferred | Still backlog outside the Phase 34-38 daily-use shell/Apple arc. Earlier roadmap notes already carried linking/autodiscovery separately. |
| 04 | Repeated suggested actions should dedupe or show `Nx` | `Now` / action clarity | validated | Phase 39-02 dedupes repeated `Now` action suggestions before rendering the compact context panel, keeping the low-noise hierarchy intact without turning the section into another inbox. |
| 05 | Connected services should show product icons | Settings / integrations | validated | Phase 39-02 added compact product marks across Google Calendar, Todoist, and local integration cards so Settings reads faster without adding more prose. |
| 06 | Context sidebar should be collapsable | Shell hierarchy | validated | Phase 36 replaced the old sidebar posture with a thin icon rail and explicitly demoted it from a required information column. |
| 07 | “Some inputs are degraded” should be less obtrusive and refresh should be calmer | Freshness / trust UX | partial-open | Phase 34 reduced noisy status emphasis, but the auto-refresh/timed-band refinement still needs final closeout proof. |
| 08 | Projects need richer detail, tags, service icons, and project-specific drilldown | Projects surface | deferred | Real backlog, but outside the compact daily-use repair arc. |
| 09 | Google connector should expose more Google products | Integrations breadth | deferred | Valid future integration scope, not a Phase 34-38 regression item. |
| 10 | LLM provider connectors / budgeting / mapping | Integrations / model config | deferred | Future platform scope, not part of the repaired daily-use arc. |
| 11 | Template viewing/editing in Settings UI | Settings depth | open | Mentioned historically, but no strong shipped UI evidence found in current web/Apple settings surfaces. |
| 12 | The app should have routes | Web shell / navigation | validated | The current web shell is route-backed and the Phase 36 hierarchy work assumes real surfaced navigation. |
| 13 | Docs/help should be contextual and have their own route, not live at the bottom of Settings | Docs / support routing | partial-open | Help routing improved, but runtime/docs still explicitly say there is no dedicated contextual-help payload yet. |
| 14 | Threads view should auto-open the latest thread | Threads continuity | validated | `ThreadView` now has focused regression coverage proving it falls back to the latest updated conversation when no thread is explicitly selected, which matches the continuity-first default behavior expected in the repaired shell. |
| 15 | More UI should be icon-driven; sidebar collapses to icon-only | Shell hierarchy / affordances | validated | Phase 36 explicitly shipped the thin icon rail and more icon-driven shell framing. |
| 16 | Subtle top navbar with time/status/help/clients/voice affordance | Global shell chrome | deferred | Not part of the Phase 34-38 repaired arc; still backlog if desired. |
| 17 | Upcoming events should show soonest first, with pagination | Schedule / `Now` | partial-open | Next-event truth and future-facing ordering were repaired; pagination/forward browsing still needs explicit closeout proof. |
| 18 | Fresh / aging / stale should show clearer timed bands | Freshness / trust UX | partial-open | Operator explicitly asked for timed bands, but current docs/evidence do not prove the full banded UX is closed everywhere. |
| 19 | Todoist items should show all Todoist data | Tasks / Todoist | superseded | Later operator guidance rejected raw metadata dumps. Current rule is meaningful structure first, not “show all Todoist data.” |
| 20 | Clients should transfer files to each other outside sync | Cross-client transfer | deferred | Outside the Phase 34-38 daily-use arc. |
| 21 | More integration pickers for messages/health/activity/notes/transcripts | Integrations breadth | deferred | Future integration scope, not part of the current closeout arc. |
| 22 | Reading/media list with tracking | Product expansion | deferred | New product surface, not part of the Phase 34-38 regression sweep. |
| 23 | Rich onboarding flow | Onboarding | deferred | Still valid backlog, but separate from the repaired daily-use loop closeout. |
| 24 | Unique onboarded user + propagation + SaaS auth scaffolding | Identity / auth | deferred | Outside current local-first daily-use closeout scope. |
| 25 | Agent lacks awareness of personal data / useful Vel tool access | Assistant quality | open | Still a valid pressure item for end-to-end daily usefulness; not clearly closed by the shell/Apple arc alone. |
| 26 | Internal Vel paths should not be shown as integration options | Settings / integrations | validated | Phase 39-02 moved Vel-managed/internal paths behind a secondary disclosure so the default integration flow stays operator-path-first and lower clutter. |
| 27 | Apple integration paths should auto-discover defaults and validate them | Apple local-source setup | open | Still relevant to Apple setup polish; not clearly closed by the Phase 37-38 embedded/local-first work. |

## Bucket Summary

### Validated

- 06 sidebar collapsible / non-obtrusive shell role
- 04 duplicate suggested actions / dedupe
- 05 integration product icons
- 12 route-backed app shell
- 14 latest-thread continuity default
- 15 icon-driven shell and icon-rail sidebar
- 26 hidden internal Vel paths in integrations

### Open For Phase 39 Follow-up

- 07 degraded/freshness obtrusiveness
- 11 template viewing/editing in Settings
- 13 contextual docs/help routing
- 17 schedule pagination / forward-browse proof
- 18 freshness time bands
- 25 assistant data/tool awareness
- 27 Apple path discovery/validation UX

### Deferred Outside Phase 34-38 Closeout

- 03 linking/autodiscovery
- 08 richer projects surface
- 09 broader Google connector scope
- 10 LLM provider connectors/budgeting
- 16 top navbar/global shell chrome
- 20 file transfer between clients
- 21 more integration pickers
- 22 reading/media list
- 23 onboarding flow
- 24 user propagation / SaaS auth scaffolding

### Superseded By Later Product Rule

- 19 “show all Todoist data”

This conflicts with the later locked product rule that `Now` and task surfaces should show meaningful, structured, action-supporting context rather than dumping raw provider metadata.

## Immediate Implication For 39-02

The highest-signal closeout candidates from `Vel.csv` are:

1. freshness/degraded-state tone and timed-band clarity
2. docs/help routing
3. latest-thread continuity behavior
4. Apple path-discovery cleanup
5. remaining assistant data/tool-awareness pressure

## Traceability Notes

- CSV source: `~/Downloads/Vel.csv`
- This matrix intentionally maps each CSV item to the repaired Phase 34-38 product rules rather than treating every CSV row as equally authoritative.
