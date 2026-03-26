# Requirements Archive: v0.5.7 hybrid-duplex-voice-runtime

**Archived:** 2026-03-26
**Status:** DEFERRED

`v0.5.7` did not complete implementation. These requirements remain carried-forward future work, not shipped behavior.

## Completed In `0.5.7`

- milestone-level duplex architecture and acceptance packet creation
- authored phase plans `101` through `105`
- explicit deferral of the duplex line into [hybrid-duplex-voice-runtime-spec.md](/home/jove/code/vel/docs/future/hybrid-duplex-voice-runtime-spec.md)

## Deferred Requirement Buckets

- [ ] **ARCH-57-01**: the native shell vs Rust-core ownership line is explicit, documented, and enforced in interfaces
- [ ] **CORE-57-01**: ring buffering, resampling, VAD/turn detection, STT orchestration, LLM orchestration, TTS orchestration, conversation state, and policy are owned by Rust
- [ ] **ADAPTER-57-01**: platform adapters provide typed PCM/device-event seams without leaking platform policy into the engine
- [ ] **CALL-57-01**: duplex call mode supports listen/respond/interruption/cancel flows while keeping one truthful active turn at a time
- [ ] **VERIFY-57-01**: structural, behavioral, temporal, adversarial, and platform validation plus manual proof close the milestone honestly

## Deferred Locked Architectural Decisions

- [ ] Rust must not own the entire iOS audio/session stack
- [ ] iOS-native shell owns `AVAudioSession` setup and mode selection
- [ ] iOS-native shell owns interruption handling
- [ ] iOS-native shell owns Bluetooth, route, and speaker changes
- [ ] iOS-native shell owns permission prompts and app lifecycle glue
- [ ] iOS-native shell owns Apple voice-processing / echo-cancelled duplex I/O choices
- [ ] Rust owns ring buffers and frame movement after adapter ingress
- [ ] Rust owns optional DSP that is truly meant to be portable
- [ ] Rust owns resampling and frame normalization
- [ ] Rust owns VAD / turn detection
- [ ] Rust owns STT orchestration
- [ ] Rust owns LLM orchestration
- [ ] Rust owns TTS orchestration even if a platform-native TTS adapter is used
- [ ] Rust owns conversation state, tools, memory, and policy
- [ ] `whisper-rs` is the preferred portable local STT seam
- [ ] the local LLM path must sit behind a Vel-owned trait and treat `llama.cpp` bindings as an adapter, not the architectural center
- [ ] TTS is abstracted behind a trait, with native Apple TTS remaining an acceptable product-first implementation on iOS
- [ ] desktop or proving adapters may use `cpal`, but that does not replace the native Apple shell on iOS

## Deferred Completion Rule

Do not treat any duplex voice behavior as shipped from `0.5.7`. Reopen this line only through a future milestone that explicitly implements and verifies the carried-forward work in [hybrid-duplex-voice-runtime-spec.md](/home/jove/code/vel/docs/future/hybrid-duplex-voice-runtime-spec.md).
