# vel_voice_interaction_spec.md

Status: Voice interaction specification for Vel  
Purpose: Define how voice interfaces interact with Vel core without duplicating logic.

## Principles
1. Voice is a **client**, not a brain.
2. All voice commands map to existing Vel actions (CLI/API).
3. Push‑to‑talk first; wake word later.
4. Responses are layered: short → expanded → explain.

## Command Categories

### Command & Control
Examples:
- "What matters now?" → vel context
- "What's next?" → vel context next_commitment
- "Mark meds done." → vel done com_meds
- "Snooze that 10 minutes." → vel snooze <id> 10

### Capture
Examples:
- "Capture: idea for Vel risk tuning."
- "Note: Dimitri prefers morning meetings."

Maps to:
POST /v1/captures

### Explanation
Examples:
- "Why is that high risk?"
- "Why are you warning me?"

Maps to:
vel explain context
vel risk <id>

### Morning Orchestration
User: "Good morning."

Vel response order:
1. Next commitment
2. Risk highlights
3. Required actions
4. Optional explanation

## Confirmation Rules
High‑risk actions require confirmation:
- cancelling commitments
- modifying policy values
- sending external messages

Low‑risk actions do not:
- done
- snooze
- capture
- query context

## Output Layering

Level 1: concise answer  
Level 2: expanded reasoning  
Level 3: full explain context

## Client Responsibilities

### Desktop Voice
- push‑to‑talk
- speech recognition
- TTS
- command parsing → Vel actions

### iPhone
- voice capture
- context queries
- explanation display

### Watch
- minimal voice capture
- confirmation or quick command

## Action Mapping

Voice → Action → Vel Core

Examples:

"mark meds done"
→ action: done
→ target: com_meds

"what matters now"
→ action: query_context

"snooze ten minutes"
→ action: snooze
→ duration: 10

Voice layer must translate to canonical action schema.
