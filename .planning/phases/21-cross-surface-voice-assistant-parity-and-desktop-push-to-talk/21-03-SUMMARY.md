# 21-03 Summary

## Outcome

Completed the Apple voice alignment slice over the shared assistant seam while preserving bounded offline/cache behavior.

## What changed

- Extended the Apple voice response contract with an optional shared continuity thread hint in [apple.rs](/home/jove/code/vel/crates/vel-core/src/apple.rs), [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs), and [Models.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/Models.swift).
- Documented the aligned Apple route behavior in [VelClient.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/VelClient.swift).
- Updated [apple_voice.rs](/home/jove/code/vel/crates/veld/src/services/apple_voice.rs) so backend-handled Apple voice turns persist transcript-first continuity into the shared conversation/message substrate with the same voice provenance shape used by assistant entry, while keeping typed Apple schedule / behavior / daily-loop responses intact.
- Updated the iOS voice shell in [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift) so successful backend replies can acknowledge shared Threads follow-up without inventing local thread policy, and reinforced the summary-first/product-authority copy.
- Updated [README.md](/home/jove/code/vel/clients/apple/README.md) to describe the shared continuity behavior and bounded offline fallback.
- Added focused regression coverage in [apple_voice_loop.rs](/home/jove/code/vel/crates/veld/tests/apple_voice_loop.rs).

## Verification

- `cargo fmt --all`
- `cargo test -p veld --test apple_voice_loop -- --nocapture`
- `make check-apple-swift`

## Notes

- Apple still owns microphone permission, speech recognition, TTS playback, and offline/cache presentation.
- Offline Apple behavior remains bounded to cached summaries and queued safe actions; it does not synthesize new backend-owned thread or planning policy locally.
