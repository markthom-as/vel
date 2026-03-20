# 22-04 Summary

Phase 22 plan `22-04` is complete.

## Outcome

The repo now teaches one assistant-capable daily-loop, closeout, and thread-resolution story across runtime, user, Apple, and CLI surfaces.

## What changed

- Updated [chat.md](/home/jove/code/vel/docs/api/chat.md) to describe:
  - assistant-capable morning and standup routing
  - assistant-capable end-of-day closeout
  - durable thread escalation for longer `check_in`, `reflow`, and action follow-through
- Updated [runtime.md](/home/jove/code/vel/docs/api/runtime.md) to record:
  - typed daily-loop follow-through thread continuity
  - typed thread metadata as the resolution-history truth
  - the current Apple/browser voice split and its limits honestly
- Updated [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) so operators are told one consistent product rule:
  - `Now` and voice can enter bounded daily-loop or closeout flows inline
  - longer clarification and resolution work belongs in `Threads`
  - the backend remains the authority for why that thread exists and what state it is in
- Updated [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md) so Apple explicitly preserves shared thread-resolution continuity instead of inventing local deferred/resolved semantics
- Updated [threads.rs](/home/jove/code/vel/crates/vel-cli/src/commands/threads.rs) so the CLI teaches `Threads` as continuity/archive plus follow-through history rather than generic thread listing

## Verification

- `rg -n "assistant|daily loop|standup|end-of-day|Threads|check-in|reflow" docs/api/chat.md docs/api/runtime.md docs/user/daily-use.md clients/apple/README.md crates/vel-cli/src/commands`

## Notes

- Current platform limits are preserved honestly:
  - browser/desktop voice still depends on local browser STT availability
  - Apple still uses `/v1/apple/voice/turn` as the compatibility quick-loop route
  - full Apple app-target validation still requires Xcode/macOS
