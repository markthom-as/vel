# 32-01 Summary

## Completed

Published the approved planning-profile application contract and explicit proposal lifecycle transitions.

## What changed

- widened [planning.rs](/home/jove/code/vel/crates/vel-core/src/planning.rs) so `PlanningProfileEditProposal` now carries explicit lifecycle `state` plus optional `outcome_summary`, reusing `AssistantProposalState` instead of inventing a planner-specific lifecycle enum
- widened [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) so transport DTOs preserve that lifecycle state and outcome summary across API boundaries
- updated [planning_profile.rs](/home/jove/code/vel/crates/veld/src/services/planning_profile.rs) and [messages.rs](/home/jove/code/vel/crates/veld/src/services/chat/messages.rs) so staged planning-profile proposals now originate with typed `staged` state and persist that typed state into thread continuity metadata instead of a hard-coded string
- updated the checked-in assets [planning-profile-edit-proposal.example.json](/home/jove/code/vel/config/examples/planning-profile-edit-proposal.example.json) and [planning-profile-edit-proposal.schema.json](/home/jove/code/vel/config/schemas/planning-profile-edit-proposal.schema.json)
- added the owner doc [planning-profile-application-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-application-contract.md) and linked it from [planning-profile-parity-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-parity-contract.md) and [config/README.md](/home/jove/code/vel/config/README.md)

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core planning -- --nocapture`
- `cargo test -p vel-api-types planning_profile -- --nocapture`
- `cargo test -p veld --test chat_assistant_entry assistant_entry_stages_planning_profile_edit_with_thread_continuity -- --nocapture`
- `node -e "JSON.parse(require('fs').readFileSync('config/examples/planning-profile-edit-proposal.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/planning-profile-edit-proposal.schema.json','utf8')); console.log('ok')"`

## Result

Phase 32 now has a real contract-first lifecycle for planning-profile proposals. The next logical step is `32-02`: backend approval/application over the canonical mutation seam.
