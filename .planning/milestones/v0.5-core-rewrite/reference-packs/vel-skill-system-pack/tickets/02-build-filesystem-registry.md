# Ticket: Build filesystem registry

## Objective

Implement skill discovery across bundled, global, user-local, and workspace-local paths.

## Deliverables

- registry search paths
- deduping and override resolution rules
- skill indexing metadata
- enable/disable state handling

## Acceptance criteria

- `vel skill list` shows discovered skills with source path and enabled state
- workspace-local overrides user/global versions
- disabled skills are hidden from normal execution unless forced
