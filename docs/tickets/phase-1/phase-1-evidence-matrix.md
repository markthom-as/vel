# Phase 1 Evidence Matrix

Status and verification evidence as of **2026-03-17**.

## Execution Summary

| Ticket | Documented Status | Evidence Status | Evidence | Blockers |
| --- | --- | --- | --- | --- |
| 001-storage-modularization | in-progress | ✅ Strong | `crates/vel-storage/src/repositories/mod.rs` declares repository modules for all active domains; `crates/vel-storage/src/db.rs` is façade + connector wiring; unit tests in `crates/vel-storage` cover repository behavior. | `StorageBackend` trait and end-to-end transaction-helper coverage for every write-path are still unresolved. |
| 002-typed-context-transition | in-progress | ✅ Strong | Typed model and migrator in `crates/vel-core/src/context.rs`; typed context reads/writes in inference/now/explain/suggestions flows; parse/migration tests in `crates/veld/src/services/inference.rs`, `crates/veld/src/services/now.rs`, `crates/veld/src/services/evaluate.rs`. | Legacy JSON access still appears in some callers/tests; migration completeness is not final. |
| 003-service-dto-layering | in-progress | ✅ Strong | Chat/cluster/sync DTO conversion is route-local (`crates/veld/src/routes/chat.rs`, `crates/veld/src/routes/sync.rs`, `crates/veld/src/routes/cluster.rs`); chat services no longer import `vel_api_types` (`rg -n "use vel_api_types" crates/veld/src/services` returns none). | Remaining non-chat services need periodic verification for strict route-only mapping; `ensure_known_loop_rows` in `crates/veld/src/routes/loops.rs` currently exceeds the <30-line handler/helper target. |
| 011-documentation-truth-repair | in-progress | ⚠️ Partial | Top-level authority chain is consistent (`README.md`, `docs/README.md`); script checks enforce current authority points (`node scripts/verify-repo-truth.mjs`). | Several concept/developer docs still need stronger explicit linkage and examples of subsystem seams. |
| 015-http-surface-auth-hardening | complete | ✅ Strong | Extensive auth and exposure tests in `crates/veld/src/app.rs` and route tests; fail-closed coverage for undefined routes and worker/operator policy combinations; `docs/api/runtime.md` documents route classes, matrix, and enforcement invariants. | route-surface inventory remains a living maintenance requirement and is validated by the same tests; no blockers for closure. |
| 018-cross-cutting-system-traits-baseline | in-progress | ✅ Moderate | Audit asset updated: `docs/cognitive-agent-architecture/architecture/cross-cutting-trait-audit.md` with ticket/coverage mapping. | Still has explicit `partial` entries requiring follow-up tickets (022/019, etc.). |
| 020-documentation-catalog-single-source | complete | ✅ Strong | Canonical source in `docs/documentation-catalog.json` drives generated `crates/vel-cli/src/commands/docs_catalog.generated.json`, `clients/web/src/data/documentationCatalog.generated.ts`, and `clients/apple/VelAPI/Sources/VelAPI/VelDocumentation.swift`; checks validate surfaced catalogs via `scripts/verify-repo-truth.mjs`, `scripts/sync-documentation-catalog.mjs --check`, and `cargo test -p vel-cli docs_catalog_points_at_current_authority_docs`. |
| 021-canonical-schema-and-config-contracts | in-progress | ⚠️ Partial | Contract docs and parser coverage exist for major config artifacts; some template/ownership tasks still tracked downstream. | Consolidated schema ownership + manifest-first consumer path not fully closed. |
| 022-data-sources-and-connector-architecture | planned | ⚠️ Planning | Canonical family/source-mode vocabulary is documented under `docs/cognitive-agent-architecture/integrations/`. |
| 023-self-awareness-and-supervised-self-modification | planned | ⚠️ Planning | Self-awareness model and tool-access policy docs exist and reference scope split. |
| 024-machine-readable-schema-and-manifest-publication | planned | ⚠️ Planning | Schema publication manifest work is planned and not yet fully operational in consuming surfaces. |
| 025-config-and-contract-fixture-parity | in-progress | ⚠️ Moderate | Verifier checks added for manifest-mapped fixtures in `scripts/verify-repo-truth.mjs`; still incomplete for full contract coverage. |

## Verification Commands Run

- `cargo check -p veld`
- `cargo test -p veld`
- `cargo test -p vel-storage`
- `cargo test -p vel-config`
- `cargo test -p veld policy_config`
- `cargo test -p vel-cli docs_catalog_points_at_current_authority_docs`
- `node scripts/verify-repo-truth.mjs`
- `node scripts/sync-documentation-catalog.mjs --check`
- `cargo fmt --all -- --check`
