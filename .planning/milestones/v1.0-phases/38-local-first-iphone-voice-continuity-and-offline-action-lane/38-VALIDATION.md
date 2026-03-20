# 38 Validation

## What must be proven

- iPhone can acknowledge voice capture immediately even when the daemon is unavailable
- cached `Now`, queued quick actions, and local thread draft continuation all behave as one local-first lane
- queued offline work later merges back into canonical thread and `Now` continuity without duplicate or confusing state
- Phase 38 remains bounded: heavy recall, heavy reasoning, integrations, and broader sync authority stay daemon-backed

## Evidence expectations

- focused Apple build/package verification for the embedded/local-first slice
- targeted tests or fixture-backed checks for queue/draft merge behavior
- doc updates that explain what is local-first now and what still requires the daemon
- explicit mention of the remaining daemon-backed limits in closeout docs

## Out of scope for this phase

- full local heavy recall
- general local planner or review/apply parity
- watch/mac embedded rollout
- a second voice-only continuity model
