# Vel Architecture Inventory

Status: Current architecture alignment inventory  
Audience: maintainers, coding agents  
Created: 2026-03-17  
Updated: 2026-03-17  
Purpose: inventory the current documentation and runtime architecture, identify drift and contradictions, and define the concrete seams to augment, simplify, split, or remove before broader decomposition.

## 1. What This Document Is

This document is a current-state inventory for architecture alignment work.

It is not the canonical shipped-behavior ledger. For shipped behavior, use [status.md](status.md).

It is also not a speculative architecture spec. Future-shape design remains in [specs/](specs/) and product/spec packs.

For the future-facing planned-system synthesis, use [docs/future-architecture-map.md](future-architecture-map.md).

Use this document when the task is:

- architecture cleanup
- boundary auditing
- module extraction planning
- doc drift analysis
- deciding what to simplify before adding more breadth

## 2. Documentation Inventory

### 2.1 Current truth

These are the highest-authority operational docs:

- [README.md](../README.md): strict repo/operator entrypoint
- [docs/README.md](README.md): documentation navigation and authority rules
- [docs/status.md](status.md): canonical implementation ledger
- [docs/api/README.md](api/README.md), [docs/api/runtime.md](api/runtime.md), [docs/api/chat.md](api/chat.md): API truth surfaces
- [docs/user/README.md](user/README.md): user/operator-facing entrypoint

### 2.2 Active convergence / execution

These are the main near-term execution packs:

- [docs/tickets/README.md](tickets/README.md): ticket-pack triage index
- [docs/tickets/repo-audit-hardening/README.md](tickets/repo-audit-hardening/README.md): current convergence lane for repo truth, hermeticity, and decomposition planning
- [docs/specs/vel-repo-audit-hardening-spec.md](specs/vel-repo-audit-hardening-spec.md): rationale and acceptance criteria for the current hardening program
- [docs/tickets/repo-feedback/README.md](tickets/repo-feedback/README.md): older but still active convergence packet for architecture cleanup

### 2.3 Design and planned architecture

These describe target shape or directional design, not shipped truth:

- [docs/architecture.md](architecture.md)
- [docs/product-spec.md](product-spec.md)
- [docs/mvp.md](mvp.md)
- [docs/specs/vel-system-architecture-map.md](specs/vel-system-architecture-map.md)
- [docs/specs/vel-architecture-for-agents.md](specs/vel-architecture-for-agents.md)
- [docs/specs/vel-cluster-sync-spec.md](specs/vel-cluster-sync-spec.md)
- [docs/specs/vel-multi-client-swarm-spec.md](specs/vel-multi-client-swarm-spec.md)

### 2.4 Historical and contextual

Useful, but non-authoritative for shipped behavior:

- [docs/reviews/README.md](reviews/README.md) and review-round docs
- product-spec packs
- older feedback packs and imported spec packs

## 3. Documentation Drift, Conflict, and Age

### 3.1 Freshest surfaces

The repo is in a live convergence pass, not a settled architecture state.

Freshest authority and convergence surfaces by recent commit date:

- `2026-03-17`: [docs/README.md](README.md), [docs/status.md](status.md), [README.md](../README.md)
- `2026-03-17`: [docs/specs/vel-repo-audit-hardening-spec.md](specs/vel-repo-audit-hardening-spec.md)
- `2026-03-17`: [docs/tickets/repo-audit-hardening/README.md](tickets/repo-audit-hardening/README.md)
- `2026-03-17`: [docs/tickets/repo-audit-hardening/004-architecture-map-and-module-boundary-audit.md](tickets/repo-audit-hardening/004-architecture-map-and-module-boundary-audit.md)
- `2026-03-17`: cluster/sync/swarm docs under [docs/specs/vel-cluster-sync-spec.md](specs/vel-cluster-sync-spec.md), [docs/specs/vel-multi-client-swarm-spec.md](specs/vel-multi-client-swarm-spec.md), and [docs/tickets/multi-client-swarm/](tickets/multi-client-swarm/)

Older foundational docs still in heavy use:

