# Phase 1 Evidence Matrix

Status and verification evidence as of **2026-03-17**.

## Execution Summary

| Ticket | Documented Status | Evidence Status | Evidence | Blockers |
| --- | --- | --- | --- | --- |
| 001-storage-modularization | complete | ✅ Strong | `crates/vel-storage/src/repositories/mod.rs` declares repository modules for active domains; `crates/vel-storage/src/db.rs` is façade/connectivity plus an explicit `StorageBackend` seam; `vel-storage` tests now include a focused cross-repository shared-transaction commit proof. | structural backend abstraction is complete for the current SQLite-backed implementation; alternate backends remain future work, not a closure blocker. |
| 002-typed-context-transition | complete | ✅ Strong | Typed model and migrator live in `crates/vel-core/src/context.rs`; runtime logic reads/writes typed current-context in inference/now/explain/suggestions flows; remaining test fixtures in `crates/veld/src/app.rs` and `crates/veld/src/services/command_lang.rs` now build `CurrentContextV1` and serialize only at the storage edge. | JSON string persistence remains the intended storage boundary; no remaining runtime seam blocks typed-context closure. |
| 003-service-dto-layering | complete | ✅ Strong | Chat/cluster/sync DTO conversion is route-local (`crates/veld/src/routes/chat.rs`, `crates/veld/src/routes/sync.rs`, `crates/veld/src/routes/cluster.rs`); services no longer import `vel_api_types` (`rg -n "use vel_api_types" crates/veld/src/services` returns none); route audit shows no route handlers/helpers over the phase target after tightening `crates/veld/src/routes/loops.rs`. | route layering remains a maintenance invariant and is enforced by the same audits; no closure blocker remains. |
| 011-documentation-truth-repair | complete | ✅ Strong | Top-level authority chain is consistent in `README.md` and `docs/README.md`; concept/spec reading order is explicit; script checks enforce current authority pointers via `node scripts/verify-repo-truth.mjs`. | docs still evolve, but no closure blocker remains for authority repair and top-level mapping. |
| 015-http-surface-auth-hardening | complete | ✅ Strong | Extensive auth and exposure tests in `crates/veld/src/app.rs` and route tests; fail-closed coverage for undefined routes and worker/operator policy combinations; `docs/api/runtime.md` documents route classes, matrix, and enforcement invariants. | route-surface inventory remains a living maintenance requirement and is validated by the same tests; no blockers for closure. |
| 018-cross-cutting-system-traits-baseline | complete | ✅ Strong | Audit asset in `docs/cognitive-agent-architecture/architecture/cross-cutting-trait-audit.md` covers major subsystems, distinguishes documentation vs implementation gaps, and maps the remaining repo-wide gaps to explicit queue coverage (`019`, `022`, `024`). | audit remains a living artifact, but the baseline and queue-hygiene work are complete. |
| 020-documentation-catalog-single-source | complete | ✅ Strong | Canonical source in `docs/documentation-catalog.json` drives generated `crates/vel-cli/src/commands/docs_catalog.generated.json`, `clients/web/src/data/documentationCatalog.generated.ts`, and `clients/apple/VelAPI/Sources/VelAPI/VelDocumentation.swift`; checks validate surfaced catalogs via `scripts/verify-repo-truth.mjs`, `scripts/sync-documentation-catalog.mjs --check`, and `cargo test -p vel-cli docs_catalog_points_at_current_authority_docs`. |
| 021-canonical-schema-and-config-contracts | complete | ✅ Strong | Canonical ownership/boundary doc exists in `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`; manifest inventory exists in `config/contracts-manifest.json`; parser coverage exists in `crates/vel-config/src/lib.rs`, `crates/vel-config/src/models.rs`, and `crates/veld/src/policy_config.rs`. | machine-readable publication rollout remains downstream ticket `024`, not a blocker for this catalog/ownership ticket. |
| 022-data-sources-and-connector-architecture | planned | ⚠️ Planning | Canonical family/source-mode vocabulary is documented under `docs/cognitive-agent-architecture/integrations/`. |
| 023-self-awareness-and-supervised-self-modification | planned | ⚠️ Planning | Self-awareness model and tool-access policy docs exist and reference scope split. |
| 024-machine-readable-schema-and-manifest-publication | planned | ⚠️ Planning | Schema publication manifest work is planned and not yet fully operational in consuming surfaces. |
| 025-config-and-contract-fixture-parity | complete | ✅ Strong | Canonical templates/examples are registered in `config/contracts-manifest.json`; runtime/policy/model templates parse in `vel-config` and `policy_config` tests; manifest-mapped JSON fixtures are schema-validated in `scripts/verify-repo-truth.mjs`. | downstream publication consumers remain tracked by ticket `024`; no parity blocker remains for current checked-in artifacts. |

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
