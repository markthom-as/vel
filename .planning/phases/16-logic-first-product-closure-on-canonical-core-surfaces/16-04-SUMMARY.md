# 16-04 Summary

Completed the trust/readiness follow-through slice by turning degraded posture from a summary-only signal into canonical backend-owned recovery and review actions that shells can render directly.

## What changed

- Extended the canonical operator action vocabulary in [operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) and [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with a typed `recovery` action kind, then widened the web transport boundary in [types.ts](/home/jove/code/vel/clients/web/src/types.ts).
- Reworked [operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) so backup-trust degradation now emits a ranked canonical recovery action with typed provenance instead of only contributing warning text.
- Widened [now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs), [now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs), [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs), [execution_context.rs](/home/jove/code/vel/crates/veld/src/services/execution_context.rs), [types.ts](/home/jove/code/vel/clients/web/src/types.ts), and [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so `trust_readiness` now carries a typed `follow_through` action list sourced from the canonical queue.
- Tightened [doctor.rs](/home/jove/code/vel/crates/veld/src/services/doctor.rs) to reuse the same backup guidance language more explicitly, so degraded trust posture presents one consistent recovery direction across surfaces.
- Updated [onboarding-and-trust-journeys.md](/home/jove/code/vel/docs/product/onboarding-and-trust-journeys.md) and [operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) to record the rule that summary-first trust/readiness must expose backend-owned follow-through actions instead of forcing shells to synthesize them.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo test -p veld trust_readiness -- --nocapture`
- `cargo test -p veld action_items_rank_freshness_linking_intervention_and_review_bands -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`

## Why this matters

Phase 15 and early Phase 16 work established trust/readiness as a summary projection, but degraded posture still stopped at warnings and counts. This slice makes the follow-through canonical: the backend now decides which recovery and review actions matter, preserves typed evidence for those actions, and lets `Now` stay summary-first without turning shells into policy engines.