- `2026-03-14`: [docs/product-spec.md](product-spec.md)
- `2026-03-14`: [docs/architecture.md](architecture.md)
- `2026-03-14`: [docs/data-model.md](data-model.md)
- `2026-03-14`: [docs/mvp.md](mvp.md)

### 3.2 Highest-signal doc contradictions

1. [docs/specs/vel-architecture-for-agents.md](specs/vel-architecture-for-agents.md) is stale by content.
It describes crates and modules that do not exist in the repo today, including `vel-api`, `vel-signals`, `vel-context`, `vel-risk`, `vel-nudges`, `vel-threads`, and `vel-synthesis`.

2. [docs/architecture.md](architecture.md) still speaks in broad distributed-system language.
That is directionally useful, but it conflicts with the stricter present-tense framing in [docs/specs/vel-system-architecture-map.md](specs/vel-system-architecture-map.md), which explicitly says Vel is not a multi-master distributed cluster.

3. [docs/mvp.md](mvp.md) now functions more as historical scope intent than current truth.
It still frames calendar automation and health ingestion as not required for v0, while [docs/status.md](status.md) says multiple ingestion adapters are already implemented.

4. [docs/vel-documentation-index-and-implementation-status.md](vel-documentation-index-and-implementation-status.md) still carries residual drift.
Examples:
- it still references the chat ticket pack as `001-035` even though the repo has `001-037`
- it omits some newer packs that now appear in [docs/tickets/README.md](tickets/README.md)

5. There is active overlap between [docs/tickets/repo-feedback/README.md](tickets/repo-feedback/README.md) and [docs/tickets/repo-audit-hardening/README.md](tickets/repo-audit-hardening/README.md).
Both are convergence packets. The hardening pack is the better normalized primary lane for current architecture-alignment work.

### 3.3 In-flight conflict zone

The live worktree currently shows a concentrated cluster of uncommitted changes in:

- [docs/status.md](status.md)
- [docs/specs/vel-cluster-sync-spec.md](specs/vel-cluster-sync-spec.md)
- [docs/specs/vel-multi-client-swarm-spec.md](specs/vel-multi-client-swarm-spec.md)
- [docs/tickets/README.md](tickets/README.md)
- [docs/tickets/multi-client-swarm/README.md](tickets/multi-client-swarm/README.md)
- several swarm tickets

This suggests the repo is actively reconciling:

- partially shipped bootstrap/control-plane sync behavior
- planned multi-client swarm architecture
- how much of “distributed sync” is still deferred

The likely correction is:

- bootstrap discovery, action batching, worker presence, queued work, and receipts are partially implemented now
- full multi-node authority handoff, broad scheduler behavior, and richer distributed runtime remain deferred

## 4. Current Runtime Inventory

### 4.1 Core subsystem ownership

#### `vel-core`

Owns domain semantics and types.

Key areas:

- runs
- commitments
- context
- risk
- integrations
- command language
- uncertainty
- vocabulary

Primary files:

- [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs)
- [crates/vel-core/src/run.rs](/home/jove/code/vel/crates/vel-core/src/run.rs)
- [crates/vel-core/src/context.rs](/home/jove/code/vel/crates/vel-core/src/context.rs)
- [crates/vel-core/src/risk.rs](/home/jove/code/vel/crates/vel-core/src/risk.rs)
- [crates/vel-core/src/vocabulary.rs](/home/jove/code/vel/crates/vel-core/src/vocabulary.rs)

Assessment:

- aligned with repo rules
- still the strongest ownership boundary in the repo

#### `vel-storage`

Owns persistence and database access.

Primary files:

- [crates/vel-storage/src/lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs)
- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs)

Assessment:

- boundary direction is correct: no dependency on `vel-api-types`
- internal modularity is weak: nearly all persistence concerns live in one file

#### `vel-api-types`

Owns transport DTOs.

Primary file:

- [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs)

Assessment:

- boundary intent is correct
- single-file DTO concentration is now a drift and maintainability risk

#### `veld`

Owns application runtime orchestration, services, routes, adapters, and worker behavior.

Primary files:

