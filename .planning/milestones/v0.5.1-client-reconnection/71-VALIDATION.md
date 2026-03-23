# Phase 71 Validation

- [x] deprecated routes are removed or named as quarantined
- [x] server endpoints with no remaining live callers are deleted in-scope or explicitly quarantined with rationale
- [x] web proof set is fully defined and executed
- [x] stale-data posture is proven explicitly with a controlled fixture
- [x] browser proof artifacts include both automated execution evidence and short notes
- [x] no-silent-fallback proof is browser-visible rather than component-only
- [x] evidence artifacts match the claim layer: artifact, state/log, or browser-visible behavior
- [x] Apple handoff/spec packet exists
- [x] `/v1/agent/inspect` and `/api/integrations/connections` are documented as intentional bounded reads

## Current Evidence

- browser proof commands now exist in `clients/web/package.json`
- evidence artifacts live under [71-evidence](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/71-evidence)
- Apple handoff packet exists at [71-APPLE-HANDOFF.md](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/71-APPLE-HANDOFF.md)

## Accepted Debt

- live browser workflow dispatch proof is deferred because no shipped canonical workflow-invocation HTTP route exists in the frozen `v0.5.1` boundary
- invocation gating and eligibility are proven in-browser
- follow-on requirement is recorded in [71-DEFERRED-WORK.md](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/71-DEFERRED-WORK.md)
