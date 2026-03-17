# Vel Repo Feedback Tickets — 2026-03-15

This packet contains repo-review tickets based on the uploaded `vel-main` snapshot.

## Summary

The repo is materially stronger than earlier iterations. The main theme now is:

**finish convergence before adding breadth.**

The highest-priority work is tightening architectural boundaries among:
- evaluation
- persisted truth
- explainability
- risk
- inference
- nudge lifecycle

The web client is promising, but it needs a stronger data layer and less template residue.

## Current convergence state

Use [docs/status.md](../../status.md) as the canonical implementation ledger.

Current practical state in this packet:

- `001` read/evaluate boundary: materially implemented and covered by a non-mutation integration test
- `002` inference reducers: in progress; deterministic selectors and staged collect/derive/persist flow are underway
- `003` risk authority: in progress; typed risk contracts are in place and consumers are moving onto them
- `004` nudge lifecycle: in progress; reactivation and in-place escalation are implemented, broader policy cleanup remains
- `005` API/time contract cleanup: in progress; websocket timestamps are now RFC3339 and typed risk factors are explicit
- `006` web realtime/state management: intentionally deferred for now
- `007` `008` `009`: next repo-feedback priority after current convergence work

## Ticket order

1. `001-enforce-evaluate-read-boundary.md`
2. `002-refactor-inference-into-deterministic-reducers.md`
3. `003-complete-risk-engine-and-make-it-the-only-risk-authority.md`
4. `004-make-nudge-lifecycle-idempotent-escalatable-and-policy-driven.md`
5. `005-normalize-api-types-time-fields-and-generated-client-contracts.md`
6. `006-harden-web-client-state-management-and-realtime-sync.md`
7. `007-clean-up-web-shell-and-remove-starter-template-residue.md`
8. `008-establish-build-ci-and-repo-truth-checks.md`
9. `009-rationalize-docs-status-and-implementation-roadmap.md`
