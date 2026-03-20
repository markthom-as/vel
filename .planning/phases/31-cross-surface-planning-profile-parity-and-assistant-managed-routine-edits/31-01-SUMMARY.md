# 31-01 Summary

Published the cross-surface planning-profile parity contract and the typed assistant-capable planning-profile edit proposal seam.

Main changes:

- expanded [crates/vel-core/src/planning.rs](/home/jove/code/vel/crates/vel-core/src/planning.rs) and [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs) with `PlanningProfileSurface`, `PlanningProfileContinuity`, and `PlanningProfileEditProposal` so later CLI, Apple, and assistant/voice work can share one explicit contract instead of inventing shell-local planning state
- widened [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with transport DTOs for the same new planning-profile parity vocabulary and added asset-registration tests for the new contract
- added checked-in machine-readable assets at [planning-profile-edit-proposal.schema.json](/home/jove/code/vel/config/schemas/planning-profile-edit-proposal.schema.json) and [planning-profile-edit-proposal.example.json](/home/jove/code/vel/config/examples/planning-profile-edit-proposal.example.json), then registered them in [contracts-manifest.json](/home/jove/code/vel/config/contracts-manifest.json) and [config/README.md](/home/jove/code/vel/config/README.md)
- added the companion owner doc [planning-profile-parity-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-parity-contract.md) and linked it from [planning-profile-management-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md)
- widened the web boundary in [types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the shared contract already has a typed frontend decoder even before later slices wire it into concrete flows

Focused verification:

- `cargo fmt --all`
- `cargo test -p vel-core planning -- --nocapture`
- `cargo test -p vel-api-types planning_profile -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `node -e "JSON.parse(require('fs').readFileSync('config/examples/planning-profile-edit-proposal.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/planning-profile-edit-proposal.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`

Notes:

- this slice publishes only the shared contract, schemas, examples, and typed transport vocabulary; it does not yet claim shipped CLI/Apple parity or live assistant-driven routine/profile edits
- the new proposal contract intentionally stages bounded planning-profile edits over the existing `PlanningProfileMutation` seam rather than inventing a second edit grammar
- no UAT was performed
