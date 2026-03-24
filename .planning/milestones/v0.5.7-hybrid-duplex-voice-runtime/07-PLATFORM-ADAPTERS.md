# Platform Adapters

## iOS / Apple Path

### Native Responsibilities

- configure `AVAudioSession`
- choose voice/voice-processing mode
- react to interruptions
- react to route changes and Bluetooth changes
- request and reflect microphone permission
- bridge PCM frames and device events into Rust

### Rust Boundary

Rust receives:

- normalized PCM frames
- output sink contract
- explicit device/interruption events

Rust does not:

- choose the session category or mode
- drive permission prompts
- directly own route switching

## Desktop / Proving Path

Desktop exists to prove the engine and to support non-Apple execution where practical. It does not need to pretend it has Apple-grade voice processing.

Likely direction:

- `cpal` or equivalent low-level adapter for audio I/O
- optional software echo-management only if it stays honest about quality limits

## Adapter Rules

- adapters stay thin
- adapters do not own turn state
- adapters do not own conversation policy
- adapters may emit device capability flags so the engine and UI can disclose degraded operation truthfully
