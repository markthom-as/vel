# Phase 101: Duplex Architecture Lock - Context

**Gathered:** 2026-03-24  
**Status:** Ready for planning

<domain>
## Phase Boundary

Lock the native-audio / Rust-engine contract before implementation work broadens. This phase is architecture and contract work, not broad feature implementation.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** iOS-native code owns session policy, voice-processing mode, interruptions, route changes, permissions, and lifecycle glue.
- **D-02:** Rust owns buffers, resampling, VAD/turn detection, STT/TTS/LLM orchestration, and turn state.
- **D-03:** Desktop or harness proving is required before Apple-specific integration is treated as the only evidence path.
- **D-04:** The callback thread must remain allocation-free and lock-free in the hot path.

## Agent Discretion

- exact trait signatures may evolve, but the ownership line must not
- the proving adapter may be a desktop path, harness path, or both as long as it exercises the same engine seam

</decisions>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/ROADMAP.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/REQUIREMENTS.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/01-ARCHITECTURE.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/02-DOMAINS.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/03-THREADING-MODEL.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/10-VALIDATION.md`

</canonical_refs>

<deferred>
## Deferred Ideas

- backend-specific STT/TTS packaging details
- final iOS bridge code
- closeout proof work

</deferred>
