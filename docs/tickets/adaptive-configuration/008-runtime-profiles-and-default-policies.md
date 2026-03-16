---
id: vel-adaptive-config-008
title: Ship conservative runtime profiles and default policies
status: proposed
priority: P2
owner: product-backend
---

## Summary
Create built-in profiles and a conservative set of system policies so the subsystem is useful before users author custom rules.

## Scope
- add profile registry in `config/profiles.rs`
- define `watch`, `mobile`, `voice`, `urgent`, `privacy_sensitive`, `low_resource`, `deep_work`
- define default system policies aligned to available signals

## Acceptance Criteria
- profiles are versioned and test-covered
- policies only touch supported v1 keys
- policies are easy to disable individually
- docs explain intended tradeoffs for each profile

## Tests
- golden tests for each profile
- scenario tests for default policies
