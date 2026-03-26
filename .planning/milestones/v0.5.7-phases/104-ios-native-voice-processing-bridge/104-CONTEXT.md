# Phase 104: iOS Native Voice-Processing Bridge - Context

**Gathered:** 2026-03-24  
**Status:** Ready for planning

<domain>
## Phase Boundary

Integrate the Rust engine with a native Apple audio shell while preserving the ownership contract from Phase 101.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** iOS integration must use native session-policy ownership
- **D-02:** Rust receives PCM frames and explicit device events; it does not manage session categories or permissions
- **D-03:** route/interruption changes are acceptance-level behaviors, not edge trivia

</decisions>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/01-ARCHITECTURE.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/07-PLATFORM-ADAPTERS.md`
- `.planning/milestones/v0.5.7-hybrid-duplex-voice-runtime/09-ACCEPTANCE-CRITERIA.md`

</canonical_refs>
