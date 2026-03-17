---
title: Vel Technical Loops Pack
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Vel Technical Loops Pack

This pack turns Vel from a mostly request/response system with a tiny worker into a real temporal runtime with explicit loops.

## What "technical loops" means here

Not vague autonomy. Concrete deterministic loops:
- sync loop
- evaluate loop
- retry loop
- drift detection loop
- synthesis loop
- cleanup/reconciliation loop

## Existing starting point

- `crates/veld/src/worker.rs` currently handles capture-ingest and retry-ready runs
- `POST /v1/evaluate` exists, but evaluation is still mostly operator-triggered
- current context, risk, nudge, and suggestion systems already exist

## Pack contents

1. `00-spec-technical-loops.md`
2. `tickets/001-loop-registry-and-kinds.md`
3. `tickets/002-scheduler-and-claims.md`
4. `tickets/003-evaluate-loop.md`
5. `tickets/004-sync-loop.md`
6. `tickets/005-synthesis-and-review-loops.md`
7. `tickets/006-loop-observability-and-control.md`
8. `tickets/007-tests.md`
