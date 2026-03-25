# Milestone v0.5.7 Requirements

**Status:** IN PROGRESS
**Milestone:** v0.5.7  
**Theme:** hybrid duplex voice runtime

## Milestone Goal

Ship a duplex voice runtime that maximizes iOS compatibility without sacrificing a large, portable Rust product core.

The governing architecture is:

- thin native shell
- fat Rust core

The shell owns privileged platform audio machinery. The Rust core owns the speech engine, turn state, orchestration, and policy.

## Must-Pass Flows

- [ ] a thread-level call mode can start duplex voice interaction without resetting normal thread state
- [ ] live microphone audio can be captured, normalized, segmented, and transcribed locally
- [ ] the assistant can stream a response and speak it back
- [ ] the user can interrupt assistant speech and immediately reclaim the turn
- [ ] the conversation can continue after interruption without state corruption
- [ ] iOS route changes, interruptions, and permission/session events do not crash or wedge the engine
- [ ] the same Rust turn/state/orchestration core can run behind both an Apple-native audio adapter and a non-Apple proving adapter

## Requirement Buckets

- [ ] **ARCH-57-01**: the native shell vs Rust-core ownership line is explicit, documented, and enforced in interfaces
- [ ] **CORE-57-01**: ring buffering, resampling, VAD/turn detection, STT orchestration, LLM orchestration, TTS orchestration, conversation state, and policy are owned by Rust
- [ ] **ADAPTER-57-01**: platform adapters provide typed PCM/device-event seams without leaking platform policy into the engine
- [ ] **CALL-57-01**: duplex call mode supports listen/respond/interruption/cancel flows while keeping one truthful active turn at a time
- [ ] **VERIFY-57-01**: structural, behavioral, temporal, adversarial, and platform validation plus manual proof close the milestone honestly

## Locked Architectural Decisions

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

## Interface Contract Minimums

### AudioInput

- [ ] streams normalized PCM frames into Rust
- [ ] exposes sample-rate/channel metadata
- [ ] supports start/stop and device-event signaling
- [ ] never requires the callback path to allocate or block

### AudioOutput

- [ ] accepts PCM frames or streaming output commands
- [ ] supports immediate cancel/flush for barge-in
- [ ] exposes underrun/drop diagnostics

### SpeechToText

- [ ] supports partial and final segments
- [ ] supports cancellation on turn reset
- [ ] provides confidence/metadata where available

### TextToSpeech

- [ ] supports streaming synthesis or chunked output
- [ ] supports cancellation mid-stream
- [ ] can be implemented natively or in Rust behind the same contract

### ConversationModel

- [ ] supports streaming assistant output
- [ ] supports cancel on interruption
- [ ] preserves a truthful single-active-turn contract

## Real-Time Rules

- [ ] the audio callback thread performs no heap allocation
- [ ] the audio callback thread acquires no blocking locks
- [ ] the audio callback thread performs no async waits
- [ ] the audio callback thread performs no STT/LLM/TTS inference
- [ ] callback-to-worker handoff uses bounded lock-free or equivalent callback-safe primitives

## Validation Gates

- [ ] architecture docs and trait boundaries are written before implementation broadens
- [ ] the Rust engine can be driven end to end through a proving adapter before iOS integration is treated as complete
- [ ] iOS duplex quality is judged on a real device, not simulator-only evidence
- [ ] latency, interruption, and glitch behavior are measured directly
- [ ] failure cases include noise, overlap, interruption, route change, and long-session soak behavior

## Completion Rule

This milestone is only ready to close when the hybrid architecture remains intact, duplex voice behaves acceptably under stress, and the evidence packet demonstrates that success and failure paths were both exercised directly.
