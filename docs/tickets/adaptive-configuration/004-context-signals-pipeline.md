---
id: vel-adaptive-config-004
title: Add normalized context signals pipeline
status: proposed
priority: P1
owner: backend
---

## Summary
Introduce a normalized context-signal layer so policies do not depend on random strings leaking from every subsystem.

## Scope
- create `signals/mod.rs` and `signals/normalize.rs`
- support normalized signals for surface, urgency, battery, privacy mode, focus mode, calendar busyness
- store observed signals with source, confidence, TTL, trace id
- provide query interface for current active signals

## Acceptance Criteria
- signals are normalized before policy evaluation
- inferred signals include confidence
- expiring signals respect TTL
- conflicting signals are logged and resolved predictably

## Notes
Start narrow. Surface + urgency + battery + privacy already buys a lot.

## Tests
- normalization tests
- TTL expiry tests
- source attribution tests
