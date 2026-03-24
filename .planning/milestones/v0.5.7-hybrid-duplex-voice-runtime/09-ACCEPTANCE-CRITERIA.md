# Acceptance Criteria

## Architecture

- native shell vs Rust-core ownership is documented and reflected in interfaces
- no Rust module directly owns Apple session-policy behavior
- no adapter module owns conversation state or turn logic

## Audio

- no callback-path allocation regressions in the chosen proving path
- no sustained underruns during a 10-minute normal duplex session
- interruption, route change, and permission events do not corrupt engine state

## STT

- partial/final transcription behavior is visible and cancel-safe
- segmentation behaves predictably under silence and overlap

## TTS

- playback is smooth in nominal cases
- cancel/flush on barge-in is immediate enough to feel truthful

## Duplex Interaction

- user barge-in stops assistant speech fast enough to feel natural
- assistant state does not reset after interruption
- only one active assistant turn can speak at a time

## Platform

- desktop/harness path proves the Rust engine independently
- iOS path uses native session/voice-processing ownership rather than a Rust-owned workaround

## Stability

- no crashes in a one-hour soak test with repeated turn transitions
- failures degrade visibly and traceably instead of silently falling back
