# Formal Validation

## Lundquist-Style Validation Axes

This milestone uses multiple validation axes so the duplex system is judged on more than “it worked once.”

### 1. Structural Validation

Questions:

- do all required modules and trait boundaries exist?
- is the native-vs-Rust ownership split explicit in code and docs?
- are callback-safe primitives used on real-time paths?

### 2. Boundary Validation

Questions:

- can platform adapters feed frames and events into Rust without leaking platform policy inward?
- can Rust swap STT/TTS/LLM implementations without rewriting turn logic?
- does the adapter boundary remain narrow under iOS-specific pressure?

### 3. Behavioral Validation

Questions:

- do turns transition correctly through listen, finalize, respond, speak, interrupt, and cancel states?
- is one-active-turn truth preserved?
- does call mode share truthful thread state rather than a parallel voice-only shadow state?

### 4. Temporal Validation

Questions:

- are latency targets met or honestly recorded as misses?
- do cancellation and flush events beat stale audio playback?
- do route/interruption events recover within acceptable time?

### 5. Adversarial Validation

Questions:

- what happens during overlap, echo, or noisy background conditions?
- what happens when interruption occurs during TTS startup?
- what happens when the route changes mid-response?

### 6. Platform Validation

Questions:

- does the desktop/harness path prove the engine without pretending to be Apple?
- does the iOS path prove the native voice-processing bridge on real hardware?

### 7. Operational Validation

Questions:

- are traces, logs, and metrics sufficient to debug glitches and latency spikes?
- does failure surface clearly to the operator?

## Scenario Matrix

| Scenario | Required Result |
|---------|-----------------|
| silence | no false turn activation |
| normal user speech | transcript starts and turn finalizes normally |
| user speaks during TTS | TTS cancels and user turn takes over |
| route change during playback | playback and state recover without crash |
| interruption during capture | engine pauses or resets safely according to policy |
| long session | no cumulative drift or buffer corruption |
| degraded adapter capabilities | UI/runtime disclose degraded truth rather than faking full quality |

## Validation Principle

> The system must prove it behaves under stress, not just under demo conditions.

Validation is incomplete if it measures only the happy path.
