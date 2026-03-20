# 21-01 Summary

## Outcome

Completed the first backend slice for cross-surface voice parity by adding a shared voice-provenance seam to assistant entry and reusing that same normalization helper in Apple transcript capture.

This does not replace the Apple voice route yet. It gives both paths one normalized backend voice metadata shape instead of letting transcript provenance drift by surface.

## Shipped changes

- Added optional voice provenance to the assistant-entry transport contract in [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs).
- Extended the shared assistant-entry backend seam in [crates/veld/src/services/chat/messages.rs](/home/jove/code/vel/crates/veld/src/services/chat/messages.rs) so user messages can carry explicit voice provenance and `input_mode`.
- Updated [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs) so `/api/assistant/entry` forwards voice metadata into that shared service seam.
- Reused the same normalization helper in [crates/veld/src/services/apple_voice.rs](/home/jove/code/vel/crates/veld/src/services/apple_voice.rs) for Apple transcript-capture provenance payloads.
- Added focused regression coverage in [crates/veld/tests/chat_assistant_entry.rs](/home/jove/code/vel/crates/veld/tests/chat_assistant_entry.rs) and [crates/veld/tests/apple_voice_loop.rs](/home/jove/code/vel/crates/veld/tests/apple_voice_loop.rs).

## Verification

- `cargo fmt --all`
- `cargo test -p veld --test chat_assistant_entry -- --nocapture`
- `cargo test -p veld --test apple_voice_loop -- --nocapture`

## Notes

- The pre-existing `dead_code` warning for `AssistantEntryRouteTarget::Inline` remains.
- Apple still uses its dedicated route for behavior and intent handling; the shared seam introduced here is a migration substrate for later slices, not the full parity endpoint yet.
