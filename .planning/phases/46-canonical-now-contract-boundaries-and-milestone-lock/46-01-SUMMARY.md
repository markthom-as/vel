# 46-01 Summary

## Outcome

Locked the product-side authority docs for the canonical `Now` surface and reconciled them against the full local source contract at `/home/jove/Downloads/vel-now-surface-contract-codex-final.md`.

## What Changed

- `docs/product/now-surface-canonical-contract.md`
  - Added explicit count-display modes, urgency signal wording, blank one-liner failure rules, multi-thread linkage allowance, nudge reappearance/escalation wording, overrun posture, stronger empty-state prompts, docked-input multi-artifact continuity, closed intent taxonomy extensibility, agent confirmation rules, deterministic ranking language, CRDT/OT non-requirement, larger Mac/iPad metadata allowance, and version-control-like governed-config wording.
- `docs/product/now-inbox-threads-boundaries.md`
  - Reframed the surface boundary under the post-`v0.2` canonical `Now` contract.
  - Locked `Inbox` ownership of daily and carry-forward tasks.
  - Locked thread-first raw capture continuity and explicit `day thread` / `raw capture` lane ownership in `Threads`.
  - Removed stale open-question language that conflicted with Phase 46 decisions.
- `docs/product/mvp-operator-loop.md`
  - Tightened the post-`v0.2` note so it explicitly hands newer `Now` behavior to the stricter surface contract.

## Verification

- `rg -n "always_show|hidden_until_active|No dead states|thread-first|carry-forward|raw capture|pulse|glow|latest user input|version-control-like|day thread|What's going on right now\\?|Do you want to start something\\?|multiple threads" docs/product/now-surface-canonical-contract.md docs/product/now-inbox-threads-boundaries.md docs/product/mvp-operator-loop.md`

## Notes

- This slice stayed in product authority only. It did not change DTOs, service code, or client code.
