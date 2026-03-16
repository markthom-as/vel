---
id: vel-adaptive-config-005
title: Implement policy engine and JSON DSL
status: proposed
priority: P1
owner: backend
---

## Summary
Implement a rule-based policy engine that consumes normalized signals and emits profile activations and direct overrides.

## Scope
- create `policy/dsl.rs` and `policy/engine.rs`
- implement `all/any/not` conditions and comparison ops
- support `apply_profile`, `set`, `unset`, `set_if_absent`
- add cooldown/anti-thrash support
- store policy match evidence for auditability

## Acceptance Criteria
- policies evaluate deterministically
- matched policies return evidence payloads
- cooldown prevents oscillation spam
- invalid policies are rejected at write time

## Default Policies to Ship
- watch-low-noise
- mobile-low-power
- urgent-fast-path
- privacy-sensitive-restrictions
- deep-work-reduced-interruptions

## Tests
- DSL parser/validator tests
- evaluation tests for nested conditions
- cooldown tests
