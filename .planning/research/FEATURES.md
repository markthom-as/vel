# Feature Landscape

**Domain:** local-first operator cognition product, true MVP closure for milestone v0.2
**Researched:** 2026-03-20

## Table Stakes

Features the MVP should treat as expected operator behavior. Missing these makes the product feel incomplete even if the substrate already exists.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Daily overview with one dominant current action | Serious planning/operator tools are expected to open on “what matters now,” not on a generic dashboard or chat feed. | Medium | `Now` should answer current commitment, next useful action, next calendar anchor, and whether the day is on track. This is the first-glance surface, not a backlog browser. |
| Commitment selection and lightweight editing | An MVP daily loop needs explicit commitment, not just passive lists. Users expect to pick 1-3 things, adjust them, and see what is active now. | Medium | Should allow continue, break down, defer, and promote to thread without leaving the main flow. Avoid a separate planner subsystem. |
| Unified today lane across calendar + commitments + priority work | Current operator products commonly win by putting tasks and time on one surface. A split task view and calendar view feels unfinished. | High | Use existing Vel commitments/calendar substrate, but present one coherent “today” lane. Meetings/events should affect actionability, not sit as unrelated summaries. |
| Same-day reflow after drift | Once the day changes, operators expect a repair path. Without reflow, the morning plan becomes stale and trust collapses by midday. | High | Keep it bounded and explainable: same-day only, clear moved/unscheduled/needs-judgment outcomes, operator-supervised application. |
| Actionable nudges and check-ins | A daily operator product is expected to notice drift, blocked work, missed anchors, and stale commitments. | Medium | Nudges must be small, specific, and resolvable: “do this now,” “defer,” “open thread,” or “reflow.” Do not show advisory cards with no clear action. |
| Thread escalation for non-trivial loops | When a decision cannot be closed inline, users expect to continue it somewhere durable. | Medium | Threads should open from `Now`/Inbox items as “deeper thought” or continuity lanes, not as generic chat rooms. |
| Review / closeout loop | Good planning tools are expected to help close the day by reviewing what moved, slipped, or needs carry-forward. | Medium | Keep review lightweight: completed, deferred, unscheduled, unresolved thread follow-up. The value is closure and feed-forward into tomorrow, not analytics theater. |
| Cross-surface consistency | For this milestone, web and Apple need the same product behavior or the MVP will feel fragmented. | High | Rust-owned behavior should be canonical. Shells may differ in presentation, but not in planning semantics or thread/reflow meaning. |

## Differentiators

Features worth keeping because they strengthen Vel’s MVP instead of widening it.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Rust-owned canonical daily loop | Makes the MVP portable across web and Apple without re-implementing planner logic in each shell. | High | This is a milestone goal, not optional polish. It is the backbone that keeps the MVP honest. |
| Explainable local reflow | Most tools offer planning; fewer offer a local-first, explainable same-day repair lane with visible rationale. | High | Keep rule-facet-backed explanations and supervised application. This is more valuable than “AI magic” auto-rescheduling. |
| Suggestions/nudges grounded in persisted context | Nudges are useful when they come from commitments, time state, calendar pressure, and thread status instead of generic assistant prose. | Medium | Preserve provenance and explicit routing to action, reflow, or thread. |
| Threads as continuity substrate, not chat | This gives Vel a cleaner operator mental model than “assistant conversation everywhere.” | Medium | A thread should show timeline, decisions made, open questions, and next step. Promote from action into thread and resolve back into action. |
| Review as operational closure | A short closeout that prepares tomorrow is more valuable in MVP than dashboards, scoring, or long retrospectives. | Low | Keep it focused on carry-forward and trust, not quantified-self reporting. |

## Anti-Features

