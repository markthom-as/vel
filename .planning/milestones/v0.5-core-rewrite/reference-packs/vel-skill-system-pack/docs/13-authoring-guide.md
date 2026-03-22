# Skill Authoring Guide

## What skill authors should optimize for

When authors create a skill, they should think in this order:

1. what job does this skill perform
2. what inputs does it need
3. what outputs should it guarantee
4. what capabilities does it require
5. what should be deterministic vs model-driven
6. what tests prove it works

## Authoring checklist

- choose namespace and stable name
- write clear description and tags
- define input schema
- define output schema
- write prompt instructions if relevant
- declare capabilities conservatively
- define limits
- add at least one smoke test
- include example input/output fixtures

## Prompt guidance

Prompts should:

- reference structured context slots explicitly
- specify expected output format
- distinguish must-do rules from stylistic guidance
- avoid long redundant boilerplate
- degrade cleanly when context slices are absent

## Hook guidance

Hooks should:

- do deterministic work only where possible
- return typed JSON
- avoid hidden side effects
- log clearly to stderr
- fail loudly on malformed input

## Anti-patterns

### Bad: skill as random prompt blob
No schema, no policy, no tests, no guarantees.

### Bad: skill with hidden shell access
Nothing says “future incident report” like surprise subprocesses.

### Bad: skill with over-broad capabilities
Ask for exactly what is needed.

### Bad: giant context dump
Use typed slices and concise renderers.

## Recommended internal conventions

- namespaces are required
- semantic versioning for packages
- kebab-case skill names
- file references are relative to skill root
- every skill should be inspectable from CLI

## Recommendation

Skill authoring should feel like writing a small, governed software package — because that is what it is.