- [crates/veld/src/app.rs](/home/jove/code/vel/crates/veld/src/app.rs)
- [crates/veld/src/routes/mod.rs](/home/jove/code/vel/crates/veld/src/routes/mod.rs)
- [crates/veld/src/services/mod.rs](/home/jove/code/vel/crates/veld/src/services/mod.rs)

Assessment:

- service layer exists and is meaningful
- route/service separation is uneven
- some files have grown large enough that architectural seams need to be made explicit before further extraction

#### Web client

Owns operator UI, local state/cache/query layer, and websocket synchronization.

Primary areas:

- [clients/web/src/components](/home/jove/code/vel/clients/web/src/components)
- [clients/web/src/data](/home/jove/code/vel/clients/web/src/data)
- [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts)

Assessment:

- data layer exists
- route-to-client contract mapping is still too monolithic in a few surfaces
- `SettingsPage` and `types.ts` are the main decomposition candidates

### 4.2 Operational flow that is actually aligned

The strongest present-tense runtime flow is:

`signals -> current context reducer -> risk -> policy/nudges -> suggestions/artifacts -> operator surfaces`

This is consistent across:

- [docs/status.md](status.md)
- [docs/specs/vel-system-architecture-map.md](specs/vel-system-architecture-map.md)
- [crates/veld/src/services/inference.rs](/home/jove/code/vel/crates/veld/src/services/inference.rs)
- [crates/veld/src/services/risk.rs](/home/jove/code/vel/crates/veld/src/services/risk.rs)
- [crates/veld/src/services/nudge_engine.rs](/home/jove/code/vel/crates/veld/src/services/nudge_engine.rs)

Important current alignment:

- explain routes are read-only again
- nudge evaluation reads current context instead of re-walking raw signals
- `vel-storage` still does not depend on transport DTOs

## 5. Oversized Files and Responsibility Drift

### 5.1 Highest-priority oversized files

Measured local hotspots:

- [crates/veld/src/app.rs](/home/jove/code/vel/crates/veld/src/app.rs): 8380 lines
- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs): 5256 lines
- [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx): 2371 lines
- [crates/veld/src/services/integrations.rs](/home/jove/code/vel/crates/veld/src/services/integrations.rs): 2076 lines
- [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs): 1765 lines
- [crates/veld/src/services/inference.rs](/home/jove/code/vel/crates/veld/src/services/inference.rs): 1621 lines
- [crates/veld/src/services/client_sync.rs](/home/jove/code/vel/crates/veld/src/services/client_sync.rs): 1548 lines
- [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts): 1495 lines
- [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs): 1055 lines

### 5.2 Files with clear boundary debt

#### [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs)

Problem:

- many unrelated persistence domains in one file

Needs:

- split by persistence domain, not by arbitrary chunk size

#### [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs)

Problem:

- route does service work
- includes persistence orchestration, intervention decisions, LLM usage, DTO mapping, and realtime emission

Needs:

- chat service extraction

#### [crates/veld/src/routes/explain.rs](/home/jove/code/vel/crates/veld/src/routes/explain.rs)

Problem:

- explain logic and hydration live in route layer

Needs:

- explain query/projection service

#### [crates/veld/src/services/integrations.rs](/home/jove/code/vel/crates/veld/src/services/integrations.rs)

Problem:

- sync runners, integration state, local source resolution, provider logic, and DTO projection all live together

Needs:

- internal submodules by provider and responsibility

#### [crates/veld/src/services/client_sync.rs](/home/jove/code/vel/crates/veld/src/services/client_sync.rs)

Problem:

- bootstrap projection, worker presence, work routing, receipts, and action application all live together

Needs:

- tighter containment and smaller modules before more swarm behavior is added

#### [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx)

Problem:

- one component owns too many domains: general settings, integrations, local adapters, loop controls, recovery guidance, and operator actions

Needs:

- split by UI and state boundary

#### [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts)

Problem:

- too much client contract and domain typing concentrated in one file

Needs:

- per-surface typing modules

## 6. Systems To Augment, Simplify, Split, or Remove

### 6.1 Augment

Systems that need more structure, not more breadth:

