---
title: Support ritual tasks and recurring anchors
status: ready
owner: agent
priority: P1
area: vel-task-hud
---

# Goal
Model recurring personal anchors like meds, meals, prep windows, and shutdown routines.

## Scope
Add support for ritual tasks:
- recurrence metadata
- due-window calculation
- lightweight completion history
- ranking/policy treatment distinct from generic tasks

## Examples
- take meds
- eat before long meeting block
- start pre-meeting prep
- begin shutdown routine

## Requirements
- rituals should be gently persistent, not punitive
- missing a ritual may increase risk, but not necessarily create alert spam
- rituals should participate in glance payloads

## Tests
- recurrence next-occurrence logic
- due ritual visibility
- post-completion rescheduling behavior

## Done when
- at least one ritual flow exists end-to-end
- rituals appear correctly in HUD groups

