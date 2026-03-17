# Phase 1 Parallel Execution Board

This board records how Phase 1 work was parallelized with explicit ownership and non-overlapping primary write scopes.

This board is execution guidance, not authority over shipped behavior.
Shipped behavior remains anchored in `docs/MASTER_PLAN.md`.

## Parallel Lanes (Historical)

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Docs/Contracts | `011`, `018`, `020`, `021`, `022`, `023`, `024`, `025` | `docs/`, `config/`, `scripts/verify-repo-truth.mjs` | closed | Architecture-first lane. Keep machine-readable artifacts and docs in lockstep. |
| B: Core Boundaries | `002`, `003`, `015` | `crates/vel-core/`, `crates/veld/src/routes/`, `crates/veld/src/services/` | closed | Started when lane A had stable contract references for touched seams. |
| C: Storage Decomposition | `001` | `crates/vel-storage/` | closed | Kept extraction narrow and transaction-safe without re-expanding `db.rs`. |

## Suggested Initial Parallel Pull (Historical)

This section is retained as historical execution context; Phase 1 is complete and no longer has active pull lanes.

## Next Priority Order

Phase 1 closure order is complete.

If you are continuing execution, pull from Phase 2 queue items using `docs/tickets/README.md` and `docs/tickets/architecture-first-parallel-queue.md`.

## Execution Snapshot (2026-03-17)

- `001-storage-modularization.md`: complete
- `002-typed-context-transition.md`: complete
- `003-service-dto-layering.md`: complete
- `011-documentation-truth-repair.md`: complete
- `015-http-surface-auth-hardening.md`: complete
- `018-cross-cutting-system-traits-baseline.md`: complete
- `020-documentation-catalog-single-source.md`: complete
- `021-canonical-schema-and-config-contracts.md`: complete
- `022-data-sources-and-connector-architecture.md`: complete
- `023-self-awareness-and-supervised-self-modification.md`: complete
- `024-machine-readable-schema-and-manifest-publication.md`: complete
- `025-config-and-contract-fixture-parity.md`: complete

## Coordination Rules

- Do not overlap primary write scopes in the same patch unless the change is a planned cross-lane integration slice.
- Merge docs/contracts lane updates first when a core/storage change depends on schema or contract language.
- Every lane reports verification evidence with command output in the ticket or PR notes.
- If a lane discovers a new cross-lane contract, update `docs/MASTER_PLAN.md` and `docs/tickets/README.md` in the same slice.

## Baseline Verification Commands

- `node scripts/verify-repo-truth.mjs`
- `cargo test -p vel-cli docs_catalog_points_at_current_authority_docs`
- `cargo test -p vel-storage`
- `cargo test -p veld policy_config`
- `npm -C clients/web test -- --run src/components/SettingsPage.test.tsx`
