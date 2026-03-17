---
title: Ticket 507 - Add explainability for context, suggestions, uncertainties, and loops
status: proposed
owner: codex
priority: medium
---

# Goal

Make the runtime legible enough that you can trust it when it starts acting like an executive layer.

# Files

## Changed
- `crates/veld/src/routes/explain.rs`
- `crates/veld/src/routes/context.rs`
- `clients/web/src/components/ProvenanceDrawer.tsx`
- `clients/web/src/components/NowView.tsx`

# Add explain surfaces
- why this suggestion exists
- which evidence contributed
- which project it is about
- what uncertainty suppressed a stronger action
- which loop produced it
- whether a sync proposal is pending

# Acceptance criteria

- operator can answer "why did Vel do this?" from UI or API
- evidence and suppression reasons are inspectable
