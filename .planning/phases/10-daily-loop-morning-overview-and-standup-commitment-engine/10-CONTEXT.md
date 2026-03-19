# Phase 10: Daily-loop morning overview and standup commitment engine - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** User-provided daily-loop MVP spec + current roadmap/codebase analysis

<domain>
## Phase Boundary

Phase 10 turns Vel's existing `Now`, commitments, calendar, Todoist, and Apple/backend voice seams into one strict daily operator loop:

1. Morning Overview — passive, voice-first situational awareness
2. Standup — active compression into bounded daily commitments

This phase is about a real repeated-use product loop, not a dashboard refresh. It must be usable every day, finish in under three minutes, and force prioritization rather than passive task consumption.

This phase does not include broad shell polish, onboarding, provider sprawl, deep analytics, workflow-engine generalization, or new autonomous-agent execution flows. Those belong elsewhere.

</domain>

<decisions>
## Implementation Decisions

### Product contract from the user spec
- [locked] Total interaction time for the full loop must be `<= 3 minutes`.
- [locked] The full loop is Morning Overview -> Standup -> Execution -> Reflection, but this phase covers only Morning Overview and Standup.
- [locked] Morning Overview is passive and does not create commitments.
- [locked] Standup is the commitment engine and must end with explicit bounded daily commitments.
- [locked] Daily commitments are capped at `1-3`.
- [locked] Each sub-flow may ask at most `3` questions.
- [locked] All flows must be interruptible, skippable, and resumable.
- [locked] Voice is preferred, but text fallback is acceptable.
- [locked] Simple heuristics and low latency are preferred over complex intelligence.

### Morning Overview
- [locked] Trigger should support manual start now (`Start morning`) and be compatible with future automatic start.
- [locked] Required inputs are calendar for the next 12 hours and Todoist for today + overdue.
- [locked] Output structure is:
  - short snapshot (`<= 4 sentences`, `<= ~20 seconds`)
  - at most `2` friction callouts
  - `1-3` intent-gathering questions
- [locked] Morning Overview can capture intent signals such as must-do hints, focus intent, or meeting doubt, but must not create commitments yet.

### Standup
- [locked] Standup may follow Morning Overview immediately or start manually.
- [locked] Standup steps are:
  - calendar reconciliation
  - task compression into must/should/stretch
  - commitment extraction
  - constraint shaping
  - opportunistic metadata enrichment
- [locked] Standup must force reduction if the user proposes more than `3` defining commitments.
- [locked] Final session output must include commitments, deferred tasks, confirmed calendar state, and proposed focus blocks.

### UX and interaction
- [locked] Vel tone should be calm, slightly firm, and brief.
- [locked] Avoid coaching, therapy, or excessive encouragement.
- [locked] Silence or `skip` should advance safely.
- [locked] If the user disengages, partial state must persist and resume later.
- [locked] If no commitments are defined, prompt once more and then exit.

### Architecture constraints
- [auto] Reuse existing `Now`, commitments, and Apple/backend voice surfaces rather than introducing a second planning subsystem.
- [auto] Keep policy and durable state in Rust backend layers. Web, CLI, and Apple remain thin shells.
- [auto] Preserve explainability: friction callouts, focus suggestions, and deferrals must be derivable from persisted inputs and explicit rules.
- [auto] Add typed session state rather than deepening untyped blobs.

### Claude's Discretion
- Exact contract/type names for morning signals, task buckets, daily sessions, resume state, and compression heuristics
- Whether the first shipped surface is CLI-first, web-first, or dual-surface, so long as voice/text fallback and resumability are preserved
- Exact scoring/ranking heuristics for friction callouts, deferrals, and focus-block suggestions

</decisions>

<specifics>
## Specific Ideas

- Existing `Now` already exposes freshness, schedule, ranked action items, pending writebacks, conflicts, and people/project review pressure. Planning should build on that instead of replacing it.
- Existing commitment flows and review surfaces already make it possible to persist bounded commitments; this phase should clarify the daily-session structure around them.
- Existing Apple voice seams from Phase 7 provide a strong boundary for voice-first morning and standup interactions, but Phase 10 should not become Apple-only.
- One explicit quality requirement from CSV triage belongs here: repeated suggested actions should dedupe so the daily loop stays trustworthy instead of noisy.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and product authority
- `.planning/ROADMAP.md` — Phase 10 goal, requirements IDs, execution order, and accepted scope
- `.planning/PROJECT.md` — accepted product-direction decisions for `Now`, projects, action/intervention balance, and daily-loop value
- `.planning/STATE.md` — current milestone decisions, especially the Phase 10 priority note
- `docs/MASTER_PLAN.md` — canonical shipped-status tracker

### Existing daily/operator surfaces
- `docs/user/daily-use.md` — current repeated operator loop that Phase 10 should tighten, not replace with a dashboard-first product
- `docs/user/surfaces.md` — current truth for `Now`, commitments, settings, and inspectable operator surfaces
- `docs/api/runtime.md` — current API surfaces for `Now`, context, explainability, sync, and review-backed operator flows

### Existing implementation seams
- `clients/web/src/components/NowView.tsx` — current `Now` presentation and freshness/schedule/action stack shell
- `clients/web/src/data/context.ts` — current frontend load path for `/v1/now` and current context
- `crates/veld/src/routes/now.rs` and `crates/veld/src/services/now.rs` — backend-owned `Now` route/service seam
- `crates/veld/src/services/operator_queue.rs` — ranked operator action/intervention projection seam
- `crates/vel-cli/src/commands/review.rs` and existing commitment commands — current CLI review/commitment behavior that the standup flow may extend
- `clients/apple/README.md` plus Phase 07 summaries — existing backend-owned Apple voice/schedule seams

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Now` data already includes ranked action items, review snapshot, projects, people, conflicts, and freshness state.
- Commitments already exist as durable first-class records with CLI and API surfaces.
- Apple voice and backend-owned schedule/behavior-summary paths already exist from Phase 7.
- Review and synthesis surfaces already expose enough context to support daily compression without inventing a parallel store.

### Missing or Thin Areas
- There is no current typed daily-session state for morning overview + standup.
- There is no explicit morning-overview voice/text flow that captures intent signals without creating commitments.
- There is no first-class standup engine that compresses tasks/calendar into 1-3 daily commitments, deferrals, and focus windows.
- Repeated suggestion/action deduping is not yet called out as a guarded product rule.

</code_context>

<deferred>
## Deferred Ideas

- Reflection, deeper end-of-day loops, or analytics dashboards
- Broad shell/navigation polish and onboarding work — Phase 12
- Agent grounding over real Vel state — Phase 11
- New provider families, reading/media tracking, or broader external-product expansion

</deferred>

---

*Phase: 10-daily-loop-morning-overview-and-standup-commitment-engine*
*Context gathered: 2026-03-19*
