# Phase 16: Logic-first product closure on canonical core surfaces - Research

**Researched:** 2026-03-19
**Domain:** Backend-owned operator logic over the seams proven in Phase 15
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- [locked] Phase 16 is logic-first, not shell-first.
- [locked] `Now` stays urgent-first and minimal.
- [locked] `Inbox` remains the explicit triage surface.
- [locked] `Threads` remains archive/search-first and is the escalation path for longer flows.
- [locked] `Projects` remains secondary in navigation, but project-specific actions stay semantically project-owned.
- [locked] Phase 16 priority order remains: `check_in` -> `reflow` -> trust/readiness -> project-scoped actions.
- [locked] `check_in` must gain backend-owned accept/bypass/completion behavior.
- [locked] `reflow` must remain auto-suggested, user-confirmed, and severity-aware.
- [locked] Trust/readiness remains summary-first and backend-owned.
- [locked] Project-scoped actions must retain compact project identity and semantic ownership everywhere.

### Deferred Ideas (OUT OF SCOPE)
- shell/nav simplification and embodiment work
- Apple parity/FFI migration
- desktop/Tauri implementation
- broad provider expansion
- broad codex-workspace port beyond the minimum needed for canonical `reflow`
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| LOGIC-01 | Implement backend-owned operator logic rather than stopping at seam publication. | Phase 15 established the seams; Phase 16 should add real state transitions and consequences. |
| FLOW-01 | Close the main operator journeys discovered in Phase 14 with canonical backend behavior. | `check_in`, `reflow`, trust/readiness follow-through, and project-owned actions are the highest-value uncovered journeys. |
| MODE-02 | Preserve the Phase 14 mode and surface rules while adding behavior. | The logic should feed `Now`, `Inbox`, `Threads`, and `Projects` without reopening the taxonomy. |
| READMODEL-02 | Keep read models backend-owned and coherent across shells. | Existing `Now`, queue, and DTO seams are the correct composition points. |
| SHELL-ARCH-01 | Leave later shell work as embodiment rather than product-definition work. | Backend routes/services should own semantics; shells should consume typed results. |
</phase_requirements>

## Summary

Phase 15 left the codebase in the right structural shape: `check_in`, `reflow`, trust/readiness, and project-scoped action identity now exist as backend-owned seams. The remaining gap is behavioral, not architectural. `services::check_in` can currently derive a card from an active daily-loop session, but there is no canonical handling path for accept, bypass, or completion outcomes. `services::reflow` can derive a suggestion card from drift or stale schedule state, but there is no backend-owned “apply reflow” behavior that turns the suggestion into durable operator state, follow-up actions, or thread escalation. Trust/readiness can now summarize the state of backup, freshness, and review pressure, but the logic for what action should follow from a degraded posture is still thin. Project-scoped actions preserve compact identity across the transport, but project-specific behavior and routing are still mostly projection-level rather than canonical product logic.

The best Phase 16 plan is therefore not “more structure.” It is a sequence of operator-logic slices that turn Phase 15 seams into authoritative behavior:

1. lock the action-transition vocabulary and backend handling boundaries
2. implement `check_in` accept/bypass/completion behavior
3. implement `reflow` confirm/apply/edit consequences
4. implement trust/readiness follow-through actions and routing
5. implement project-scoped action behavior and typed escalation semantics

The strongest code anchors are already present:

- `crates/veld/src/services/check_in.rs` for current card derivation
- `crates/veld/src/services/reflow.rs` for current suggestion derivation
- `crates/veld/src/services/daily_loop.rs` for morning/standup-linked state transitions
- `crates/veld/src/services/operator_queue.rs` for canonical queue synthesis
- `crates/veld/src/services/now.rs` for cross-surface read-model composition
- `crates/veld/src/routes/threads.rs` and existing thread persistence for escalation targets

