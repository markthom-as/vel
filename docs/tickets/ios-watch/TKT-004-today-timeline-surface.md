---
id: TKT-004
status: proposed
title: Implement Today timeline optimized for ADHD-friendly glanceability
priority: P1
estimate: 3-5 days
depends_on: [TKT-003]
owner: agent
---

## Goal

Build the primary Today surface: a chronological, interrupt-aware list of what matters now, next, and soon.

## Product requirements

The screen should answer, at a glance:

- What am I supposed to do now?
- What is approaching that will become painful if ignored?
- What can I dismiss quickly without opening six screens like I’m filing taxes?

## Scope

Timeline sections:

- Now
- Next few hours
- Later today
- Missed / unresolved

Row types:

- task/routine due
- meds due
- meeting prep reminder
- suggestion/nudge
- high-risk unresolved item

Actions per row:

- complete
- snooze
- skip
- open detail

## Implementation notes

- Use large tap targets and legible hierarchy
- Color/severity treatment should come from risk state, not arbitrary UI opinion
- Minimize modal depth
- Add optional “focus mode” that shows only actionable items

## Acceptance criteria

- Timeline renders mixed item types from fixtures
- User can complete or snooze items inline
- Severity/risk badges are visible but not clownishly loud
