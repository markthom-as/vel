# Phase 11: Agent grounding and operator-relevant data/tool awareness - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** User priority correction + current roadmap/codebase analysis

<domain>
## Phase Boundary

Phase 11 makes supervised agents aware of actual Vel state instead of behaving like generic assistants with weak repo-only or prompt-only context. The phase must ground agents in the operator's current context, projects, people, commitments, review queues, and bounded tool affordances, while preserving trust, explainability, and review gates.

This phase is not broad autonomy or unsupervised delegation. It is the productization of already-shipped state and tool seams so agents can help with real operator work using real Vel data under explicit supervision.

</domain>

<decisions>
## Implementation Decisions

### Product intent
- [locked] This phase exists because agent awareness of Vel data and operator-relevant tool access is important enough to be committed roadmap work, not backlog-only future work.
- [locked] Agents should be grounded in real Vel state: current context, projects, people, commitments, review queues, execution handoffs, and bounded tool affordances.
- [locked] Grounding must improve operator-trustworthy action, not create a second opaque assistant layer.

### Trust and review model
- [auto] Agent-visible context must remain inspectable and traceable back to persisted Vel records or explicit execution context packs.
- [auto] Tool affordances should remain bounded and operator-visible. Do not give agents ambient access to everything Vel can see or mutate.
- [auto] Review gates, handoff state, and SAFE MODE/writeback constraints must remain intact. Agent grounding should not bypass the review model shipped in earlier phases.
- [auto] Unknown or unsupported tool/data requests should fail closed.

### Data grounding scope
- [auto] The minimum useful grounding bundle should consider:
  - `Now` state and current context
  - projects and project review candidates
  - people records and people-needing-review signals
  - commitments and commitment-linked project state
  - execution context and execution handoffs
  - pending writebacks, open conflicts, and other operator review obligations
- [auto] Grounding should prefer typed summaries and references over raw unbounded dumps.
- [auto] Repo-local coding context from Phase 08 should remain one grounding input, not the whole product.

### Tool-awareness scope
- [auto] Tool access should be described in operator-relevant terms, not just low-level runtime/internal names.
- [auto] The system should distinguish:
  - read-only context access
  - bounded review/inspection actions
  - bounded mutation affordances still subject to existing approval or SAFE MODE constraints
- [auto] If an agent lacks a required data/tool grant, the operator should be able to see why and what narrow escalation would be needed.

### Client/surface discipline
- [auto] Backend Rust layers own grounding policy, summarization boundaries, and capability decisions.
- [auto] Web/CLI/operator surfaces should expose what the agent can currently see and do, plus why.
- [auto] This phase may add operator-surface visibility for agent grounding, but broad shell/navigation cleanup remains Phase 12.

### Claude's Discretion
- Exact contract/type names for grounded context packs, tool descriptors, review-bound capability summaries, and operator-facing inspect surfaces
- Whether the first shipped grounding product surface is CLI-first, runtime/API-first, web-first, or a thin combination
- How much grounding is pushed through existing execution-context/handoff seams versus a new agent-facing summary seam, provided layering and trust rules are preserved

</decisions>

<specifics>
## Specific Ideas

- The codebase already ships most of the raw materials needed for this phase:
  - `/v1/now`
  - current context explain/drift explain
  - projects
  - people
  - execution context and execution handoffs
  - pending writebacks/conflicts/review signals
- The missing product layer is one trustworthy agent-facing grounding path over those seams.
- This phase should likely reuse the execution-context/handoff vocabulary from Phase 08 where possible, but it must widen beyond coding-only context so agents can reason over actual Vel state.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and product authority
- `.planning/ROADMAP.md` — Phase 11 goal, requirements IDs, execution order, and priority note
- `.planning/PROJECT.md` — accepted decision that agent awareness is roadmap work and not just backlog
- `.planning/STATE.md` — accumulated decisions and current milestone ordering
- `docs/MASTER_PLAN.md` — canonical shipped-status tracker

### Prior phases that Phase 11 must extend
- `.planning/phases/05-now-inbox-core-and-project-substrate/05-CONTEXT.md` and summaries — typed projects, action/intervention surfaces, and continuity substrate
- `.planning/phases/06-high-value-write-back-integrations-and-lightweight-people-graph/06-CONTEXT.md` and summaries — people, provenance, conflicts, and pending writeback visibility
- `.planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-01-SUMMARY.md` through `08-06-SUMMARY.md` — execution context, handoffs, connect transport, review queue, and operator guidance

### Existing shipped surfaces and seams
- `docs/user/daily-use.md` — current operator review/daily/coding workflows that grounding should reinforce
- `docs/api/runtime.md` — current runtime, execution, connect, inspect, and review endpoints
- `clients/web/src/components/NowView.tsx` — operator-visible `Now` projection already exposing action/review pressure, people review, and execution review hints
- `clients/web/src/components/SettingsPage.tsx` — current runtime/settings/operator review shell including execution handoff review
- `clients/web/src/data/context.ts` — current frontend load path for `/v1/now`, current context, explain surfaces, and commitments
- `crates/vel-cli/src/commands/review.rs` — current CLI review output over captures, projects, commitments, people, writebacks, and conflicts

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Now` data already carries projects, people, action items, conflicts, pending writebacks, and review counts.
- Projects and people are durable typed records with authenticated runtime routes.
- Execution context and execution handoff records already exist with review states, launch preview, and explicit routing metadata.
- CLI review flows already summarize captures, projects, commitments, people-needing-review, pending writebacks, and conflicts.

### Missing or Thin Areas
- There is no explicit agent-facing grounded-context product surface over these existing records.
- There is no clear operator-visible model of what an agent can currently see or do across Vel data/tool seams.
- Repo-local execution context exists for coding, but there is no broader cross-product grounding contract for non-coding operator work.
- Trust, SAFE MODE, and review constraints are present, but they are not yet packaged into a coherent agent-grounding UX/API story.

</code_context>

<deferred>
## Deferred Ideas

- Broad shell/nav/docs/onboarding cleanup — Phase 12
- New provider families, hosted auth scaffolding, and broad platform expansion — backlog/later milestone
- Full autonomy or unsupervised agent behavior that widens beyond explicit handoff/review boundaries

</deferred>

---

*Phase: 11-agent-grounding-and-operator-relevant-data-tool-awareness*
*Context gathered: 2026-03-19*
