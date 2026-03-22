# Vel Core Architecture Pack

This pack defines a full-fat architecture for Vel as a **typed, governed core platform** rather than a pile of integrations, prompts, and one-off features.

The central idea is simple:

- **Vel Core owns the canonical object model**
- **All reads/writes go through Vel Core tools/API**
- **Skills, workflows, tools, integrations, and modules are first-class governed objects**
- **External systems map into Vel’s superset schema instead of defining Vel’s ontology**

That means Todoist is not “the task system.” Calendar is not “the event system.” Chat is not “the thread system.”
They are sources, adapters, and sync partners.
Vel remains the semantic home of truth.

## Included docs

- `docs/01-core-architecture.md` — system-wide architecture and design principles
- `docs/02-core-object-model.md` — canonical object model and composition system
- `docs/03-template-system.md` — templates as composable type blueprints
- `docs/04-module-system.md` — extension/module system and package model
- `docs/05-core-tools-api.md` — CLI and tool-use API surface
- `docs/06-permissions-and-policy.md` — permission model, policy engine, grants, audit
- `docs/07-skills-and-workflows.md` — skills and workflows as governed executable objects
- `docs/08-sync-and-source-adapters.md` — sync, ownership, provenance, conflict resolution
- `docs/09-todoist-integration-spec.md` — concrete first integration spec
- `docs/10-development-phases.md` — MVP-first phased implementation roadmap
- `docs/11-data-storage-and-runtime.md` — suggested runtime/storage architecture
- `docs/12-ui-cli-and-observability.md` — UI/CLI/admin/debugging expectations
- `examples/` — YAML examples for object types, templates, modules, workflows, and adapters

## Executive summary

Vel should be built as a **semantic operating substrate** with:

1. **Canonical top-level object types** such as `Task`, `Project`, `Event`, `Message`, `Thread`, `Nudge`, `Person`, `Template`, `Skill`, `Tool`, `Workflow`, `Tag`, and `Config`.
2. **Composable traits/aspects** such as `Schedulable`, `Completable`, `Syncable`, `Taggable`, `Relational`, `Auditable`, and `Templated`.
3. **A core tools/API layer** that is the only legal surface for mutation and access.
4. **A policy and permissions layer** that enforces capability grants, field ownership, confirmation rules, and auditability.
5. **A module/extension system** that can register templates, workflows, tools, skills, adapters, UI extensions, and policies.
6. **A sync and adapter framework** that maps external systems into Vel’s canonical schema without letting them own it.
7. **A workflow/skill runtime** that executes governed behavior in the context of typed core objects.

## Guiding philosophy

Vel should not become an undifferentiated pile of possible actions. The architecture needs enough law to make power usable:

- **type before feature**
- **object before integration**
- **policy before mutation**
- **composition before subtype explosion**
- **adapters before ontology capture**
- **audit before automation bravado**

That is the difference between a durable platform and a very expensive gelatinous to-do goblin.
