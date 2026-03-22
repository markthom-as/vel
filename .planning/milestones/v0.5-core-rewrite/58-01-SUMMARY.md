# 58-01 Summary

Completed the foundational typed-ID, durable-envelope, and storage-trait slice for Phase 58.

## Delivered

- added [ids.rs](/home/jove/code/vel/crates/vel-core/src/ids.rs)
- added [object_envelope.rs](/home/jove/code/vel/crates/vel-core/src/object_envelope.rs)
- updated [lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs) in `vel-core`
- added [storage_backend.rs](/home/jove/code/vel/crates/vel-storage/src/storage_backend.rs)
- updated [lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs)
- updated [mod.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/mod.rs)
- updated [Cargo.toml](/home/jove/code/vel/crates/vel-storage/Cargo.toml)

## Locked Truths

- `vel-core` now has dedicated `0.5` typed IDs for `TaskId`, `WorkflowId`, `ModuleId`, `SkillId`, `ToolId`, `IntegrationAccountId`, `SyncLinkId`, and `WriteIntentId`.
- `vel-core` now exposes a storage-agnostic, serde-backed, version-aware canonical durable envelope with `object_class` and `source_summary`.
- `vel-storage` now exposes explicit store seams for `ObjectStore`, `RegistryStore`, `RelationStore`, `SyncLinkStore`, `RuntimeStore`, `AuditStore`, `ProjectionStore`, and `TransactionManager`.
- the existing SQL-backed `Storage` remains intact underneath; this slice added compiler-visible substrate contracts without prematurely rewriting persistence behavior.

## Verification

- `rg -n "TaskId|EventId|WorkflowId|ModuleId|SkillId|ToolId|IntegrationAccountId|SyncLinkId|WriteIntentId|source_summary|object_class|ObjectStore|RegistryStore|RelationStore|SyncLinkStore|RuntimeStore|AuditStore|ProjectionStore|TransactionManager" crates/vel-core/src/ids.rs crates/vel-core/src/object_envelope.rs crates/vel-storage/src/storage_backend.rs`
- `cargo test -p vel-core --lib`
- `cargo test -p vel-storage storage_backend --lib`
- `cargo check -p vel-storage`

## Baseline Note

The initial pre-change baseline for `cargo test -p vel-core -p vel-storage --lib` still included one unrelated existing failure in `semantic_memory_repo`:

- `repositories::semantic_memory_repo::tests::capture_fts_query_text_strips_conversational_punctuation`

That failure is outside the `58-01` seam and was not widened by this slice.

## Outcome

Phase 58 now begins from compiler-enforced substrate contracts instead of Phase 57 prose alone. The next slices can build real persistence behavior on top of typed IDs, a durable envelope, and explicit storage seams.
