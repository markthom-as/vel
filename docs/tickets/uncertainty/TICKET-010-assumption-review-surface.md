---
title: Build assumption review surface
status: todo
priority: P2
owner: frontend
labels: [uncertainty, assumptions, ui]
---

# Goal

Give users a clean way to inspect and confirm/reject assumptions Vel is actively using.

# Deliverables

- `packages/ui/uncertainty-panel/AssumptionReview.tsx`
- actions: confirm, reject, edit, pin as preference
- linkage from assumptions to source uncertainty and evidence

# Requirements

- Distinguish user-stated facts from inferred assumptions.
- Show reversibility and confidence.
- Allow fast batch confirmation for low-risk assumptions if UX warrants.

# Acceptance criteria

- User actions update the underlying ledger.
- Rejected assumptions invalidate or recompute dependent plan state.
- UI clearly displays provenance and effect radius.

# Notes

Assumptions are where Vel's desire to complete the task meets the limits of the symbolic. Surface them.
