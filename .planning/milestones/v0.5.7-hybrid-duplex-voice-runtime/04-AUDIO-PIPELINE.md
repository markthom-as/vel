# Audio Pipeline

## Pipeline Steps

1. capture
2. platform-native voice processing when available and appropriate
3. ingress buffering
4. resampling and normalization
5. VAD / turn detection
6. segment emission
7. STT
8. TTS playback buffering on the return path

## Sample-Rate Expectations

- device capture may arrive at `48kHz`
- STT path often wants `16kHz`
- TTS path is backend dependent and must declare its output rate explicitly

## Resampling Rule

All cross-rate normalization belongs in Rust so platform adapters stay thin and the engine sees consistent contracts.

## Duplex Requirement

- microphone remains live throughout assistant playback
- platform-native voice-processing or equivalent echo-managed path must run before VAD decisions when available
- barge-in decisions are made on the post-processing capture stream, not the raw speaker-contaminated stream

## Required Telemetry

- callback buffer watermark
- overflow / underrun counts
- resample latency
- frame-drop counts
- segment timing markers

## Failure Conditions

- echo loop
- drift and desynchronization
- output underrun
- input overflow
- speaker playback not being canceled fast enough after barge-in
