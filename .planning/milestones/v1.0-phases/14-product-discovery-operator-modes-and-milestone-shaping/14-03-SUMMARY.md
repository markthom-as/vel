# 14-03 Summary

## Outcome

Completed the Phase 14 operator-mode and progressive-disclosure policy slice.

This slice now has one canonical policy document that resolves the main open disclosure questions:

- default mode should stay minimal and ADHD-friendly
- `Now` should only show direct action cards for genuinely urgent items
- `Inbox` remains the explicit triage queue
- `Threads` remains archive/search-first and becomes durable mainly for meaningfully multi-step flows
- `Projects` stays secondary in navigation but may still own project-specific actions
- `reflow` is heavier than routine nudges or normal `check_in`
- `check_in` stays inline by default but may become blocking in bounded cases

## Main Artifacts

- `docs/product/operator-mode-policy.md`
- `docs/product/operator-surface-taxonomy.md`
- `docs/product/operator-action-taxonomy.md`
- `docs/product/now-inbox-threads-boundaries.md`
- `clients/apple/README.md`

## Key Decisions Captured

- `Now` should prefer direct cards only for urgent items; non-urgent items should collapse into badges, counts, or deep links
- `reflow` should first surface as a compact `Day changed` preview with `Accept` and `Edit`
- `Edit` on `reflow` should escalate into `Threads`
- urgency, importance, blocking state, and disruption level remain separate action-model axes
- blocking `check_in` items stay pinned, but allow bypass with warning plus operator note
- Apple remains summary-first even while eventual parity stays desirable

## Verification

- `rg -n "ADHD|Day changed|Edit|blocking|Projects|Stats|Apple|reflow|check_in|Threads" docs/product/operator-mode-policy.md docs/product/operator-surface-taxonomy.md docs/product/operator-action-taxonomy.md docs/product/now-inbox-threads-boundaries.md clients/apple/README.md`
- manual consistency pass across the policy, taxonomy, action-model, boundary docs, and Apple README

## Notes

- No UAT was performed, per operator instruction.
- This remains doc-first; no shell implementation was attempted in this slice.
