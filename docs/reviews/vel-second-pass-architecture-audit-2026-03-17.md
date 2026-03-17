# Vel Second-Pass Architecture Audit

Date: 2026-03-17  
Audience: maintainers, coding agents  
Purpose: perform a second-pass architecture audit focused on modularity, cross-platform sharing, subsystem boundaries, DRYness, and performance after the flat execution packs were introduced

## Summary

The repo is now in a better planning state than the earlier audit pass:

- the worktree is clean
- the current code ownership seams are stable enough to plan against
- the flat execution packs already reflect real code roots

The next architecture pass should not invent a new pack layout.

It should refine the current one around five stronger priorities:

1. modularity by ownership boundary
2. cross-platform sharing through contracts and semantics
3. DRYness through shared runtime/read-model logic
4. UI/UX consistency through shared state vocabulary rather than duplicated widgets
5. performance through smaller modules, clearer read models, and less repeated shaping work

## Main architectural conclusion

Vel should converge on **shared semantics, not fake universal code reuse**.

That means:

- domain logic stays shared in Rust
- transport contracts stay shared in `vel-api-types`
- Apple and web share bootstrap/sync/read-model semantics
- Apple and web do not need to share literal UI code
- same-platform UI primitives should be shared within each platform
- cross-platform UX should be shared through vocabulary, state classes, and read-model contracts

## Current strengths

- `vel-core` remains the cleanest domain boundary
- `vel-storage` remains the correct persistence boundary
- `veld` already has a meaningful service layer
- web and Apple clients already behave like clients of the daemon rather than separate brains
- client sync/bootstrap is already emerging as a shared substrate

## Remaining architecture debt

### 1. Shared-contract drift

Web types, Apple `VelAPI` models, and daemon DTOs still risk drifting unless the contract families are treated as one explicit architecture seam.

### 2. Large-file concentration

The main performance and maintainability issue is still structural concentration:

- `crates/veld/src/app.rs`
- `crates/vel-storage/src/db.rs`
- `crates/veld/src/services/client_sync.rs`
- `crates/veld/src/services/inference.rs`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/types.ts`

### 3. Read-model duplication pressure

The system is at risk of recomputing or reshaping the same state in multiple places:

- daemon read models
- web surface-specific shaping
- Apple bootstrap/offline shaping

That should converge on shared read-model semantics owned closer to the daemon and transport contract.

### 4. Cross-pack source drift

The older concept packs remain useful source material, but they are still too easy to treat as active backlog. The flat packs should stay canonical.

## Architecture direction for the next pass

The repo should converge on these system layers:

1. domain semantics
2. persistence
3. runtime orchestration and read-model assembly
4. transport contracts
5. client/platform shared adapters
6. platform-specific surfaces
7. docs and truth surfaces

This is a cleaner architecture than subsystem families that mix runtime, UI, and future product ideas in the same backlog.

## Recommended planning rule

When a task affects both web and Apple:

- first ask whether the shared concern is domain logic, transport contract, sync contract, or UX semantics
- solve it at that shared layer first
- only then do the platform-specific surface work

That is the main DRY rule for this repo.

## Relationship to flat execution packs

The current flat packs remain correct.

What changes after this audit is emphasis:

- `runtime-core-storage/` becomes the primary home for shared domain and transport contracts
- `daemon-api-runtime/` becomes the primary home for shared read-model and runtime assembly work
- `web-operator-runtime/` and `apple-client-bootstrap/` become primarily consumer packs over those shared contracts
- `docs-truth-and-planning/` becomes the place where shared UX/state semantics are kept explicit and current

## Recommended next move

Adopt a stronger architecture convergence spec that makes these rules explicit and update the flat packs so they optimize for:

- shared contracts first
- platform divergence only where necessary
- clear write scopes
- performance-oriented decomposition
