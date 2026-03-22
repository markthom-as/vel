# 58-03 Summary

Completed the integration-account, SyncLink, runtime-record, and projection persistence slice for Phase 58.

## Delivered

- added [0048_phase58_sync_runtime_projection.sql](/home/jove/code/vel/migrations/0048_phase58_sync_runtime_projection.sql)
- added [integration_accounts_repo.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/integration_accounts_repo.rs)
- added [sync_links_repo.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/sync_links_repo.rs)
- added [runtime_records_repo.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/runtime_records_repo.rs)
- added [projections_repo.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/projections_repo.rs)
- updated [mod.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/mod.rs)
- updated [lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs)

## Locked Truths

- integration-account canonical metadata now persists without storing secrets
- `SyncLink` lifecycle state now persists in a dedicated table with explicit `deleted_upstream`, `superseded`, `reconciled`, and `conflicted` posture
- runtime/control records such as `write_intent`, `approval`, and `run` now persist outside canonical content storage
- projections now have a dedicated rebuildable persistence seam, including `source_summary`, instead of being smeared into canonical rows

## Verification

- `rg -n "deleted_upstream|superseded|conflicted|reconciled|auth_state|policy_profile|provider|write_intent|approval|run|source_summary|projection|rebuild" migrations/0048_phase58_sync_runtime_projection.sql crates/vel-storage/src/repositories/sync_links_repo.rs crates/vel-storage/src/repositories/integration_accounts_repo.rs crates/vel-storage/src/repositories/runtime_records_repo.rs crates/vel-storage/src/repositories/projections_repo.rs`
- `cargo test -p vel-storage integration_accounts_repo --lib`
- `cargo test -p vel-storage sync_links_repo --lib`
- `cargo test -p vel-storage runtime_records_repo --lib`
- `cargo test -p vel-storage projections_repo --lib`
- `cargo check -p vel-storage`

## Outcome

Phase 58 now has complete durable seams for canonical object state, registry state, relations, integration accounts, `SyncLink`, runtime/control records, and rebuildable projections. The remaining substrate work can focus on bootstrap/migration scaffolding and query/rebuild closure rather than missing storage families.
