# Roadmap: Vel

## Overview

Milestone `v0.1` is archived in [`.planning/milestones/v0.1-phases`](/home/jove/code/vel/.planning/milestones/v0.1-phases). Milestone `v0.2` starts at Phase 40 because Phase 39 is the last completed phase of `v0.1`, and the previously planned Phase 40 is being absorbed and reworked as the first phase of the new milestone.

The goal of `v0.2` is to establish the true MVP:

- one strict daily operator loop
- canonical Rust-owned types, logic, and docs for that loop
- explainable same-day reflow with local-first input where possible
- threads as the bounded continuation path for tools, context, and data
- minimal fresh web and Apple shells over the same Rust-owned core
- explicit post-MVP roadmap work pushed out of scope

## Scope Guardrails

`v0.2` is only about the strict loop:

`overview -> commitments -> reflow -> threads -> review`

If a change does not directly strengthen one step of that loop, or directly help keep that loop Rust-owned and cross-surface portable, it should be deferred.

Do not widen this milestone into:

- multi-day planning
- broad provider/platform expansion
- broad shell polish outside MVP loop screens
- shell-owned business logic or planner fallback logic
- broad Apple FFI migration
- broad calendar write-back automation
- generic chat/tool product expansion

## Phases

- [ ] **Phase 40: MVP definition, canonical contracts, and architecture refinement** - Freeze the true MVP, define canonical Rust-owned types and business logic, and refine the MVP architecture/docs before UI or migration work widens
- [ ] **Phase 41: Rust-owned overview, commitment flow, and orientation core** - Make `Now`, daily overview, commitment flow, and nudges/orientation fully backend-owned and decision-first
- [ ] **Phase 42: Explainable same-day reflow and local-first calendar path** - Finish bounded reflow in Rust and add the narrow local-first calendar input path needed to support it
- [ ] **Phase 43: Thread continuation, tools, context, and data** - Formalize threads as the bounded continuation substrate for tools, context, memory, and follow-through
- [ ] **Phase 44: Minimal fresh web and Apple shells** - Rebuild the shipped web and Apple MVP surfaces as thin clients over the canonical Rust core
- [ ] **Phase 45: Review, MVP verification, and post-MVP roadmap shaping** - Close the loop with review, end-to-end MVP verification, and explicit post-MVP roadmap/deferred scope

## Progress

**Execution Order:** 40 -> 41 -> 42 -> 43 -> 44 -> 45

| Phase | Requirements | Status |
|-------|--------------|--------|
| 40. MVP definition, canonical contracts, and architecture refinement | MVP-04, MODEL-01, MODEL-02, MODEL-03, MODEL-04, CORE-01 | Planned |
| 41. Rust-owned overview, commitment flow, and orientation core | MVP-01, MVP-02, ORIENT-01, ORIENT-02, ORIENT-03 | Planned |
| 42. Explainable same-day reflow and local-first calendar path | REFLOW-01, REFLOW-02, REFLOW-03, REFLOW-04 | Planned |
| 43. Thread continuation, tools, context, and data | THREAD-01, THREAD-02, THREAD-03 | Planned |
| 44. Minimal fresh web and Apple shells | CORE-02, CORE-03, CLIENT-01, CLIENT-02, CLIENT-03 | Planned |
| 45. Review, MVP verification, and post-MVP roadmap shaping | MVP-03, REVIEW-01, REVIEW-02, FUTURE-01, FUTURE-02 | Planned |

## Phase Details

### Phase 40: MVP definition, canonical contracts, and architecture refinement

**Goal:** Define the true MVP precisely enough that implementation and UI work stop guessing, while locking Rust-owned contracts and refining the architecture/docs that explain where MVP authority lives.
**Requirements:** MVP-04, MODEL-01, MODEL-02, MODEL-03, MODEL-04, CORE-01
**Depends on:** Milestone v0.1 archive truth
**Success Criteria:**
1. One explicit MVP operator loop is documented and agreed: overview -> commitments -> nudges/reflow -> threads -> review.
2. Canonical Rust-owned types and read-model contracts exist for overview, commitment flow, reflow, thread continuation, and review.
3. MVP business logic and cross-surface architecture docs describe one authority model without shell ambiguity.
4. Stable MVP contracts ship with docs/examples/templates close to the code boundary.
**Non-goals:**
- broad UI redesign
- provider expansion
- implementation work not needed to lock the MVP contracts and boundaries
**Plans:** 4 plans

Plans:
- [ ] 40-01-PLAN.md — Define the strict v0.2 MVP boundary and operator loop
- [ ] 40-02-PLAN.md — Publish canonical Rust-owned MVP contracts and typed data models
- [ ] 40-03-PLAN.md — Refine architecture, shell-boundary, and authority docs for the MVP core
- [ ] 40-04-PLAN.md — Reconcile contract docs, examples, and owner guidance before implementation widens

### Phase 41: Rust-owned overview, commitment flow, and orientation core

**Goal:** Make the operator’s daily overview and commitment flow genuinely backend-owned so every shell sees the same current-day truth, nudges, and intervention pressure.
**Requirements:** MVP-01, MVP-02, ORIENT-01, ORIENT-02, ORIENT-03
**Depends on:** Phase 40
**Success Criteria:**
1. `Now`/overview is one canonical Rust-owned read model for current-day truth.
2. Commitment flow is bounded, explicit, and reusable across shells without shell-local decision logic.
3. Suggestions and nudges are grounded in persisted data and explainable from Rust-owned rules.
4. Orientation output emphasizes what to do and what needs intervention instead of status clutter.
**Non-goals:**
- broad thread/tool expansion
- calendar reflow implementation beyond what the overview/commitment seam needs
- shell-specific decision logic
**Plans:** 4 plans

