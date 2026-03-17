---
id: VEL-PROJ-008
title: Add Projects navigation and page shell to the web client
status: proposed
priority: P1
estimate: 2-3 days
dependencies:
  - VEL-PROJ-005
labels:
  - web
  - navigation
  - ui
---

# Goal

Add the top-level Projects entry and the initial web page shell that loads project list and project workspace data.

# Scope

- Update `clients/web/src/App.tsx` to support a `Projects` surface alongside `Now`, `Inbox`, and `Threads`.
- Add project query resources in `clients/web/src/data/resources.ts`.
- Add typed decoders in `clients/web/src/types.ts`.
- Create a top-level Projects page shell component with:
  - project list pane
  - selected project summary header
  - loading/error/empty states

# Deliverables

- `ProjectsPage.tsx` or equivalent page shell
- nav wiring in `App.tsx`
- resource/query hooks for projects APIs
- basic tests for navigation and page load

# Acceptance criteria

- Projects appears in web navigation.
- Selecting Projects loads project list data.
- A selected project’s workspace can be fetched and displayed in shell form.
- Empty/error states are sane and non-embarrassing.

# Notes for agent

Do not stuff this into `Now`. The whole point is a distinct workbench surface.
