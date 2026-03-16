---
title: Instrument uncertainty telemetry and calibration feedback
status: todo
priority: P2
owner: analytics
labels: [uncertainty, telemetry, calibration]
---

# Goal

Measure whether Vel's uncertainty handling is actually useful instead of just ornate.

# Deliverables

- telemetry schema for uncertainty events
- counters for resolver usage and effectiveness
- basic calibration dashboard or log output for internal debugging
- rework correlation tracking where feasible

# Requirements

Track at minimum:
- uncertainty kind frequency
- proceed vs ask vs block decisions
- user interruption rate
- agent consultation rate
- answer usefulness / resolution success
- false proceed and false block incidents

# Acceptance criteria

- Events are emitted for every policy decision.
- At least one analysis view or report summarizes calibration drift.
- Privacy/retention implications are documented.

# Notes

Without telemetry, confidence tuning becomes artisanal superstition.
