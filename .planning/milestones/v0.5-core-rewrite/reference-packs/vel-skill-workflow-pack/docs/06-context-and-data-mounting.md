# Context and Data Mounting

## The problem

Most skill-like systems die of prompt obesity.

Everything gets stuffed into one giant context blob, and then everyone pretends this is architecture. It is not architecture. It is a buffet plate collapsing under too much gravy.

Vel should instead mount context into skills through explicit typed slices.

## Recommended context buckets

A skill execution context should be structured roughly like this:

```json
{
  "user": {},
  "workspace": {},
  "session": {},
  "time": {},
  "thread": {},
  "calendar": {},
  "tasks": {},
  "projects": {},
  "memory": {},
  "artifacts": {},
  "policy": {},
  "inputs": {}
}
```

Not every skill gets every bucket.

## Context mounting rules

The runtime should decide:

- which buckets are available
- whether data is embedded or referenced
- how much history is included
- what has been redacted
- what freshness guarantees apply
- what the token budget is

The skill should request needed context slices; the runtime should grant only what is allowed.

## Context references vs embeddings

For larger context surfaces, prefer a mixed approach:

- small, high-value facts embedded directly
- larger datasets referenced as handles or summarized forms
- raw records retrievable through tool access if granted

This keeps prompts from turning into symbolic lasagna.

## Suggested context classes

### Stable identity context
User preferences, workspace defaults, project metadata.

### Temporal context
Now, today, timezone, scheduling windows, deadlines, active commitments.

### Thread/session context
Current thread, recent messages, active artifact references, current UI selection.

### Domain context
Calendar items, tasks, projects, integrations, recent notes, repo state.

### Policy context
What the skill may do in this run.

## Prompt construction discipline

Prompt templates should not blindly dump every context bucket.

Instead they should include:

- a minimal role/instruction section
- explicit slot references
- a rendering strategy for each context slice
- a model-specific compact mode if needed

## Example strategy

For a morning brief skill:

- embed today’s top 6 events
- embed must-do tasks
- embed active project commitments
- summarize recent unresolved nudges
- reference extended task/project corpus through handles or tool access

## Context provenance

Every mounted context region should be traceable to:

- source
- timestamp
- transformation stage
- freshness level

This matters for debugging and trust.

## Recommendation

Build a dedicated context-mounting layer early, even if simple. If you leave it implicit, every new skill will invent its own little theology of what “relevant context” means.
