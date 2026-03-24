# System Surface

This page explains the web `System` surface as it exists for the current operator milestone.

Use `System` when you need:

- Core setup required before Vel can operate normally,
- capability and integration status,
- sync and trust inspection,
- configuration and preferences,
- repair and recovery actions,
- a denser structural view than `Now` or `Threads`.

`System` is not a general dashboard and it is not a second inbox.

## What belongs here

- top-level `Core settings` for required setup and host identity,
- integration connection status and configuration,
- `LLM routing` controls for handshake and localhost OpenAI OAuth proxy launch,
- sync health and recovery affordances,
- preferences and accessibility settings,
- object/control inspection,
- backup and recovery related operator controls.

## What does not belong here

- daily task triage,
- open-ended thread continuation,
- generic helper copy that hides the actual controls,
- fake settings that do not persist.

## Operator posture

- keep required setup at the top so incomplete Core identity is obvious,
- prefer direct, inspectable rows over decorative panels,
- keep status summary near the top of each section,
- keep config fields colocated with the thing they affect,
- treat trust and sync issues as actionable state, not ambient decoration.

For broader surface definitions, see [Surfaces](surfaces.md). For shipped-vs-planned truth, see [MASTER_PLAN.md](../MASTER_PLAN.md).