Plans:
- [ ] 41-01-PLAN.md — Implement the canonical overview read model and current-day truth seam
- [ ] 41-02-PLAN.md — Close the Rust-owned commitment flow and session continuity
- [ ] 41-03-PLAN.md — Normalize nudges/orientation as explainable backend-owned outputs
- [ ] 41-04-PLAN.md — Verify overview and commitment parity across transport surfaces

### Phase 42: Explainable same-day reflow and local-first calendar path

**Goal:** Make same-day reflow real, explainable, and Rust-owned, while adding the narrow local-first calendar ingestion needed to support practical local execution where possible.
**Requirements:** REFLOW-01, REFLOW-02, REFLOW-03, REFLOW-04
**Depends on:** Phase 41
**Success Criteria:**
1. Same-day reflow produces explicit scheduled, deferred, conflicted, and did-not-fit outcomes.
2. Reflow policy and proposal/apply behavior live in Rust services rather than shell code.
3. Thread escalation exists for ambiguous or review-gated reflow edits.
4. Local-first calendar input is available through a narrow adapter path without creating Apple-local planner logic.
**Non-goals:**
- multi-day planning
- broad calendar write-back
- Apple-local planner behavior
**Plans:** 4 plans

Plans:
- [ ] 42-01-PLAN.md — Tighten reflow contracts, proposal state, and provenance requirements
- [ ] 42-02-PLAN.md — Implement the bounded Rust-owned same-day reflow engine
- [ ] 42-03-PLAN.md — Add the local-first calendar input/export path for reflow
- [ ] 42-04-PLAN.md — Verify explainability, supervision, and degraded-state behavior for reflow

### Phase 43: Thread continuation, tools, context, and data

**Goal:** Make threads the bounded continuation substrate for using tools, context, and data during non-trivial daily-loop follow-through without becoming a generic chat surface.
**Requirements:** THREAD-01, THREAD-02, THREAD-03
**Depends on:** Phase 42
**Success Criteria:**
1. Threads can access current context, relevant memory, and bounded tools through the same Rust-owned authority model.
2. Tool use, data access, and follow-through preserve provenance and explicit review/apply semantics.
3. Threads act as the continuation path for messy reflow, planning, and action work rather than a second product model.
**Non-goals:**
- generic chat product work
- broad agent-platform expansion
- tool access beyond the bounded MVP continuation lane
**Plans:** 3 plans

Plans:
- [ ] 43-01-PLAN.md — Define the bounded thread continuation contract for tools/context/data
- [ ] 43-02-PLAN.md — Implement thread-backed continuation over Rust-owned context and tool surfaces
- [ ] 43-03-PLAN.md — Verify provenance, bounded capability use, and review/apply semantics in threads

### Phase 44: Minimal fresh web and Apple shells

**Goal:** Rebuild the shipped web and Apple MVP surfaces around one-screen-one-job boundaries while keeping all MVP product behavior in Rust-owned services and transport contracts.
**Requirements:** CORE-02, CORE-03, CLIENT-01, CLIENT-02, CLIENT-03
**Depends on:** Phase 43
**Success Criteria:**
1. Web presents a fresh minimal MVP UI over typed backend DTOs rather than legacy surface sprawl.
2. Apple presents a fresh minimal MVP UI over the same MVP flows without new shell-local policy.
3. Remaining shell-owned MVP logic is removed, migrated, or explicitly deprecated.
4. Both clients preserve one-screen-one-job boundaries for overview, threads, reflow, and review.
**Non-goals:**
- redesign of non-MVP surfaces
- shell-specific product experiments
- broad platform migration beyond what thin MVP shells require
**Plans:** 4 plans

Plans:
- [ ] 45-01-PLAN.md — Rework web MVP surfaces around the canonical overview/thread/review contracts
- [ ] 45-02-PLAN.md — Rework Apple MVP surfaces around the same Rust-owned flows
- [ ] 45-03-PLAN.md — Remove or deprecate remaining shell-owned MVP behavior
- [ ] 45-04-PLAN.md — Verify cross-surface parity and minimal-client boundaries

### Phase 45: Review, MVP verification, and post-MVP roadmap shaping

**Goal:** Close the MVP with lightweight but trustworthy review, verify the full operator loop, and document the future work that should follow MVP instead of widening this milestone.
**Requirements:** MVP-03, REVIEW-01, REVIEW-02, FUTURE-01, FUTURE-02
**Depends on:** Phase 44
**Success Criteria:**
1. Operators can review commitments, suggestions, reflow outcomes, and thread-mediated actions in one coherent loop.
2. The full overview -> commitments -> nudges/reflow -> threads -> review loop is verified across shipped MVP surfaces, including degraded states.
3. Review state and carry-forward behavior are persisted and explainable from terminal outcomes.
4. Remaining gaps, deferred work, and post-MVP opportunities are documented explicitly rather than quietly retained in scope.
**Non-goals:**
- new MVP-scope feature additions discovered late in the milestone
- broad polish work not required for verification or deferral clarity
**Plans:** 3 plans

Plans:
- [ ] 46-01-PLAN.md — Publish canonical review and closure contracts, then implement the review loop
- [ ] 46-02-PLAN.md — Run milestone-level MVP verification and degraded-state evidence across the full loop
- [ ] 46-03-PLAN.md — Document deferred work and the post-MVP roadmap explicitly

---
*Last updated: 2026-03-20 for milestone v0.2 true MVP planning*
