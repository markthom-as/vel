# Milestone v0.5.7: Hybrid Duplex Voice Runtime

**Status:** DEFERRED
**Milestone:** v0.5.7  
**Disposition:** planning packet archived without implementation execution

## Overview

`v0.5.7` was scoped as the duplex voice follow-on to the shipped `0.5.6` line.

The intended architecture remained:

- native Apple shell owns session policy, interruptions, route changes, permissions, and voice-processing I/O
- portable Rust core owns buffering, resampling, VAD, STT/TTS/LLM orchestration, turn state, and conversation policy
- desktop or harness proof should validate the same Rust-owned engine before iOS-specific quality claims

That work did not execute in this milestone. The packet below was completed as planning and requirements material only, then deferred as future work instead of being represented as shipped behavior.

## Completed In This Milestone

- milestone-level architecture, domains, threading, audio-pipeline, adapter, acceptance, validation, verification, and risk docs were assembled
- planned execution packets `101` through `105` were authored and archived under [v0.5.7-phases](/home/jove/code/vel/.planning/milestones/v0.5.7-phases)
- the duplex line was deferred explicitly into [hybrid-duplex-voice-runtime-spec.md](/home/jove/code/vel/docs/future/hybrid-duplex-voice-runtime-spec.md)

## Not Completed

- no phase execution started
- no implementation shipped
- no validation matrix was run
- no real-device or harness proof was produced

## Archived Packet

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
- [REQUIREMENTS.md](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/REQUIREMENTS.md)
- archived phase packet: [v0.5.7-phases](/home/jove/code/vel/.planning/milestones/v0.5.7-phases)

## Carried-Forward Requirement Buckets

| ID | Deferred requirement |
|----|----------------------|
| ARCH-57-01 | The hybrid native-audio / Rust-engine boundary remains future work and did not ship in `0.5.7`. |
| CORE-57-01 | The Rust speech engine seam remains future work and did not ship in `0.5.7`. |
| ADAPTER-57-01 | Platform adapter implementation remains future work and did not ship in `0.5.7`. |
| CALL-57-01 | Duplex thread call mode behavior remains future work and did not ship in `0.5.7`. |
| VERIFY-57-01 | Validation and real-device proof remain future work and did not ship in `0.5.7`. |

## Deferred Execution Packet

- Phase `101`: duplex architecture lock and contract packet
- Phase `102`: portable Rust speech engine spine
- Phase `103`: proving adapter and duplex thread loop
- Phase `104`: iOS native voice-processing bridge
- Phase `105`: duplex validation, proof, and closeout

These planned phases remain archived as deferred material only. Reopening duplex voice should start from the future spec instead of reactivating `0.5.7` as if it had shipped implementation.
