# Phase 102: Rust Speech Engine Spine - Context

**Gathered:** 2026-03-24  
**Status:** Ready for planning

<domain>
## Phase Boundary

Build the Rust-owned speech-engine spine independently of Apple session policy.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** engine contracts must remain adapter-driven and platform-agnostic
- **D-02:** resampling, normalization, VAD/turn detection, and turn state belong in Rust
- **D-03:** STT, TTS, and local LLM backends sit behind Vel-owned traits
- **D-04:** cancellation semantics are first-class, not bolted on after streaming exists

</decisions>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/02-DOMAINS.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/03-THREADING-MODEL.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/04-AUDIO-PIPELINE.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/05-SPEECH-PIPELINE.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/06-AGENT-LOOP.md`

</canonical_refs>
