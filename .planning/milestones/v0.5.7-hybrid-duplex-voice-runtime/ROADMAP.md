# Milestone v0.5.7: Hybrid Duplex Voice Runtime

**Status:** IN PROGRESS
**Milestone:** v0.5.7  
**Theme:** native Apple audio shell + portable Rust speech engine

## Overview

`v0.5.7` is now active as the follow-on to the closed `0.5.6` single-node MVP line.

Its purpose is to add truthful duplex voice interaction without collapsing into either of the two bad architectural extremes:

- an all-native stack where Rust becomes decorative
- an all-Rust audio stack that fights Apple’s session, route, interruption, and voice-processing machinery

This milestone therefore locks and implements a hybrid design:

- native Apple shell owns audio-session policy, route/interruption handling, permissions, and voice-processing I/O
- portable Rust core owns the speech engine, turn state, orchestration, and policy
- desktop/non-Apple proving paths use platform adapters around the same Rust interfaces instead of inventing a second brain

## Primary Inputs

- [00-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/00-CONTEXT.md)
- [01-ARCHITECTURE.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/01-ARCHITECTURE.md)
- [02-DOMAINS.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/02-DOMAINS.md)
- [03-THREADING-MODEL.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/03-THREADING-MODEL.md)
- [04-AUDIO-PIPELINE.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/04-AUDIO-PIPELINE.md)
- [05-SPEECH-PIPELINE.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/05-SPEECH-PIPELINE.md)
- [06-AGENT-LOOP.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/06-AGENT-LOOP.md)
- [07-PLATFORM-ADAPTERS.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/07-PLATFORM-ADAPTERS.md)
- [08-PHASES.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/08-PHASES.md)
- [09-ACCEPTANCE-CRITERIA.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/09-ACCEPTANCE-CRITERIA.md)
- [10-VALIDATION.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/10-VALIDATION.md)
- [11-VERIFICATION.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/11-VERIFICATION.md)
- [12-RISKS.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/12-RISKS.md)
- [13-NEXT-STEPS.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/13-NEXT-STEPS.md)
- [clients/apple](/home/jove/code/vel/clients/apple)

## In Scope

- lock the native-shell / Rust-core duplex voice architecture as a formal boundary
- define and implement a portable Rust speech-engine seam for buffers, resampling, VAD, STT, TTS orchestration, LLM orchestration, and turn state
- add a desktop or harness-grade adapter path that proves the Rust engine without depending on Apple-specific hardware first
- add an iOS-native audio/session bridge using Apple voice-processing/session APIs where privileged machinery is required
- integrate truthful duplex call-mode behavior into the assistant/thread flow with barge-in, interruption, and cancellation semantics
- add formal structural, behavioral, temporal, adversarial, and platform validation for the duplex path

## Out of Scope

- wake word as a milestone requirement
- diarization or multi-speaker attribution
- distributed inference or remote mandatory dependencies
- broad Apple-client redesign outside the duplex voice seam
- new top-level surfaces beyond the existing thread/call-mode entry points
- speculative emotional/prosody systems beyond what a chosen TTS backend naturally provides
- pretending software AEC on desktop is equivalent to Apple voice-processing quality

## Requirement Buckets

| ID | Description |
|----|-------------|
| ARCH-57-01 | The hybrid native-audio / Rust-engine boundary is explicit, documented, and narrow enough to keep platform policy native and product logic portable. |
| CORE-57-01 | The Rust speech engine owns buffers, resampling, VAD/turn detection, STT/TTS/LLM orchestration, and conversation state without doing privileged platform-session work. |
| ADAPTER-57-01 | Platform adapters implement typed input/output seams so iOS can use native voice-processing while desktop proving can use a lower-level cross-platform path. |
| CALL-57-01 | Duplex thread call mode supports streaming listen/respond, cancellation, and barge-in without resetting conversation state. |
| VERIFY-57-01 | Formal validation, execution-backed verification, and real-device/manual proof close the milestone honestly. |

## Planned Phases

### Phase 101: Duplex architecture lock and contract packet

**Goal:** lock the hybrid architecture before implementation normalizes bad boundaries.  
**Depends on:** `0.5.6` closeout or explicit approval to plan ahead  
**Status:** NOT STARTED

Expected outcomes:

- trait boundaries exist for `AudioInput`, `AudioOutput`, `SpeechToText`, `TextToSpeech`, and `ConversationModel`
- native-vs-Rust ownership is explicit for iOS session policy, route/interruption handling, permissions, buffering, orchestration, and state
- real-time threading and no-allocation/no-lock callback rules are documented as hard constraints
- acceptance and validation gates exist before code broadens

### Phase 102: Portable Rust speech engine spine

**Goal:** build the Rust-owned engine seam independent of Apple-specific session policy.  
**Depends on:** Phase 101  
**Status:** QUEUED

Expected outcomes:

- ring-buffered audio ingress/egress with fixed frame contracts
- Rust-owned resampling, normalization, VAD/turn detection, and segment lifecycle
- `whisper-rs` STT abstraction and isolated local-LLM adapter seam
- cancellable TTS abstraction and turn manager that can stop playback and inference cleanly

### Phase 103: Proving adapter and duplex thread loop

**Goal:** prove the engine end to end on a non-privileged adapter path before adding iOS-specific machinery.  
**Depends on:** Phase 102  
**Status:** QUEUED

Expected outcomes:

- harness or desktop adapter can feed PCM into the Rust engine and play synthesized output back
- assistant thread/call-mode flow uses the same turn manager as future iOS call mode
- barge-in, interruption, and single-active-turn semantics work in execution-backed tests
- traces/metrics exist for latency, turn transitions, cancellation, and underruns

### Phase 104: iOS native voice-processing bridge

**Goal:** add the Apple-quality duplex path without moving product logic out of Rust.  
**Depends on:** Phase 103  
**Status:** QUEUED

Expected outcomes:

- Swift/Obj-C shell owns `AVAudioSession` policy, voice mode, route changes, interruptions, permissions, and PCM bridging
- Apple-native voice-processing path feeds normalized PCM into Rust and receives synthesized PCM or playback commands back
- iOS call mode can survive interruption, route change, and output-device changes without corrupting turn state
- Rust stays ignorant of session-policy details while still receiving explicit device-event signals at the adapter boundary

### Phase 105: Duplex validation, proof, and closeout

**Goal:** close the line only if duplex voice works under success and stress cases.  
**Depends on:** Phase 104  
**Status:** QUEUED

Expected outcomes:

- formal validation matrix is executed, not just written
- latency, glitch, and cancellation targets are measured and recorded
- browser/app/manual proof exists for desktop and at least one real iOS device path
- deferred items are explicit rather than hidden under “works on my machine”

## Execution Order

Planned sequence:

`101 -> 102 -> 103 -> 104 -> 105`

## Acceptance Standard

`v0.5.7` closes only when:

- duplex call mode works without fake resets or “push to talk” simplifications hidden as full duplex
- iOS-native audio session and voice-processing responsibilities remain outside the Rust core
- the Rust speech engine remains the single source of truth for turn state, orchestration, and policy
- formal validation proves behavior under interruption, route change, overlap, and latency stress
- any residual debt is explicitly deferred with rationale
