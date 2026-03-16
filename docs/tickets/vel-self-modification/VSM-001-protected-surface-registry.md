---
id: VSM-001
title: Protected Surface Registry
status: proposed
priority: P0
owner: platform
labels: [self-modification, governance, safety, config]
---

## Summary
Create a machine-readable registry that maps repository paths and subsystems to risk class, mutability rules, required validations, owners, and approval requirements.

## Why
Vel needs a constitutional layer that distinguishes editable seams from protected core. Without this, “self-improvement” degenerates into arbitrary patching with plausible deniability.

## Scope
- Add a versioned registry file format, preferably YAML or JSONC.
- Support glob-based path matching.
- Support classes A–E.
- Support `protected: true` and `auto_apply: true|false`.
- Support required validations and approval policies.
- Support subsystem owner metadata.

## Proposed structure
```yaml
version: 1
paths:
  "prompts/**":
    class: A
    protected: false
    auto_apply: true
    validations: [schema, lint, prompt_eval]
    owners: ["ux-ai"]
  "services/planner/**":
    class: C
    protected: false
    auto_apply: false
    validations: [lint, unit, integration, eval_regression]
    owners: ["platform"]
  "core/policy/**":
    class: E
    protected: true
    auto_apply: false
    validations: [lint, unit, integration, policy_enforcement]
    owners: ["security", "platform"]
```

## Implementation tasks
1. Define registry schema and validation rules.
2. Add parser/loader in backend runtime.
3. Add matching engine for file paths and subsystem queries.
4. Add defaults and conflict resolution rules.
5. Add tests for glob precedence and fallback behavior.

## Acceptance criteria
- Registry is checked into repo.
- Loader rejects invalid configs with actionable errors.
- Matching returns deterministic class/policy for any changed path.
- Protected paths cannot be marked auto-apply by accidental merge order.
- Unit tests cover precedence, unknown paths, and mixed-class diffs.

## Dependencies
- None.

## Follow-ups
- VSM-016 protected core enforcement tests.
- VSM-020 constitutional change workflow.

