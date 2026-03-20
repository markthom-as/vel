# Phase 15: Incremental core migration and canonical Rust service seams - Research

**Researched:** 2026-03-19
**Domain:** Minimal migration work to support backend-owned operator actions, `check_in`, `reflow`, trust/readiness summaries, and project-scoped actions
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- [locked] Phase 15 is migration-focused, not a broad product-logic phase.
- [locked] Use the minimum structural work needed so later logic lands in the right seams.
- [locked] Priority order is `check_in` -> `reflow` -> trust/readiness -> project-scoped actions.
- [locked] `Now` stays minimal and urgent-first.
- [locked] `Inbox` is the explicit triage surface.
- [locked] `Threads` stays archive/search-first and only escalates longer flows.
- [locked] `check_in` defaults to inline `Now` cards and escalates to `Threads` only when needed.
- [locked] `reflow` is heavier, auto-suggested, user-confirmed, and should start from compact preview plus `Accept` / `Edit`.
- [locked] Non-urgent items should remain summarized rather than bloating `Now`.
- [locked] Project-scoped work should retain visible project identity wherever surfaced.

### Deferred Ideas (OUT OF SCOPE)
- shell simplification and navigation cleanup
- broad Phase 16 product logic
- Apple FFI migration
- Tauri/desktop implementation
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| MIGRATE-01 | Migrate the smallest set of core/storage/service seams needed for Phase 16 logic to land canonically. | Current backend already exposes partial operator seams (`operator_queue`, `Now`, daily loop, backup, projects); migration should tighten and connect them rather than replatform them. |
| MIGRATE-02 | Reduce shell-owned or route-owned interpretation by making operator action/readiness ownership explicit in Rust layers. | Existing routes are mostly thin, but product semantics remain scattered across services and DTOs. |
| SERVICE-01 | Establish backend application-service seams for `check_in`, `reflow`, trust/readiness, and project-scoped actions. | `services::operator_queue`, `services::now`, `services::daily_loop`, `services::backup`, `services::doctor`, and `services::projects` are the nearest existing insertion points. |
| DTO-01 | Tighten transport boundary discipline so new operator seams map cleanly into `vel-api-types` instead of leaking ad hoc JSON outward. | `vel-api-types` already owns `NowData`, `ActionItemData`, `ReviewSnapshotData`, `BackupStatusData`, and project DTOs. |
| READMODEL-01 | Define read-model ownership so cross-surface consumers share backend semantics rather than reassembling them locally. | `NowOutput` is already a cross-cutting read model, but it needs clearer composition and future-ready ownership boundaries. |
</phase_requirements>

## Summary

The repo already has the beginnings of a canonical operator-action system, but the important pieces do not yet compose into one backend-owned operator model. `vel-core::operator_queue` provides a useful baseline with `ActionItem`, `ActionKind`, `ActionSurface`, `ActionState`, and `ReviewSnapshot`, while `services::operator_queue` already synthesizes the current queue from freshness, linking, interventions, projects, commitments, execution handoffs, writebacks, and conflicts. That is the right seam to evolve, but it is still rebuild-on-read and too narrow for the full Phase 14 contract. `services::now` aggregates a broad snapshot that includes action items, freshness, pending writebacks, conflicts, and people, but it does not yet distinguish `check_in`, `reflow`, trust/readiness summaries, or project-scoped action semantics explicitly. `services::daily_loop` already owns bounded daily-loop session state and is the best nearby seam for future schedule-drift/reflow logic. `services::backup` and `services::doctor` already provide part of the trust/readiness story. `services::projects` already gives project identity and typed project transport.

That means the right plan is not to invent a brand-new operator stack all at once. The right plan is to evolve the existing operator queue baseline, introduce focused service seams where semantics are missing, and map those through current route and DTO boundaries. The migration should be proof-bearing and ordered by product priority:

