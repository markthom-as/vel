# Voice Architecture

Vel voice should be a fast-turnaround execution path, not a philosophical salon.

## Pipeline

```text
wake word / push-to-talk
→ ASR
→ intent classification
→ policy check
→ action or answer
→ TTS / notification fallback
```

## Recommended Capabilities

- create reminders
- check next event
- mark commitment complete
- ask brief state questions
- route complex tasks to chat

## Constraints

- low latency
- short responses
- interruption-safe
- recover gracefully from ASR ambiguity

## Escalation Rule

If intent is ambiguous or requires structured review:
- hand off to chat surface
- preserve context from the voice turn
