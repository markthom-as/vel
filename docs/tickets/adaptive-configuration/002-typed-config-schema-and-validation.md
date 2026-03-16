---
id: vel-adaptive-config-002
title: Implement typed config schema and key validation
status: proposed
priority: P0
owner: backend
---

## Summary
Define the set of supported config keys, their allowed types, default values, and validation rules.

## Scope
- create `config/schema.rs` and `config/types.rs`
- define enums/structs for keys and domains
- implement value validation and coercion rejection
- expose defaults registry

## Acceptance Criteria
- invalid keys are rejected
- invalid value types are rejected with structured errors
- defaults exist for all supported keys
- keys can be grouped by domain for UI and explainability

## Suggested v1 Keys
- `llm.model`
- `llm.reasoning_depth`
- `context.retrieval_depth`
- `context.max_budget_tokens`
- `autonomy.level`
- `autonomy.low_confidence_threshold`
- `ux.verbosity`
- `ux.proactive_nudges`
- `privacy.external_tool_access`
- `resource.low_power_mode`

## Tests
- table-driven validation tests
- default registry completeness test
