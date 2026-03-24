# System Architecture

## High-Level Layers

1. Platform Layer
2. Real-Time Audio Layer
3. Speech Engine Layer
4. Agent / Conversation Layer

## Data Flow

Mic -> platform voice processing / raw capture -> Rust ingress ring buffer -> resample / normalize -> VAD / segmentation -> STT -> turn manager -> LLM / tools -> TTS -> output buffer -> platform playback

## Ownership Table

| Layer | Primary Language | Responsibility |
|------|-------------------|----------------|
| Platform session shell | Swift / Obj-C on Apple, adapter-specific elsewhere | session policy, permissions, route/interruption/device events, privileged audio I/O |
| Real-time transport | Rust | callback-safe buffering, frame contracts, telemetry handoff |
| Speech engine | Rust | resampling, normalization, VAD, STT/TTS orchestration, turn management |
| Agent layer | Rust | model orchestration, tool calls, memory, policy, conversation state |

## Architectural Principle

> Real-time is not where you think.  
> Intelligence is not where you capture.

The callback path exists to move frames safely.

The engine path exists to interpret them.

The agent path exists to decide what to do.

Conflating those responsibilities is how underruns, deadlocks, and incoherent turn state get normalized into architecture.

## Hybrid Boundary Rules

### Native Shell Must Own

- Apple session category/mode and voice-processing selection
- interruption responses
- Bluetooth/headphones/speaker routing
- permission prompts
- app lifecycle coupling
- any hardware/OS-specific AEC reference plumbing

### Rust Core Must Own

- frame contracts after ingress
- buffer topology
- resampling
- turn detection/VAD logic
- STT and TTS orchestration
- cancel/flush behavior
- single-active-turn state machine
- conversation memory, tools, and policy

## Success Shape

The system should feel like one voice product with one state machine, even though the ears and mouth remain native where the platform demands it.
