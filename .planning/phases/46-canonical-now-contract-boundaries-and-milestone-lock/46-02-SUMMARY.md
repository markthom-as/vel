# 46-02 Summary

## Outcome

Locked the architecture-side authority docs so the canonical `Now` contract is explicitly Rust-owned across the full local source contract, including the support seams that were previously only implied.

## What Changed

- `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md`
  - Added explicit ownership for count-display policy, urgency-trigger inputs, neutral fallback behavior, future ordering fields, intent taxonomy, multi-artifact linkage, canonical `day thread` / `raw capture` continuity lanes, metadata filters, failed-action retry posture, latest-input-versus-merge rules, deterministic ranking, approval policy, and governed config for `Now` and watch behavior.
  - Expanded anti-patterns to ban shell-local approval and governed-config semantics for `Now`.
- `docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md`
  - Clarified that the older MVP loop contracts must stay aligned with the stricter post-`v0.2` `Now` contract instead of acting as a competing authority.
  - Tightened `ThreadEscalation` wording so filtered thread categories and metadata filters are part of the preserved continuation contract.
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md`
  - Explicitly kept `Now` config and approval posture in the same Rust-owned product-core lane.
  - Made shells consumers of the post-`v0.2` canonical `Now` contract for title, count-display, ranking, continuity, sync, and approval semantics.

## Verification

- `rg -n "count-display|intent taxonomy|approval|deterministic-enough|raw capture|watch-safe|metadata filters|config-mutation|local approval" docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md`

## Notes

- This slice stayed at the architecture/authority layer. DTOs, service code, and client adapters are deferred to later phases.
