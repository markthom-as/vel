---
title: MVP Loop Contracts
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-20
updated: 2026-03-20
keywords:
  - mvp
  - read model
  - overview
  - commitments
  - reflow
  - threads
  - review
index_terms:
  - v0.2 contracts
  - overview read model
  - thread escalation
  - review snapshot
related_files:
  - docs/product/mvp-operator-loop.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md
  - docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md
  - crates/veld/src/services/now.rs
  - crates/veld/src/services/daily_loop.rs
  - crates/veld/src/services/reflow.rs
  - crates/veld/src/routes/threads.rs
  - crates/vel-api-types/src/lib.rs
summary: Canonical Rust-owned contracts for the v0.2 MVP loop across overview, commitments, reflow, threads, and review.
---

# Purpose

Define the canonical Rust-owned contracts for milestone `v0.2`.

This document translates the product loop in [mvp-operator-loop.md](../../product/mvp-operator-loop.md) into durable backend-owned contract language for services, DTOs, and shells.

# Contract Rule

The active MVP loop is:

`overview -> commitments -> reflow -> threads -> review`

Web and Apple consume these contracts as thin shells. They do not redefine the loop locally.

Every contract below must remain:

- current-day only
- explainable from persisted inputs, rules, or run state
- explicit about degraded behavior
- explicit about provenance

# OverviewReadModel

## Intent

`OverviewReadModel` is the operator's first decision surface.

Its required shape is:

`action + timeline`

## Required fields

- `dominant_action`
  Meaning: the one current action the operator should treat as primary when the system has enough evidence.
- `today_timeline`
  Meaning: a compact current-day timeline rather than a full planner surface.
- `visible_nudge`
  Meaning: the single default visible nudge.
- `why_state`
  Meaning: explicit `Why + state` disclosure data for context hidden behind icons or affordances.
- `suggestions`
  Meaning: 1-3 suggestions shown only when no `dominant_action` exists.
- `decision_options`
  Meaning: the allowed operator outcomes for the no-dominant-action state: `accept`, `choose`, `thread`, `close`.

## Rules

- `dominant_action` and `suggestions` are mutually exclusive default states.
- `today_timeline` stays compact and current-day only.
- `visible_nudge` is one item by default; additional context belongs in `why_state`, not as more top-level nudges.
- `why_state` explains why the system surfaced the current view and what state is driving it.

## Degraded behavior

- if timeline freshness is weak, the contract must still show why it is weak rather than fabricate certainty
- if no trustworthy dominant action exists, the contract must fall back to `suggestions`
- if suggestions cannot be generated safely, the contract must degrade to explicit absence rather than silently invent local shell behavior

## Provenance

`OverviewReadModel` should be explainable from persisted context, schedule state, commitments, and thread/history evidence.

# CommitmentFlow

## Intent

`CommitmentFlow` is the bounded inline path from orientation to an explicit day shape.

## Required fields

- `prompt`
- `candidate_commitments`
- `selected_commitments`
- `decision_state`
- `allowed_actions`

## Rules

- the inline action set is `accept / defer / choose / close`
- commitment flow remains inside overview until escalation is required
- commitment flow produces a small explicit set of commitments rather than a broad planning workspace

## Degraded behavior

- if source freshness or availability is weak, the flow must expose that state
- if no safe recommendation exists, the flow must still preserve bounded operator choices

## Provenance

Selections and prompts should remain explainable from persisted operator state, commitments, and bounded rules.

# ReflowProposal

## Intent

`ReflowProposal` is same-day repair over the remaining day.

## Required fields

- `trigger`
- `severity`
- `changes`
- `outcome_counts`
- `rule_facets`
- `thread_escalation_available`
- `review_gating`

## Rules

- reflow is same-day only
- proposal state must distinguish `moved`, `unscheduled`, and `needs_judgment` outcomes, plus whether the bounded inline path is still review-gated or has escalated into `Threads`
- ambiguous or review-gated cases escalate to threads
- `ReflowProposal` does not add local-calendar milestone work to `v0.2`

## Degraded behavior

- stale or incomplete inputs must be called out explicitly
- when safe apply is not possible, the contract must preserve proposal visibility and escalation instead of pretending nothing happened

## Provenance

Reflow decisions should remain explainable from commitments, persisted context, calendar signals already available to the runtime, normalized rule facets, and the typed transition/status records that describe whether the operator accepted or escalated the proposal.

# ThreadEscalation

## Intent

`ThreadEscalation` is the bounded continuation contract for work that becomes genuinely multi-step.

## Required fields

- `thread_target`
- `escalation_reason`
- `continuation_context`
- `review_requirements`
- `bounded_capability_state`

## Rules

Escalation happens only when at least two of the following are true:

- the work needs explanation
- the work needs multiple decisions
- the work needs tool or context work

Threads are not a second inbox and not a generic chat product.

The transport detail for an escalated thread should preserve:

- why the work moved into `Threads`
- the bounded context pack that came with it
- what review/apply gate still exists
- the bounded capability posture instead of implying ambient access

## Degraded behavior

- if thread context is incomplete, the contract must state what is missing
- if tools or context are unavailable, the thread must fail closed rather than widen its capabilities implicitly

## Provenance

Escalation reasons and thread context should remain inspectable from the triggering overview, commitment, or reflow state.

# ReviewSnapshot

## Intent

`ReviewSnapshot` closes the loop with explicit state rather than summary-only claims.

## Required fields

- `commitment_changes`
- `reflow_outcomes`
- `thread_outcomes`
- `remaining_attention`
- `terminal_state`

## Rules

- review must explain what changed and what remains unresolved
- terminal state matters more than dashboard accumulation
- review remains part of the same MVP loop rather than a separate analytics product

## Degraded behavior

- missing downstream state must be surfaced explicitly
- review should preserve uncertainty markers rather than compressing them away

## Provenance

Review output should trace back to commitment transitions, reflow proposals or applications, and thread-mediated actions.

# Shell Consumption Rule

Web and Apple consume these contracts through Rust-owned services and typed transport DTOs.

They may:

- embody the loop visually
- choose appropriate platform interaction patterns
- disclose `Why + state` through icons, drawers, or secondary surfaces

They may not:

- invent alternate MVP loop steps
- invent shell-owned prioritization logic
- widen degraded states into silent local heuristics
