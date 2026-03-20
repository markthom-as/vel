# 31-04 Summary

## Completed

Closed Phase 31 with doc and verification alignment for the cross-surface planning-profile model.

## What changed

- updated [docs/api/runtime.md](/home/jove/code/vel/docs/api/runtime.md) so `/v1/planning-profile`, `/v1/apple/voice/turn`, and `/api/assistant/entry` now describe one backend-owned planning-profile model with staged conversational edits, explicit thread continuity, and no silent profile mutation
- updated [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) and [docs/user/setup.md](/home/jove/code/vel/docs/user/setup.md) so operators are told to treat assistant/voice planning-profile requests as confirmation-first `Threads` follow-through rather than already-saved planner changes
- updated [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) and [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md) so cross-surface parity and Apple voice behavior reflect the shipped bounded edit/staging rule

## Verification

- `rg -n "planning-profile|planning profile|assistant/entry|apple/voice|thread continuity|silently" docs/api/runtime.md docs/user/daily-use.md docs/user/setup.md docs/product/operator-mode-policy.md clients/apple/README.md crates/vel-cli/src/commands/agent.rs`
- `node -e "JSON.parse(require('fs').readFileSync('config/examples/planning-profile-edit-proposal.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/planning-profile-edit-proposal.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`
- `cargo test -p veld --test chat_assistant_entry -- --nocapture`
- `cargo test -p veld --test apple_voice_loop -- --nocapture`
- `cargo test -p vel-api-types planning_profile -- --nocapture`

## Result

Phase 31 is now closed. The next logical step is defining and planning Phase 32.