- a concrete current-state architecture map for the repo, not just a conceptual flow map
- service ownership for chat, explain, and threads
- storage submodules behind the existing `Storage` facade
- transport projection modules so DTO shaping stops spreading across routes and services
- web contract modules for settings, now, suggestions, sync, and integrations

### 6.2 Simplify

Systems that should be made narrower before expansion:

- convergence pack usage: default to [docs/tickets/repo-audit-hardening/README.md](tickets/repo-audit-hardening/README.md) for architecture alignment instead of splitting attention between multiple cleanup packs
- cluster/sync framing: separate shipped bootstrap/control-plane behavior from planned distributed runtime behavior
- route modules that still perform manual JSON shaping and direct storage orchestration
- service modules that return transport DTOs when they should return domain/application results

### 6.3 Split

Highest-value split candidates, grouped by seam:

#### Route to service

- [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs)
- [crates/veld/src/routes/explain.rs](/home/jove/code/vel/crates/veld/src/routes/explain.rs)
- [crates/veld/src/routes/threads.rs](/home/jove/code/vel/crates/veld/src/routes/threads.rs)
- [crates/veld/src/routes/suggestions.rs](/home/jove/code/vel/crates/veld/src/routes/suggestions.rs)

#### Service internal decomposition

- [crates/veld/src/services/integrations.rs](/home/jove/code/vel/crates/veld/src/services/integrations.rs)
- [crates/veld/src/services/client_sync.rs](/home/jove/code/vel/crates/veld/src/services/client_sync.rs)
- [crates/veld/src/services/inference.rs](/home/jove/code/vel/crates/veld/src/services/inference.rs)

#### Storage

- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs)

#### Transport and web contracts

- [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs)
- [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts)

#### Web UI

- [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx)

### 6.4 Remove or replace

Docs or structures that should not continue in current form:

- replace or rewrite [docs/specs/vel-architecture-for-agents.md](specs/vel-architecture-for-agents.md)
- demote [docs/architecture.md](architecture.md) to explicit design-direction status unless it is rewritten to match present-tense repo truth
- reduce direct storage access in read/query routes where a service should own query logic

## 7. Recommended Extraction Seams

### 7.1 Route-to-service extraction order

1. `chat_service`
2. `explain_service`
3. `thread_service`
4. `suggestion_query_service` and `suggestion_action_service`

Rationale:

- these remove the clearest route boundary violations first
- they reduce the chance that HTTP-layer changes keep dragging logic changes with them

### 7.2 Storage extraction order

Split [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs) by persistence domain:

1. runs, artifacts, refs
2. context, risk, nudges, suggestions
3. chat, messages, interventions
4. integrations, settings
5. cluster, workers, work assignments, loops
6. shared row mappers and JSON/time helpers

### 7.3 Web decomposition order

1. settings UI sections and hooks
2. per-surface web contract modules
3. `types.ts` split by feature area

## 8. Recommended Next Slice

If the repo follows the hardening pack in order, the next concrete execution slice after this inventory is:

1. keep ticket `004` open and use this file as its audit artifact
2. extract `chat_service` and `explain_service` plans before code motion
3. prepare the storage modularization plan for [docs/tickets/repo-audit-hardening/006-storage-layer-modularization-plan.md](tickets/repo-audit-hardening/006-storage-layer-modularization-plan.md)
4. prepare the frontend decomposition plan for [docs/tickets/repo-audit-hardening/007-frontend-surface-decomposition-plan.md](tickets/repo-audit-hardening/007-frontend-surface-decomposition-plan.md)
5. tighten doc authority by replacing or rewriting [docs/specs/vel-architecture-for-agents.md](specs/vel-architecture-for-agents.md)

## 9. Short Conclusion

Vel’s architecture is no longer speculative in the core runtime path.

The main problem now is not missing primitives. The main problem is uneven boundary discipline:

- some docs still speak in future-tense or fictional module terms
- some routes still perform service work
- some services still behave like route-adjacent DTO assemblers
- storage and web contract layers are too concentrated

That is a solvable phase.

The right move is to compress and clarify the existing architecture before adding more breadth.
