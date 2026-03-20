# 27-02 Summary

## Completed

- persisted canonical scheduler semantics into commitment metadata under `scheduler_rules`
- exposed typed scheduler access through `Commitment::scheduler_rules()`
- switched reflow to consume persisted/domain scheduler rules instead of recomputing normalization from raw labels on the hot path
- added focused storage tests proving normalized scheduler semantics survive insert and update round-trips

## Main files

- `crates/vel-core/src/scheduler.rs`
- `crates/vel-core/src/commitment.rs`
- `crates/vel-storage/src/repositories/commitments_repo.rs`
- `crates/veld/src/services/reflow.rs`
- `docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md`

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core scheduler -- --nocapture`
- `cargo test -p vel-storage commitments_repo -- --nocapture`
- `cargo test -p veld reflow -- --nocapture`

## Notes

- this slice keeps normalized scheduler semantics inside commitment metadata for now, which preserves storage compatibility while making the rules durable and queryable
- raw provider labels still remain in metadata for compatibility and recall, but they are no longer the only backend source for scheduler meaning
