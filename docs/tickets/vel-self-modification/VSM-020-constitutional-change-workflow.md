---
id: VSM-020
title: Constitutional Change Workflow
status: proposed
priority: P1
owner: security
labels: [governance, protected-core, approvals]
---

## Summary
Create a separate workflow for editing self-modification policy, audit behavior, protected-surface defaults, and other constitutional layers.

## Why
Normal self-improvement must not be allowed to reinterpret the rules that govern self-improvement. That is not adaptation; that is a palace coup with YAML.

## Scope
- Separate review/apply workflow.
- Stronger approval requirements.
- Signed records and explicit operator intent.

## Implementation tasks
1. Define constitutional path set and governance rules.
2. Add separate API/CLI/UI flow for these changes.
3. Require stronger approvals and signatures.
4. Record immutable approval evidence.
5. Add regression tests that normal path cannot invoke this workflow.

## Acceptance criteria
- Constitutional changes cannot travel through ordinary auto-apply path.
- Approval threshold is stronger than Class C/D.
- Audit evidence is durable and easy to inspect.
- Tests prove separation of workflows.

## Dependencies
- VSM-001, VSM-016.

