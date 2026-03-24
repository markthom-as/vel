# Duplex Voice System - Context

## Goal

Implement a fully local, low-latency, two-way voice interaction system with:

- continuous listening during call mode
- interruptible assistant speech
- local or local-first inference for STT, LLM, and TTS orchestration
- a portable Rust speech engine
- an iOS-native audio shell where Apple has privileged machinery

## Why This Milestone Exists

The existing roadmap already expects thread-level call mode and richer multimodal assistant behavior. That requirement becomes an architectural trap if the codebase either:

- pushes all audio/session ownership into native code and leaves Rust as a thin wrapper, or
- pushes all audio/session work into Rust and then fights Apple’s voice-processing/session model

This milestone exists to prevent both mistakes.

## Non-Negotiable Design Direction

Use a thin native shell plus a fat Rust core:

- native shell handles the sharp platform edges
- Rust handles the portable engine and product logic

## Constraints

- Rust must own:
  - ring buffering and frame flow after adapter ingress
  - portable DSP that is intentionally cross-platform
  - resampling and normalization
  - VAD / turn detection
  - STT orchestration
  - TTS orchestration
  - LLM orchestration
  - conversation state / memory / tool-policy routing
- native platform layers must own:
  - audio session policy
  - voice-processing mode selection
  - interruptions
  - route changes
  - permissions
  - lifecycle glue
- real-time audio paths must:
  - never block
  - never allocate
  - never perform inference

## Non-Goals For This Milestone

- wake word
- diarization
- remote mandatory inference
- emotional prosody modeling
- full Apple-client redesign outside the duplex seam

## Definition Of Done

A user can:

1. enter a thread call mode
2. speak naturally
3. be transcribed locally
4. receive a spoken assistant response
5. interrupt the assistant mid-response
6. continue the conversation without reset or state corruption

And the system does this without:

- crashes
- runaway feedback loops
- obvious audible glitches
- hidden fallback from “full duplex” to a simpler interaction model
