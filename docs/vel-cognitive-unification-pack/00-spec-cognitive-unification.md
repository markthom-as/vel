---
title: Vel Cognitive Unification Spec
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Summary

Vel now has four nearly-connected subsystems:

1. external awareness
   - Todoist
   - calendar
   - project registry

2. suggestion engine
   - current repo has `migrations/0017_suggestions.sql`
   - current service only emits repeated-pattern suggestions

3. uncertainty handling
   - proposed in prior work but not yet materially integrated with external gaps

4. technical loops
   - current runtime docs describe loops, but the repo still lacks a unified loop registry and durable loop state

The next step is to connect them into one cognition pipeline.

# Core model

Vel should compute from the following layers:

## L0 raw evidence
- captures
- external items
- source snapshots
- metrics

## L1 normalized signals
- task seen
- event scheduled
- project drift
- overdue pressure
- focus block unavailable
- unresolved project mapping

## L2 commitments and project state
- open obligations
- project clusters
- schedule pressure
- conflict windows

## L3 evaluations
- risk
- suggestion candidates
- uncertainties
- loop triggers

## L4 interventions
- nudges
- questions
- sync proposals
- summaries

This layered model is not philosophical garnish. It is the only way to prevent silent incoherence.

# Unification rules

## Rule 1: suggestions must cite evidence
No suggestion should exist without:
- one or more signal ids
- optional linked commitments
- optional linked external items
- confidence / uncertainty metadata

## Rule 2: uncertainty must be explicit, not improvised
When data is missing or ambiguous, Vel should record an uncertainty item, not silently guess.

Examples:
- Todoist project cannot map to canonical project slug
- calendar event likely belongs to a project but confidence is low
- calendar move proposal conflicts with another event window
- multiple tasks compete for "now" with no ranking margin

## Rule 3: loops should evaluate from stable state, not live improvised reads
Loops should read:
- `current_context`
- project registry
- unresolved uncertainties
- active suggestions and nudges
- sync watermarks

They should not reconstruct the whole world from scattered tables every time.

## Rule 4: write actions become proposals
Loops and suggestions may propose:
- relabel task
- reschedule block
- ask question
- create review artifact

They do not directly mutate external systems.

# Schema additions for unification

Add the following new durable structures.

## `uncertainties`
Fields:
- `uncertainty_id TEXT PRIMARY KEY`
- `kind TEXT NOT NULL`
- `state TEXT NOT NULL`
- `summary TEXT NOT NULL`
- `subject_kind TEXT`
- `subject_id TEXT`
- `evidence_json TEXT NOT NULL`
- `missing_json TEXT`
- `confidence REAL`
- `created_at INTEGER NOT NULL`
- `resolved_at INTEGER`

## `loop_runs`
Fields:
- `loop_run_id TEXT PRIMARY KEY`
- `loop_kind TEXT NOT NULL`
- `state TEXT NOT NULL`
- `started_at INTEGER NOT NULL`
- `finished_at INTEGER`
- `watermark_json TEXT`
- `result_json TEXT`

## `loop_claims`
Fields:
- `loop_kind TEXT PRIMARY KEY`
- `claimed_by TEXT NOT NULL`
- `claimed_at INTEGER NOT NULL`
- `expires_at INTEGER NOT NULL`

## suggested `suggestions` extension
Current suggestions table is too thin.
Add columns or parallel evidence table:
- `confidence REAL`
- `summary TEXT`
- `evidence_json TEXT`
- `project_slug TEXT`
- `commitment_id TEXT`
- `external_item_id TEXT`
- `proposal_id TEXT`

# Unified evaluation pipeline

## 1. sync loops
Update external items and watermarks.

## 2. context loop
Recompute project-aware, task-aware, schedule-aware context.

## 3. evaluation loop
Compute:
- risk
- suggestion candidates
- unresolved uncertainties

## 4. intervention loop
Materialize:
- nudges
- questions
- sync proposals

## 5. review loops
Generate:
- morning orientation
- midday recalibration
- evening review
- weekly project review

# New files

## New
- `crates/vel-core/src/uncertainty.rs`
- `crates/vel-core/src/looping.rs`
- `crates/veld/src/services/uncertainty.rs`
- `crates/veld/src/services/loop_registry.rs`
- `crates/veld/src/services/loop_scheduler.rs`
- `crates/veld/src/services/question_engine.rs`
- `migrations/0029_uncertainties.sql`
- `migrations/0030_loop_runs.sql`
- `migrations/0031_loop_claims.sql`
- `docs/specs/vel-cognitive-unification.md`

## Changed
- `crates/veld/src/services/suggestions.rs`
- `crates/veld/src/services/inference.rs`
- `crates/veld/src/services/evaluate.rs`
- `crates/veld/src/worker.rs`
- `crates/veld/src/routes/suggestions.rs`
- `crates/veld/src/routes/context.rs`
- `crates/veld/src/routes/sync.rs`

# Acceptance criteria

- suggestions, uncertainties, and loops all read from the same project-aware context model
- every surfaced intervention has traceable evidence
- missing data generates explicit uncertainty instead of hidden guesswork
- loops are durable, inspectable, and idempotent
- external mutations remain proposal-based
