# 27-01 Summary

## Completed

- published the first typed canonical scheduler-rule contract in `vel-core`
- added bounded normalization for:
  - `block:*`
  - duration markers
  - `cal:free`
  - `time:*`
  - urgent/defer
  - fixed-start detection from labels or due datetimes
- moved the current reflow rule-facet derivation onto that shared core contract instead of keeping ad hoc service-local parsing
- documented the boundary that raw provider labels are compatibility metadata, not durable product truth

## Main files

- `crates/vel-core/src/scheduler.rs`
- `crates/vel-core/src/lib.rs`
- `crates/veld/src/services/reflow.rs`
- `docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core scheduler -- --nocapture`
- `cargo test -p veld reflow -- --nocapture`

## Notes

- this slice defines the canonical normalization seam but does not yet persist normalized scheduler semantics into durable storage
- raw labels and title tokens are still available as compatibility/search inputs, but reflow now consumes a shared core normalization model
