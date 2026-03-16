---
title: Build uncertainty panel and clarification inbox
status: todo
priority: P1
owner: frontend
labels: [uncertainty, ui, inbox]
---

# Goal

Expose uncertainty as a first-class surface in Vel's UI.

# Deliverables

- `packages/ui/uncertainty-panel/UncertaintyPanel.tsx`
- `packages/ui/uncertainty-panel/ClarificationInbox.tsx`
- task-level display of confidence vector, open uncertainties, active assumptions, and pending clarifications

# Requirements

- Panel should support compact and expanded modes.
- Clarification inbox should make it obvious what is blocking, what is optional, and what Vel recommends.
- Show whether a question is routed to user, agent, retrieval, or validation.
- Keep noise low; rank by severity and decision relevance.

# Acceptance criteria

- UI can render mixed resolver states.
- A blocking uncertainty is visually distinct from a low-priority note.
- Snapshot or component tests cover ranking and empty states.

# Notes

Do not make this look like a guilt dashboard. It should feel useful, not like Vel is performing anxiety.
