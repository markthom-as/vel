---
id: APPLE-009
title: Add voice capture entrypoint for quick logging
status: proposed
owner: agent
priority: p2
area: voice
depends_on: [APPLE-004]
---

# Goal

Create the first Apple-side voice entrypoint for quick capture of user state or command fragments.

# Scope for MVP

- tap-to-record flow on iPhone
- optional watch handoff / trigger later
- store audio/transcript job metadata
- enqueue parse/interpret request into Vel pipeline
- present transcript + parsed action preview when available

# Requirements

- isolate voice logic in `VelAppleVoice`
- permission handling
- failure states for offline / denied mic / transcription unavailable
- preserve raw capture provenance in event log

# Non-goals for MVP

- full conversational assistant on watch
- always-on wake word
- elaborate duplex voice UX

# Acceptance criteria

- user can record a short clip
- clip generates local event / pending processing state
- transcript result can be associated with capture
- parsed suggestion or action can be reviewed before commit if the pipeline supports it
