# 31-03 Summary

Routed assistant and Apple voice routine/profile edits through the typed planning-profile mutation seam with explicit confirmation and thread continuity.

Main changes:

- widened the core and transport contract in [crates/vel-core/src/planning.rs](/home/jove/code/vel/crates/vel-core/src/planning.rs), [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs), [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts), and [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so `PlanningProfileEditProposal` now carries explicit continuity thread metadata and assistant-entry responses can return a typed `planning_profile_proposal`
- added bounded freeform staging logic in [crates/veld/src/services/planning_profile.rs](/home/jove/code/vel/crates/veld/src/services/planning_profile.rs) for routine-block and planning-constraint edits, keeping assistant/voice requests on the canonical `PlanningProfileMutation` seam instead of inventing a second planner grammar
- wired assistant entry in [crates/veld/src/services/chat/messages.rs](/home/jove/code/vel/crates/veld/src/services/chat/messages.rs) and [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs) so qualifying requests now stage a typed planning-profile proposal, persist a `planning_profile_edit` thread, emit proposal events, and return thread continuity without silently mutating the profile
- wired Apple voice mutation handling in [crates/veld/src/services/apple_voice.rs](/home/jove/code/vel/crates/veld/src/services/apple_voice.rs) so voice can stage the same typed planning-profile proposal in confirmation mode with the same thread continuity and provenance story instead of bypassing supervision
- updated the checked-in contract artifacts in [planning-profile-edit-proposal.example.json](/home/jove/code/vel/config/examples/planning-profile-edit-proposal.example.json), [planning-profile-edit-proposal.schema.json](/home/jove/code/vel/config/schemas/planning-profile-edit-proposal.schema.json), and [planning-profile-parity-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-parity-contract.md) so the thread continuity expansion is documented and parseable

Focused verification:

- `cargo fmt --all`
- `cargo test -p veld --test chat_assistant_entry -- --nocapture`
- `cargo test -p veld --test apple_voice_loop -- --nocapture`
- `cargo test -p vel-api-types planning_profile -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

Notes:

- this slice stages planning-profile edits only; it does not yet apply or confirm them through a shipped approval UI
- the freeform parser is intentionally narrow and bounded to the current routine-block and planning-constraint vocabulary
- Rust test runs still emit the same pre-existing unused/dead-code warnings in `veld`
- no UAT was performed
