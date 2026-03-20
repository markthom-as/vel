# 21-04 Summary

## Outcome

Closed Phase 21 with cross-surface voice documentation and verification parity notes.

## What changed

- Updated [chat.md](/home/jove/code/vel/docs/api/chat.md) to describe one transcript-driven assistant-entry story across typed and voiced input, including web voice provenance and the Apple compatibility-route continuity note.
- Updated [runtime.md](/home/jove/code/vel/docs/api/runtime.md) so the Apple quick-loop route now explicitly documents shared `thread_id` continuity hints and the remaining route split between Apple compatibility replies and browser assistant entry.
- Updated [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) to teach one operator rule for voice across desktop/browser and Apple: shells own permissions, push-to-talk, local STT/TTS, and offline presentation; the backend owns routing, continuity, and daily-loop authority.
- Updated [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md) to record the current parity shape and remaining platform limits honestly.

## Verification

- `rg -n "voice|assistant|Now|Threads|Apple|push-to-talk|local speech-to-text" docs/api/chat.md docs/api/runtime.md docs/user/daily-use.md clients/apple/README.md`

## Limits preserved

- Browser/desktop voice still depends on browser speech-recognition availability.
- Apple still uses `/v1/apple/voice/turn` as a compatibility route for typed quick-loop replies.
- Full Apple app-target validation still requires Xcode/macOS; Linux verification remains package-level through `make check-apple-swift`.
