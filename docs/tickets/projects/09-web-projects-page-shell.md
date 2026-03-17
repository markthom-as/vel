---
title: Build web Projects page shell and navigation
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Add a first-class Projects page to the existing web shell.

## Scope
- add `Projects` navigation to main shell
- create `ProjectsPage.tsx`
- implement project list rail and project workspace container
- use shared query layer instead of bespoke component-local fetch spaghetti

## Requirements
- preserve existing Now / Inbox / Threads / Settings flows
- allow selecting a project from list and deep-linking if router state exists
- handle empty, loading, and error states well
- surface quick actions: add task, queue message, sync if appropriate

## Suggested component structure
```text
clients/web/src/components/ProjectsPage.tsx
clients/web/src/components/projects/ProjectList.tsx
clients/web/src/components/projects/ProjectWorkspace.tsx
clients/web/src/components/projects/ProjectSummaryCard.tsx
```

## UX notes
- left rail: projects
- center: workspace tabs/panes
- optional right drawer: activity/settings
- dense but legible, more operator cockpit than consumer app

## Tests
- navigation into Projects
- empty project list state
- project selection renders workspace sections

## Done when
- Projects is a visible first-class page
- it renders from real backend data
- page shell is test-covered
