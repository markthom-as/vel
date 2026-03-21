# Phase 47-01 Summary

## Outcome

Published the first canonical `Now` transport seam over shared Rust DTOs and client boundary models.

The DTO layer now carries explicit types for:

- header buckets and filter routing
- count-display policy
- status-row inputs
- context one-liner
- stacked nudge bars
- canonical task lane
- closed v1 docked-input intent taxonomy

The web and Apple boundary models were updated in the same slice so later phases can consume the same transport vocabulary without inventing shell-local names.

## Files Changed

- `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/types.ts`
- `clients/web/src/types.test.ts`
- `clients/apple/VelAPI/Sources/VelAPI/Models.swift`

## Verification

- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo check -p vel-api-types`

## Notes

- `cargo test -p vel-api-types --lib -- --nocapture` still has unrelated pre-existing failures in grounding-pack fixtures and one floating-point assertion:
  - `tests::agent_grounding_contract_assets_parse_and_register`
  - `tests::agent_grounding_round_trips_typed_sections`
  - `tests::recall_context_round_trips_named_counts_and_scores`
- The canonical DTO seam itself compiles cleanly.
