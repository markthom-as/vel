---
title: Normalize task tags and project task filters
status: ready
owner: agent
priority: P1
area: projects
---

# Goal
Make project task tagging a first-class operator feature without prematurely over-normalizing the storage model.

## Scope
- define tag read/write helpers against commitment metadata
- expose tags explicitly in DTOs
- add filter support in project workspace/task routes

## Requirements
- canonical near-term storage path: `metadata.tags: string[]`
- dedupe and normalize tags case-safely according to chosen rule
- preserve source-native label data when needed under metadata
- support filtering by tag, status, due state, source type, commitment kind

## Suggested API additions
- `POST /v1/projects/:slug/tasks/:id/tags`
- optional query params on workspace/tasks route for filtering

## UI/CLI implications
- web task composer and edit affordances can add/remove tags cleanly
- CLI supports repeated `--tag` flags

## Tests
- create/update tags
- duplicate tags collapse correctly
- workspace filtering returns expected subset

## Done when
- tags are explicit in contracts
- task filtering is implemented
- web/CLI can rely on normalized tags instead of raw metadata spelunking
