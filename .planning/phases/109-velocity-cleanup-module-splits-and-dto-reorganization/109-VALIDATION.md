# Phase 109 Validation

## Required Checks

### Structure

- `app.rs` is visibly smaller and delegates grouped responsibilities to submodules
- `now.rs` public API remains stable while internal responsibilities are split
- `vel-api-types/src/lib.rs` becomes a re-export index rather than the DTO blob itself

### Automated

- targeted `veld` route tests
- targeted `Now` service tests
- compile/test checks that prove `vel-api-types` re-exports did not break existing consumers

### Manual Review

- inspect the new module names for obvious ownership clarity
- confirm that no hidden behavior changes were bundled into the file moves

## Failure Conditions

- large-file cleanup causes broad consumer churn without strong justification
- the split replaces one giant file with many poorly named files
- behavior changes are mixed into the reorganization without clear necessity
