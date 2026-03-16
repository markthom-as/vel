# Ticket 006 — Context Scoping

## Purpose
Prevent Navs from receiving excessive state.

## Deliverables
Context tiers:
- GlobalContext
- SessionContext
- TaskContext

Context builder:
ContextBuilder::for_task()

## Acceptance Criteria
Nav execution receives only required context.

