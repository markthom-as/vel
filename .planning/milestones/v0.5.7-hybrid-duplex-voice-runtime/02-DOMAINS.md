# Domain Model

## Core Traits

### AudioInput

Purpose:
- feed normalized PCM frames into the engine

Contract:
- fixed frame cadence or explicit frame timestamps
- explicit sample rate and channel count
- callback-safe delivery
- device-event propagation out of band

### AudioOutput

Purpose:
- consume synthesized PCM or playback commands

Contract:
- append audio frames for playback
- support stop, flush, and cancel-now semantics
- expose underrun and buffer-pressure diagnostics

### SpeechToText

Purpose:
- convert a live segment stream into partial/final text

Contract:
- supports cancellation
- supports partial plus final output
- exposes confidence/metadata when available

### TextToSpeech

Purpose:
- synthesize assistant output into playable audio

Contract:
- chunked or streaming synthesis
- explicit cancellation mid-stream
- neutral to implementation source: native or Rust

### ConversationModel

Purpose:
- produce assistant response streams from user turns

Contract:
- stream text or structured response tokens
- cancel on interruption
- remain ignorant of platform audio policy

## Core Entities

### Frame

- monotonic timestamp
- sample rate
- channels
- frame count
- PCM payload

### Segment

- start time
- end time
- partial/final status
- transcript text
- confidence or confidence-unavailable marker

### Turn

- turn id
- trigger mode
- user transcript or capture artifact reference
- assistant response state
- cancellation/interruption metadata

### DeviceEvent

- interruption began/ended
- route changed
- permissions changed
- output path changed
- voice-processing availability changed

## Invariants

- frames are processed in timestamp order
- one active assistant turn exists at a time
- cancellation is explicit and terminal for the canceled turn
- platform events do not mutate conversation state directly; they enter through the engine boundary
