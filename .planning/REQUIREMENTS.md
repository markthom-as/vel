# Requirements: Vel

**Defined:** 2026-03-20
**Milestone:** v0.2
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust.

## v0.2 Requirements

Requirements for the true-MVP milestone. This milestone absorbs the previously planned Phase 40 work, narrows the shipped product to one strict daily operator loop, pushes all MVP logic into platform-portable Rust core layers, and refreshes web and Apple as thin shells over that core.

## MVP Acceptance Checklist

v0.2 is only complete if all of these are true:

- [x] the shipped product has one clear loop: `overview -> commitments -> reflow -> threads -> review`
- [x] every step in that loop is backed by Rust-owned contracts and business logic
- [x] web and Apple are thin clients over the same MVP loop
- [x] same-day reflow stays bounded, explainable, and current-day only
- [x] thread/tool use stays bounded and supervised rather than turning into generic chat
- [x] all major non-MVP work is explicitly deferred instead of remaining implied in scope

## Non-Goals

- Multi-day planning, broader planner redesign, or autonomous planning
- Broad provider/integration expansion
- Broad shell redesign outside the MVP loop surfaces
- Broad Apple FFI migration or Apple-local planner logic
- Broad calendar write-back automation
- New shell-owned product logic or fallback rules
- Generic assistant/chat product expansion beyond bounded MVP continuation

### MVP Definition

- [x] **MVP-01**: Operator can start the day with a reliable daily overview over today’s real schedule, commitments, and active pressure
- [x] **MVP-02**: Operator can complete a bounded commitment flow that ends with a small explicit set of commitments for the day
- [x] **MVP-03**: Operator can review the current day later and see what changed, what slipped, and what still needs attention
- [x] **MVP-04**: MVP scope is defined explicitly enough that non-MVP future work is deferred instead of leaking into implementation

### Canonical Types, Logic, And Docs

- [x] **MODEL-01**: MVP data types are explicitly defined as canonical Rust-owned domain and read-model contracts
- [x] **MODEL-02**: MVP business logic is documented clearly enough that shells, tests, and future phases share the same behavioral truth
- [x] **MODEL-03**: Architecture docs are refined to describe the real MVP seams, authority boundaries, and cross-surface contracts
- [x] **MODEL-04**: Stable MVP-facing contracts ship with close-by docs, examples, templates, or schemas where appropriate

### Orientation, Suggestions, Nudges

- [x] **ORIENT-01**: Vel can orient the operator using grounded suggestions and nudges derived from persisted context, schedule state, commitments, and thread/history evidence
- [x] **ORIENT-02**: Suggestions and nudges are explainable from persisted inputs and Rust-owned rules rather than shell-local heuristics
- [x] **ORIENT-03**: Orientation surfaces emphasize decision-making and intervention pressure instead of dashboard clutter

### Calendar Reflow

- [x] **REFLOW-01**: Operator can run same-day calendar reflow against real commitments and remaining-day constraints
- [x] **REFLOW-02**: Reflow results show scheduled, deferred, conflicted, and did-not-fit outcomes clearly
- [x] **REFLOW-03**: Reflow behavior is backend-owned, typed, and consistent across web, Apple, and thread flows
- [x] **REFLOW-04**: Reflow inputs, decisions, and degraded states are documented clearly enough that same-day behavior stays explainable without shell-local policy

### Threads, Tools, Context, Data

- [x] **THREAD-01**: Threads are a first-class path for using tools, context, and project/data recall against the same Rust-owned runtime authority
- [x] **THREAD-02**: Thread actions preserve provenance, bounded tool use, and explicit review/apply semantics where required
- [x] **THREAD-03**: Thread flows can access current context, relevant memory, and bounded tools without inventing shell-owned behavior

### Review

- [x] **REVIEW-01**: Operator can review commitments, suggestions, reflow outcomes, and thread-mediated actions in one coherent MVP loop
- [x] **REVIEW-02**: Review surfaces preserve explainability, provenance, and terminal state instead of summary-only claims

### Rust Core Portability

- [x] **CORE-01**: All MVP product logic lives in platform-portable Rust core and service layers rather than web- or Apple-local policy
- [x] **CORE-02**: Web and Apple consume shared Rust-owned commands, queries, read models, and transport contracts for MVP behavior
- [x] **CORE-03**: Remaining shell-owned MVP behavior is removed, migrated, or explicitly deprecated during this milestone

### Minimal Fresh Clients

- [x] **CLIENT-01**: Web ships a fresh minimal MVP UI focused on action, orientation, threads, reflow, and review instead of legacy surface sprawl
- [x] **CLIENT-02**: Apple ships a fresh minimal MVP UI over the same Rust-owned MVP flows
- [x] **CLIENT-03**: Client surfaces keep one-screen-one-job boundaries and do not reintroduce local product authority

### Future Work Boundary

- [x] **FUTURE-01**: The milestone produces an explicit future roadmap for post-MVP work that is out of scope for v0.2
- [x] **FUTURE-02**: Deferred work is documented clearly enough that v0.2 can stay minimal without ambiguity

## Future Requirements

- [ ] Multi-day or autonomous planning beyond same-day repair
- [ ] Broad provider expansion, new sync stacks, or new databases
- [ ] Broad FFI migration or Apple-local planner logic
- [ ] Chat-first shell redesign or generic tool-chat product direction
- [ ] Heavy analytics, scoring, journaling, or routine-builder redesign

## Out of Scope

- Broad new cloud/platform architecture
  Reason: v0.2 is product closure over the existing local-first Rust authority model, not a platform reset.
- Broad calendar write-back automation
  Reason: same-day reflow should stay explainable, bounded, and supervised in MVP.
- New shell-owned business logic
  Reason: the milestone exists to remove policy drift, not move it behind a cleaner UI.
- Major provider or integration expansion
  Reason: v0.2 should verify one strict daily loop before widening ecosystem scope.
- Multi-day or autonomous planning
  Reason: v0.2 is explicitly limited to current-day operator behavior.
- Broad UI cleanup outside the MVP loop
  Reason: fresh client work should only cover the loop surfaces needed for the true MVP.

## Traceability

| Requirement | Phase |
|-------------|-------|
| MVP-01 | Phase 41 |
| MVP-02 | Phase 41 |
| MVP-03 | Phase 45 |
| MVP-04 | Phase 40 |
| MODEL-01 | Phase 40 |
| MODEL-02 | Phase 40 |
| MODEL-03 | Phase 40 |
| MODEL-04 | Phase 40 |
| ORIENT-01 | Phase 41 |
| ORIENT-02 | Phase 41 |
| ORIENT-03 | Phase 41 |
| REFLOW-01 | Phase 42 |
| REFLOW-02 | Phase 42 |
| REFLOW-03 | Phase 42 |
| REFLOW-04 | Phase 42 |
| THREAD-01 | Phase 43 |
| THREAD-02 | Phase 43 |
| THREAD-03 | Phase 43 |
| REVIEW-01 | Phase 45 |
| REVIEW-02 | Phase 45 |
| CORE-01 | Phase 40 |
| CORE-02 | Phase 44 |
| CORE-03 | Phase 44 |
| CLIENT-01 | Phase 44 |
| CLIENT-02 | Phase 44 |
| CLIENT-03 | Phase 44 |
| FUTURE-01 | Phase 45 |
| FUTURE-02 | Phase 45 |

---
*Last updated: 2026-03-21 after Phase 45 verification and post-MVP roadmap closeout*
