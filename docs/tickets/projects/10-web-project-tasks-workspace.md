---
title: Implement web project task workspace
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Ship the task-facing half of the Projects page.

## Scope
- task list grouped/filterable in project context
- task creation composer
- tag editing affordances
- quick status mutations
- explicit error states for Todoist write-through failures

## Requirements
- use project workspace/task DTOs, not raw commitment JSON hacking
- support tag chips/editing
- support grouping by status and due pressure at minimum
- show source badges (`todoist`, `manual`, etc.)
- show external write state when relevant

## Suggested components
```text
clients/web/src/components/projects/ProjectTasksPane.tsx
clients/web/src/components/projects/ProjectTaskComposer.tsx
clients/web/src/components/projects/ProjectTaskList.tsx
clients/web/src/components/projects/ProjectTaskRow.tsx
```

## Tests
- create task from page
- tag add/remove
- mark done/cancel
- remote failure state visible when mutation fails

## Done when
- task workflow is usable end-to-end from the Projects page
- Todoist-backed errors are honest and actionable
