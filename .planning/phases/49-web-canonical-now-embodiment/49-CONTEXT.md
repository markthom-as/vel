# Phase 49: Web canonical Now embodiment - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 49 rebuilds the web `Now` surface around the canonical compact contract that was locked in Phase 46 and made Rust-owned in Phases 47 and 48.

This phase owns the web embodiment of:

- `NAME's Now` header and filter icon bar
- non-collapsing status row
- backend-first context one-liner
- stacked nudge/action bars
- compact task lane
- bottom-docked capture/voice entry shell
- compact mesh and repair visibility from the shared Rust-owned support seam

This phase must consume the shared Rust-owned transport and governed config already established in Phases 47 and 48. It must not reintroduce shell-local ranking, fallback policy, or a second local planner.

</domain>

<decisions>
## Implementation Decisions

### Surface ownership
- **D-01:** `Now` is a compact execution surface, not a dashboard.
- **D-02:** `Inbox` remains the canonical owner of daily and carry-forward tasks; `Now` may surface only the highest-priority subset.
- **D-03:** `Threads` remains the deep lane for explanation, continuity, and multi-step follow-through.

### Web embodiment rules
- **D-04:** The header should render the Rust-owned title and typed header buckets directly, not invent local labels or local counts.
- **D-05:** Header bucket interaction routes into filtered thread views or explicit backend-owned targets rather than local categorization.
- **D-06:** The top status row does not collapse fields away; it should render backend-provided fallbacks plainly.
- **D-07:** The context line remains backend-first and should not gain shell-authored narrative copy.
- **D-08:** Nudge bars stay compact, stacked, and action-first. Urgent mesh posture may appear there as a trust warning, but deeper repair remains in support surfaces.
- **D-09:** The task lane stays checkbox-first and compact. It should not regress into a broad task dashboard.
- **D-10:** Docked input is the shell for capture/voice entry and transcript preview only; routing and continuity remain backend-owned.

### Non-goals
- **D-11:** Phase 49 does not widen into a full web shell redesign outside the canonical `Now` embodiment.
- **D-12:** Phase 49 does not rebuild Apple surfaces or watch behavior.
- **D-13:** Phase 49 does not move detailed linking, repair, or setup workflows out of `Settings`.
- **D-14:** Phase 49 does not invent additional local thread categories or local ranking heuristics.

### the agent's Discretion
- Exact React component split, as long as the visible behavior matches the canonical contract and remains thin over shared Rust-owned DTOs.
- Exact CSS/layout choices, as long as the page stays compact, execution-first, and avoids reintroducing dashboard sprawl.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `docs/product/now-surface-canonical-contract.md` is the durable authority for the target web `Now` shape.
- `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md` and Phase 48 artifacts already define the shared Rust-owned transport, mesh summary, and governed config seam.
- `clients/web/src/components/NowView.tsx` is the current embodiment target to replace or substantially simplify.
- `clients/web/src/components/ThreadView.tsx` and existing thread filtering surfaces already provide the deeper continuation lane that `Now` should route into.
- `clients/web/src/types.ts` already decodes canonical `Now` transport blocks from the backend.

### Established Patterns
- Web shell code should render backend-owned contracts directly and avoid local policy inference.
- Route handlers and backend services already own the typed `Now` DTOs; the web should stay a consumer.
- Phase-level UI changes should ship with targeted component tests that assert the compact contract rather than brittle copy-heavy snapshots.

### Integration Points
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/NowView.test.tsx`
- `clients/web/src/components/MainPanel.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/types.ts`
- `docs/user/daily-use.md`

</code_context>

<specifics>
## Specific Ideas

- Land the compact shell in narrow passes: structure first, then interaction/routing, then transcript/input continuity, then closeout verification.
- Prefer removing explanatory copy over adding more helper copy.
- Use the shared mesh summary and governed title/count policy that already exist rather than hardcoding another web-specific interpretation.

</specifics>

<deferred>
## Deferred Ideas

- Apple/iPad/Mac/watch embodiment
- broader web navigation changes outside `Now`
- new backend ranking or product policy beyond what the canonical contract already defines

</deferred>

---

*Phase: 49-web-canonical-now-embodiment*
*Context gathered: 2026-03-21*
