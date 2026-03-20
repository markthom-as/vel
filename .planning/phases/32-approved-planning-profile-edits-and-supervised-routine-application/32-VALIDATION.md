# Phase 32 Validation

## Goal

Make staged planning-profile proposals resolvable through supervised approval and canonical backend mutation without creating a second planner or silent conversational writes.

## Required Truths

- one canonical backend-owned planning-profile mutation seam remains the only durable write path
- assistant and Apple voice planning-profile proposals keep explicit lifecycle state and thread continuity
- approved proposals can apply through the canonical mutation model with validation and durable persistence
- web, CLI, Apple, and assistant/voice surfaces reflect proposal state and applied outcomes without owning planner semantics locally
- summary-first surfaces stay summary-first; `Threads` remains the durable follow-through lane

## Plan Shape

Phase 32 should be executed in four slices:

1. approved-application contract and lifecycle widening
2. backend approval/application through the canonical mutation seam
3. shipped-surface review/outcome continuity
4. docs/examples/verification closure

## Block Conditions

Block if any slice:

- creates a second planning-profile write path outside the canonical backend mutation seam
- lets assistant or voice apply profile changes without explicit supervised follow-through
- duplicates planner authority in shells or invents shell-local planning state
- widens into broad calendar editing, autonomous planner mutation, or multi-day planning

## Exit Condition

Phase 32 is complete when the product can honestly say:

- staged planning-profile proposals can be reviewed and resolved through the existing supervised model
- approved outcomes apply through the same backend-owned profile mutation seam as direct edits
- `Threads` continuity and summary surfaces reflect the real lifecycle of those changes
- docs teach the supervised planning-profile application story and its current limits