That means Phase 16 should extend existing services and routes rather than inventing a separate “operator engine” unless a narrowly scoped helper clearly reduces duplication.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core` | workspace `0.1.0` | Canonical action/check-in/reflow/project vocabulary | Already owns the operator-action contract and should remain the semantic authority |
| `veld` services | workspace `0.1.0` | Backend application logic | Current home for `check_in`, `reflow`, queue composition, daily loop, and trust projection |
| `vel-api-types` | workspace `0.1.0` | Transport DTO mapping | Keeps new logic consumable by web/Apple/CLI without shell-owned policy |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `vel-storage` | workspace `0.1.0` | Persist durable outcomes and state transitions | Use when `check_in` / `reflow` / project actions need durable consequences |
| Existing `threads` routes/storage | current | Escalation target and durable continuity lane | Use when backend logic must create or reference thread-linked follow-up work |
| Existing daily-loop services/routes | current | Morning/standup-linked `check_in` and schedule logic | Use for the first real check-in and reflow consequences |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extend current services and typed seams | Build a new broad operator-orchestration subsystem first | More abstraction up front, but higher risk of duplicating semantics and stalling delivery |
| Typed backend transitions for `check_in` and `reflow` | Shell-triggered interpretation of cards | Faster locally, but would violate the logic-first goal and reintroduce drift |
| Compose trust follow-through over current backup/freshness/review inputs | Invent a new isolated trust pipeline | More code churn without better product semantics |
| Typed thread escalation consequences | Keep `Threads` escalation as forever-metadata only | Leaves Phase 16 logic closure incomplete and pushes product behavior into Phase 17 |

## Architecture Patterns

### Pattern 1: From Suggestion Card To Durable Outcome
**What:** Phase 15 produced typed `check_in` / `reflow` cards; Phase 16 should add explicit backend handlers that turn those into real state changes or follow-up actions.
**When to use:** Any action currently represented only as a read-model card.
**Example:** `check_in` from a daily-loop prompt should be answerable/bypassable through one backend-owned path that updates session state and emits the next canonical read model.

### Pattern 2: Daily-Loop-Adjacent Logic Before Generic Planner Logic
**What:** Use the current daily-loop and current-context seams as the first place where logic becomes real.
**When to use:** `check_in` and `reflow`.
**Example:** `reflow` confirm/apply can first update daily-loop-adjacent state or generate follow-up actions, instead of trying to land a universal scheduler in one phase.

### Pattern 3: Queue Synthesis As The Canonical Action Router
**What:** Action consequences should feed the canonical queue/read-model seams rather than bypassing them.
**When to use:** trust follow-through actions, project review/reflow/status items, and post-check-in/post-reflow follow-up work.
**Example:** degraded readiness may yield explicit review/recovery items that surface through the queue with preserved ownership.

### Pattern 4: Typed Escalation, Not Shell-Local Routing
**What:** Escalation to `Threads` should remain a typed backend decision with durable references when the interaction becomes multi-step.
**When to use:** `check_in` escalation, `reflow` edit flows, and project-scoped longer-form work.
**Example:** a `reflow` edit path should resolve to a thread-linked follow-up contract or durable reference, not a frontend-only navigation guess.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| `check_in` handling | ad hoc route-local branching over raw form fields | typed service-owned check-in lifecycle over existing daily-loop/session seams | Preserves product semantics and reviewability |
| `reflow` behavior | generic scheduler rewrite | typed apply/edit consequences anchored in current-context + daily-loop seams | Keeps scope realistic and phase-aligned |
| Trust follow-through | shell-specific badges with separate business rules | backend-owned readiness projection plus explicit follow-up actions | Keeps trust posture coherent across shells |
| Thread escalation | “open threads UI” as a frontend convention | typed escalation metadata and durable backend references | Lets later shells embody the same logic |
| Project-specific behavior | global queue items with project color added later | project-owned actions with explicit routing and identity from the backend | Keeps project semantics durable |

## Common Pitfalls

### Pitfall 1: Treating Phase 16 As A UI Cleanup Phase
**What goes wrong:** Plans drift into cards, menus, nav, or screen ownership instead of backend behavior.
**How to avoid:** Keep the slice boundaries centered on commands, state transitions, and read-model consequences.

### Pitfall 2: Stopping At Cards Again
**What goes wrong:** The phase widens DTOs and card metadata but still leaves no backend-owned accept/apply/bypass behavior.
**How to avoid:** Every Phase 16 slice should answer “what durable behavior happens next?”

### Pitfall 3: Reflow Becomes A Vague Planner Rewrite
**What goes wrong:** The phase expands into a whole scheduling system and stops being executable.
**How to avoid:** Keep `reflow` anchored to concrete typed triggers, confirm/apply behavior, and explicit leftovers/follow-up actions.

### Pitfall 4: Project Ownership Gets Lost In Shared Routing
**What goes wrong:** Project review/reflow/status actions become generic global queue items again once behavior is added.
**How to avoid:** Treat project ownership as semantic, not cosmetic, and keep it explicit in service/DTO/read-model flows.

## Recommended Execution Shape

- **16-01:** Ratify the canonical operator-action transition contract and logic entry points before behavior widens
- **16-02:** Implement backend-owned `check_in` accept/bypass/completion behavior over daily-loop/session seams
- **16-03:** Implement backend-owned `reflow` confirm/apply/edit behavior and follow-up state generation
- **16-04:** Tighten trust/readiness follow-through so degraded posture yields canonical backend-owned recovery/review actions
- **16-05:** Close project-scoped action behavior and typed thread escalation/routing without reopening shell debates

---

*Phase: 16-logic-first-product-closure-on-canonical-core-surfaces*
*Research completed: 2026-03-19*
