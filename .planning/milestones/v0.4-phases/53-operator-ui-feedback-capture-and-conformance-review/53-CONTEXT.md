# Phase 53 Context

## Phase

**53: Operator UI feedback capture and conformance review**

## Objective

Capture the operator review of the implemented web conformance slice, distinguish true must-fix polish from optional ideas, and establish the exact bounded authority for Phase 54.

## Review Authority

Authority order for this phase:

1. latest operator review notes from 2026-03-21
2. prior operator correction memo and follow-up clarifications
3. implemented Phase 52 web/reference behavior

## Accepted Operator Findings

### Navbar / Shell

- `Vel` should be title case, largest text in the navbar, unboxed, and use a lighter orange accent that complements the existing gray palette
- the area to the right of `Vel` should show time/date plus a grayed current-task summary
- page title should include time/date context plus notification count
- center nav cluster should show color-coded icons/counts for:
  - nudges
  - unread threads
  - sync
- `Documentation` should become the top-level info button rather than a labeled nav destination
- top-nav links should be smaller, include section icons, use underline active state, and avoid visible pill containers

### Right Sidebar

- current panel behavior is broken and can extend beyond viewport width/height
- icon treatment should simplify to a chevron collapse panel affordance
- context lane should include contextual documentation for the current app view, placeholder content acceptable for now

### Now

- time/context should move out of the top area in favor of the nav changes
- `No trustworthy backup` should stand alone as its own nudge
- nudge chips/actions need smaller rounded-outline styling, better icons, improved project-tag treatment, subtle timestamps, and urgency-weighted hierarchy
- highest-priority nudge should carry more elevation and urgency treatment
- tasks need:
  - `Tasks` header one level below the `Now` title
  - subsection styling promoted from the current container labels
  - right-side summary values for completed, remaining, backlog
  - active/current task urgency styling comparable to top-priority nudge treatment
  - icons, project color treatment, inline reschedule controls, and Todoist metadata carry-through

### Floating Composer

- composer should resemble the ChatGPT floating pill pattern
- microphone stays visible; send icon appears only when input is non-empty
- no scroll arrows in the text area
- composer should use brand color/styling to read as a high-priority Vel surface
- composer must remain centered under the `Now` content even when the sidebar opens

### Threads

- top header should match `Now` heading scale/style
- left sidebar should include toggleable filters
- search bar should include an integrated right-aligned magnifier icon
- thread rows should convey start date and last message
- bottom of the thread sidebar should show a centered subtle summary line

### Settings

- current compact rewrite regressed functional change capability versus the previous version
- minimum previous functional settings behavior must be restored
- left rail should use smaller centered labels, no pills, icon-only back affordance
- accepted category grouping for the next pass:
  - `Profile`
  - `Clients & Sync`
  - `Integrations`
  - `Configuration`
  - `Backups`
  - `Permissions & Policies`
  - `Templates`
  - `Projects`
- settings changes should autosave

## Scope Guardrail Interpretation

- accepted review items remain in scope because they are direct conformance or MVP-usability fixes to the corrected surfaces
- `review/follow-up` notification buckets were collapsed into the existing `nudges` concept
- Todoist metadata/reschedule work is in scope if required to make the task surface functionally credible
- contextual documentation in the sidebar may ship as placeholder content for the current app view

## Resulting Expectation For Phase 54

Phase 54 must implement the accepted review findings without widening into new product surfaces or unrelated architecture work.
