# 29-01 Summary

Published the contract-first Phase 29 routine-planning vocabulary in a dedicated core module instead of extending shell or service-local planning logic.

Main changes:

- added [crates/vel-core/src/planning.rs](/home/jove/code/vel/crates/vel-core/src/planning.rs) with:
  - `DurableRoutineBlock`
  - `PlanningConstraintKind`
  - `PlanningConstraint`
  - `RoutinePlanningProfile`
- re-exported the new planning contract types from [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs)
- added machine-readable assets:
  - [config/schemas/routine-planning-profile.schema.json](/home/jove/code/vel/config/schemas/routine-planning-profile.schema.json)
  - [config/examples/routine-planning-profile.example.json](/home/jove/code/vel/config/examples/routine-planning-profile.example.json)
- updated [config/README.md](/home/jove/code/vel/config/README.md) and [config/contracts-manifest.json](/home/jove/code/vel/config/contracts-manifest.json)
- added owner documentation in [docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md)
- linked the durable input contract from [docs/cognitive-agent-architecture/architecture/day-plan-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-contract.md)

Focused verification:

- `cargo fmt --all`
- `cargo test -p vel-core planning -- --nocapture`
- `node -e "JSON.parse(require('fs').readFileSync('config/examples/routine-planning-profile.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/routine-planning-profile.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`
- targeted `rg` truth checks across the new core module, owner docs, and checked-in assets

Notes:

- this slice publishes the durable routine/planning contract only; persistence and runtime consumption remain Phase `29-02` and `29-03`
- the planning substrate remains intentionally bounded and same-day; this contract does not imply a broader autonomous planner or full habit system
