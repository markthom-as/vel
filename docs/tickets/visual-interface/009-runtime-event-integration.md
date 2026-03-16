---
status: todo
owner: agent
priority: medium
---

# 009 — Hook runtime events into affect core

## Goal
Connect actual app/runtime events to the visual system.

## Deliverables
- event adapters
- runtime mapping table
- tests

## Instructions
1. Map:
   - listening state from user speech input
   - thinking state from tool/LLM execution
   - speaking state from TTS/output
   - warning from urgency/escalation systems
   - overload from concurrent task pressure
2. Keep adapters thin.
3. Do not pollute renderer code with business logic.

## Acceptance criteria
- Visual state updates are driven by real system activity.
- Boundaries between runtime, affect core, and renderers stay clean.
