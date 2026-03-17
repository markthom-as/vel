---
title: Add project workspace DTOs and storage queries
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Create the shared read contract for project index and project workspace so web and CLI consume the same projection.

## Scope
- storage queries for project list/detail
- summarized task counts by project
- recent activity timestamps
- typed DTOs for project workspace payloads

## Requirements
- workspace contract must be stable enough for both web and CLI
- do not require the frontend to infer everything from raw commitments/sessions rows
- summary should include at least:
  - open task count
  - overdue count
  - due soon count
  - active session count
  - last activity at
- task payload should normalize tags from commitment metadata

## DTOs
Add at least:
- `ProjectIndexItemData`
- `ProjectWorkspaceData`
- `ProjectTaskData`
- `ProjectActivityEventData`

## Suggested behavior
Use a dedicated service-layer projector that hydrates from:
- `projects`
- `commitments`
- `assistant_transcripts` evidence where helpful
- runs/artifacts/events where needed for activity summary

## Tests
- project with no tasks
- project with mixed open/done/overdue commitments
- project with normalized tags from metadata
- stable ordering of project list and workspace sections

## Done when
- typed contracts exist in `vel-api-types`
- storage queries/projection helpers exist
- web/CLI can target a single workspace payload
