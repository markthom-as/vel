# Phase 25 Context

## Title

Local recall, semantic memory, and grounded assistant context

## Why This Phase Exists

Phases 20 through 24 made the assistant materially usable:

- `Now`, `Inbox`, `Threads`, and `Settings` are clearer daily-use shells
- voice, typed entry, morning/standup, end-of-day, and thread escalation all route through one grounded assistant seam
- assistant proposals can now be staged, reviewed, approved, applied, and reversed within existing supervised write lanes

The next product-value gap is recall quality. The assistant can see bounded Vel state and tools, but Vel still needs stronger local recall, semantic memory, and reusable context assembly so the operator can ask real questions about their data and get grounded answers without drifting into generic chat behavior.

This phase exists to improve the quality of memory-backed recall and assistant context before widening into more providers or heavier UI work.

## Expected Outcome

By the end of this phase, Vel should be able to:

- retrieve locally relevant captures, notes, tasks, people, and project context more reliably
- assemble better bounded grounding context for assistant turns without exposing raw storage internals
- improve recall quality for everyday operator questions before pushing further on new shell polish or provider expansion
- preserve explainability and provenance for recalled context

## Constraints

- Reuse the Phase 11 grounding posture and the Phase 20-24 assistant seam rather than inventing a second assistant stack.
- Keep Rust backend layers authoritative for recall, ranking, and context assembly.
- Do not widen into broad new provider expansion, hosted features, or speculative multi-agent work.
- Prefer local-first retrieval and explainable ranking over opaque convenience.
- Preserve the repository layering rules in `AGENTS.md`: core semantics in `vel-core`, transport DTOs in `vel-api-types`, thin routes, and backend-owned policy.

## Likely Focus Areas

- typed semantic retrieval and ranking improvements over existing local data
- recall-oriented read models or bounded retrieval helpers for assistant/tool use
- better project/person/context linking for grounded answers
- clear surfacing of why a memory or context item was retrieved
- tests that prove recall behavior from persisted local inputs rather than synthetic frontend-only shaping

## Non-Goals

- broad new connector families
- hosted auth or SaaS expansion
- large shell redesigns beyond what recall surfaces strictly need
- bypassing the existing supervised write, review, or SAFE MODE boundaries

## Handoff To Planning

Planning should break this phase into the narrowest sequence that improves real recall quality first. The most likely order is:

1. publish or tighten the recall/grounding contract
2. improve local semantic retrieval and ranking
3. improve assistant context assembly over the new recall seam
4. expose the improved recall cleanly in the existing shells and docs
