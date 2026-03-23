# Phase 71 Deferred Work

## Accepted Debt

### Browser-Proven Live Workflow Invocation Path

Status: accepted debt for `v0.5.1` closeout

Reason:

- `v0.5.1` froze the backend/client boundary
- no shipped canonical workflow-invocation HTTP route exists in the `v0.5.1` client surface
- adding one now would reopen frozen backend law only to satisfy proof shape

What is proven in `v0.5.1`:

- invocation eligibility and gating in `Threads`
- exactly-one-bound-object constraint
- explicit attach/create-object guidance when no bound object exists
- absence of floating or multi-object invocation controls

What is not proven in `v0.5.1`:

- browser-visible live workflow dispatch
- browser-visible workflow result handling
- browser-visible workflow failure/degraded dispatch handling

Non-action taken:

- no new invocation route was added in `v0.5.1`

Follow-up requirement:

- the first milestone that introduces canonical workflow invocation transport must also add full browser proof for invocation dispatch, result, and degraded/failure handling
