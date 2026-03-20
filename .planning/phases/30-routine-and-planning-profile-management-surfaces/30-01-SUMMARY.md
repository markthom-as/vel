# 30-01 Summary

Published the typed planning-profile management contract and transport seam over the existing durable routine/planning substrate.

Main changes:

- expanded [crates/vel-core/src/planning.rs](/home/jove/code/vel/crates/vel-core/src/planning.rs) with explicit `PlanningProfileMutationKind`, `PlanningProfileRemoveTarget`, and `PlanningProfileMutation` vocabulary so future inspect/edit flows do not need shell-owned planning semantics
- re-exported the new planning-profile management types from [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs)
- widened [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with transport DTOs and bidirectional conversions for durable routine blocks, planning constraints, planning profiles, and typed planning-profile mutation requests
- added the machine-readable contract assets [config/schemas/planning-profile-mutation.schema.json](/home/jove/code/vel/config/schemas/planning-profile-mutation.schema.json) and [config/examples/planning-profile-mutation.example.json](/home/jove/code/vel/config/examples/planning-profile-mutation.example.json)
- updated [config/contracts-manifest.json](/home/jove/code/vel/config/contracts-manifest.json) and [config/README.md](/home/jove/code/vel/config/README.md) so the new management seam is discoverable alongside the existing durable planning-profile assets
- added the owner doc [docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md) and linked it from [docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md)

Focused verification:

- `cargo fmt --all`
- `cargo test -p vel-core planning -- --nocapture`
- `cargo test -p vel-api-types planning_profile_management_contract_assets_parse_and_register -- --nocapture`
- `node -e "JSON.parse(require('fs').readFileSync('config/examples/planning-profile-mutation.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/planning-profile-mutation.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`
- `rg -n "PlanningProfileMutation|planning-profile-management-contract|planning-profile-mutation" crates/vel-core/src/planning.rs crates/vel-api-types/src/lib.rs docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md config/README.md config/contracts-manifest.json`

Notes:

- this slice is contract-first only; it does not yet ship route/service CRUD behavior over the planning profile
- the new management seam is intentionally explicit and bounded so later mutation paths can stay backend-owned and reviewable
- no UAT was performed
