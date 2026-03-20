# 13-02 Summary

## Outcome

Completed the cross-surface contract-vocabulary slice for Phase 13:

- published one canonical vocabulary doc for commands, queries, events, read models, and transport DTOs
- made ownership rules explicit across `vel-core`, `veld`, `vel-api-types`, and shell adapters
- aligned runtime, Apple, and config authority docs to point to that vocabulary instead of leaving those boundaries implicit

This gives later migration and product phases a shared language for deciding what belongs in core semantics versus transport or shell embodiment.

## Implementation

### New contract-vocabulary authority

- added [docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md](../../../../docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md)
- the doc now defines:
  - commands
  - queries
  - events
  - read models
  - transport DTOs
  - ownership rules for `vel-core`, `veld`, `vel-api-types`, and shell adapters
  - anti-patterns such as screen-shaped contracts, shell-owned policy, DTO leakage into core, and generic JSON as the primary contract

### Authority alignment

- updated [docs/api/runtime.md](../../../../docs/api/runtime.md) to point runtime readers to the cross-surface contract vocabulary
- updated [clients/apple/README.md](../../../../clients/apple/README.md) to reference the new architecture and contract-vocabulary docs alongside the existing HTTP-first Apple boundary
- updated [config/README.md](../../../../config/README.md) so config/contract ownership rules now explicitly reference the cross-surface vocabulary owner doc

## Verification

Automated:

- `rg -n "Commands|Queries|Events|Read models|DTO|transport|Apple|HTTP" docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md docs/api/runtime.md clients/apple/README.md config/README.md`

Manual:

- read the vocabulary doc against the live daily-loop and agent-inspect seams to confirm it pushes future shells toward backend-owned contracts rather than screen-shaped APIs

## Notes

- this slice intentionally stayed documentation-first and did not change runtime behavior
- Phase 14 discovery progressed in parallel and now has both [14-RESEARCH.md](../../14-product-discovery-operator-modes-and-milestone-shaping/14-RESEARCH.md) and [14-CONTEXT.md](../../14-product-discovery-operator-modes-and-milestone-shaping/14-CONTEXT.md) started
