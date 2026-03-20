# Domain Pitfalls

**Domain:** true MVP closure for Vel v0.2 (Rust-core-first with fresh thin clients)
**Researched:** 2026-03-20

## Critical Pitfalls

### Pitfall 1: Calling the existing surface area the MVP
**What goes wrong:** The milestone inherits too much of the already-built product and treats "make current features cleaner" as MVP closure.
**Why it happens:** Brownfield teams anchor on shipped capability instead of the smallest loop that creates repeated daily value.
**Consequences:** The milestone turns into cleanup plus parity work, Phase 40 gets absorbed without narrowing the product, and the team ships another broad beta instead of a crisp daily-use MVP.
**Prevention:** Freeze the v0.2 MVP around one daily loop only: morning overview, commitment choice, suggestion/nudge handling, same-day reflow, thread-backed follow-through, and review. Everything else must justify itself as necessary to that loop. Write an explicit "not in MVP" list before implementation starts.
**Detection:** Requirements keep mentioning general assistant, broader project management, richer memory, multi-day planning, or automation breadth without tying them to the five operator flows above.

### Pitfall 2: Re-platforming everything instead of extracting only canonical product logic
**What goes wrong:** "Rust-owned" becomes a full migration mandate, so teams try to move every legacy path, UI behavior, and integration abstraction before shipping the MVP loop.
**Why it happens:** Architecture cleanup feels safer than choosing which behaviors are essential.
**Consequences:** The core migration stalls, clients remain half-old/half-new, and the MVP misses the chance to prove a stable product seam.
**Prevention:** Extract only the canonical commands, queries, and typed read models needed for the MVP flows. Leave non-MVP surfaces behind feature gates or legacy boundaries. Define migration completion per flow, not per module.
**Detection:** Plans are organized by subsystem rewrites instead of operator outcomes, or success criteria talk about "all core logic moved" without naming the user-visible loop it unblocks.

### Pitfall 3: Thin clients in name only
**What goes wrong:** Web and Apple get fresh UI shells, but decision logic, fallback rules, local caching policy, or derived state still lives in TypeScript/Swift.
**Why it happens:** New UI work often reintroduces convenience logic to make screens feel responsive before backend seams are fully shaped.
**Consequences:** Cross-surface drift returns, Apple and web disagree on suggestions/reflow/review state, and the Rust core stops being the product truth.
**Prevention:** Treat clients as rendering + input + optimistic UX only. All suggestion generation, reflow decisions, thread/tool policy, review state, and current-day truth must come from typed Rust APIs. Require shared fixtures for web and Apple against the same backend payloads.
**Detection:** Client PRs introduce business-rule conditionals, status synthesis, or duplicate filtering/sorting beyond presentation concerns.

### Pitfall 4: Shipping a fresh UI before locking the canonical decision model
**What goes wrong:** The new shells are designed around screens and cards before the MVP's operator decisions are formalized.
**Why it happens:** Phase 40 creates pressure to redesign navigation and presentation first.
**Consequences:** The UI looks cleaner but still forces the operator to interpret too many states, and backend contracts get bent to fit visual layouts instead of decisions.
**Prevention:** Lock the decision model first: "what needs commitment," "what needs intervention," "what can be deferred," "what can be reflowed," and "what must be reviewed." Build the UI hierarchy from those decisions, not from current information architecture.
**Detection:** Mockups or plans lead with page taxonomy, component trees, or dashboard sections rather than decision verbs and dominant actions.

### Pitfall 5: Treating local-first reflow as a planner rewrite
**What goes wrong:** Calendar reflow expands from same-day repair into a broad local scheduling engine.
**Why it happens:** Reflow touches commitments, routines, calendars, nudges, and suggestions, so it is easy to widen from "repair today" into "solve planning."
**Consequences:** The team reopens already-closed planner complexity, loses explainability, and delays the one thing the MVP actually needs: reliable same-day recovery.
**Prevention:** Bound reflow to same-day repair over persisted commitments, explicit constraints, and local calendar state. No speculative multi-day optimization, no hidden automatic rescheduling, no new planning ontology unless required by same-day reflow.
**Detection:** Proposed rules mention week planning, goal balancing, automatic routine generation, or opaque score-based schedule optimization.

### Pitfall 6: Underestimating local-first conflict and provenance requirements
**What goes wrong:** Reflow, suggestions, and tool actions mutate local state or upstream-linked state without a clear provenance trail and reversible review path.
**Why it happens:** MVP pressure encourages "just apply the result" because the app is local-first and single-user.
**Consequences:** The operator loses trust, review becomes post-hoc guesswork, and any sync or upstream reconciliation later becomes harder.
**Prevention:** Every reflow, nudge, tool proposal, and applied action needs persisted inputs, rule basis, run/thread IDs, and terminal state. Keep review artifacts first-class even in MVP scope.
**Detection:** You can see the current plan or action outcome, but not which inputs, rules, or thread/tool invocation produced it.

## Moderate Pitfalls

### Pitfall 1: Making threads a generic chat surface instead of a supervised work context
**What goes wrong:** Threads become a broad conversational bucket for anything the system cannot place cleanly.
**Prevention:** Limit MVP threads to context-bearing follow-through: explain a suggestion, gather tool/context inputs, stage a reflow or action, and support review. Keep generic assistant breadth out of scope.

