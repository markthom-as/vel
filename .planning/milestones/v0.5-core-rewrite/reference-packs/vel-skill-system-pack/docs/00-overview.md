# Vel Skill System: Overview

## Why this exists

Vel is accumulating a large and growing feature surface: task workflows, planning helpers, enrichment, automation, context retrieval, external integrations, project-aware behaviors, message handling, local machine actions, and eventually richer agentic behaviors across desktop, mobile, and cloud surfaces.

Without a coherent substrate, all of that becomes one of two things:

1. a giant pile of hardcoded special cases in the assistant prompt and backend orchestration, or
2. a plugin swamp with ambient trust and no stable authoring model.

Both options are bad. One is brittle. The other is cursed.

A skill system provides the missing symbolic law for this abundance. It turns “random capabilities Vel might do” into governable execution units with names, interfaces, permissions, policies, and lifecycle hooks.

## What a skill is

A skill is a **versioned, typed, permissioned execution package** that captures a reusable applied behavior.

Examples:

- morning brief
- standup generation
- calendar reconciliation
- metadata enrichment
- project triage
- inbox summarization
- task creation from voice notes
- repo-aware daily engineering summary

A skill is **not** just a tool, and it is **not** equivalent to an agent.

## Thesis

Vel should adopt a **Vel-native skill intermediate representation** and runtime first, then implement Claude/Codex-style compatibility through adapters later.

This allows:

- internal coherence before ecosystem cosplay
- stable CLI integration
- strict permission mediation
- typed composition between skills
- project-local and user-local extensibility
- future remote registry / community ecosystem without early lock-in

## Design stance

### The architecture should separate:

- **tools** — primitive capabilities like “read calendar,” “query Todoist,” “run shell,” “read file,” “execute local script”
- **skills** — reusable applied behaviors that use tools and context to perform a job
- **agents** — planners/orchestrators that decide when and how to invoke tools and skills
- **workflows** — declarative or semi-declarative composition of skills into multi-step execution chains

That separation is critical.

If tools and skills are collapsed into one concept, the whole thing degrades into spaghetti with a plugin API.

## What success looks like

A mature version of this system lets Vel:

- discover internal and local skill packs from filesystem registries
- validate them against stable schemas
- expose them as first-class CLI commands
- mediate capabilities and confirmation policies
- mount structured context into skill execution
- execute prompt-only, code-backed, or hybrid skills
- compose skills into workflows
- emit logs, artifacts, telemetry, and audit traces
- selectively enable or disable skills by workspace, user, runtime policy, or device profile
- later import/export or partially emulate external skill formats

## Primary recommendation

Start with a **filesystem-native internal skill system** and build only enough compatibility later to avoid future regret.

Do not begin by optimizing for another vendor’s format. That is how your internal architecture becomes a hostage situation.
