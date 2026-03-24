# Development Phases

## Phase 101 - Duplex Architecture Lock

Purpose:
- freeze the hybrid boundary before implementation drift begins

Outputs:
- trait set
- ownership map
- threading rules
- validation gates

## Phase 102 - Rust Speech Engine Spine

Purpose:
- build the engine core that every adapter uses

Outputs:
- rings and worker topology
- resampling and normalization path
- VAD / turn manager
- STT / TTS / LLM trait adapters

## Phase 103 - Proving Adapter And Duplex Loop

Purpose:
- prove the engine in a non-Apple path first

Outputs:
- executable harness or desktop adapter
- thread call-mode integration
- interruption and single-active-turn behavior
- traceable metrics

## Phase 104 - iOS Native Voice Bridge

Purpose:
- add the Apple-quality duplex path without moving logic out of Rust

Outputs:
- native session shell
- voice-processing bridge
- interruption / route event mapping
- real-device duplex execution

## Phase 105 - Validation, Proof, And Closeout

Purpose:
- treat stress behavior as part of the product, not postscript

Outputs:
- formal validation evidence
- latency and soak metrics
- real-device proof notes
- deferred-work ledger if anything remains
