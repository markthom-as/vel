---
title: Implement project routes and workspace projection service
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Expose read APIs for projects and a service-layer projector that returns the full workspace payload.

## Scope
- add `routes/projects.rs`
- wire routes into app router
- add service helpers for index + detail/workspace

## Routes
- `GET /v1/projects`
- `POST /v1/projects`
- `GET /v1/projects/:slug`
- `PATCH /v1/projects/:slug`
- `GET /v1/projects/:slug/workspace`

## Requirements
- route handlers stay thin
- service layer owns aggregation logic
- use repo-consistent `ApiResponse<T>` wrappers
- return clear 404 when project slug is unknown
- preserve future ability to mirror under `/api` if desired, but do not duplicate logic now

## Tests
- list empty
- create/get/patch project
- workspace returns expected summary and tasks for seeded project
- unknown project 404

## Done when
- project routes are mounted
- workspace projection is reachable over HTTP
- tests cover the core read path
