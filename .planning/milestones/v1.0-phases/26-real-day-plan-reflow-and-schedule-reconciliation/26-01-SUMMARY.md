# 26-01 Summary

## Outcome

Published the first canonical reflow/reconciliation contract for real day-plan recovery.

The main contract changes are:

- `ReflowCard` can now carry a typed backend-owned `proposal`
- proposal output can describe `moved`, `unscheduled`, and `needs_judgment` outcomes
- the scheduler-rule mapping seam now has canonical `rule_facets` instead of relying on raw provider tag syntax
- the web/API boundary and decoder fixtures are aligned to the same typed shape

## Main Files

- `crates/vel-core/src/operator_queue.rs`
- `crates/vel-core/src/lib.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/routes/now.rs`
- `clients/web/src/types.ts`
- `clients/web/src/types.test.ts`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`
- `docs/product/operator-action-taxonomy.md`

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core context -- --nocapture`
- `cargo test -p veld reflow -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

## Notes

- The new `proposal` seam is intentionally contract-first. It does not claim that full remaining-day recomputation already exists.
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds.
