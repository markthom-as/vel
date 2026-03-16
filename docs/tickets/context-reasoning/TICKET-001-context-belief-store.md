---
title: Implement Context Belief Store
status: open
---

## Boundary

This ticket must extend the existing context/explain runtime, not create a competing source of truth.

- The current runtime authority for present-tense state remains `current_context`, `context_timeline`, and the explain routes.
- If a new store is introduced, it should be scoped to inspection, uncertainty, or feedback support around that runtime.
- Do not land a second independently authoritative context model in parallel.
- Prefer extending existing context/explain persistence first; add new tables only when the current runtime cannot carry the needed inspection metadata cleanly.

# Goal

Create the minimum persistent structure needed to support inspectable context beliefs or uncertainty metadata around the existing context runtime.

# Tasks

1. First evaluate whether the needed metadata can live in `current_context`, `context_timeline`, or explain-supporting storage.
2. If a new schema is still needed, scope it to inspectability or uncertainty support around the existing reducer output.
3. Implement confidence ranking queries for inspection or explanation use.
4. Implement expiration logic only for scoped supporting entries if they are introduced.
5. Implement suppression flags only if they integrate directly with current context explanation or feedback flows.

# Acceptance Criteria

- Any new storage clearly extends the existing context runtime instead of replacing it.
- The authoritative present-tense context still comes from the existing reducer/runtime.
- Stored entries are retrievable and explainable.
- Expired entries are auto-invalidated if expiration is supported.
- Confidence sorting is supported where relevant.
