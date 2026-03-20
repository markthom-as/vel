---
title: MVP Loop Contract Checklist
doc_type: template
status: active
owner: staff-eng
created: 2026-03-20
updated: 2026-03-20
keywords:
  - mvp
  - checklist
  - overview
  - reflow
  - threads
  - review
index_terms:
  - v0.2 checklist
  - overview contract checklist
  - loop drift review
related_files:
  - docs/product/mvp-operator-loop.md
  - docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
summary: Reusable checklist for validating MVP-loop product, contract, shell-boundary, and degraded-state changes against the locked v0.2 scope.
---

# Purpose

Use this checklist when planning, reviewing, or documenting changes that touch the `v0.2` MVP loop:

`overview -> commitments -> reflow -> threads -> review`

The goal is to prevent drift away from the locked Phase 40 authority.

# Product-Loop Scope

- [ ] The change clearly strengthens overview, commitments, reflow, threads, or review.
- [ ] The change is current-day only.
- [ ] The change does not widen into multi-day planning, generic chat expansion, shell-owned policy, or local-calendar milestone work.

# OverviewReadModel

- [ ] The overview still uses `action + timeline`.
- [ ] The contract still supports one dominant action as the primary default state.
- [ ] The contract still supports a compact timeline rather than a broad planner surface.
- [ ] The contract still supports a single visible nudge by default.
- [ ] Additional context still lives behind `Why + state` disclosure instead of expanding the default surface.
- [ ] When no dominant action exists, the overview still supports 1-3 suggestions.
- [ ] The no-dominant-action path still allows `accept`, `choose`, `thread`, and `close`.

# CommitmentFlow

- [ ] The inline action set remains `accept / defer / choose / close`.
- [ ] Commitment flow remains bounded and explainable.
- [ ] Commitment flow does not become a separate planner workspace.

# ReflowProposal

- [ ] Reflow remains same-day only.
- [ ] Reflow outcomes remain explicit: scheduled, deferred, conflicted, and did-not-fit.
- [ ] Ambiguous or review-gated reflow states still escalate to threads.
- [ ] Reflow does not introduce local-calendar milestone scope.

# ThreadEscalation

- [ ] Threads still require genuinely multi-step work.
- [ ] Escalation still means at least two of: explanation, multiple decisions, tool/context work.
- [ ] Threads remain bounded continuation, not a generic chat surface.

# ReviewSnapshot

- [ ] Review still explains what changed and what remains unresolved.
- [ ] Review preserves terminal state and provenance instead of summary-only claims.
- [ ] Review does not widen into generic analytics or journaling.

# Shell-Boundary Checks

- [ ] Web and Apple still consume Rust-owned contracts as thin shells.
- [ ] The change does not move prioritization, ranking, or degraded-state interpretation into shell code.
- [ ] Surface docs still describe `Now` as overview/commitments/reflow pressure and `Threads` as bounded continuation.

# Provenance And Degraded-State Checks

- [ ] The changed behavior is explainable from persisted inputs, rules, or run state.
- [ ] Degraded states are explicit rather than hidden behind local heuristics.
- [ ] Missing or stale inputs fail closed or degrade visibly.

# Anti-Drift Non-Goals

- [ ] No multi-day planning expansion
- [ ] No generic chat-first product expansion
- [ ] No shell-owned planner logic
- [ ] No local-calendar milestone work
- [ ] No broad UI redesign outside the MVP loop
