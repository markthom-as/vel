# Phase 1 Parallel Execution Board

Use this board to run Phase 1 work in parallel with explicit ownership and non-overlapping primary write scopes.

This board is execution guidance, not authority over shipped behavior.
Shipped behavior remains anchored in `docs/MASTER_PLAN.md`.

## Parallel Lanes

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Docs/Contracts | `011`, `018`, `020`, `021`, `022`, `023`, `024`, `025` | `docs/`, `config/`, `scripts/verify-repo-truth.mjs` | active | Architecture-first lane. Keep machine-readable artifacts and docs in lockstep. |
| B: Core Boundaries | `002`, `003`, `015` | `crates/vel-core/`, `crates/veld/src/routes/`, `crates/veld/src/services/` | active | Start only when lane A has stable contract references for touched seams. |
| C: Storage Decomposition | `001` | `crates/vel-storage/` | active | Keep extraction narrow and transaction-safe. Avoid re-expanding `db.rs`. |

## Suggested Initial Parallel Pull

1. Lane A: continue ticket `018` with subsystem audit updates when any subsystem seam changes.
2. Lane B: start route exposure inventory for `015` in `crates/veld/src/app.rs` and `docs/api/runtime.md`.
3. Lane C: continue repository extraction under `crates/vel-storage/src/repositories/` with focused tests.

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
