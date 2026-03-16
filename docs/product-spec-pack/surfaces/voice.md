# Surface Spec: Voice

## Intent

Ambient interaction layer.

Voice is designed for:

- short commands
- reminders
- quick queries

## Interaction Model

wake word
↓
intent detection
↓
short response

Example:

User:

"Vel, remind me to take my pills before my meeting"

Vel:

"Reminder set for 9:15."

## Constraints

- response time < 5 seconds
- short responses
- fallback to notification if ambiguous