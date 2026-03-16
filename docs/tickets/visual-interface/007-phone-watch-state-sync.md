---
status: todo
owner: agent
priority: medium
---

# 007 — Add phone/watch state sync

## Goal
Propagate compact affect packets to the watch embodiment.

## Deliverables
- sender
- receiver
- interpolation on receive
- fallback on disconnect

## Instructions
1. Target 2–4 Hz update cadence for active sessions.
2. Send immediately on meaningful events.
3. Interpolate on watch between packets.
4. If packets stop arriving, decay toward idle instead of freezing.

## Acceptance criteria
- No frame streaming in default path.
- State transitions remain smooth during packet loss or delay.
