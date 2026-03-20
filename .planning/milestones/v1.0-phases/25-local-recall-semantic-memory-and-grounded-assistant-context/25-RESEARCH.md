# Phase 25 Research

## Domain

Improving local recall quality and grounded assistant context over existing Vel data without widening into a new provider or architecture lane.

## Locked Inputs

- Phase 11 already published the bounded grounding contract and inspect/export surfaces.
- Phases 20 through 24 made the assistant the default usable entry path, unified voice continuity, supported daily-loop and closeout through the shared assistant seam, and added supervised proposal/apply lanes.
- The repository already ships a local semantic-memory baseline with typed contracts, local token-overlap scoring, and provenance-bearing hits.
- Current shell/product direction still prefers local-first trust, explainability, and bounded backend-owned policy over opaque convenience.

## Problem

Vel can answer from bounded tools and semantic memory today, but recall quality is still relatively shallow:

- retrieval is mostly exposed as a raw search tool rather than a richer backend-owned recall/context seam
- assistant context assembly still leans on repeated point tools instead of a stronger reusable recall pack
- current recall quality needs to improve across captures, notes, people, projects, threads, and transcripts before broader expansion is worthwhile

Phase 25 should improve recall quality where it matters most for daily use: better retrieval, better bounded context assembly, and more explainable grounded answers.

## Required Truths

1. Canonical recall contract
   - recall and grounding should build on the existing semantic-memory and grounding contracts rather than inventing assistant-only memory structures
   - backend-owned context assembly should consume typed retrieval outputs, not raw storage-shaped blobs

2. Better local retrieval quality
   - hybrid retrieval should remain local-first and explainable
   - ranking and source selection should improve across the runtime entities Vel already owns

3. Stronger assistant context assembly
   - the assistant should get a better bounded context pack for everyday recall questions
   - shells should not need to infer or reassemble memory context locally

4. Honest limits
   - this phase is not broad provider expansion, hosted memory infrastructure, or speculative graph-RAG scope
   - explainability and provenance remain mandatory

## Recommended Execution Shape

Phase 25 should be executed in four slices:

1. tighten the recall/grounding contract around canonical semantic retrieval and bounded assistant context inputs
2. improve local semantic retrieval quality, ranking, and durable provenance across the current runtime entities
3. assemble stronger backend-owned assistant context from recall results instead of ad hoc tool responses
4. align shell/docs/verification so the shipped recall story is explicit and honest

## Code Context

- `crates/vel-core/src/semantic.rs`
- `crates/vel-core/src/context.rs`
- `crates/vel-storage/src/repositories/semantic_memory_repo.rs`
- `crates/vel-storage/src/db.rs`
- `crates/veld/src/services/retrieval.rs`
- `crates/veld/src/services/chat/tools.rs`
- `crates/veld/src/services/chat/assistant.rs`
- `crates/veld/src/services/agent_grounding.rs`
- `crates/veld/src/services/context_runs.rs`
- `crates/veld/tests/semantic_memory.rs`
- `crates/veld/tests/chat_grounding.rs`
- `clients/web/src/types.ts`
- `docs/cognitive-agent-architecture/cognition/semantic-memory-contract.md`
- `docs/api/chat.md`
- `docs/user/setup.md`

## Risks

- over-widening this phase into a broad semantic-platform rewrite
- inventing assistant-only context state outside canonical retrieval and grounding seams
- improving recall in ways that are hard to explain from scores, source records, and provenance
- letting shell surfaces infer recall logic locally instead of consuming backend-owned context

## Success Condition

Phase 25 is complete when the product can honestly say:

- local semantic recall is better across the durable entities Vel already owns
- assistant turns can consume a stronger backend-owned recall/context seam instead of repeatedly reconstructing context from raw tools
- recall answers remain explainable from persisted records, ranking inputs, and provenance
