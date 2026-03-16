---
id: vel-adaptive-config-003
title: Build deterministic effective config resolver
status: proposed
priority: P0
owner: backend
---

## Summary
Implement the merge engine that resolves defaults, scoped user settings, profiles, policy overrides, session overrides, and hard constraints into one effective config with provenance.

## Scope
- create `config/resolver.rs`
- implement precedence rules from spec
- support scope narrowing and tie-breaks
- emit provenance entries per resolved key
- hash final config deterministically

## Acceptance Criteria
- same inputs always yield same output and hash
- precedence is covered by tests
- provenance identifies source kind and source id per key
- no-op resolutions do not create changed snapshots

## Pseudocode Target
1. load defaults
2. load scoped settings
3. apply profiles
4. apply policy overrides
5. apply session overrides
6. apply hard constraints
7. validate final config
8. return config + provenance + hash

## Tests
- unit tests for precedence
- property test for determinism
- scope conflict tests
