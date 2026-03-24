# Phase 82 — `Now` UI Spec

## Purpose

Define the page-level contract for `Now` as a strictly bounded, execution-first control surface.

## Surface Role

`Now` answers:

- what is active
- what is next
- what is interrupting
- whether the operator can trust the current moment

It does not act as:

- inbox
- backlog
- project browser
- chat history
- today dashboard

## Page Structure

1. Top orientation band
   - time
   - active context/client
   - breadcrumb only when a focused subview exists

2. Main bounded lane
   - dominant active task row
   - one subordinate next-up slot
   - current event
   - next event

3. Nudge lane
   - dedicated interruption lane
   - separate from work rows

4. Trust lane
   - only appears when degraded or critical

5. Docked action bar
   - voice
   - capture
   - ask
   - command
   - one contextual quick slot

## Allowed Content

- task
- event
- nudge
- trust/system state

## Forbidden Content

- message rows
- thread rows
- artifacts
- run/log listings
- people lists
- config
- raw integration state
- project grouping
- long task queues

## Visual Hierarchy

- active task is the visual anchor
- event is a constraint, not the dominant object
- nudge lane is visually distinct from work lanes
- projects appear only as tags/metadata

## Disclosure Map

### Inline

- complete / defer / confirm / reject task actions
- nudge acknowledge / dismiss / defer
- quick capture
- at most a one-line note/tweak on the active item

### Drawer

- shallow event inspection
- shallow trust explanation
- minimal object preview

### Escalate To `Threads`

- discussion
- reasoning
- evidence
- multi-object context
- ambiguity

### Escalate To `System`

- sync/auth/integration problems
- deeper trust/system detail
- preferences/configuration

## Empty / Degraded / Completed Behavior

- completed items disappear immediately after acknowledgment
- a transient ghost acknowledgment is allowed, but no completion history list
- trust/status band appears only when degraded or critical
- no chronic background system anxiety strip

## Banned Patterns

- inbox-like queue expansion
- project-grouped sections
- scroll-heavy day view
- mixing nudges inline with work rows
- giant expandable active task panels
- dashboard summary card stacks
