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

## Next Priority Order

Current ranked execution order for remaining high-value phase-1 slices:

1. `003-service-dto-layering.md`
   focus: remove `vel-api-types` imports from `crates/veld/src/services/*` by moving DTO mapping to route boundaries.
2. `001-storage-modularization.md`
   focus: continue extracting `db.rs` seams into `crates/vel-storage/src/repositories/` with transaction-friendly APIs.
3. `002-typed-context-transition.md`
   focus: adopt `vel-core` typed context (`CurrentContextV1`) at inference/read boundaries while preserving migration from stored JSON.
4. `021-canonical-schema-and-config-contracts.md` + `025-config-and-contract-fixture-parity.md`
   focus: keep schema/template/fixture/test parity strict as service and storage contracts evolve.

## Execution Snapshot (2026-03-17)

- `003-service-dto-layering.md`: in progress
  - completed slices: journal, doctor, components, google auth-start, todoist status boundary mapping, chat reads, chat conversations, chat messages/interventions, chat assistant/provenance, chat mapping, chat websocket event reference cleanup, context_generation/context_runs, now, evaluate, explain, integrations, command_lang, client_sync, sync/cluster route response helper extraction, client_sync heartbeat boundary tightened to service-local typed input, remaining client_sync request signatures migrated from generic `impl Serialize` to concrete service-local typed inputs
  - next slices: audit remaining service-call boundaries for DTO-free signatures as routes and worker/service seams evolve
- `001-storage-modularization.md`: in progress
  - completed slices: signals, current context, context timeline, inferred state, processing_jobs repository extraction (including tx-safe helper variants), assistant_transcripts repository extraction (including tx-safe insert helper), artifacts repository extraction (including tx-safe create helper), suggestion_feedback repository extraction (including tx-safe insert helper), uncertainty_records repository extraction (including tx-safe insert/resolve helpers), commitment_risk repository extraction (including tx-safe insert helper), suggestions dedupe/state repository extraction (including tx-safe state update helper)
  - next slices: continue extracting remaining `db.rs` seams with focused repository tests
- `002-typed-context-transition.md`: in progress
  - completed slices: `CurrentContextV1` + `ContextMigrator` foundation and inference compatibility coverage, typed read adoption in `nudge_engine`, typed read adoption in `explain`, typed risk-score read adoption in `suggestions`, typed load/fallback handling in `evaluate` context-updated broadcast path, typed-first context reads in `now` (with raw fallback preserved), typed context assertions in integrations bootstrap/evaluate coverage
  - next slices: adopt typed reads at additional boundaries without changing stored payload fidelity
- `021` + `025`: in progress
  - completed slices: canonical `model_routing` example fixture added and registered in `contracts-manifest`, verifier parity assertion added, docs/config references updated, parsing test added in `vel-config`; `reminders_snapshot_path` parity added across schema/template/runtime verifier and config tests; canonical `model_profile` example fixture added and wired through manifest/docs/tests/verifier
  - next slices: continue filling schema/template/example/test parity gaps for remaining manifest-bearing config contracts
- `015-http-surface-auth-hardening.md`: in progress
  - completed slices: explicit `future_external` fail-closed route reservations for `/v1/connect*` and `/v1/cluster/clients*` in `app.rs`, plus coverage tests and runtime API docs alignment; expanded `/api/*` + `/ws` exposure/auth and fail-closed matrix tests with runtime API docs matrix; explicit worker-auth test coverage for `POST /v1/sync/work-queue/claim-next` when worker token policy is configured
  - next slices: continue route exposure inventory/test matrix tightening for mounted `/api/*` and websocket surfaces

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
