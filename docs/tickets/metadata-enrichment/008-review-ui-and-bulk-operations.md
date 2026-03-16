---
id: VEL-META-008
title: Review queue UI object detail and bulk enrichment operations
status: proposed
priority: P1
estimate: 4-5 days
dependencies: [VEL-META-005, VEL-META-007]
---

# Goal

Give the user a credible control surface for enrichment proposals.

# Scope

- Review queue page/table.
- Object detail view with current metadata, gaps, candidates, and action history.
- Bulk apply/reject with filters.
- Inline edit-before-apply.
- “Always allow / never do this” controls.

# Deliverables

- UI components/pages
- queue API integration
- optimistic update handling
- empty/error/loading states

# Acceptance criteria

- User can review, edit, apply, reject, and bulk-process proposals.
- Confidence and reasons are visible.
- Bulk actions are bounded and previewed.
- Changes reflect in queue and audit state.

# Notes

Make it feel less like spam triage and more like a competent operations console.
