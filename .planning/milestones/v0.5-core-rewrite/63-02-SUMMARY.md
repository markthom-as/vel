# 63-02 Summary

## Completed

Implemented canonical Todoist task, project, tag, and attached-comment mapping for the `0.5` task-side proving adapter.

### Added

- `crates/vel-adapters-todoist/src/task_mapping.rs`
  - canonical-first Todoist task mapping
  - first-class `task_type`
  - canonical status / priority / due posture
  - raw tags preserved alongside interpreted `task_semantics`
- `crates/vel-adapters-todoist/src/project_mapping.rs`
  - canonical Todoist project mapping
  - deterministic canonical `ProjectId` from account + remote project identity
  - sections preserved as non-first-class provider facet metadata
- `crates/vel-adapters-todoist/src/comment_records.rs`
  - shared `AttachedCommentRecord`-style mapping for Todoist comments
  - comments remain attached metadata, not canonical messages
- `crates/veld/tests/phase63_task_mapping.rs`
  - black-box proof for task/project/tag/comment mapping posture

### Updated

- `crates/vel-adapters-todoist/src/backlog_import.rs`
  - now reuses canonical task mapping instead of maintaining a parallel import-only task shape
- `crates/vel-adapters-todoist/src/lib.rs`
  - exports new task/project/comment mapping surfaces

## Verification

Passed:

- `rg -n "Task|status|due|priority|task_type|task_semantics|Tag|Project|section|non-first-class|provider facet|AttachedCommentRecord|author|timestamp|body" crates/vel-adapters-todoist/src/task_mapping.rs crates/vel-adapters-todoist/src/project_mapping.rs crates/vel-adapters-todoist/src/comment_records.rs crates/veld/tests/phase63_task_mapping.rs`
- `cargo test -p vel-adapters-todoist --lib`
- `cargo test -p veld --test phase63_task_mapping`
- `cargo check -p vel-adapters-todoist`
- `cargo check -p veld`

## Outcome

Phase 63 now proves that Todoist task-side semantics map into Vel-owned canonical objects instead of dictating them:

- projects are first-class canonical content objects
- sections remain provider-facet only
- labels remain raw canonical tags while also driving structured `task_semantics`
- `task_type` is first-class
- comments remain attached records instead of being forced into thread/message ontology
