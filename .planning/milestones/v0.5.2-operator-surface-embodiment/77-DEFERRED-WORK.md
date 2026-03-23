# Phase 77 Deferred Work

## Accepted Debt

### Browser-Proven Live Workflow Invocation Transport

Status: carried forward as accepted debt at `v0.5.2` closeout

Reason:

- `v0.5.2` embodied the frozen `0.5` / `0.5.1` backend-client boundary rather than widening it
- no shipped canonical workflow-invocation HTTP route exists in the embodied web surface
- adding one in this milestone would have reopened backend law for the sake of proof shape rather than shipped scope

What `v0.5.2` now proves:

- `Threads` invocation eligibility and gating remain explicit
- bound-object-first reading posture is browser-proven
- the embodied operator loop crosses `Now`, `Threads`, and `System`
- cross-surface navigation and one canonical `Now` mutation are browser-proven

What is still not proven:

- browser-visible live workflow dispatch
- browser-visible workflow result handling
- browser-visible workflow failure/degraded dispatch handling

Follow-up requirement:

- the first milestone that introduces canonical workflow invocation transport must also add full browser proof for invocation dispatch, result, and degraded/failure handling
