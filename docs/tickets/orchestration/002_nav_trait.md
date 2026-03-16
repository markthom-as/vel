# Ticket 002 — Nav Trait

## Purpose
Provide a uniform execution interface for all Navs.

## Deliverables
Trait:

```rust
pub trait Nav {
    fn id(&self) -> &str;
    fn capabilities(&self) -> Vec<Capability>;
    fn estimate(&self, task: &Task, ctx: &VelContext) -> NavEstimate;
    fn execute(&self, task: &Task, ctx: &VelContext) -> NavResult;
}
```

## Acceptance Criteria
NavResult includes:
- status
- artifacts
- summary
- confidence
- warnings

