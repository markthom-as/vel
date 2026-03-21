---
title: v0.2 MVP Operator Loop
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-20
updated: 2026-03-20
keywords:
  - mvp
  - operator loop
  - overview
  - commitments
  - reflow
  - threads
  - review
index_terms:
  - true mvp
  - daily operator loop
  - thread escalation rule
  - inline action set
related_files:
  - docs/MASTER_PLAN.md
  - docs/product/now-inbox-threads-boundaries.md
  - docs/product/operator-surface-taxonomy.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
summary: Durable product authority for the strict v0.2 MVP loop and its anti-drift boundaries.
---

# Purpose

Define the true MVP for milestone `v0.2` as one strict daily operator loop:

`overview -> commitments -> reflow -> threads -> review`

This document is the durable product authority for what that loop means, how it stays bounded, and when work must escalate out of the inline flow.

# Product Rule

`v0.2` is current-day only.

The MVP exists to help the operator:

- orient to today's real pressure
- commit to a small explicit set of work
- recover from drift through same-day reflow
- continue messy work in bounded threads
- review what changed and what still needs attention

If a feature does not directly strengthen one step of this loop, or keep that loop Rust-owned across shells, it is out of scope for `v0.2`.

# Loop Steps

## 1. Overview

The overview is the top-level decision surface.

Its required shape is:

`action + timeline`

It must show:

- one dominant current action when one exists
- a compact today timeline
- one visible top nudge
- additional context behind explicit `Why + state` affordances

When no dominant action exists, overview must present a decision prompt with 1-3 suggestions and allow the operator to:

- accept a suggestion
- choose from other suggestions
- enter thread-based resolution
- close

## 2. Commitments

The commitment step turns the overview into a bounded day shape.

It must let the operator:

- accept a suggestion
- defer an item
- choose from alternate suggested items
- close the prompt when no action should be taken

The inline action set is:

`accept / defer / choose / close`

## 3. Reflow

Reflow is same-day schedule repair over the remaining day.

It must remain:

- bounded to the current day
- explainable from persisted inputs and rules
- Rust-owned rather than shell-owned
- explicit about scheduled, deferred, conflicted, and did-not-fit outcomes

Reflow does not imply multi-day planning or broad calendar automation.

## 4. Threads

Threads are the continuation path for genuinely multi-step work.

Threads are not a second inbox and not a generic chat product.

Thread escalation happens only when at least two of the following are true:

- the work needs explanation
- the work needs multiple decisions
- the work needs tool or context work

If those conditions are not met, the operator should stay in the inline loop.

## 5. Review

Review closes the loop.

It is the backend-owned closeout lane over the same persisted state that drove overview, commitments, reflow, and thread continuation.

It must let the operator see:

- what commitments changed
- what reflow decided
- what was deferred or left unresolved
- what thread-mediated work changed state
- what still needs attention before tomorrow

Review is not a generic analytics surface. It exists to preserve explainability and closeout.

In `v0.2`, review should stay grounded in the existing closeout seams:

- `review_snapshot` for compact current-day remaining attention
- the run-backed end-of-day summary for what was done, what remains open, and what may matter tomorrow
- the same thread continuity and reflow outcomes already visible earlier in the loop

# Surface Implications

## `Now`

`Now` is the MVP home for overview, commitments, and immediate reflow pressure.

It is the first-glance surface for:

- dominant action
- compact timeline
- top nudge
- inline commitment choices
- same-day intervention pressure

## `Threads`

`Threads` is the bounded continuation surface for work that becomes multi-step.

It exists for:

- explanation-heavy decisions
- tool- or context-backed follow-through
- explicit continuity when inline choices are no longer enough

## `Inbox` and other surfaces

`Inbox`, `Projects`, `Settings`, and deeper inspection surfaces may still exist, but they are not allowed to redefine the MVP loop.

They support the loop. They do not replace it.

# Non-Goals

`v0.2` does not include:

- multi-day planning
- generic chat-first product expansion
- shell-owned prioritization, ranking, or planner logic
- local-calendar milestone work
- broad calendar write-back automation
- broad UI redesign outside the loop surfaces
- provider or platform expansion not required by the loop

# Authority Rule

The MVP loop is product authority.

Implementation work should map back to this loop before introducing new screens, flows, or contracts. If a proposed change cannot be explained as strengthening overview, commitments, reflow, threads, or review, it does not belong in `v0.2`.
