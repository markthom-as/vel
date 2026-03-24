# Speech Pipeline

## STT Direction

- prefer `whisper-rs` behind a Vel-owned trait boundary
- enable partial transcription where it materially helps turn UX
- isolate backend-specific flags or build modes behind adapter/config seams

## Segmentation

A user utterance is closed when:

- silence exceeds the configured threshold
- or a hard timeout/turn boundary is hit
- or an explicit interruption/cancel event forces segment finalization

## TTS Direction

- TTS must be cancellable
- TTS may be native on Apple and different elsewhere
- the engine contract remains stable even if the synthesis backend changes

## Barge-In Logic

If:

- user speech is detected while assistant playback is active

Then:

- stop or cancel TTS immediately
- flush queued output
- cancel any stale response generation still bound to the interrupted turn
- promote the user capture stream to the active turn

## Latency Targets

| Stage | Target |
|------|--------|
| interruption reaction | under 150 ms |
| STT first meaningful partial | under 300 ms after speech onset, where backend allows |
| agent first useful response tokens | under 800 ms after utterance finalization |
| TTS first audio | under 300 ms after response text is available |
| total perceived turn latency | under 1.5 s for a normal local path |

These are acceptance targets, not guarantees of perfection. Missing them requires explicit audit notes rather than hand-waving.