1. establish the contract and migration map for canonical operator action ownership
2. add a real `check_in` seam
3. add a real `reflow` seam
4. add a summary-first trust/readiness seam that composes existing sources
5. add a project-scoped action seam that preserves project identity without flattening it

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core::operator_queue` | workspace `0.1.0` | Existing action/review domain baseline | Best current place to evolve canonical operator action vocabulary |
| `vel-core::daily_loop` | workspace `0.1.0` | Daily-loop domain types | Best current place to anchor future reflow-related planning semantics |
| `vel-core::project` | workspace `0.1.0` | Project identity/status semantics | Preserves project-scoped ownership without UI coupling |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `vel-api-types` | workspace `0.1.0` | Transport DTO boundary | Map any new operator/readiness types here only after core/service seams are clear |
| `vel-storage` repositories | workspace `0.1.0` | Persistence and query seams | Extend focused repositories instead of dumping new behavior into `db.rs` |
| `services::operator_queue` | current | Current synthesized backend action queue | Best current place to evolve canonical queue/action semantics before broader persistence or projection changes |
| `services::now` | current | Broad operator read-model assembly | Likely first read-model consumer/composition point for new action/readiness seams |
| `services::backup` | current | Real trust evidence source | Reuse instead of inventing parallel trust state |
| `services::doctor` | current | Existing backup trust classification | Reuse for readiness composition rather than duplicating trust logic |
| `services::daily_loop` | current | Existing bounded daily planning seam | Reuse for drift/reflow input gathering and projections |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Evolve `operator_queue` and nearby services | Build a new operator subsystem from scratch | Cleaner on paper, but more migration risk and duplicate semantics |
| Introduce focused services plus existing route reuse | Add one giant `/v1/operator` endpoint immediately | Premature aggregation and likely mixed concerns |
| Compose trust/readiness from existing backup/freshness/review sources | Build a new isolated trust subsystem | Unnecessary duplication and more storage/DTO churn |
| Add project-scoped action semantics through existing project/action types | Treat everything as global inbox items | Loses project ownership and contradicts Phase 14 decisions |

## Architecture Patterns

### Pattern 1: Evolve Existing Domain Baselines Instead Of Replacing Them
**What:** Extend `operator_queue` and nearby domain types before introducing parallel abstractions.
**When to use:** `check_in`, `reflow`, readiness, and project-scoped action semantics.
**Example:** add the dimensions and variants Phase 14 requires rather than bolting those meanings onto route-layer JSON.

### Pattern 2: Evolve The Queue Service, Then Compose Read Models On Top
**What:** Keep canonical action semantics in domain + queue services, then let `Now` and other read models consume that result.
**When to use:** `check_in`, `reflow`, project-scoped actions, and review counts.
**Example:** `services::operator_queue::build_action_items` already synthesizes action items; later `Now` mapping should consume the evolved shape instead of rediscovering it.

### Pattern 3: Reuse Bounded Product Flows As Seams For Future Logic
**What:** Build future logic where the bounded flow already exists.
**When to use:** `reflow` and schedule-drift work.
**Example:** `services::daily_loop` already has session state and prompt transitions, making it the best place to anchor future recalculation inputs rather than creating a separate planner stack immediately.

### Pattern 4: Keep Semantic Ownership Distinct From Surface Affinity
**What:** An action can surface in `Now` or `Inbox` while remaining globally, project-, thread-, or daily-loop-scoped.
**When to use:** project-scoped actions and thread escalation.
**Example:** Phase 14’s project tag/color rule should be backed by core-owned scope metadata, not shell-only tagging.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| New operator action semantics | route-local JSON blobs | `vel-core` types + service-layer composition + DTO mapping | Preserves cross-surface ownership |
| Queue ownership | brand-new queue abstraction that ignores current `operator_queue` | evolve `operator_queue` into the canonical migration seam | Reduces churn and uses real current code |
| Trust/readiness | isolated trust subsystem | composition over backup/freshness/review/linking evidence | Reuses real existing sources |
| Reflow planning | separate generic planner immediately | daily-loop-adjacent service seam with explicit drift inputs | Keeps migration incremental |
| Project identity in actions | frontend-only labels | core/service-owned project-scoped action metadata | Prevents shell-local drift |

## Common Pitfalls

### Pitfall 1: Turning Phase 15 Into Phase 16
**What goes wrong:** Migration work widens into final product behavior, making the phase too broad and unstable.
**How to avoid:** Keep this phase focused on seams, types, service ownership, and bounded projections.

### Pitfall 2: Adding New DTOs Before Core Ownership Is Clear
**What goes wrong:** Transport shapes become the de facto product model.
**How to avoid:** Start from `vel-core` and service semantics, then map outward.

### Pitfall 3: Treating `Now` Or Chat Inbox As The Whole Operator System
**What goes wrong:** `Now` becomes a dumping ground or intervention/chat evidence becomes the accidental canonical action layer.
**How to avoid:** Use `operator_queue` as the main queue/action seam and keep `Now` and chat inbox as consumers or projections.

### Pitfall 4: Flattening Project Actions Into Generic Global Queue Items
**What goes wrong:** Project ownership and filtering context disappear.
**How to avoid:** Carry project scope and identity through core/service/DTO seams.

## Recommended Execution Shape

- **15-01:** publish the migration map and tighten canonical operator action/read-model ownership boundaries around `operator_queue`
- **15-02:** introduce the first backend-owned `check_in` seam over the existing action/read-model path
- **15-03:** introduce the first backend-owned `reflow` seam and drift/readiness inputs near daily-loop ownership
- **15-04:** compose summary-first trust/readiness projections from existing backup/freshness/review sources
- **15-05:** preserve project-scoped action ownership through core/storage/service/DTO seams and align docs/tests

---

*Phase: 15-incremental-core-migration-and-canonical-rust-service-seams*
*Research completed: 2026-03-19*
