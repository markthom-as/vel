---
id: TKT-010
status: proposed
title: Implement Apple voice capture into Vel inbox and task extraction pipeline
priority: P1
estimate: 5-7 days
depends_on: [TKT-003, TKT-007, TKT-008]
owner: agent
---

## Goal

Allow the user to capture fast voice notes on iPhone/watch and hand them to Vel for later parsing into tasks, reminders, or notes.

## Scope

- iPhone voice capture screen
- watch quick voice capture action
- local transcription path when available, server transcription fallback when needed
- inbox item creation with raw audio/transcript metadata
- explicit review queue before auto-creating high-confidence structured items

## Implementation notes

- Preserve raw capture plus transcript; transcripts hallucinate, and we do not need gaslighting as a feature
- Record source metadata: device, timestamp, locale, confidence
- If using on-device speech APIs, handle partial/failure states cleanly
- Build extraction as asynchronous post-processing; do not block capture UX

## Acceptance criteria

- User can record a note on iPhone and see it appear in inbox
- Watch can trigger a shortened capture flow
- Transcript/extraction status is visible
