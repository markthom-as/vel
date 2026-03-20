# 28-01 Summary

## Completed

- published the first canonical `day_plan` contract alongside the existing `reflow` vocabulary instead of inventing a second planner model
- added typed core/domain contracts for `RoutineBlock`, `DayPlanChange`, and `DayPlanProposal`
- exposed matching transport DTOs in `vel-api-types`
- added a checked-in schema and example for bounded same-day planning output
- documented the relationship between proactive day planning and later `reflow`

## Main files

- `crates/vel-core/src/operator_queue.rs`
- `crates/vel-core/src/lib.rs`
- `crates/vel-api-types/src/lib.rs`
- `config/schemas/day-plan-proposal.schema.json`
- `config/examples/day-plan-proposal.example.json`
- `config/README.md`
- `docs/cognitive-agent-architecture/architecture/day-plan-contract.md`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`
- `docs/product/operator-mode-policy.md`

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p vel-api-types day_plan_proposal_data_serializes_counts_and_routine_blocks -- --nocapture`
- `node -e "JSON.parse(require('fs').readFileSync('config/schemas/day-plan-proposal.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/examples/day-plan-proposal.example.json','utf8')); console.log('json-ok')"`
- `rg -n "day-plan contract|routine blocks|same-day and bounded|second planner model|calendar anchors" docs/cognitive-agent-architecture/architecture/day-plan-contract.md docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md docs/product/operator-mode-policy.md config/README.md`

## Notes

- this slice intentionally publishes the planning contract before planner behavior lands
- the contract stays same-day and bounded; it does not claim multi-day optimization or automatic broad calendar mutation
