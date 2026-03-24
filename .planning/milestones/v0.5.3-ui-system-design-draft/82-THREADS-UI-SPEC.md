# Phase 82 — `Threads` UI Spec

## Purpose

Define the page-level contract for `Threads` as the continuity and depth surface.

## Surface Role

`Threads` answers:

- what this is about right now
- what object/context anchors the thread
- what changed
- what evidence or provenance matters
- what next decision or response is appropriate

It does not act as:

- generic detached chat
- inbox replacement
- system admin surface

## Page Structure

1. Top orientation band
   - current thread context
   - breadcrumb only for focused states

2. Thread header/context shelf
   - object/context first
   - chronology secondary

3. Continuity stream
   - messages
   - object cards
   - nudges
   - run/action summaries
   - bounded config blocks

4. Shared review/detail surface
   - provenance
   - action traces
   - run results
   - logs

5. Docked action bar
   - same shell law as the other surfaces

## Ordering

- hybrid ordering: recency first, with relevance and pinned context influence
- default open state is the continuity stream

## Visual Hierarchy

- bound object/context leads
- messages are part of continuity, not the dominant identity
- run/action blocks are more structured and system-like than message blocks

## Disclosure Map

### Inline Expansion

- messages
- object cards
- nudges
- run/action summaries
- bounded config blocks

### Focus Mode

- media
- artifacts
- logs
- runs
- utility blocks
- richer config detail

### Inline Editing

- bounded config
- some object fields
- lightweight metadata edits
- message drafting

### Shared Review / Detail Surface

- provenance
- run results
- action traces
- logs

## Sticky Rules

- filters are sticky per thread
- provenance is collapsed by default
- project identity may shape context, but thread identity remains its own thing

## Banned Patterns

- chat-first layout that buries object context
- provider-colored chat bubbles dominating the surface
- provenance pasted inline on every block
- `System`-style integration/config browsers embedded into the thread
- mixed giant catch-all cards
