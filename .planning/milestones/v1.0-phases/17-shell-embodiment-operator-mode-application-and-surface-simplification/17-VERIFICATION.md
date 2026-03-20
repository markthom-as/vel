---
phase: 17-shell-embodiment-operator-mode-application-and-surface-simplification
verified: 2026-03-19T00:00:00Z
status: passed
score: 4/4 summary slices backed by durable closeout report
re_verification: true
---

# Phase 17: Shell embodiment, operator-mode application, and surface simplification — Verification Report

**Goal:** Apply the product taxonomy and backend-owned behavior decisions across web, Apple, and CLI shells so the operator experience is simpler and more consistent.
**Verified:** 2026-03-19
**Status:** PASSED
**Re-verification:** Yes — retroactive milestone-closeout verification

## Shipped Outcome

Phase 17 shipped the shared shell taxonomy, urgent-first `Now`, triage-first `Inbox`, archive/search-first `Threads`, secondary `Projects`, advanced `Settings`, and aligned Apple/CLI wording around the same product hierarchy.

## Evidence Sources

- [17-01-SUMMARY.md](/home/jove/code/vel/.planning/phases/17-shell-embodiment-operator-mode-application-and-surface-simplification/17-01-SUMMARY.md) through [17-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/17-shell-embodiment-operator-mode-application-and-surface-simplification/17-04-SUMMARY.md)
- [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md#L344)

## Verification Substrate

Phase summaries record web component tests throughout and final Apple/CLI checks in [17-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/17-shell-embodiment-operator-mode-application-and-surface-simplification/17-04-SUMMARY.md):

- `cargo fmt --all`
- `make check-apple-swift`
- `cargo test -p vel-cli`

## Limitations Preserved

- `Now` embodiment intentionally reflects typed backend `reflow` state rather than directly performing live reflow transitions from the shell.
- Apple verification remained package-level on Linux; full Xcode target builds still require macOS/Xcode.
- `vel-cli` tests still emitted two pre-existing `dead_code` warnings in `client.rs`.

## Summary

Phase 17 is verified as the shell-embodiment closure phase, with the closeout record preserving the remaining full-Xcode validation gap on Apple surfaces.

_Verified: 2026-03-19_
_Verifier: Codex (Phase 18 closeout backfill)_
