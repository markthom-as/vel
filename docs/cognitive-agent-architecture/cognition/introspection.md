# Introspective / Self-Improvement Mode

Vel's introspection must improve policy, not mutate personality into mush.

For repository visibility, self-model structure, and supervised code-change rules, see [self-awareness-and-supervised-self-modification.md](self-awareness-and-supervised-self-modification.md).

## Purpose

Evaluate whether Vel is helping or harming.

## Reflection Inputs

- ignored suggestions
- accepted suggestions
- missed commitments
- reminder response latency
- false-positive risk alerts
- user overrides
- explicit user corrections

## Reflection Cadence

Use bounded reflection cycles:

- lightweight: daily
- policy review: weekly
- architecture review: manual / developer initiated

## Output Types

- threshold adjustment proposals
- explanation-style recommendations
- integration health findings
- failure mode reports
- candidate experiments

## Hard Rules

- reflection cannot directly ship policy changes to production without review gate
- user-facing tone/personality must not drift automatically
- reflection may propose changes, not self-authorize broad UX mutation
- every policy change must be reversible
- repo-aware introspection may inspect code, config, tickets, and docs, but write capability must remain explicitly scoped and supervised

## Example

```text
Finding:
Medication reminders sent 10 minutes before meetings are confirmed too late.

Proposal:
Increase default lead time for pre-meeting medication reminders to 25 minutes.

Confidence:
0.82
```