### Pitfall 2: Overloading the review flow with analytics, journaling, and retrospective ambition
**What goes wrong:** Review tries to become a rich life-logging or performance-analysis surface.
**Prevention:** Keep review narrow: what was committed, what changed, what slipped, what was applied, and what needs carrying forward. Anything beyond that belongs after MVP.

### Pitfall 3: Assuming "local if possible" means provider-free
**What goes wrong:** Calendar reflow or tooling plans assume local-only access even where current integrations still require provider APIs, permissions, or inconsistent device stores.
**Prevention:** Separate "local computation" from "local authority." Compute reflow locally where possible, but model source authority, permissions, freshness, and fallback explicitly per provider or device boundary.

### Pitfall 4: Keeping JSON blobs as the escape hatch for new MVP concepts
**What goes wrong:** New MVP entities for overview state, reflow proposals, or review packets are shoved back into untyped JSON to move quickly.
**Prevention:** Use typed versioned Rust structs at the domain edge and serialize only at storage/API boundaries. Brownfield speed is not a reason to regress the architecture.

### Pitfall 5: Letting broad tool access sneak back in through the assistant path
**What goes wrong:** Because MVP needs threads plus tools/context/data, subordinate tooling expands beyond the exact flows under supervision.
**Prevention:** Keep explicit allowlists: calendar read/reflow inputs, thread context, bounded review artifacts, and approved action lanes only. Unknown tool requests should fail closed.

## Minor Pitfalls

### Pitfall 1: Measuring progress by migrated screens rather than completed operator loops
**What goes wrong:** The milestone looks healthy because web and Apple have new surfaces, but the end-to-end daily loop is still broken.
**Prevention:** Track completion by flow rehearsal: overview -> commitment -> nudge/reflow -> thread/tool follow-through -> review.

### Pitfall 2: Fresh-shell polish masking broken degraded states
**What goes wrong:** Happy-path screenshots improve while stale sync, missing permissions, failed tool calls, or offline edges remain confusing.
**Prevention:** Make degraded states part of the MVP acceptance criteria for both web and Apple.

### Pitfall 3: Reusing old labels that no longer match MVP semantics
**What goes wrong:** Terms like `Now`, `Inbox`, `thread`, `suggestion`, or `review` carry old meaning into the new milestone and confuse planning.
**Prevention:** Rewrite the vocabulary contract for v0.2 up front and use it consistently in APIs, UI copy, and roadmap plans.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| MVP definition / milestone framing | Scope collapse into "improve everything already shipped" | Start with a strict operator-loop contract and an explicit out-of-scope list; make every plan map to one of the five MVP flows |
| Decision model / contracts | UI-first redesign before product decisions are typed | Publish canonical decision/read-model contracts before web or Apple redesign starts |
| Rust-core extraction | Full-system migration instead of flow-scoped migration | Extract only commands/queries needed for overview, commitments, nudges, reflow, threads, and review |
| Web shell refresh | Client-side policy sneaks back in via convenience logic | Ban product-rule synthesis in TypeScript; require backend-owned derived state and contract fixtures |
| Apple thin client refresh | Swift reimplements backend decisions for offline UX | Keep Apple to rendering, capture, caching, and transport; move policy and explanation generation to Rust |
| Daily overview / commitment flow | Overview becomes dashboard-heavy instead of decision-first | One dominant next action, bounded supporting context, explicit deferral path, and visible rationale |
| Suggestions / nudges | Suggestion system widens into general assistant behavior | Restrict MVP nudges to current-day intervention and commitment follow-through with provenance |
| Calendar reflow | Same-day repair expands into speculative planner rewrite | Limit to explainable same-day reconciliation over explicit constraints and current calendar truth |
| Threads + tools/context/data | Generic chat and broad tools widen scope/security surface | Keep threads task-bounded and tools allowlisted; no ambient access expansion |
| Review | Review grows into analytics/journaling product | Limit output to commitments, changes, slips, applied actions, and carry-forward items |
| Verification | Phase 40 visual work closes without execution-backed loop checks | Require cross-surface end-to-end rehearsals plus degraded-state checks for each MVP flow |

## Sources

- Internal authority: [.planning/PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md), [.planning/ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md), [.planning/STATE.md](/home/jove/code/vel/.planning/STATE.md), [docs/MASTER_PLAN.md](/home/jove/code/vel/docs/MASTER_PLAN.md), [.planning/codebase/CONCERNS.md](/home/jove/code/vel/.planning/codebase/CONCERNS.md)
- Ink & Switch, "Local-first software" (used for local-first trust/conflict/provenance pressure; HIGH confidence): https://www.inkandswitch.com/local-first/
- SQLite documentation on WAL and concurrency limits (used for local-first single-writer/reconciliation caution; HIGH confidence): https://sqlite.org/wal.html
- Silicon Valley Product Group, product outcome vs feature-delivery framing (used for MVP scope-discipline caution; MEDIUM confidence): https://www.svpg.com/the-product-model-in-traditional-it/

