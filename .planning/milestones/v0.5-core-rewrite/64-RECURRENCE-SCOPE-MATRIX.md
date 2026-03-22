# Phase 64 Recurrence Scope Matrix

## Purpose

Make Google recurrence fidelity and write support concrete enough that the adapter cannot imply support it does not actually have.

## Scope Matrix

| Operation | Canonical Support In `0.5` | Google Mapping Support | Write Supported In `0.5` | Dry-Run Supported | Explainable | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| single non-recurring event update | yes | yes | yes | yes | yes | conservative supported field set only |
| recurring series update | yes | yes | yes, bounded | yes | yes | explicit supported fields only |
| single occurrence override | yes | yes | yes, bounded | yes | yes | maps through exception/override semantics |
| series-wide title/location/time update | yes | yes | yes, bounded | yes | yes | only where canonical and provider semantics align |
| delete single occurrence | yes | partial/guarded | guarded | yes | yes | must preserve tombstone/reconciliation clarity |
| delete whole series | yes | yes, guarded | guarded | yes | yes | broad destructive mutation remains conservative |
| this and following | deferred | not required | no by default | maybe as refusal/explain | yes | reserve seam, do not pretend support |

## Fidelity Rule

Recurrence fidelity in Phase 64 means:

- canonical series object preserved
- derived/materialized occurrences supported
- exceptions/overrides preserved where needed
- provider recurrence metadata retained as facet where the native model does not fully absorb it
- no silent flattening into one-off events
- no fake round-trip promises for unsupported scopes
