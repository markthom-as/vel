## 42-01 Summary

Tightened the durable same-day reflow contract and made the transport-level review/escalation semantics explicit in web decoding coverage.

### What changed

- Updated [day-plan-reflow-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md) to describe the actual `v0.2` typed reflow lane:
  - `ReflowCard` as the live proposal/review surface
  - `CurrentContextReflowStatus` as the durable applied or escalated state
  - explicit `moved / unscheduled / needs_judgment` outcome vocabulary
  - explicit review-gating and `Threads` escalation posture
- Updated [mvp-loop-contracts.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md) so the MVP contract now describes reflow proposal state in terms of the shipped outcome vocabulary plus review gating and escalation, rather than older placeholder wording.
- Strengthened [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the consolidated `Now` decoder test now asserts:
  - `needs_judgment_count`
  - the bounded transition targets `apply_suggestion` and `threads`
  - confirm-required review gating for the inline accept path
  - durable reflow thread continuity via `reflow_status.thread_id`

### Verification

- `npm --prefix clients/web test -- --run src/types.test.ts`

### Outcome

Phase 42 now starts from one explicit contract for same-day reflow proposal state, escalation, and supervision, with web transport tests asserting those semantics directly instead of relying on implied fixture shape.
