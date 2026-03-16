---
id: VSM-010
title: Failure Pattern Detector
status: proposed
priority: P1
owner: platform
labels: [detection, analytics, self-improvement]
---

## Summary
Detect repeated operator interventions, recurrent execution failures, and unresolved clusters that should trigger self-improvement proposals.

## Why
Vel should improve because reality nudged it repeatedly, not because it woke up with refactor fever.

## Scope
- Identify repeated failure signatures.
- Detect human correction loops.
- Aggregate evidence into proposal-worthy clusters.

## Candidate signals
- same exception or tool failure repeated N times
- repeated human edits to same prompt/config
- unresolved TODO block touched across tasks
- repeated connector auth absence
- recurring policy block from same cause

## Implementation tasks
1. Define normalized failure signature format.
2. Build clustering/de-duplication logic.
3. Add configurable thresholds and decay windows.
4. Emit candidate improvement events.
5. Link evidence refs into proposal creation.

## Acceptance criteria
- Similar failures de-duplicate correctly.
- Detector avoids proposal spam for transient noise.
- Evidence chains are explorable by operators.
- Thresholds are configurable and tested.

## Dependencies
- VSM-003, VSM-004.

