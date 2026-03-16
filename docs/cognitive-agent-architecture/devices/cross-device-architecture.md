# Cross-Device Architecture

Vel should behave like one system with multiple surfaces, not five cousins who never text back.

## Supported Surfaces

- CLI
- desktop chat
- mobile chat
- voice
- watch notifications
- dashboard

## Cross-Device Principles

- shared state, surface-specific presentation
- action continuity across devices
- notification deduplication
- graceful escalation between surfaces

## Example

Watch:
"Medication due before 10:00 meeting"

Tap:
"Snooze 10m"

Phone opens:
detailed context + confirm flow

Desktop chat shows:
updated commitment and snooze history
