# Future Spec: Hybrid Duplex Voice Runtime

**Status:** future-only  
**Origin:** deferred from milestone `v0.5.7` on 2026-03-26

This document preserves the duplex voice line as explicit future work. It is not proof that any duplex implementation has shipped.

## Why It Was Deferred

`v0.5.7` assembled a serious planning packet, but no implementation phases executed and no proof packet was produced. Closing that milestone honestly required moving the unfinished work out of active planning and into `docs/future/`.

## Intended Product Outcome

When this work is reopened, Vel should support truthful duplex thread call mode while preserving the intended ownership split:

- native Apple shell owns session policy, route changes, interruptions, permissions, and voice-processing I/O
- portable Rust core owns buffering, resampling, VAD, STT/TTS/LLM orchestration, turn state, and conversation policy
- non-Apple proof uses adapters around the same Rust engine instead of inventing a second implementation path

## Carried-Forward Requirements

| ID | Future requirement |
|----|--------------------|
| ARCH-57-01 | Keep the native Apple audio/session boundary explicit and narrow. |
| CORE-57-01 | Build a Rust-owned speech engine seam for buffers, resampling, VAD, orchestration, cancellation, and state. |
| ADAPTER-57-01 | Feed typed PCM and device events through platform adapters without leaking session policy into Rust. |
| CALL-57-01 | Support duplex call mode with interruption, barge-in, and recovery without shadow thread state. |
| VERIFY-57-01 | Close the future line only with executed validation plus real harness and real-device proof. |

## Reopen Order

1. Reconfirm that the hybrid ownership split still matches the current Apple and Rust architecture.
2. Recreate the architecture and validation lock as milestone-local `Phase 01`.
3. Implement the portable Rust speech-engine spine before any iOS-specific bridge claims.
4. Prove the engine on a harness or desktop adapter before treating iOS work as complete.
5. Require real-device evidence before any future closeout.

## Archived Planning Inputs

The deferred planning packet remains available for reuse:

- [v0.5.7 milestone roadmap](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/ROADMAP.md)
- [v0.5.7 requirements archive](/home/jove/code/vel/.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/REQUIREMENTS.md)
- [v0.5.7 archived phases](/home/jove/code/vel/.planning/milestones/v0.5.7-phases)

Treat those artifacts as source material, not active scope.
