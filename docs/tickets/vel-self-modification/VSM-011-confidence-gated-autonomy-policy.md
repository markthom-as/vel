---
id: VSM-011
title: Confidence-Gated Autonomy Policy
status: proposed
priority: P1
owner: platform
labels: [policy, confidence, autonomy]
---

## Summary
Map diagnosis confidence, novelty, test coverage quality, and risk class to allowed actions.

## Why
Confidence should not be decorative telemetry. It should cash out into actual constraints.

## Scope
- Define policy table for auto-apply vs approval vs block.
- Include novelty and evidence quality.
- Emit explainable decision objects.

## Example policy axes
- confidence score
- novelty score
- changed-path risk class
- coverage completeness
- validation margin
- environment target

## Implementation tasks
1. Define scoring inputs and thresholds.
2. Implement policy evaluation function.
3. Add explanation payload for UI/debugging.
4. Add tests for edge cases and threshold transitions.
5. Integrate with apply/approval flow.

## Acceptance criteria
- Policy decisions are deterministic and explainable.
- Low-confidence high-risk edits cannot auto-apply.
- Operators can inspect why approval was required.
- Threshold regressions are covered by tests.

## Dependencies
- VSM-001, VSM-002, VSM-005.