Features to explicitly NOT build in this milestone.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Multi-day or autonomous planner | This milestone is about a trustworthy same-day MVP. Multi-day optimization will dominate scope and weaken explainability. | Keep planning and reflow bounded to the current day. |
| Generic chat-first assistant shell | Threads are for open loops, not for making the whole product feel like a chatbot wrapper. | Keep `Now` decision-first and use threads only when inline resolution is insufficient. |
| Full project/workspace management expansion | Projects already exist as substrate. Rebuilding project dashboards will distract from the daily loop. | Use project context only when it sharpens action, nudges, or thread routing. |
| Opaque AI automation or silent calendar edits | Silent mutation breaks trust fast, especially in calendar/task systems. | Stage proposals, explain outcomes, and require explicit supervised apply for meaningful changes. |
| Heavy review analytics, scores, or productivity reporting | These add UI mass without improving daily operation for MVP. | Keep review to closure, carry-forward, and what needs attention tomorrow. |
| Routine builder redesign or rich drag-and-drop planner studio | This is a classic scope trap. It turns the milestone into a scheduling UI rewrite. | Reuse existing planning-profile/routine substrate with thin inline edits and readable time-block cards. |
| Shell-specific planning logic | Re-implementing behavior in web and Apple will break parity and undercut the milestone’s Rust-owned goal. | Keep shells thin and route all planning semantics through the canonical backend seams. |
| Debug/runtime state in default surfaces | Internal model fields and confidence readouts add noise and make the product feel unfinished. | Move internals behind explicit `State` / `Why` / `Debug` disclosure. |

## Expected Operator Behaviors

For this milestone, the MVP should feel like this:

1. Open `Now` and immediately see one dominant current action, next relevant time context, and at most a few next actions.
2. Commit to a small set of work for the day and edit it inline without entering a separate planning tool.
3. Receive small, actionable nudges when drift appears, with clear next moves: continue, defer, thread, or reflow.
4. Repair the day when reality changes through a bounded reflow proposal that stays local-first and operator-supervised.
5. Escalate messy or ambiguous work into a thread that preserves context, decisions, and next step.
6. End with a brief review that closes loops and cleanly carries unresolved work forward.

## Feature Dependencies

```text
Unified today lane -> Daily overview -> Commitment flow
Commitment flow -> Nudges/check-ins
Unified today lane + commitments + routine constraints -> Same-day reflow
Daily overview + nudges/check-ins -> Thread escalation
Commitment flow + reflow + thread outcomes -> Review / closeout
Rust-owned canonical daily loop -> Web and Apple parity for all MVP flows
```

## MVP Recommendation

Prioritize:

1. Daily overview plus one dominant current action on `Now`
2. Commitment flow with inline continue / break down / defer / thread escalation
3. Same-day reflow with explicit, explainable outcomes
4. Lightweight review / closeout that feeds the next day

Keep as selective differentiators:

1. Rust-owned canonical flow across shells
2. Explainable local reflow
3. Threads as structured continuity, not generic chat

Defer:

- Rich routine editing UI: reuse existing substrate and keep edits thin
- Broad analytics/reporting: no MVP value compared with review closure
- Multi-day planning: too much surface and policy for this milestone
- New project/dashboard concepts: substrate already exists; do not recentralize the product around it

## Sources

- Repo authority: [PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md)
- Repo authority: [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md)
- Repo authority: [STATE.md](/home/jove/code/vel/.planning/STATE.md)
- Repo authority: [MASTER_PLAN.md](/home/jove/code/vel/docs/MASTER_PLAN.md)
- Product boundary: [now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md)
- Architecture contract: [day-plan-reflow-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md)
- Phase input: [40-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.2-phases/40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces/40-CONTEXT.md)
- Phase input: [40-RESEARCH.md](/home/jove/code/vel/.planning/milestones/v0.2-phases/40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces/40-RESEARCH.md)
- External pattern check: https://www.sunsama.com/daily-planning
- External pattern check: https://www.sunsama.com/features/guided-planning-and-reviews
- External pattern check: https://help.sunsama.com/docs/importing-meetings
- External pattern check: https://routine.co/features/planner
- External pattern check: https://product.akiflow.com/help/articles/5397877-how-to-plan-a-perfect-week
