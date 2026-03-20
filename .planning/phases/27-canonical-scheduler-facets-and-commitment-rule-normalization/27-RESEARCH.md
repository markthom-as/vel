# Phase 27 Research

## Domain

Turning the proven `codex-workspace` scheduling and tagging system into canonical Vel-backed scheduler facets that can be persisted, queried, explained, and reused by reflow, recall, and agent grounding.

## Locked Inputs

- Phase 26 made `reflow` a real backend-owned same-day recovery lane with typed proposals, but it still normalizes scheduler intent from raw labels and title text at runtime.
- The operator already has a working single-day scheduling model in `/home/jove/code/codex-workspace`.
- `Vel.csv` reinforced the need for subtle contextual status, explainable freshness/recovery, rich project context, and easy thread continuity rather than louder dashboard state.
- The product direction already rejects provider-specific labels as durable product truth; adapter compatibility must remain separate from canonical Vel semantics.
- SQL-backed structured facets were explicitly called out as useful for agents, recall, and explainable filtering, with keywords and raw labels remaining secondary search material.

## Problem

Vel currently understands scheduling intent only as a bridge:

- `reflow` reparses raw labels and text each time
- agent reasoning cannot reliably inspect durable scheduling semantics
- recall/search can see lexical traces of the rule system, but not one canonical scheduler-rule substrate
- upstream/provider syntax still carries too much product meaning

That is good enough for a transition slice, but not for durable backend-owned planning and recovery behavior.

## Required Truths

1. Canonical scheduler facets
   - Vel needs a canonical scheduler-rule model that is independent of any one provider label system.
   - The model should preserve the proven `codex-workspace` semantics intentionally:
     - `block:<name>`
     - `cal:free`
     - duration markers like `30m` and `1h`
     - time windows like `time:prenoon`, `time:afternoon`, `time:evening`, `time:day`, `time:night`
     - urgent/defer semantics
     - anchored due datetimes

2. SQL-backed persistence
   - Scheduler semantics should be stored in canonical Vel form, not only reconstructed from labels at runtime.
   - SQL-backed fields/facets make exact filtering, explainability, agent use, and reconciliation much safer than repeated prose parsing.

3. Adapter boundary discipline
   - Raw upstream labels and tokens should remain compatibility metadata at ingest boundaries.
   - Ingest should normalize them into Vel-owned semantics instead of letting raw provider labels become the product model.

4. Shared substrate across behavior lanes
   - `reflow`, recall, assistant grounding, and future commitment-aware planning must consume the same scheduler semantics.
   - Shells should present typed semantics, not reinterpret raw labels locally.

## Recommended Execution Shape

Phase 27 should be executed in four slices:

1. define the canonical scheduler facet schema, ingest mapping, and owner docs
2. persist normalized scheduler facets for commitments and expose them through backend/domain seams
3. replace ad hoc runtime parsing in reflow, recall, and agent context with the normalized facet substrate
4. close with docs, examples, and verification that teach the canonical rule model honestly

## Code Context

- `crates/vel-core/src/`
- `crates/vel-storage/src/`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/retrieval.rs`
- `crates/veld/src/services/chat/`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`
- `docs/cognitive-agent-architecture/cognition/semantic-memory-contract.md`
- `docs/product/operator-action-taxonomy.md`
- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/docs/tui.md`
- `/home/jove/code/codex-workspace/schemas/task-schema.md`
- `/home/jove/code/codex-workspace/schemas/todoist-obsidian-alignment.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`
- `/home/jove/Downloads/Vel.csv`

## Risks

- preserving the names of old tags without preserving their actual semantics
- letting provider labels continue to act as hidden product truth under a thin normalization veneer
- creating a second scheduler model just for agents or just for recall
- widening into a broad multi-day planner rewrite instead of tightening the canonical semantics seam
- overfitting to Todoist-compatible syntax when the goal is a canonical Vel model

## Success Condition

Phase 27 is complete when the product can honestly say:

- scheduler semantics are persisted in canonical Vel form
- raw provider labels are compatibility metadata, not durable product truth
- reflow, recall, and assistant grounding can inspect one shared scheduler-facet substrate
- operators can understand why a commitment is scheduled or constrained without reading raw label syntax
